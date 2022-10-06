import tinyaudio as ta
import unittest
import threading
import os

FORMAT = ta.Format.F32
CHANNELS = 2
SAMPLE_RATE = 44100
FRAME_COUNT = 128
WAIT_TIMEOUT = 100 / 1000.0
PROJECT_DIR = os.path.abspath(os.path.dirname(__file__))

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
            offline=True,
            format=FORMAT,
            channels=CHANNELS,
            sample_rate=SAMPLE_RATE,
            frame_count=FRAME_COUNT,
            input_file_path=os.path.join(PROJECT_DIR, "../audio-samples/2MB.wav"),
            output_file_path=os.path.join(PROJECT_DIR, "test-tinyaudio-offline.wav"),
            looping_input_file=False,
        )

        self.assertTrue(audio.offline)
        self.assertEqual(audio.format, FORMAT)
        self.assertEqual(audio.channels, CHANNELS)
        self.assertEqual(audio.sample_rate, SAMPLE_RATE)
        self.assertEqual(audio.frame_count, FRAME_COUNT)
        self.assertFalse(audio.looping_input_file)
        self.assertFalse(audio.started)

        def data_callback(input_frames, output_frames):
            self.assertTrue(audio.started)
            self.assertEqual(len(input_frames), len(output_frames))
            output_frames[:] = input_frames

        def stop_callback():
            self.assertFalse(audio.started)

        audio.start(data_callback, stop_callback)

        self.assertFalse(audio.started)

    def test_online(self):
        audio = ta.Tinyaudio(
            offline=False,
            format=FORMAT,
            channels=CHANNELS,
            sample_rate=SAMPLE_RATE,
            frame_count=FRAME_COUNT,
            input_file_path=os.path.join(PROJECT_DIR, "../audio-samples/2MB.wav"),
            output_file_path=os.path.join(PROJECT_DIR, "test-tinyaudio-online.wav"),
            looping_input_file=False,
        )

        self.assertFalse(audio.offline)
        self.assertEqual(audio.format, FORMAT)
        self.assertEqual(audio.channels, CHANNELS)
        self.assertEqual(audio.sample_rate, SAMPLE_RATE)
        self.assertEqual(audio.frame_count, FRAME_COUNT)
        self.assertFalse(audio.looping_input_file)
        self.assertFalse(audio.started)

        def data_callback(input_frames, output_frames):
            self.assertEqual(len(input_frames), len(output_frames))
            output_frames[:] = input_frames
            notify()

        def stop_callback():
            self.assertFalse(audio.started)

        audio.start(data_callback, stop_callback)
        self.assertTrue(audio.started)

        wait()

        audio.stop()
        self.assertFalse(audio.started)

    def test_get_audio_file_info(self):
        info = ta.get_audio_file_info(
            os.path.join(PROJECT_DIR, "../audio-samples/1MB.wav")
        )

        self.assertEqual(info.format, ta.Format.S16)
        self.assertEqual(info.channels, 2)
        self.assertEqual(info.sample_rate, 8000)
        self.assertEqual(info.total_frame_count, 268237)


if __name__ == "__main__":
    unittest.main()
