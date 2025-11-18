import{_ as D}from"./CommonPage-6jwfclzC.js";import{j as O,i as k,bv as K,l as L,p as M,q as d,bK as W,ai as U,s as q,t as V,bL as A,y as B,bM as F,ao as H,z as J,aE as X,g as x,a7 as Y,I as G,e as P,o as z,J as l,b as y,a as o,d as n,aS as Q,c as R,L as m,K as p,as as C,n as Z,F as ee,at as te}from"./index-CQZOGmuN.js";import{_ as ne}from"./index-a76VWflo.js";import{a as w,_ as se}from"./ResAddOrEdit-q7wouPOU.js";import ie from"./MenuTree-exLI2j0W.js";import{_ as ae,a as oe}from"./DescriptionsItem-DIH_9yv-.js";import{_ as le}from"./Select-YXfe3yCA.js";import{N as re}from"./Switch-Do9MmNvN.js";import"./TheFooter-CwTjSf-x.js";import"./_plugin-vue_export-helper-DlAUqK2U.js";import"./AppCard-BysSpdtI.js";import"./Checkbox-DVGPFB5x.js";import"./Dropdown-Bea_11Hq.js";import"./create-CfPrN9OH.js";import"./Input-BM48kO3D.js";import"./Eye-CMZyCEZw.js";import"./download-C2161hUv.js";import"./_isme_icons-CUZYZBf_.js";import"./useForm-yb3qavLy.js";import"./QuestionLabel-OG3pKipG.js";import"./Tree-CLyBnlGJ.js";import"./InputNumber-CpqoW5xr.js";import"./Add-C3g-VeQk.js";import"./Tag-Dxmn7TKr.js";const ce=O([O("@keyframes spin-rotate",`
 from {
 transform: rotate(0);
 }
 to {
 transform: rotate(360deg);
 }
 `),k("spin-container",`
 position: relative;
 `,[k("spin-body",`
 position: absolute;
 top: 50%;
 left: 50%;
 transform: translateX(-50%) translateY(-50%);
 `,[K()])]),k("spin-body",`
 display: inline-flex;
 align-items: center;
 justify-content: center;
 flex-direction: column;
 `),k("spin",`
 display: inline-flex;
 height: var(--n-size);
 width: var(--n-size);
 font-size: var(--n-size);
 color: var(--n-color);
 `,[L("rotate",`
 animation: spin-rotate 2s linear infinite;
 `)]),k("spin-description",`
 display: inline-block;
 font-size: var(--n-font-size);
 color: var(--n-text-color);
 transition: color .3s var(--n-bezier);
 margin-top: 8px;
 `),k("spin-content",`
 opacity: 1;
 transition: opacity .3s var(--n-bezier);
 pointer-events: all;
 `,[L("spinning",`
 user-select: none;
 -webkit-user-select: none;
 pointer-events: none;
 opacity: var(--n-opacity-spinning);
 `)])]),de={small:20,medium:18,large:16},ue=Object.assign(Object.assign({},V.props),{contentClass:String,contentStyle:[Object,String],description:String,stroke:String,size:{type:[String,Number],default:"medium"},show:{type:Boolean,default:!0},strokeWidth:Number,rotate:{type:Boolean,default:!0},spinning:{type:Boolean,validator:()=>!0,default:void 0},delay:Number}),me=M({name:"Spin",props:ue,slots:Object,setup(r){const{mergedClsPrefixRef:f,inlineThemeDisabled:a}=q(r),i=V("Spin","-spin",ce,A,r,f),t=B(()=>{const{size:c}=r,{common:{cubicBezierEaseInOut:v},self:g}=i.value,{opacitySpinning:S,color:$,textColor:e}=g,s=typeof c=="number"?F(c):g[H("size",c)];return{"--n-bezier":v,"--n-opacity-spinning":S,"--n-size":s,"--n-color":$,"--n-text-color":e}}),u=a?J("spin",B(()=>{const{size:c}=r;return typeof c=="number"?String(c):c[0]}),t,r):void 0,_=X(r,["spinning","show"]),b=x(!1);return Y(c=>{let v;if(_.value){const{delay:g}=r;if(g){v=window.setTimeout(()=>{b.value=!0},g),c(()=>{clearTimeout(v)});return}}b.value=_.value}),{mergedClsPrefix:f,active:b,mergedStrokeWidth:B(()=>{const{strokeWidth:c}=r;if(c!==void 0)return c;const{size:v}=r;return de[typeof v=="number"?"medium":v]}),cssVars:a?void 0:t,themeClass:u?.themeClass,onRender:u?.onRender}},render(){var r,f;const{$slots:a,mergedClsPrefix:i,description:t}=this,u=a.icon&&this.rotate,_=(t||a.description)&&d("div",{class:`${i}-spin-description`},t||((r=a.description)===null||r===void 0?void 0:r.call(a))),b=a.icon?d("div",{class:[`${i}-spin-body`,this.themeClass]},d("div",{class:[`${i}-spin`,u&&`${i}-spin--rotate`],style:a.default?"":this.cssVars},a.icon()),_):d("div",{class:[`${i}-spin-body`,this.themeClass]},d(W,{clsPrefix:i,style:a.default?"":this.cssVars,stroke:this.stroke,"stroke-width":this.mergedStrokeWidth,class:`${i}-spin`}),_);return(f=this.onRender)===null||f===void 0||f.call(this),a.default?d("div",{class:[`${i}-spin-container`,this.themeClass],style:this.cssVars},d("div",{class:[`${i}-spin-content`,this.active&&`${i}-spin-content--spinning`,this.contentClass],style:this.contentStyle},a),d(U,{name:"fade-in-transition"},{default:()=>this.active?b:null})):b}}),pe={class:"flex"},fe={class:"ml-40 w-0 flex-1"},_e={class:"flex justify-between"},he={class:"mb-12"},ye={key:0,class:"flex items-center"},ve={class:"opacity-50"},be={key:1},ge={class:"mt-32 flex justify-between"},Ae={__name:"index",setup(r){const f=x([]),a=x(!1),i=x(null),t=x(null);async function u(e){if(e?.type==="BUTTON"){i.value.handleSearch();return}a.value=!0;const s=await w.getMenuTree();f.value=s?.data||[],a.value=!1,e&&(t.value=e)}u();const _=x(null);function b(e={}){_.value?.handleOpen({action:"edit",title:`编辑菜单 - ${e.name}`,row:e,okText:"保存"})}const c=[{title:"名称",key:"name"},{title:"编码",key:"code"},{title:"状态",key:"enable",render:e=>d(re,{size:"small",rubberBand:!1,value:e.enable,loading:!!e.enableLoading,onUpdateValue:()=>$(e)},{checked:()=>"启用",unchecked:()=>"停用"})},{title:"操作",key:"actions",width:320,align:"right",fixed:"right",render(e){return[d(C,{size:"small",type:"primary",style:"margin-left: 12px;",onClick:()=>g(e)},{default:()=>"编辑",icon:()=>d("i",{class:"i-material-symbols:edit-outline text-14"})}),d(C,{size:"small",type:"error",style:"margin-left: 12px;",onClick:()=>S(e.id)},{default:()=>"删除",icon:()=>d("i",{class:"i-material-symbols:delete-outline text-14"})})]}}];G(()=>t.value,async e=>{await te(),e&&i.value.handleSearch()});function v(){_.value?.handleOpen({action:"add",title:"新增按钮",row:{type:"BUTTON",parentId:t.value.id},okText:"保存"})}function g(e){_.value?.handleOpen({action:"edit",title:`编辑按钮 - ${e.name}`,row:e,okText:"保存"})}function S(e){const s=$dialog.warning({content:"确定删除？",title:"提示",positiveText:"确定",negativeText:"取消",async onPositiveClick(){try{s.loading=!0,await w.deletePermission(e),$message.success("删除成功"),i.value.handleSearch(),s.loading=!1}catch(T){console.error(T),s.loading=!1}}})}async function $(e){try{e.enableLoading=!0,await w.savePermission(e.id,{enable:!e.enable}),$message.success("操作成功"),i.value?.handleSearch(),e.enableLoading=!1}catch(s){console.error(s),e.enableLoading=!1}}return(e,s)=>{const T=me,h=ae,j=oe,E=le,I=D;return z(),P(I,null,{default:l(()=>[y("div",pe,[o(T,{size:"small",show:n(a)},{default:l(()=>[o(ie,{"current-menu":n(t),"onUpdate:currentMenu":s[0]||(s[0]=N=>Q(t)?t.value=N:null),class:"w-320 shrink-0","tree-data":n(f),onRefresh:u},null,8,["current-menu","tree-data"])]),_:1},8,["show"]),y("div",fe,[n(t)?(z(),R(ee,{key:0},[y("div",_e,[y("h3",he,m(n(t).name),1),o(n(C),{size:"small",type:"primary",onClick:s[1]||(s[1]=N=>b(n(t)))},{default:l(()=>[...s[2]||(s[2]=[y("i",{class:"i-material-symbols:edit-outline mr-4 text-14"},null,-1),p(" 编辑 ",-1)])]),_:1})]),o(j,{"label-placement":"left",bordered:"",column:2},{default:l(()=>[o(h,{label:"编码"},{default:l(()=>[p(m(n(t).code),1)]),_:1}),o(h,{label:"名称"},{default:l(()=>[p(m(n(t).name),1)]),_:1}),o(h,{label:"路由地址"},{default:l(()=>[p(m(n(t).path??"--"),1)]),_:1}),o(h,{label:"组件路径"},{default:l(()=>[p(m(n(t).component??"--"),1)]),_:1}),o(h,{label:"菜单图标"},{default:l(()=>[n(t).icon?(z(),R("span",ye,[y("i",{class:Z(`${n(t).icon}?mask text-22 mr-8`)},null,2),y("span",ve,m(n(t).icon),1)])):(z(),R("span",be,"无"))]),_:1}),o(h,{label:"layout"},{default:l(()=>[p(m(n(t).layout||"跟随系统"),1)]),_:1}),o(h,{label:"是否显示"},{default:l(()=>[p(m(n(t).show?"是":"否"),1)]),_:1}),o(h,{label:"是否启用"},{default:l(()=>[p(m(n(t).enable?"是":"否"),1)]),_:1}),o(h,{label:"KeepAlive"},{default:l(()=>[p(m(n(t).keepAlive?"是":"否"),1)]),_:1}),o(h,{label:"排序"},{default:l(()=>[p(m(n(t).order??"--"),1)]),_:1})]),_:1}),y("div",ge,[s[4]||(s[4]=y("h3",{class:"mb-12"}," 按钮 ",-1)),o(n(C),{size:"small",type:"primary",onClick:v},{default:l(()=>[...s[3]||(s[3]=[y("i",{class:"i-fe:plus mr-4 text-14"},null,-1),p(" 新增 ",-1)])]),_:1})]),o(n(ne),{ref_key:"$table",ref:i,columns:c,"scroll-x":-1,"get-data":n(w).getButtons,"query-items":{parentId:n(t).id}},null,8,["get-data","query-items"])],64)):(z(),P(E,{key:1,class:"h-450 f-c-c",size:"large",description:"请选择菜单查看详情"}))])]),o(se,{ref_key:"modalRef",ref:_,menus:n(f),onRefresh:u},null,8,["menus"])]),_:1})}}};export{Ae as default};
