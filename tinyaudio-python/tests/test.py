import tinyaudio as ta
import unittest
import threading

FORMAT = ta.Format.F32
CHANNELS = 2
SAMPLE_RATE = 44100
FRAME_COUNT = 128
WAIT_TIMEOUT = 1000 / 1000.0

cv = threading.Condition()


def wait(timeout=WAIT_TIMEOUT):
    with cv:
        cv.wait(timeout)


def notify():
    with cv:
        cv.notify_all()


class TestTinyaudio(unittest.TestCase):
    def test_offline(self):
        audio = ta.Tinyaudio(
            True,
            FORMAT,
            CHANNELS,
            SAMPLE_RATE,
            FRAME_COUNT,
            "../audio-samples/2MB.wav",
            "test-tinyaudio-offline.wav",
            True,
        )

        self.assertTrue(audio.is_offline())
        self.assertEqual(audio.get_format(), FORMAT)
        self.assertEqual(audio.get_channels(), CHANNELS)
        self.assertEqual(audio.get_sample_rate(), SAMPLE_RATE)
        self.assertEqual(audio.get_frame_count(), FRAME_COUNT)
        self.assertFalse(audio.is_looping_input_file())
        self.assertFalse(audio.is_started())

        def data_callback(input_frames, output_frames):
            self.assertTrue(audio.is_started())
            self.assertEqual(len(input_frames), len(output_frames))
            output_frames[:] = input_frames

        def stop_callback():
            self.assertFalse(audio.is_started())

        audio.start(data_callback, stop_callback)

        self.assertFalse(audio.is_started())

    def test_online(self):
        audio = ta.Tinyaudio(
            False,
            FORMAT,
            CHANNELS,
            SAMPLE_RATE,
            FRAME_COUNT,
            "../audio-samples/2MB.wav",
            "test-tinyaudio-online.wav",
            False,
        )

        self.assertFalse(audio.is_offline())
        self.assertEqual(audio.get_format(), FORMAT)
        self.assertEqual(audio.get_channels(), CHANNELS)
        self.assertEqual(audio.get_sample_rate(), SAMPLE_RATE)
        self.assertEqual(audio.get_frame_count(), FRAME_COUNT)
        self.assertFalse(audio.is_looping_input_file())
        self.assertFalse(audio.is_started())

        def data_callback(input_frames, output_frames):
            self.assertTrue(audio.is_started())
            self.assertEqual(len(input_frames), len(output_frames))
            output_frames[:] = input_frames
            notify()

        def stop_callback():
            self.assertFalse(audio.is_started())

        audio.start(data_callback, stop_callback)
        self.assertTrue(audio.is_started())

        wait()

        audio.stop()
        self.assertFalse(audio.is_started())

    def test_get_audio_file_info(self):
        info = ta.get_audio_file_info("../audio-samples/1MB.wav")

        self.assertEqual(info.format, ta.Format.S16)
        self.assertEqual(info.channels, 2)
        self.assertEqual(info.sample_rate, 8000)
        self.assertEqual(info.total_frame_count, 268237)


if __name__ == "__main__":
    unittest.main()
