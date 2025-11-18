import{i as g,j as f,k as C,l as D,m as H,p as T,q as k,s as O,t as P,v as M,w as N,x as F,y,z as K,g as E,h as q,A as G,B as J,C as U,D as Q,E as X,G as Y,H as Z,I as W,e as B,o as v,J as $,c as z,d as s,K as S,L,F as I,M as ee,a as l,b as h,n as j,u as re,r as te}from"./index-CQZOGmuN.js";import{c as oe,b as ne,_ as ae,a as se}from"./UserAvatar-C97dN2pm.js";import{_ as ie,a as ce,b as le,c as de,A as me}from"./ThemeSetting-BhjWUuDQ.js";import{_ as ue}from"./AppCard-BysSpdtI.js";import{_ as pe}from"./Dropdown-Bea_11Hq.js";import"./_plugin-vue_export-helper-DlAUqK2U.js";import"./create-CfPrN9OH.js";import"./Avatar-DB8QP23x.js";import"./utils-C_7EMGUW.js";import"./Tag-Dxmn7TKr.js";import"./Add-C3g-VeQk.js";import"./Input-BM48kO3D.js";import"./Eye-CMZyCEZw.js";const _e=g("breadcrumb",`
 white-space: nowrap;
 cursor: default;
 line-height: var(--n-item-line-height);
`,[f("ul",`
 list-style: none;
 padding: 0;
 margin: 0;
 `),f("a",`
 color: inherit;
 text-decoration: inherit;
 `),g("breadcrumb-item",`
 font-size: var(--n-font-size);
 transition: color .3s var(--n-bezier);
 display: inline-flex;
 align-items: center;
 `,[g("icon",`
 font-size: 18px;
 vertical-align: -.2em;
 transition: color .3s var(--n-bezier);
 color: var(--n-item-text-color);
 `),f("&:not(:last-child)",[D("clickable",[C("link",`
 cursor: pointer;
 `,[f("&:hover",`
 background-color: var(--n-item-color-hover);
 `),f("&:active",`
 background-color: var(--n-item-color-pressed); 
 `)])])]),C("link",`
 padding: 4px;
 border-radius: var(--n-item-border-radius);
 transition:
 background-color .3s var(--n-bezier),
 color .3s var(--n-bezier);
 color: var(--n-item-text-color);
 position: relative;
 `,[f("&:hover",`
 color: var(--n-item-text-color-hover);
 `,[g("icon",`
 color: var(--n-item-text-color-hover);
 `)]),f("&:active",`
 color: var(--n-item-text-color-pressed);
 `,[g("icon",`
 color: var(--n-item-text-color-pressed);
 `)])]),C("separator",`
 margin: 0 8px;
 color: var(--n-separator-color);
 transition: color .3s var(--n-bezier);
 user-select: none;
 -webkit-user-select: none;
 `),f("&:last-child",[C("link",`
 font-weight: var(--n-font-weight-active);
 cursor: unset;
 color: var(--n-item-text-color-active);
 `,[g("icon",`
 color: var(--n-item-text-color-active);
 `)]),C("separator",`
 display: none;
 `)])])]),A=H("n-breadcrumb"),fe=Object.assign(Object.assign({},P.props),{separator:{type:String,default:"/"}}),he=T({name:"Breadcrumb",props:fe,setup(e){const{mergedClsPrefixRef:o,inlineThemeDisabled:t}=O(e),n=P("Breadcrumb","-breadcrumb",_e,M,e,o);N(A,{separatorRef:F(e,"separator"),mergedClsPrefixRef:o});const i=y(()=>{const{common:{cubicBezierEaseInOut:u},self:{separatorColor:b,itemTextColor:m,itemTextColorHover:r,itemTextColorPressed:c,itemTextColorActive:p,fontSize:d,fontWeightActive:x,itemBorderRadius:_,itemColorHover:w,itemColorPressed:R,itemLineHeight:V}}=n.value;return{"--n-font-size":d,"--n-bezier":u,"--n-item-text-color":m,"--n-item-text-color-hover":r,"--n-item-text-color-pressed":c,"--n-item-text-color-active":p,"--n-separator-color":b,"--n-item-color-hover":w,"--n-item-color-pressed":R,"--n-item-border-radius":_,"--n-font-weight-active":x,"--n-item-line-height":V}}),a=t?K("breadcrumb",void 0,i,e):void 0;return{mergedClsPrefix:o,cssVars:t?void 0:i,themeClass:a?.themeClass,onRender:a?.onRender}},render(){var e;return(e=this.onRender)===null||e===void 0||e.call(this),k("nav",{class:[`${this.mergedClsPrefix}-breadcrumb`,this.themeClass],style:this.cssVars,"aria-label":"Breadcrumb"},k("ul",null,this.$slots))}});function be(e=J?window:null){const o=()=>{const{hash:i,host:a,hostname:u,href:b,origin:m,pathname:r,port:c,protocol:p,search:d}=e?.location||{};return{hash:i,host:a,hostname:u,href:b,origin:m,pathname:r,port:c,protocol:p,search:d}},t=E(o()),n=()=>{t.value=o()};return q(()=>{e&&(e.addEventListener("popstate",n),e.addEventListener("hashchange",n))}),G(()=>{e&&(e.removeEventListener("popstate",n),e.removeEventListener("hashchange",n))}),t}const ve={separator:String,href:String,clickable:{type:Boolean,default:!0},onClick:Function},xe=T({name:"BreadcrumbItem",props:ve,slots:Object,setup(e,{slots:o}){const t=U(A,null);if(!t)return()=>null;const{separatorRef:n,mergedClsPrefixRef:i}=t,a=be(),u=y(()=>e.href?"a":"span"),b=y(()=>a.value.href===e.href?"location":null);return()=>{const{value:m}=i;return k("li",{class:[`${m}-breadcrumb-item`,e.clickable&&`${m}-breadcrumb-item--clickable`]},k(u.value,{class:`${m}-breadcrumb-item__link`,"aria-current":b.value,href:e.href,onClick:e.onClick},o),k("span",{class:`${m}-breadcrumb-item__separator`,"aria-hidden":"true"},Q(o.separator,()=>{var r;return[(r=e.separator)!==null&&r!==void 0?r:n.value]})))}}}),ge={class:"flex items-center"},ke={__name:"BreadCrumb",setup(e){const o=X(),t=Y(),n=Z(),i=E([]);W(()=>t.name,r=>{i.value=a(n.permissions,r)},{immediate:!0});function a(r,c,p=[]){for(const d of r){if(d.code===c)return[...p,d];if(d.children?.length){const x=a(d.children,c,[...p,d]);if(x)return x}}return null}function u(r){r.path&&r.code!==t.name&&o.push(r.path)}function b(r=[]){return r.filter(c=>c.show).map(c=>({label:c.name,key:c.code,icon:()=>k("i",{class:c.icon})}))}function m(r){r&&r!==t.name&&o.push({name:r})}return(r,c)=>{const p=xe,d=pe,x=he;return v(),B(x,null,{default:$(()=>[s(i)?.length?(v(!0),z(I,{key:1},ee(s(i),(_,w)=>(v(),B(p,{key:_.code,clickable:!!_.path,onClick:R=>u(_)},{default:$(()=>[l(d,{options:w<s(i).length-1?b(_.children):[],onSelect:m},{default:$(()=>[h("div",ge,[h("i",{class:j([_.icon,"mr-8"])},null,2),S(" "+L(_.name),1)])]),_:2},1032,["options"])]),_:2},1032,["clickable","onClick"]))),128)):(v(),B(p,{key:0,clickable:!1},{default:$(()=>[S(L(s(t).meta.title),1)]),_:1}))]),_:1})}}},Ce={class:"ml-auto flex flex-shrink-0 items-center px-12 text-18"},$e={__name:"index",setup(e){function o(t){window.open(t)}return(t,n)=>{const i=de,a=ue;return v(),B(a,{class:"flex items-center px-12","border-b":"1px solid light_border dark:dark_border"},{default:$(()=>[l(s(oe)),l(s(ke)),h("div",Ce,[l(s(ie)),l(s(ce)),l(s(le)),h("i",{class:"i-fe:github mr-16 cursor-pointer",onClick:n[0]||(n[0]=u=>o("https://github.com/zclzone/vue-naive-admin/tree/2.x"))}),h("i",{class:"i-me:gitee mr-16 cursor-pointer",onClick:n[1]||(n[1]=u=>o("https://gitee.com/isme-admin/vue-naive-admin/tree/2.x"))}),l(i,{class:"mr-16"}),l(s(ne))])]),_:1})}}},Be={__name:"index",setup(e){return(o,t)=>(v(),z(I,null,[l(s(ae),{"border-b":"1px solid light_border dark:dark_border"}),l(s(se),{class:"cus-scroll-y mt-4 h-0 flex-1"})],64))}},we={class:"wh-full flex"},ye={class:"w-0 flex-col flex-1"},ze={class:"p-12","border-b":"1px solid light_border dark:dark_border"},Me={__name:"index",setup(e){const o=re();return(t,n)=>(v(),z("div",we,[h("aside",{class:j(["flex-col flex-shrink-0 transition-width-300",s(o).collapsed?"w-64":"w-220"]),"border-r":"1px solid light_border dark:dark_border"},[l(Be)],2),h("article",ye,[l($e,{class:"h-60 flex-shrink-0"}),h("div",ze,[l(s(me),{class:"flex-shrink-0"})]),te(t.$slots,"default")])]))}};export{Me as default};
