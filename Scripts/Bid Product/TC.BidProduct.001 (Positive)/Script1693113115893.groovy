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

WebUI.click(findTestObject('Object Repository/Bid Product/Page_SecondHand/a_Masuk'))

WebUI.setText(findTestObject('Object Repository/Bid Product/Page_SecondHand/input_Email_useremail'), 'websitetest@testing.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Bid Product/Page_SecondHand/input_Password_userpassword'), 'JltEQgaE0YE=')

WebUI.click(findTestObject('Object Repository/Bid Product/Page_SecondHand/input_Password_commit'))

WebUI.click(findTestObject('Bid Product/Page_SecondHand/div_gamis      Hobi      Rp 200.000'))

WebUI.click(findTestObject('Bid Product/Page_SecondHand/Page_SecondHand/button_Saya tertarik dan ingin nego'))

WebUI.click(findTestObject('Object Repository/Bid Product/Page_SecondHand/div_Masukkan Harga Tawarmu          Harga t_b92df9'))

WebUI.setText(findTestObject('Object Repository/Bid Product/Page_SecondHand/input_Harga Tawar_offerprice'), '20000000')

WebUI.click(findTestObject('Object Repository/Bid Product/Page_SecondHand/input_Password_commit'))

WebUI.doubleClick(findTestObject('Object Repository/Bid Product/Page_SecondHand/div_kupu          Hobi          Rp 394.430._e9c2ba'))

WebUI.click(findTestObject('Object Repository/Bid Product/Page_SecondHand/div_kupu          Hobi          Rp 394.430._e9c2ba'))

WebUI.verifyElementVisible(findTestObject('Object Repository/Bid Product/Page_SecondHand/div_kupu          Hobi          Rp 394.430._e9c2ba'))

WebUI.closeBrowser()

