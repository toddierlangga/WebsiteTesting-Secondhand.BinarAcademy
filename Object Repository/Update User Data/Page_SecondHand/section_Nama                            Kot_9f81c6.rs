<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>section_Nama                            Kot_9f81c6</name>
   <tag></tag>
   <elementGuidId>bb03796b-feda-4106-8963-317b89511bfe</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Lengkapi Info Akun'])[1]/following::section[1]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>section.pt-5.mt-5</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>section</value>
      <webElementGuid>922fa611-1337-4d98-bae5-a8dd2909588e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>pt-5 mt-5</value>
      <webElementGuid>041beb91-16c3-4fb7-8999-e97caf8aa5de</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
      
  
  
    
  


  
  
    

      
        
            
              
            

          
        
      

      
        Nama
        
      

      
        Kota
        Pilih Kota
Solo
Jogja
Jakarta
Bandung
Semarang
      

      
        Alamat
        Halan
      

      
        No Handphone
        
      

      
        
      
  



  var formAvatarView = document.getElementById('form-avatar-view')
  var formAvatarInput = document.getElementById('form-avatar-input')
  var formAvatar = document.getElementById('form-avatar')
  var formAvatarImage = document.getElementById('form-avatar-image')
  var image;

  formAvatarView.addEventListener('click', function(e) {
    formAvatar.click();
  });

  formAvatar.addEventListener('change', function () {
    if (!!formAvatarImage) {
      formAvatarImage.style[&quot;display&quot;] = &quot;none&quot;;
    }

    if (!!image) {
      image.remove();
    }

    for (const file of formAvatar.files) {
      image = document.createElement('img');
      image.classList.add(&quot;p-0&quot;, &quot;img-thumbnail&quot;, &quot;img-avatar-profile&quot;, &quot;overflow-hidden&quot;, &quot;rounded-4&quot;, &quot;border-0&quot;);
      image.src = URL.createObjectURL(file);
      formAvatarView.prepend(image);
      formAvatarInput.style['display'] = 'none';
    }
  });




    </value>
      <webElementGuid>d19534e7-94bd-49b7-acaa-10ebc105d014</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/section[@class=&quot;pt-5 mt-5&quot;]</value>
      <webElementGuid>650423cb-73dd-41cf-83e0-5ae821db888a</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Lengkapi Info Akun'])[1]/following::section[1]</value>
      <webElementGuid>712093fe-b90b-4202-b98c-d686f6e36a19</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//section</value>
      <webElementGuid>456b7b63-6b61-46ae-b3e2-8b5e24e09848</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//section[(text() = concat(&quot;
      
  
  
    
  


  
  
    

      
        
            
              
            

          
        
      

      
        Nama
        
      

      
        Kota
        Pilih Kota
Solo
Jogja
Jakarta
Bandung
Semarang
      

      
        Alamat
        Halan
      

      
        No Handphone
        
      

      
        
      
  



  var formAvatarView = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-view&quot; , &quot;'&quot; , &quot;)
  var formAvatarInput = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-input&quot; , &quot;'&quot; , &quot;)
  var formAvatar = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar&quot; , &quot;'&quot; , &quot;)
  var formAvatarImage = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-image&quot; , &quot;'&quot; , &quot;)
  var image;

  formAvatarView.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
    formAvatar.click();
  });

  formAvatar.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function () {
    if (!!formAvatarImage) {
      formAvatarImage.style[&quot;display&quot;] = &quot;none&quot;;
    }

    if (!!image) {
      image.remove();
    }

    for (const file of formAvatar.files) {
      image = document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
      image.classList.add(&quot;p-0&quot;, &quot;img-thumbnail&quot;, &quot;img-avatar-profile&quot;, &quot;overflow-hidden&quot;, &quot;rounded-4&quot;, &quot;border-0&quot;);
      image.src = URL.createObjectURL(file);
      formAvatarView.prepend(image);
      formAvatarInput.style[&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;] = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    }
  });




    &quot;) or . = concat(&quot;
      
  
  
    
  


  
  
    

      
        
            
              
            

          
        
      

      
        Nama
        
      

      
        Kota
        Pilih Kota
Solo
Jogja
Jakarta
Bandung
Semarang
      

      
        Alamat
        Halan
      

      
        No Handphone
        
      

      
        
      
  



  var formAvatarView = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-view&quot; , &quot;'&quot; , &quot;)
  var formAvatarInput = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-input&quot; , &quot;'&quot; , &quot;)
  var formAvatar = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar&quot; , &quot;'&quot; , &quot;)
  var formAvatarImage = document.getElementById(&quot; , &quot;'&quot; , &quot;form-avatar-image&quot; , &quot;'&quot; , &quot;)
  var image;

  formAvatarView.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
    formAvatar.click();
  });

  formAvatar.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function () {
    if (!!formAvatarImage) {
      formAvatarImage.style[&quot;display&quot;] = &quot;none&quot;;
    }

    if (!!image) {
      image.remove();
    }

    for (const file of formAvatar.files) {
      image = document.createElement(&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;);
      image.classList.add(&quot;p-0&quot;, &quot;img-thumbnail&quot;, &quot;img-avatar-profile&quot;, &quot;overflow-hidden&quot;, &quot;rounded-4&quot;, &quot;border-0&quot;);
      image.src = URL.createObjectURL(file);
      formAvatarView.prepend(image);
      formAvatarInput.style[&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;] = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    }
  });




    &quot;))]</value>
      <webElementGuid>05456c62-505b-40bd-a3b5-04deded7817c</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
