import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://secondhand.binaracademy.org/')

WebUI.click(findTestObject('Object Repository/Add Product/Page_SecondHand/a_Masuk'))

WebUI.setText(findTestObject('Object Repository/Add Product/Page_SecondHand/input_Email_useremail'), 'websitetest@testing.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Add Product/Page_SecondHand/input_Password_userpassword'), 'JltEQgaE0YE=')

WebUI.click(findTestObject('Object Repository/Add Product/Page_SecondHand/input_Password_commit'))

WebUI.click(findTestObject('Object Repository/Add Product/Page_SecondHand/a_Jual'))

WebUI.setText(findTestObject('Object Repository/Add Product/Page_SecondHand/input_Nama Produk_productname'), 'Kucing')

WebUI.setText(findTestObject('Object Repository/Add Product/Page_SecondHand/input_Harga Produk_productprice'), '250000')

WebUI.selectOptionByValue(findTestObject('Object Repository/Add Product/Page_SecondHand/select_Pilih KategoriHobiKendaraanBajuElekt_20972a'), 
    '1', true)

WebUI.setText(findTestObject('Object Repository/Add Product/Page_SecondHand/textarea_Deskripsi_productdescription'), 'Kucing galau')

WebUI.click(findTestObject('Object Repository/Add Product/Page_SecondHand/i_Deskripsi_bi bi-plus display-6'))

WebUI.click(findTestObject('Object Repository/Add Product/Page_SecondHand/label_Terbitkan'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Add Product/Page_SecondHand/a_Edit'), 0)

WebUI.verifyElementPresent(findTestObject('Add Product/Page_SecondHand/a_Edit'), 0)

WebUI.closeBrowser()

