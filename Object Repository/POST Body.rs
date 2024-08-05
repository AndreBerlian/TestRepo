<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Body</name>
   <tag></tag>
   <elementGuidId>2a4e550b-a2da-418e-a6ef-40dc10f7febe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${Token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;lob\&quot; : \&quot;MMU\&quot;,\n    \&quot;fullName\&quot; : \&quot;ANDRE\&quot;,\n    \&quot;mobilePhone\&quot; : \&quot;089900001111\&quot;,\n    \&quot;noKtp\&quot; : \&quot;3173082011809999\&quot;,\n    \&quot;birthDate\&quot; : \&quot;1999-11-20\&quot;,\n    \&quot;birthPlace\&quot; : \&quot;JAKARTA\&quot;,\n    \&quot;gender\&quot; : \&quot;M\&quot;,\n    \&quot;mothersName\&quot; : \&quot;RODIAH\&quot;,\n    \&quot;financingType\&quot; : \&quot;U\&quot;,\n    \&quot;packageCode\&quot; : \&quot;PR0002\&quot;,\n    \&quot;tenor\&quot; : 12,\n    \&quot;downPayment\&quot; : 500000,\n    \&quot;channel\&quot; : \&quot;FNA\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>188f71dd-deae-4d27-8d3f-fd6330c35382</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Token}</value>
      <webElementGuid>0cc0499f-e816-4dcb-a07f-2ab8c5cff25b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://fifada-qa-lb01.fifgroup.co.id/backend/leads/draft</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>e675da51-34c3-413e-9cb3-5959cc8440ba</id>
      <masked>false</masked>
      <name>Token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

GlobalVariable.get_draftid = WS.getElementPropertyValue(response, 'result.draftId')

System.out.println(GlobalVariable.get_draftid)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
