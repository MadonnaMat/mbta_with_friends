(function(t){function e(e){for(var s,o,i=e[0],u=e[1],c=e[2],l=0,f=[];l<i.length;l++)o=i[l],a[o]&&f.push(a[o][0]),a[o]=0;for(s in u)Object.prototype.hasOwnProperty.call(u,s)&&(t[s]=u[s]);d&&d(e);while(f.length)f.shift()();return r.push.apply(r,c||[]),n()}function n(){for(var t,e=0;e<r.length;e++){for(var n=r[e],s=!0,i=1;i<n.length;i++){var u=n[i];0!==a[u]&&(s=!1)}s&&(r.splice(e--,1),t=o(o.s=n[0]))}return t}var s={},a={app:0},r=[];function o(e){if(s[e])return s[e].exports;var n=s[e]={i:e,l:!1,exports:{}};return t[e].call(n.exports,n,n.exports,o),n.l=!0,n.exports}o.m=t,o.c=s,o.d=function(t,e,n){o.o(t,e)||Object.defineProperty(t,e,{enumerable:!0,get:n})},o.r=function(t){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(t,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(t,"__esModule",{value:!0})},o.t=function(t,e){if(1&e&&(t=o(t)),8&e)return t;if(4&e&&"object"===typeof t&&t&&t.__esModule)return t;var n=Object.create(null);if(o.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:t}),2&e&&"string"!=typeof t)for(var s in t)o.d(n,s,function(e){return t[e]}.bind(null,s));return n},o.n=function(t){var e=t&&t.__esModule?function(){return t["default"]}:function(){return t};return o.d(e,"a",e),e},o.o=function(t,e){return Object.prototype.hasOwnProperty.call(t,e)},o.p="/";var i=window["webpackJsonp"]=window["webpackJsonp"]||[],u=i.push.bind(i);i.push=e,i=i.slice();for(var c=0;c<i.length;c++)e(i[c]);var d=u;r.push([0,"chunk-vendors"]),n()})({0:function(t,e,n){t.exports=n("cd49")},"0d59":function(t,e,n){"use strict";n("3a3d");var s=n("de39");n.d(e,"Friend",function(){return s["a"]});n("41f4")},"27ca":function(t,e,n){},2856:function(t,e,n){},"3a3d":function(t,e,n){"use strict";n("c665")},"41f4":function(t,e){},4515:function(t,e,n){"use strict";var s=n("7883"),a=n.n(s);a.a},"5c0b":function(t,e,n){"use strict";var s=n("2856"),a=n.n(s);a.a},7883:function(t,e,n){},"9ebe":function(t,e,n){"use strict";var s=n("27ca"),a=n.n(s);a.a},"9f02":function(t,e,n){},a7d1:function(t,e,n){"use strict";var s=n("9f02"),a=n.n(s);a.a},cd49:function(t,e,n){"use strict";n.r(e);n("cadf"),n("551c"),n("097d");var s=n("2b0e"),a=n("bc3a"),r=n.n(a),o=n("a7fe"),i=n.n(o),u=n("43f9"),c=n.n(u),d=n("f213"),l=(n("51de"),n("e094"),function(){var t=this,e=t.$createElement,n=t._self._c||e;return t.user?n("div",{staticClass:"page-container"},[n("TabsLoggedIn"),n("vue-snotify")],1):n("div",{staticClass:"page-container"},[n("md-app",{attrs:{"md-waterfall":"","md-mode":"fixed"}},[n("md-app-toolbar",{staticClass:"md-primary"},[n("div",{staticClass:"md-toolbar-row"},[n("md-tabs",{staticClass:"md-primary",attrs:{"md-active-tab":t.activeTab}},[n("md-tab",{attrs:{id:"tab-home","md-label":"Home",to:"/"}}),n("md-tab",{attrs:{id:"tab-about","md-label":"About",to:"/about"}}),n("md-tab",{attrs:{id:"tab-login","md-label":"Login",to:"/login"}})],1)],1)]),n("md-app-content",[n("router-view")],1)],1),n("vue-snotify")],1)}),f=[],m=n("c665"),p=n("dc0a"),b=n("aa9a"),v=n("d328"),h=n("11d9"),g=n("9ab4"),y=n("65d9"),_=n.n(y),O=n("60a3"),j=n("4bb5"),w=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("md-app",{attrs:{"md-waterfall":"","md-mode":"fixed"}},[n("md-app-toolbar",{staticClass:"md-primary"},[n("div",{staticClass:"md-toolbar-row"},[n("md-tabs",{staticClass:"md-primary",attrs:{"md-active-tab":t.activeTab}},[n("md-tab",{attrs:{id:"tab-home","md-label":"Home",to:"/"}}),n("md-tab",{attrs:{id:"tab-about","md-label":"About",to:"/about"}}),n("md-tab",{attrs:{id:"tab-add-friend","md-label":"Add Friend",to:"/add-friend"}})],1),n("md-button",{attrs:{id:"tab-logout"},on:{click:function(e){return e.preventDefault(),t.logout(e)}}},[t._v("Logout")])],1)]),n("md-app-content",[n("router-view")],1)],1)},C=[],$=function(t){function e(){return Object(m["a"])(this,e),Object(v["a"])(this,Object(h["a"])(e).apply(this,arguments))}return Object(b["a"])(e,[{key:"logout",value:function(){var t=this;this.axios.delete("/api/session").then(function(){t.setUser(null),t.setUsers([]),t.$snotify.success("Logged Out!"),t.$router.push("/")}).catch(function(e){if(e.response&&e.response.data){var n=e.response.data;t.$snotify.error(n)}})}},{key:"activeTab",get:function(){var t=this.$route.path.slice(1);return t=""==t?"home":t,"tab-".concat(t)}}]),Object(p["a"])(e,t),e}(O["c"]);g["a"]([Object(j["c"])("user")],$.prototype,"user",void 0),g["a"]([Object(j["b"])("set_user")],$.prototype,"setUser",void 0),g["a"]([Object(j["b"])("set_users")],$.prototype,"setUsers",void 0),$=g["a"]([O["a"]],$);var k=$,U=k,x=n("2877"),A=Object(x["a"])(U,w,C,!1,null,null,null);A.options.__file="TabsLoggedIn.vue";var S=A.exports,L=function(t){function e(){return Object(m["a"])(this,e),Object(v["a"])(this,Object(h["a"])(e).apply(this,arguments))}return Object(b["a"])(e,[{key:"loggedIn",get:function(){return!!this.user}},{key:"notLoggedIn",get:function(){return!this.user}},{key:"activeTab",get:function(){var t=this.$route.path.slice(1);return t=""==t?"home":t,"tab-".concat(t)}}]),Object(p["a"])(e,t),e}(O["c"]);g["a"]([Object(j["c"])("user")],L.prototype,"user",void 0),g["a"]([Object(j["b"])("set_user")],L.prototype,"setUser",void 0),g["a"]([Object(j["b"])("set_users")],L.prototype,"setUsers",void 0),L=g["a"]([_()({components:{TabsLoggedIn:S}})],L);var F=L,E=F,T=(n("5c0b"),Object(x["a"])(E,l,f,!1,null,null,null));T.options.__file="App.vue";var I=T.exports,z=n("2f62"),P=n("1b39"),H=n.n(P),N=n("0d59"),D=new H.a({state:{users:[]}}),M=D.get({action:"listUsers",property:"users",path:"/api/users",onSuccess:function(t,e){t.users=e.data.map(function(t){return new N["Friend"](t)})}}).getStore(),W=n("2ef0"),J=n.n(W);s["default"].use(z["a"]);var q={state:{user:null,hasConfig:!1},mutations:{set_config:function(t,e){t.hasConfig=!0,t.user=e.user},set_user:function(t,e){t.user=e},set_users:function(t,e){t.users=e}}},B=M,G=J.a.merge(q,B),K=new z["a"].Store(G),Q=K,R=n("8c4f"),V=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{staticClass:"home"},[n("HelloWorld")],1)},X=[],Y=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",[n("div",t._l(t.users,function(e){return n("md-card",{attrs:{"md-with-hover":""}},[n("md-ripple",[n("md-card-header",[n("div",{staticClass:"md-title"},[t._v("\n            "+t._s(e.username)+"\n          ")])]),n("md-card-content",[n("p",[t._v("Id: "+t._s(e.id))]),n("p",[t._v("Is Friend: "+t._s(e.is_friend))])])],1)],1)}))])},Z=[],tt=function(t){function e(){var t;return Object(m["a"])(this,e),t=Object(v["a"])(this,Object(h["a"])(e).apply(this,arguments)),t.sending=!1,t.form={username:"",password:""},t}return Object(b["a"])(e,[{key:"mounted",value:function(){this.user&&0==this.users.length&&this.listUsers()}}]),Object(p["a"])(e,t),e}(O["c"]);g["a"]([Object(O["b"])()],tt.prototype,"msg",void 0),g["a"]([Object(j["c"])("users")],tt.prototype,"users",void 0),g["a"]([Object(j["c"])("user")],tt.prototype,"user",void 0),g["a"]([Object(j["a"])("listUsers")],tt.prototype,"listUsers",void 0),tt=g["a"]([_.a],tt);var et=tt,nt=et,st=(n("4515"),Object(x["a"])(nt,Y,Z,!1,null,"27a50f0d",null));st.options.__file="HelloWorld.vue";var at=st.exports,rt=function(t){function e(){return Object(m["a"])(this,e),Object(v["a"])(this,Object(h["a"])(e).apply(this,arguments))}return Object(p["a"])(e,t),e}(O["c"]);rt=g["a"]([Object(O["a"])({components:{HelloWorld:at}})],rt);var ot=rt,it=ot,ut=Object(x["a"])(it,V,X,!1,null,null,null);ut.options.__file="Home.vue";var ct=ut.exports,dt=function(){var t=this,e=t.$createElement;t._self._c;return t._m(0)},lt=[function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{staticClass:"about"},[n("h1",[t._v("This is an about page")])])}],ft={},mt=Object(x["a"])(ft,dt,lt,!1,null,null,null);mt.options.__file="About.vue";var pt=mt.exports,bt=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{staticClass:"login"},[n("LoginUser")],1)},vt=[],ht=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",[t.user?n("div",[t._v("\n    "+t._s(t.user.username)+"\n  ")]):t._e(),n("form",{staticClass:"md-layout",attrs:{novalidate:""}},[n("md-card",{staticClass:"md-layout-item md-size-50 md-small-size-100"},[n("md-card-header",[n("div",{staticClass:"md-title"},[t._v("\n          Log In\n        ")])]),n("md-card-content",[n("div",{staticClass:"md-layout md-gutter"},[n("div",{staticClass:"md-layout-item md-small-size-100"},[n("md-field",[n("label",{attrs:{for:"username"}},[t._v("User Name")]),n("md-input",{attrs:{name:"username",id:"username",disabled:t.sending},model:{value:t.form.username,callback:function(e){t.$set(t.form,"username",e)},expression:"form.username"}})],1)],1),n("div",{staticClass:"md-layout-item md-small-size-100"},[n("md-field",[n("label",{attrs:{for:"password"}},[t._v("Password")]),n("md-input",{attrs:{name:"password",id:"password",type:"password",disabled:t.sending},model:{value:t.form.password,callback:function(e){t.$set(t.form,"password",e)},expression:"form.password"}})],1)],1),t.sending?n("md-progress-bar",{attrs:{"md-mode":"indeterminate"}}):t._e(),n("md-card-actions",[n("md-button",{staticClass:"md-primary",attrs:{type:"submit",name:"create",disabled:t.sending},on:{click:function(e){e.preventDefault(),t.onSubmit("new")}}},[t._v("\n              New User\n            ")]),n("md-button",{staticClass:"md-primary",attrs:{type:"submit",name:"login",disabled:t.sending},on:{click:function(e){e.preventDefault(),t.onSubmit("login")}}},[t._v("\n              Log In\n            ")])],1)],1)])],1)],1)])},gt=[],yt=function(t){function e(){var t;return Object(m["a"])(this,e),t=Object(v["a"])(this,Object(h["a"])(e).apply(this,arguments)),t.sending=!1,t.form={username:"",password:""},t}return Object(b["a"])(e,[{key:"onSubmit",value:function(t){"new"==t?this.onSubmitCreate():this.onSubmitLogin()}},{key:"mounted",value:function(){this.user&&this.$router.push("/")}},{key:"onSubmitLogin",value:function(){var t=this;this.sending=!0,this.axios.post("/api/session",this.form).then(function(e){var n=e.data;t.setUser(n),t.setUsers([]),t.$router.push("/",function(){t.$snotify.success("Logged In!")})}).catch(function(e){if(e.response&&e.response.data){var n=e.response.data;t.$snotify.error(n)}}).finally(function(){t.sending=!1})}},{key:"onSubmitCreate",value:function(){var t=this;this.sending=!0,this.axios.post("/api/users",this.form).then(function(e){var n=e.data;t.setUser(n),t.setUsers([]),t.$router.push("/",function(){t.$snotify.success("User Created!")})}).catch(function(e){if(e.response&&e.response.data){var n=e.response.data;t.$snotify.error(n)}}).finally(function(){t.sending=!1})}}]),Object(p["a"])(e,t),e}(O["c"]);g["a"]([Object(j["c"])("user")],yt.prototype,"user",void 0),g["a"]([Object(j["b"])("set_user")],yt.prototype,"setUser",void 0),g["a"]([Object(j["b"])("set_users")],yt.prototype,"setUsers",void 0),yt=g["a"]([_.a],yt);var _t=yt,Ot=_t,jt=(n("9ebe"),Object(x["a"])(Ot,ht,gt,!1,null,"42b34997",null));jt.options.__file="LoginUser.vue";var wt=jt.exports,Ct=function(t){function e(){return Object(m["a"])(this,e),Object(v["a"])(this,Object(h["a"])(e).apply(this,arguments))}return Object(p["a"])(e,t),e}(O["c"]);Ct=g["a"]([Object(O["a"])({components:{LoginUser:wt}})],Ct);var $t=Ct,kt=$t,Ut=Object(x["a"])(kt,bt,vt,!1,null,null,null);Ut.options.__file="Login.vue";var xt=Ut.exports,At=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{staticClass:"add-friend"},[n("AddAFriend")],1)},St=[],Lt=function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",[n("form",{staticClass:"md-layout",attrs:{novalidate:""}},[n("md-card",{staticClass:"md-layout-item md-size-50 md-small-size-100"},[n("md-card-header",[n("div",{staticClass:"md-title"},[t._v("\n          Add A Friend\n        ")])]),n("md-card-content",[n("div",{staticClass:"md-layout md-gutter"},[n("div",{staticClass:"md-layout-item md-small-size-100"},[n("md-field",[n("label",{attrs:{for:"username"}},[t._v("User Name")]),n("md-input",{attrs:{name:"username",id:"username",disabled:t.sending},model:{value:t.form.username,callback:function(e){t.$set(t.form,"username",e)},expression:"form.username"}})],1)],1),t.sending?n("md-progress-bar",{attrs:{"md-mode":"indeterminate"}}):t._e(),n("md-card-actions",[n("md-button",{staticClass:"md-primary",attrs:{type:"submit",name:"login",disabled:t.sending},on:{click:function(e){return e.preventDefault(),t.onSubmit(e)}}},[t._v("\n              Add Friend\n            ")])],1)],1)])],1)],1)])},Ft=[],Et=function(t){function e(){var t;return Object(m["a"])(this,e),t=Object(v["a"])(this,Object(h["a"])(e).apply(this,arguments)),t.sending=!1,t.form={username:""},t}return Object(b["a"])(e,[{key:"onSubmit",value:function(){var t=this;this.sending=!0,this.axios.post("/api/friends",this.form).then(function(e){var n=e.data.map(function(t){return new N["Friend"](t)});t.setUsers(n),t.$router.push("/",function(){t.$snotify.success("Friend Added!")})}).catch(function(e){if(e.response&&e.response.data){var n=e.response.data;t.$snotify.error(n)}}).finally(function(){t.sending=!1})}},{key:"mounted",value:function(){this.user||this.$router.push("/")}}]),Object(p["a"])(e,t),e}(O["c"]);g["a"]([Object(j["c"])("user")],Et.prototype,"user",void 0),g["a"]([Object(j["b"])("set_users")],Et.prototype,"setUsers",void 0),Et=g["a"]([_.a],Et);var Tt=Et,It=Tt,zt=(n("a7d1"),Object(x["a"])(It,Lt,Ft,!1,null,"46edb7f1",null));zt.options.__file="AddAFriend.vue";var Pt=zt.exports,Ht=function(t){function e(){return Object(m["a"])(this,e),Object(v["a"])(this,Object(h["a"])(e).apply(this,arguments))}return Object(p["a"])(e,t),e}(O["c"]);Ht=g["a"]([Object(O["a"])({components:{AddAFriend:Pt}})],Ht);var Nt=Ht,Dt=Nt,Mt=Object(x["a"])(Dt,At,St,!1,null,null,null);Mt.options.__file="AddFriend.vue";var Wt=Mt.exports;s["default"].use(R["a"]);var Jt=new R["a"]({mode:"history",base:"/",routes:[{path:"/",name:"home",component:ct},{path:"/about",name:"about",component:pt},{path:"/login",name:"login",component:xt},{path:"/add-friend",name:"add-friend",component:Wt}]});Jt.beforeEach(function(t,e,n){Q.state.hasConfig?n():r.a.get("/api/config").then(function(t){Q.commit("set_config",t.data),n()})});var qt=Jt,Bt=n("9483");Object(Bt["a"])("".concat("/","service-worker.js"),{ready:function(){console.log("App is being served from cache by a service worker.\nFor more details, visit https://goo.gl/AFskqB")},cached:function(){console.log("Content has been cached for offline use.")},updated:function(){console.log("New content is available; please refresh.")},offline:function(){console.log("No internet connection found. App is running in offline mode.")},error:function(t){console.error("Error during service worker registration:",t)}}),s["default"].use(c.a),s["default"].use(d["a"]),s["default"].use(i.a,r.a),s["default"].config.productionTip=!1,new s["default"]({router:qt,store:Q,render:function(t){return t(I)}}).$mount("#app")},de39:function(t,e,n){"use strict";n.d(e,"a",function(){return a});var s=n("c665"),a=function t(e){Object(s["a"])(this,t),this.id=e.id,this.username=e.username,this.is_friend=e.is_friend}}});
//# sourceMappingURL=app.ccc03bfa.js.map