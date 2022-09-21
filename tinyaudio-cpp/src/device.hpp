#pragma once

#include "common.hpp"

#include <chrono>
#include <condition_variable>
#include <exception>
#include <functional>
#include <mutex>
#include <thread>

namespace tinyaudio
{
enum class DeviceType
{
    PLAYBACK = ma_device_type_playback,
    CAPTURE = ma_device_type_capture,
    DUPLEX = ma_device_type_duplex,
    LOOPBACK = ma_device_type_loopback
};

enum class DeviceState
{
    UNINITIALIZED = ma_device_state_uninitialized,
    STOPPED = ma_device_state_stopped,
    STARTED = ma_device_state_started,
    STARTING = ma_device_state_starting,
    STOPPING = ma_device_state_stopping,
};

class Device
{
public:
    using DataCallback = std::function<void(const void *input_frames, void *output_frames, size_t frame_count)>;

    Device(DeviceType device_type, Format format, size_t channels, size_t sample_rate, size_t frame_count)
    {
        auto config = ma_device_config_init(static_cast<ma_device_type>(device_type));

        config.sampleRate = sample_rate;
        config.periodSizeInFrames = frame_count;
        config.dataCallback = device_data_callback;
        config.pUserData = this;

        config.playback.format = static_cast<ma_format>(format);
        config.playback.channels = channels;

        config.capture.format = static_cast<ma_format>(format);
        config.capture.channels = channels;

        check_result(ma_device_init(nullptr, &config, &raw_device));
    }

    virtual ~Device()
    {
        stop();
        ma_device_uninit(&raw_device);
    }

    DeviceType get_device_type() const
    {
        return static_cast<DeviceType>(raw_device.type);
    }

    Format get_format() const
    {
        return get_device_type() == DeviceType::PLAYBACK ? static_cast<Format>(raw_device.playback.format)
                                                         : static_cast<Format>(raw_device.capture.format);
    }

    size_t get_channels() const
    {
        return get_device_type() == DeviceType::PLAYBACK ? raw_device.playback.channels : raw_device.capture.channels;
    }

    size_t get_sample_rate() const
    {
        return raw_device.sampleRate;
    }

    size_t get_frame_count() const
    {
        return get_device_type() == DeviceType::PLAYBACK ? raw_device.playback.intermediaryBufferCap
                                                         : raw_device.capture.intermediaryBufferCap;
    }

    DeviceState get_device_state() const
    {
        return static_cast<DeviceState>(ma_device_get_state(&raw_device));
    }

    bool is_started() const
    {
        return ma_device_is_started(&raw_device);
    }

    void start(const DataCallback &callback)
    {
        control_thread = std::thread(
            [this]()
            {
                std::unique_lock<std::mutex> lock(control_mutex);
                control_cv.wait(lock);
                check_result(ma_device_stop(&raw_device));
            }
        );

        data_callback = (DataCallback *)&callback;
        check_result(ma_device_start(&raw_device));
    }

    void stop()
    {
        {
            std::unique_lock<std::mutex> lock(control_mutex);
            control_cv.notify_all();
        }

        if (std::this_thread::get_id() == data_callback_thread_id)
        {
            return;
        }

        if (control_thread.joinable())
        {
            control_thread.join();
        }
    }

private:
    ma_device raw_device;
    DataCallback *data_callback = nullptr;

    std::thread::id data_callback_thread_id;
    std::thread control_thread;
    std::mutex control_mutex;
    std::condition_variable control_cv;

    static void device_data_callback(
        ma_device *raw_device,
        void *output_frames,
        const void *input_frames,
        ma_uint32 frame_count
    )
    {
        auto &device = *static_cast<Device *>(raw_device->pUserData);
        device.data_callback_thread_id = std::this_thread::get_id();
        (*device.data_callback)(input_frames, output_frames, frame_count);
    }
};

namespace tests::device
{
const Format FORMAT = Format::F32;
const size_t CHANNELS = 2;
const size_t SAMPLE_RATE = 44100;
const size_t FRAME_COUNT = 128;

TEST_CASE("[device] returns correct metadata")
{
    auto test = [&](DeviceType device_type)
    {
        Device device(device_type, FORMAT, CHANNELS, SAMPLE_RATE, FRAME_COUNT);

        REQUIRE_EQ(device.get_device_type(), device_type);
        REQUIRE_EQ(device.get_format(), FORMAT);
        REQUIRE_EQ(device.get_channels(), CHANNELS);
        REQUIRE_EQ(device.get_sample_rate(), SAMPLE_RATE);
        REQUIRE_EQ(device.get_frame_count(), FRAME_COUNT);
    };

    test(DeviceType::PLAYBACK);
    test(DeviceType::CAPTURE);
    test(DeviceType::DUPLEX);

#ifdef _WIN32
    test(DeviceType::LOOPBACK);
#endif
}

TEST_CASE("[device] starts and stops without error")
{
    auto test = [&](DeviceType device_type)
    {
        Device device(device_type, FORMAT, CHANNELS, SAMPLE_RATE, FRAME_COUNT);

        device.start(
            [&](auto input_frames, auto output_frames, auto frame_count)
            {
                switch (device.get_device_type())
                {
                case DeviceType::PLAYBACK:
                    REQUIRE_EQ(input_frames, nullptr);
                    REQUIRE_NE(output_frames, nullptr);
                    break;
                case DeviceType::CAPTURE:
                    REQUIRE_NE(input_frames, nullptr);
                    REQUIRE_EQ(output_frames, nullptr);
                    break;
                case DeviceType::DUPLEX:
                    REQUIRE_NE(input_frames, nullptr);
                    REQUIRE_NE(output_frames, nullptr);
                    break;
                case DeviceType::LOOPBACK:
                    REQUIRE_NE(input_frames, nullptr);
                    REQUIRE_EQ(output_frames, nullptr);
                    break;
                }
            }
        );

        std::this_thread::sleep_for(std::chrono::milliseconds(100));

        device.stop();
    };

    test(DeviceType::PLAYBACK);
    test(DeviceType::CAPTURE);
    test(DeviceType::DUPLEX);

#ifdef _WIN32
    test(DeviceType::LOOPBACK);
#endif
}

TEST_CASE("[device] can be stopped by calling stop() from data callback")
{
    Device device(DeviceType::PLAYBACK, FORMAT, CHANNELS, SAMPLE_RATE, FRAME_COUNT);

    auto stopped_by_callback = false;

    device.start(
        [&](auto input_frames, auto output_frames, auto frame_count)
        {
            stopped_by_callback = true;
            device.stop();
        }
    );

    std::this_thread::sleep_for(std::chrono::milliseconds(100));

    REQUIRE(stopped_by_callback);
}
} // namespace tests::device
} // namespace tinyaudio
