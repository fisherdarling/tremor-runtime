(self.webpackJsonp=self.webpackJsonp||[]).push([[19],{539:function(e,t,a){(e.exports=a(5)(!1)).push([e.i,".tabs--single[data-v-86b51ddc]{display:none}.dlg-tabs[data-v-86b51ddc] .tabs__items{border-width:1px 0 0!important}.dlg-tabs[data-v-86b51ddc] .tabs--single+.tabs__items{border-width:0!important}.uppercase[data-v-86b51ddc]{text-transform:uppercase}",""])},540:function(e,t,a){var i=a(539);"string"==typeof i&&(i=[[e.i,i,""]]),i.locals&&(e.exports=i.locals),(0,a(36).default)("260f1130",i,!0,{})},577:function(e,t,a){"use strict";a.r(t);var i=a(7),s=a.n(i),n=a(9),l=a(3),c=a(22),r=a.n(c),o=a(85),d=a.n(o),_=a(34),p=a.n(_),u={directives:{markdown:a(46).a},data:function(){return{tab:null,activated:!1}},created:function(){this.setTab(this.SPEC)},computed:s()({},Object(n.c)([l.X,l.C]),{active:{get:function(){return this.activated=this.UI_DIALOG("security")||this.activated,this.UI_DIALOG("security")},set:function(e){e||this.UI_SET_DIALOG()}}}),methods:s()({},Object(n.d)([l.Oa]),{setTab:function(e){this.tab=e?function(e){if(e){var t=r()(e);if(t.length)return t[0]}}(e.securityDefinitions):null},authorize:function(e){!function(e){e._._accessToken=null,e._._tokenType=null,e._._expiresIn=null,e._._validFrom=null;var t=encodeURIComponent(e._._clientId),a=e.authorizationUrl+"?response_type=token"+(t?"&client_id="+t:"")+"&scope="+function(e){var t=[];for(var a in e)e[a]&&t.push(a);return t.join("+")}(e._._scopes)+"&redirect_uri="+e._._callbackUrl;window.open(a),window.onOAuthFinished=function(a){a.code?p()({method:"POST",url:e.tokenUrl,headers:{Accept:"application/json"},params:{grant_type:"authorization_code",code:a.code,redirect_url:e._._callbackUrl,client_id:t,client_secret:e._._clientSecret}}).then(function(t){var a=t.data;d()(e,{_accessToken:a.access_token,_tokenType:a.token_type,_expiresIn:a.expires_in?parseInt(a.expires_in):null,_validFrom:new Date})}):d()(e,{_accessToken:a.access_token,_tokenType:a.token_type,_expiresIn:a.expires_in?parseInt(a.expires_in):null,_validFrom:new Date})}}(e)},name:function(e){return function(e){var t=e.flow||e.type;return(t=(t=(t=t.replace(/([a-z])([A-Z])/g,"$1 $2")).split(" "))[t.length-1])[0].toUpperCase()+t.substr(1)}(e)}}),watch:{SPEC:function(e){this.setTab(e)},active:function(e){e&&this.setTab(this.SPEC)}}},v=a(10),h=Object(v.a)(u,function(){var e=this,t=e.$createElement,a=e._self._c||t;return a("v-dialog",{attrs:{"max-width":"480"},model:{value:e.active,callback:function(t){e.active=t},expression:"active"}},[a("v-card",[a("v-toolbar",{staticClass:"elevation-0",staticStyle:{"background-color":"transparent"}},[a("v-toolbar-title",[e._v("Security")]),a("v-spacer"),a("v-btn",{attrs:{icon:"icon"},nativeOn:{click:function(t){t.stopPropagation(),e.UI_SET_DIALOG()}}},[a("v-icon",[e._v("close")])],1)],1),a("v-divider"),e.SPEC&&(e.tab&&e.active||e.activated)?a("v-tabs",{staticClass:"dlg-tabs",staticStyle:{"max-height":"calc(90vh - 64px - 1px)",overflow:"auto"},attrs:{scrollable:!1},model:{value:e.tab,callback:function(t){e.tab=t},expression:"tab"}},[e._l(e.SPEC.securityDefinitions,function(t,i){return a("v-tab",{key:i,attrs:{ripple:"ripple",href:"#"+i}},[e._v(e._s(e.name(t)))])}),e._l(e.SPEC.securityDefinitions,function(t,i){return a("v-tab-item",{key:i,attrs:{id:i}},["oauth2"===t.type?[a("h3",{staticClass:"pl-3 pr-3 pt-3 title"},[a("span",{staticClass:"uppercase"},[e._v(e._s(t.flow))]),e._v(" OAuth 2.0 Authentication")]),t._.description?a("div",{directives:[{name:"markdown",rawName:"v-markdown",value:t._,expression:"sec._"}],staticClass:"pl-3 pr-3 pt-1"}):e._e(),a("div",{staticClass:"pa-3"},[e._l(t.scopes,function(i,s){return a("v-checkbox",{key:s,staticClass:"pa-0",attrs:{"hide-details":"hide-details",label:s,color:"primary"},model:{value:t._._scopes[s],callback:function(a){e.$set(t._._scopes,s,a)},expression:"sec._._scopes[scopeKey]"}})}),a("div",{staticClass:"pt-3"},[a("v-text-field",{attrs:{"hide-details":"hide-details",label:"Client ID"},model:{value:t._._clientId,callback:function(a){e.$set(t._,"_clientId",a)},expression:"sec._._clientId"}}),"implicit"!==t.flow?a("v-text-field",{attrs:{"hide-details":"hide-details",label:"Client secret"},model:{value:t._._clientSecret,callback:function(a){e.$set(t._,"_clientSecret",a)},expression:"sec._._clientSecret"}}):e._e(),a("v-text-field",{attrs:{"hide-details":"hide-details",label:"Authorization callback URL"},model:{value:t._._callbackUrl,callback:function(a){e.$set(t._,"_callbackUrl",a)},expression:"sec._._callbackUrl"}}),t.authorizationUrl?a("v-text-field",{attrs:{"hide-details":"hide-details",readonly:"readonly",label:"Authorization URL"},model:{value:t.authorizationUrl,callback:function(a){e.$set(t,"authorizationUrl",a)},expression:"sec.authorizationUrl"}}):e._e(),t.tokenUrl?a("v-text-field",{attrs:{"hide-details":"hide-details",readonly:"readonly",label:"Token URL"},model:{value:t.tokenUrl,callback:function(a){e.$set(t,"tokenUrl",a)},expression:"sec.tokenUrl"}}):e._e(),t._._accessToken?a("v-text-field",{attrs:{"persistent-hint":"persistent-hint",hint:(t._validFrom?"Valid from "+t._validFrom:"")+(t._validFrom&&t._expiresIn?", ":"")+(t._expiresIn?"Expires in "+t._expiresIn:""),readonly:"readonly",label:"Access token"},model:{value:t._accessToken,callback:function(a){e.$set(t,"_accessToken",a)},expression:"sec._accessToken"}}):e._e()],1)],2),a("v-btn",{staticClass:"ml-3 mb-3",attrs:{color:"primary"},on:{click:function(a){e.authorize(t)}}},[e._v("Authorize")])]:"apiKey"===t.type?[a("h3",{staticClass:"pl-3 pr-3 pt-3 title"},[e._v("API Key Authentication")]),t._.description?a("div",{directives:[{name:"markdown",rawName:"v-markdown",value:t._,expression:"sec._"}],staticClass:"pl-3 pr-3 pt-1"}):e._e(),a("div",{staticClass:"pa-3"},[a("v-text-field",{attrs:{"hide-details":"hide-details",label:"API key"},model:{value:t._._apiKey,callback:function(a){e.$set(t._,"_apiKey",a)},expression:"sec._._apiKey"}})],1)]:"basic"===t.type?[a("h3",{staticClass:"pl-3 pr-3 pt-3 title"},[e._v("Basic Authentication")]),t._.description?a("div",{directives:[{name:"markdown",rawName:"v-markdown",value:t._,expression:"sec._"}],staticClass:"pl-3 pr-3 pt-1"}):e._e(),a("div",{staticClass:"pa-3"},[a("v-text-field",{attrs:{"hide-details":"hide-details",label:"User"},model:{value:t._._user,callback:function(a){e.$set(t._,"_user",a)},expression:"sec._._user"}}),a("v-text-field",{attrs:{"hide-details":"hide-details",type:"password",label:"Password"},model:{value:t._._password,callback:function(a){e.$set(t._,"_password",a)},expression:"sec._._password"}})],1)]:e._e()],2)})],2):e._e()],1)],1)},[],!1,function(e){a(540)},"data-v-86b51ddc",null);t.default=h.exports}}]);