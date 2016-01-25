from selenium import webdriver
import unittest


class CalibrationTest(unittest.TestCase):

    def setUp(self):
        self.browser = webdriver.Firefox()

    def tearDown(self):
        self.browser.quit()

    def test_start(self):
        self.browser.get('http://localhost:6767')
        assert 'Calibration' in self.browser.title


if __name__ == '__main__':
    unittest.main(warnings='ignore')
