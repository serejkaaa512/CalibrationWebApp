from selenium import webdriver
from selenium.webdriver.common.keys import Keys
import unittest


class CalibrationTest(unittest.TestCase):

    def setUp(self):
        self.browser = webdriver.Firefox()
        self.browser.implicitly_wait(3)

    def tearDown(self):
        self.browser.quit()

    def test_start_page(self):
        # заходим на главную страницу
        self.browser.get('http://localhost:6767')
        # видим, что Калибровка в заголовке
        self.assertIn('Калибровка', self.browser.title)

        # видим приветствие
        header_text = self.browser.find_element_by_tag_name('h1').text
        self.assertIn('Калибровка', header_text)

        # видим поля для ввода IP и порта Генератора и Измерителя мощности.
        # в полях для ввода присутствуют значения по умолчанию
        inputbox = self.browser.find_element_by_id('generator_ip')
        self.assertEqual(inputbox.get_attribute('placeholder'), '10.10.0.7')

        inputbox = self.browser.find_element_by_id('generator_port')
        self.assertEqual(inputbox.get_attribute('placeholder'), '3333')

        inputbox = self.browser.find_element_by_id('powermeter_ip')
        self.assertEqual(inputbox.get_attribute('placeholder'), '10.10.0.7')

        inputbox = self.browser.find_element_by_id('powermeter_port')
        self.assertEqual(inputbox.get_attribute('placeholder'), '5025')

        # заполняем поля значениями и нажимаем enter
        inputbox.send_keys('4444')
        inputbox.send_keys(Keys.ENTER)
        # происходит переход в случае успешного подключения на страницу
        # калибровки

        # конец теста
        self.fail("Finish the test!")


if __name__ == '__main__':
    unittest.main(warnings='ignore')