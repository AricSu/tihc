import{p as M,q as v,aB as We,m as X,j as C,i as x,l as S,az as D,k as g,aY as Ze,av as B,aw as Je,C as K,y as b,aZ as ue,aR as Ye,a_ as ae,w as J,F as ve,a$ as re,g as k,b0 as Qe,aC as Xe,s as eo,t as Ie,b1 as oo,a7 as Ce,x as ye,an as ze,aE as to,z as no,b2 as ro,au as io,ar as F,u as ee,c as V,o as T,b as L,n as Re,d as w,b3 as Se,b4 as Ae,b5 as lo,e as Y,J as $,a as E,aS as ao,b6 as co,M as so,K as ie,L as Q,as as uo,b7 as vo,b8 as ce,b9 as mo,aj as ho,aI as po,E as Ne,G as fo,H as Pe,I as go,at as xo,ba as bo,aV as Co,f as yo}from"./index-CQZOGmuN.js";import{_ as zo}from"./_plugin-vue_export-helper-DlAUqK2U.js";import{s as _o,r as wo,_ as He,a as Io}from"./Dropdown-Bea_11Hq.js";import{c as le,V as Ro}from"./create-CfPrN9OH.js";import{_ as So}from"./Avatar-DB8QP23x.js";const Ao=M({name:"ChevronDownFilled",render(){return v("svg",{viewBox:"0 0 16 16",fill:"none",xmlns:"http://www.w3.org/2000/svg"},v("path",{d:"M3.20041 5.73966C3.48226 5.43613 3.95681 5.41856 4.26034 5.70041L8 9.22652L11.7397 5.70041C12.0432 5.41856 12.5177 5.43613 12.7996 5.73966C13.0815 6.0432 13.0639 6.51775 12.7603 6.7996L8.51034 10.7996C8.22258 11.0668 7.77743 11.0668 7.48967 10.7996L3.23966 6.7996C2.93613 6.51775 2.91856 6.0432 3.20041 5.73966Z",fill:"currentColor"}))}}),No=M({name:"RadioButton",props:wo,setup:_o,render(){const{mergedClsPrefix:e}=this;return v("label",{class:[`${e}-radio-button`,this.mergedDisabled&&`${e}-radio-button--disabled`,this.renderSafeChecked&&`${e}-radio-button--checked`,this.focus&&[`${e}-radio-button--focus`]]},v("input",{ref:"inputRef",type:"radio",class:`${e}-radio-input`,value:this.value,name:this.mergedName,checked:this.renderSafeChecked,disabled:this.mergedDisabled,onChange:this.handleRadioInputChange,onFocus:this.handleRadioInputFocus,onBlur:this.handleRadioInputBlur}),v("div",{class:`${e}-radio-button__state-border`}),We(this.$slots.default,r=>!r&&!this.label?null:v("div",{ref:"labelRef",class:`${e}-radio__label`},r||this.label)))}}),Po=X("n-layout-sider"),U=X("n-menu"),ke=X("n-submenu"),me=X("n-menu-item-group"),_e=[C("&::before","background-color: var(--n-item-color-hover);"),g("arrow",`
 color: var(--n-arrow-color-hover);
 `),g("icon",`
 color: var(--n-item-icon-color-hover);
 `),x("menu-item-content-header",`
 color: var(--n-item-text-color-hover);
 `,[C("a",`
 color: var(--n-item-text-color-hover);
 `),g("extra",`
 color: var(--n-item-text-color-hover);
 `)])],we=[g("icon",`
 color: var(--n-item-icon-color-hover-horizontal);
 `),x("menu-item-content-header",`
 color: var(--n-item-text-color-hover-horizontal);
 `,[C("a",`
 color: var(--n-item-text-color-hover-horizontal);
 `),g("extra",`
 color: var(--n-item-text-color-hover-horizontal);
 `)])],Ho=C([x("menu",`
 background-color: var(--n-color);
 color: var(--n-item-text-color);
 overflow: hidden;
 transition: background-color .3s var(--n-bezier);
 box-sizing: border-box;
 font-size: var(--n-font-size);
 padding-bottom: 6px;
 `,[S("horizontal",`
 max-width: 100%;
 width: 100%;
 display: flex;
 overflow: hidden;
 padding-bottom: 0;
 `,[x("submenu","margin: 0;"),x("menu-item","margin: 0;"),x("menu-item-content",`
 padding: 0 20px;
 border-bottom: 2px solid #0000;
 `,[C("&::before","display: none;"),S("selected","border-bottom: 2px solid var(--n-border-color-horizontal)")]),x("menu-item-content",[S("selected",[g("icon","color: var(--n-item-icon-color-active-horizontal);"),x("menu-item-content-header",`
 color: var(--n-item-text-color-active-horizontal);
 `,[C("a","color: var(--n-item-text-color-active-horizontal);"),g("extra","color: var(--n-item-text-color-active-horizontal);")])]),S("child-active",`
 border-bottom: 2px solid var(--n-border-color-horizontal);
 `,[x("menu-item-content-header",`
 color: var(--n-item-text-color-child-active-horizontal);
 `,[C("a",`
 color: var(--n-item-text-color-child-active-horizontal);
 `),g("extra",`
 color: var(--n-item-text-color-child-active-horizontal);
 `)]),g("icon",`
 color: var(--n-item-icon-color-child-active-horizontal);
 `)]),D("disabled",[D("selected, child-active",[C("&:focus-within",we)]),S("selected",[O(null,[g("icon","color: var(--n-item-icon-color-active-hover-horizontal);"),x("menu-item-content-header",`
 color: var(--n-item-text-color-active-hover-horizontal);
 `,[C("a","color: var(--n-item-text-color-active-hover-horizontal);"),g("extra","color: var(--n-item-text-color-active-hover-horizontal);")])])]),S("child-active",[O(null,[g("icon","color: var(--n-item-icon-color-child-active-hover-horizontal);"),x("menu-item-content-header",`
 color: var(--n-item-text-color-child-active-hover-horizontal);
 `,[C("a","color: var(--n-item-text-color-child-active-hover-horizontal);"),g("extra","color: var(--n-item-text-color-child-active-hover-horizontal);")])])]),O("border-bottom: 2px solid var(--n-border-color-horizontal);",we)]),x("menu-item-content-header",[C("a","color: var(--n-item-text-color-horizontal);")])])]),D("responsive",[x("menu-item-content-header",`
 overflow: hidden;
 text-overflow: ellipsis;
 `)]),S("collapsed",[x("menu-item-content",[S("selected",[C("&::before",`
 background-color: var(--n-item-color-active-collapsed) !important;
 `)]),x("menu-item-content-header","opacity: 0;"),g("arrow","opacity: 0;"),g("icon","color: var(--n-item-icon-color-collapsed);")])]),x("menu-item",`
 height: var(--n-item-height);
 margin-top: 6px;
 position: relative;
 `),x("menu-item-content",`
 box-sizing: border-box;
 line-height: 1.75;
 height: 100%;
 display: grid;
 grid-template-areas: "icon content arrow";
 grid-template-columns: auto 1fr auto;
 align-items: center;
 cursor: pointer;
 position: relative;
 padding-right: 18px;
 transition:
 background-color .3s var(--n-bezier),
 padding-left .3s var(--n-bezier),
 border-color .3s var(--n-bezier);
 `,[C("> *","z-index: 1;"),C("&::before",`
 z-index: auto;
 content: "";
 background-color: #0000;
 position: absolute;
 left: 8px;
 right: 8px;
 top: 0;
 bottom: 0;
 pointer-events: none;
 border-radius: var(--n-border-radius);
 transition: background-color .3s var(--n-bezier);
 `),S("disabled",`
 opacity: .45;
 cursor: not-allowed;
 `),S("collapsed",[g("arrow","transform: rotate(0);")]),S("selected",[C("&::before","background-color: var(--n-item-color-active);"),g("arrow","color: var(--n-arrow-color-active);"),g("icon","color: var(--n-item-icon-color-active);"),x("menu-item-content-header",`
 color: var(--n-item-text-color-active);
 `,[C("a","color: var(--n-item-text-color-active);"),g("extra","color: var(--n-item-text-color-active);")])]),S("child-active",[x("menu-item-content-header",`
 color: var(--n-item-text-color-child-active);
 `,[C("a",`
 color: var(--n-item-text-color-child-active);
 `),g("extra",`
 color: var(--n-item-text-color-child-active);
 `)]),g("arrow",`
 color: var(--n-arrow-color-child-active);
 `),g("icon",`
 color: var(--n-item-icon-color-child-active);
 `)]),D("disabled",[D("selected, child-active",[C("&:focus-within",_e)]),S("selected",[O(null,[g("arrow","color: var(--n-arrow-color-active-hover);"),g("icon","color: var(--n-item-icon-color-active-hover);"),x("menu-item-content-header",`
 color: var(--n-item-text-color-active-hover);
 `,[C("a","color: var(--n-item-text-color-active-hover);"),g("extra","color: var(--n-item-text-color-active-hover);")])])]),S("child-active",[O(null,[g("arrow","color: var(--n-arrow-color-child-active-hover);"),g("icon","color: var(--n-item-icon-color-child-active-hover);"),x("menu-item-content-header",`
 color: var(--n-item-text-color-child-active-hover);
 `,[C("a","color: var(--n-item-text-color-child-active-hover);"),g("extra","color: var(--n-item-text-color-child-active-hover);")])])]),S("selected",[O(null,[C("&::before","background-color: var(--n-item-color-active-hover);")])]),O(null,_e)]),g("icon",`
 grid-area: icon;
 color: var(--n-item-icon-color);
 transition:
 color .3s var(--n-bezier),
 font-size .3s var(--n-bezier),
 margin-right .3s var(--n-bezier);
 box-sizing: content-box;
 display: inline-flex;
 align-items: center;
 justify-content: center;
 `),g("arrow",`
 grid-area: arrow;
 font-size: 16px;
 color: var(--n-arrow-color);
 transform: rotate(180deg);
 opacity: 1;
 transition:
 color .3s var(--n-bezier),
 transform 0.2s var(--n-bezier),
 opacity 0.2s var(--n-bezier);
 `),x("menu-item-content-header",`
 grid-area: content;
 transition:
 color .3s var(--n-bezier),
 opacity .3s var(--n-bezier);
 opacity: 1;
 white-space: nowrap;
 color: var(--n-item-text-color);
 `,[C("a",`
 outline: none;
 text-decoration: none;
 transition: color .3s var(--n-bezier);
 color: var(--n-item-text-color);
 `,[C("&::before",`
 content: "";
 position: absolute;
 left: 0;
 right: 0;
 top: 0;
 bottom: 0;
 `)]),g("extra",`
 font-size: .93em;
 color: var(--n-group-text-color);
 transition: color .3s var(--n-bezier);
 `)])]),x("submenu",`
 cursor: pointer;
 position: relative;
 margin-top: 6px;
 `,[x("menu-item-content",`
 height: var(--n-item-height);
 `),x("submenu-children",`
 overflow: hidden;
 padding: 0;
 `,[Ze({duration:".2s"})])]),x("menu-item-group",[x("menu-item-group-title",`
 margin-top: 6px;
 color: var(--n-group-text-color);
 cursor: default;
 font-size: .93em;
 height: 36px;
 display: flex;
 align-items: center;
 transition:
 padding-left .3s var(--n-bezier),
 color .3s var(--n-bezier);
 `)])]),x("menu-tooltip",[C("a",`
 color: inherit;
 text-decoration: none;
 `)]),x("menu-divider",`
 transition: background-color .3s var(--n-bezier);
 background-color: var(--n-divider-color);
 height: 1px;
 margin: 6px 18px;
 `)]);function O(e,r){return[S("hover",e,r),C("&:hover",e,r)]}const $e=M({name:"MenuOptionContent",props:{collapsed:Boolean,disabled:Boolean,title:[String,Function],icon:Function,extra:[String,Function],showArrow:Boolean,childActive:Boolean,hover:Boolean,paddingLeft:Number,selected:Boolean,maxIconSize:{type:Number,required:!0},activeIconSize:{type:Number,required:!0},iconMarginRight:{type:Number,required:!0},clsPrefix:{type:String,required:!0},onClick:Function,tmNode:{type:Object,required:!0},isEllipsisPlaceholder:Boolean},setup(e){const{props:r}=K(U);return{menuProps:r,style:b(()=>{const{paddingLeft:t}=e;return{paddingLeft:t&&`${t}px`}}),iconStyle:b(()=>{const{maxIconSize:t,activeIconSize:i,iconMarginRight:a}=e;return{width:`${t}px`,height:`${t}px`,fontSize:`${i}px`,marginRight:`${a}px`}})}},render(){const{clsPrefix:e,tmNode:r,menuProps:{renderIcon:t,renderLabel:i,renderExtra:a,expandIcon:l}}=this,s=t?t(r.rawNode):B(this.icon);return v("div",{onClick:p=>{var u;(u=this.onClick)===null||u===void 0||u.call(this,p)},role:"none",class:[`${e}-menu-item-content`,{[`${e}-menu-item-content--selected`]:this.selected,[`${e}-menu-item-content--collapsed`]:this.collapsed,[`${e}-menu-item-content--child-active`]:this.childActive,[`${e}-menu-item-content--disabled`]:this.disabled,[`${e}-menu-item-content--hover`]:this.hover}],style:this.style},s&&v("div",{class:`${e}-menu-item-content__icon`,style:this.iconStyle,role:"none"},[s]),v("div",{class:`${e}-menu-item-content-header`,role:"none"},this.isEllipsisPlaceholder?this.title:i?i(r.rawNode):B(this.title),this.extra||a?v("span",{class:`${e}-menu-item-content-header__extra`}," ",a?a(r.rawNode):B(this.extra)):null),this.showArrow?v(Je,{ariaHidden:!0,class:`${e}-menu-item-content__arrow`,clsPrefix:e},{default:()=>l?l(r.rawNode):v(Ao,null)}):null)}}),Z=8;function he(e){const r=K(U),{props:t,mergedCollapsedRef:i}=r,a=K(ke,null),l=K(me,null),s=b(()=>t.mode==="horizontal"),p=b(()=>s.value?t.dropdownPlacement:"tmNodes"in e?"right-start":"right"),u=b(()=>{var d;return Math.max((d=t.collapsedIconSize)!==null&&d!==void 0?d:t.iconSize,t.iconSize)}),h=b(()=>{var d;return!s.value&&e.root&&i.value&&(d=t.collapsedIconSize)!==null&&d!==void 0?d:t.iconSize}),z=b(()=>{if(s.value)return;const{collapsedWidth:d,indent:I,rootIndent:N}=t,{root:A,isGroup:P}=e,R=N===void 0?I:N;return A?i.value?d/2-u.value/2:R:l&&typeof l.paddingLeftRef.value=="number"?I/2+l.paddingLeftRef.value:a&&typeof a.paddingLeftRef.value=="number"?(P?I/2:I)+a.paddingLeftRef.value:0}),_=b(()=>{const{collapsedWidth:d,indent:I,rootIndent:N}=t,{value:A}=u,{root:P}=e;return s.value||!P||!i.value?Z:(N===void 0?I:N)+A+Z-(d+A)/2});return{dropdownPlacement:p,activeIconSize:h,maxIconSize:u,paddingLeft:z,iconMarginRight:_,NMenu:r,NSubmenu:a,NMenuOptionGroup:l}}const pe={internalKey:{type:[String,Number],required:!0},root:Boolean,isGroup:Boolean,level:{type:Number,required:!0},title:[String,Function],extra:[String,Function]},ko=M({name:"MenuDivider",setup(){const e=K(U),{mergedClsPrefixRef:r,isHorizontalRef:t}=e;return()=>t.value?null:v("div",{class:`${r.value}-menu-divider`})}}),Te=Object.assign(Object.assign({},pe),{tmNode:{type:Object,required:!0},disabled:Boolean,icon:Function,onClick:Function}),$o=ue(Te),To=M({name:"MenuOption",props:Te,setup(e){const r=he(e),{NSubmenu:t,NMenu:i,NMenuOptionGroup:a}=r,{props:l,mergedClsPrefixRef:s,mergedCollapsedRef:p}=i,u=t?t.mergedDisabledRef:a?a.mergedDisabledRef:{value:!1},h=b(()=>u.value||e.disabled);function z(d){const{onClick:I}=e;I&&I(d)}function _(d){h.value||(i.doSelect(e.internalKey,e.tmNode.rawNode),z(d))}return{mergedClsPrefix:s,dropdownPlacement:r.dropdownPlacement,paddingLeft:r.paddingLeft,iconMarginRight:r.iconMarginRight,maxIconSize:r.maxIconSize,activeIconSize:r.activeIconSize,mergedTheme:i.mergedThemeRef,menuProps:l,dropdownEnabled:ae(()=>e.root&&p.value&&l.mode!=="horizontal"&&!h.value),selected:ae(()=>i.mergedValueRef.value===e.internalKey),mergedDisabled:h,handleClick:_}},render(){const{mergedClsPrefix:e,mergedTheme:r,tmNode:t,menuProps:{renderLabel:i,nodeProps:a}}=this,l=a?.(t.rawNode);return v("div",Object.assign({},l,{role:"menuitem",class:[`${e}-menu-item`,l?.class]}),v(Ye,{theme:r.peers.Tooltip,themeOverrides:r.peerOverrides.Tooltip,trigger:"hover",placement:this.dropdownPlacement,disabled:!this.dropdownEnabled||this.title===void 0,internalExtraClass:["menu-tooltip"]},{default:()=>i?i(t.rawNode):B(this.title),trigger:()=>v($e,{tmNode:t,clsPrefix:e,paddingLeft:this.paddingLeft,iconMarginRight:this.iconMarginRight,maxIconSize:this.maxIconSize,activeIconSize:this.activeIconSize,selected:this.selected,title:this.title,extra:this.extra,disabled:this.mergedDisabled,icon:this.icon,onClick:this.handleClick})}))}}),Ee=Object.assign(Object.assign({},pe),{tmNode:{type:Object,required:!0},tmNodes:{type:Array,required:!0}}),Eo=ue(Ee),Mo=M({name:"MenuOptionGroup",props:Ee,setup(e){const r=he(e),{NSubmenu:t}=r,i=b(()=>t?.mergedDisabledRef.value?!0:e.tmNode.disabled);J(me,{paddingLeftRef:r.paddingLeft,mergedDisabledRef:i});const{mergedClsPrefixRef:a,props:l}=K(U);return function(){const{value:s}=a,p=r.paddingLeft.value,{nodeProps:u}=l,h=u?.(e.tmNode.rawNode);return v("div",{class:`${s}-menu-item-group`,role:"group"},v("div",Object.assign({},h,{class:[`${s}-menu-item-group-title`,h?.class],style:[h?.style||"",p!==void 0?`padding-left: ${p}px;`:""]}),B(e.title),e.extra?v(ve,null," ",B(e.extra)):null),v("div",null,e.tmNodes.map(z=>fe(z,l))))}}});function se(e){return e.type==="divider"||e.type==="render"}function Fo(e){return e.type==="divider"}function fe(e,r){const{rawNode:t}=e,{show:i}=t;if(i===!1)return null;if(se(t))return Fo(t)?v(ko,Object.assign({key:e.key},t.props)):null;const{labelField:a}=r,{key:l,level:s,isGroup:p}=e,u=Object.assign(Object.assign({},t),{title:t.title||t[a],extra:t.titleExtra||t.extra,key:l,internalKey:l,level:s,root:s===0,isGroup:p});return e.children?e.isGroup?v(Mo,re(u,Eo,{tmNode:e,tmNodes:e.children,key:l})):v(de,re(u,Oo,{key:l,rawNodes:t[r.childrenField],tmNodes:e.children,tmNode:e})):v(To,re(u,$o,{key:l,tmNode:e}))}const Me=Object.assign(Object.assign({},pe),{rawNodes:{type:Array,default:()=>[]},tmNodes:{type:Array,default:()=>[]},tmNode:{type:Object,required:!0},disabled:Boolean,icon:Function,onClick:Function,domId:String,virtualChildActive:{type:Boolean,default:void 0},isEllipsisPlaceholder:Boolean}),Oo=ue(Me),de=M({name:"Submenu",props:Me,setup(e){const r=he(e),{NMenu:t,NSubmenu:i}=r,{props:a,mergedCollapsedRef:l,mergedThemeRef:s}=t,p=b(()=>{const{disabled:d}=e;return i?.mergedDisabledRef.value||a.disabled?!0:d}),u=k(!1);J(ke,{paddingLeftRef:r.paddingLeft,mergedDisabledRef:p}),J(me,null);function h(){const{onClick:d}=e;d&&d()}function z(){p.value||(l.value||t.toggleExpand(e.internalKey),h())}function _(d){u.value=d}return{menuProps:a,mergedTheme:s,doSelect:t.doSelect,inverted:t.invertedRef,isHorizontal:t.isHorizontalRef,mergedClsPrefix:t.mergedClsPrefixRef,maxIconSize:r.maxIconSize,activeIconSize:r.activeIconSize,iconMarginRight:r.iconMarginRight,dropdownPlacement:r.dropdownPlacement,dropdownShow:u,paddingLeft:r.paddingLeft,mergedDisabled:p,mergedValue:t.mergedValueRef,childActive:ae(()=>{var d;return(d=e.virtualChildActive)!==null&&d!==void 0?d:t.activePathRef.value.includes(e.internalKey)}),collapsed:b(()=>a.mode==="horizontal"?!1:l.value?!0:!t.mergedExpandedKeysRef.value.includes(e.internalKey)),dropdownEnabled:b(()=>!p.value&&(a.mode==="horizontal"||l.value)),handlePopoverShowChange:_,handleClick:z}},render(){var e;const{mergedClsPrefix:r,menuProps:{renderIcon:t,renderLabel:i}}=this,a=()=>{const{isHorizontal:s,paddingLeft:p,collapsed:u,mergedDisabled:h,maxIconSize:z,activeIconSize:_,title:d,childActive:I,icon:N,handleClick:A,menuProps:{nodeProps:P},dropdownShow:R,iconMarginRight:oe,tmNode:j,mergedClsPrefix:G,isEllipsisPlaceholder:te,extra:q}=this,H=P?.(j.rawNode);return v("div",Object.assign({},H,{class:[`${G}-menu-item`,H?.class],role:"menuitem"}),v($e,{tmNode:j,paddingLeft:p,collapsed:u,disabled:h,iconMarginRight:oe,maxIconSize:z,activeIconSize:_,title:d,extra:q,showArrow:!s,childActive:I,clsPrefix:G,icon:N,hover:R,onClick:A,isEllipsisPlaceholder:te}))},l=()=>v(Qe,null,{default:()=>{const{tmNodes:s,collapsed:p}=this;return p?null:v("div",{class:`${r}-submenu-children`,role:"menu"},s.map(u=>fe(u,this.menuProps)))}});return this.root?v(He,Object.assign({size:"large",trigger:"hover"},(e=this.menuProps)===null||e===void 0?void 0:e.dropdownProps,{themeOverrides:this.mergedTheme.peerOverrides.Dropdown,theme:this.mergedTheme.peers.Dropdown,builtinThemeOverrides:{fontSizeLarge:"14px",optionIconSizeLarge:"18px"},value:this.mergedValue,disabled:!this.dropdownEnabled,placement:this.dropdownPlacement,keyField:this.menuProps.keyField,labelField:this.menuProps.labelField,childrenField:this.menuProps.childrenField,onUpdateShow:this.handlePopoverShowChange,options:this.rawNodes,onSelect:this.doSelect,inverted:this.inverted,renderIcon:t,renderLabel:i}),{default:()=>v("div",{class:`${r}-submenu`,role:"menu","aria-expanded":!this.collapsed,id:this.domId},a(),this.isHorizontal?null:l())}):v("div",{class:`${r}-submenu`,role:"menu","aria-expanded":!this.collapsed,id:this.domId},a(),l())}}),Ko=Object.assign(Object.assign({},Ie.props),{options:{type:Array,default:()=>[]},collapsed:{type:Boolean,default:void 0},collapsedWidth:{type:Number,default:48},iconSize:{type:Number,default:20},collapsedIconSize:{type:Number,default:24},rootIndent:Number,indent:{type:Number,default:32},labelField:{type:String,default:"label"},keyField:{type:String,default:"key"},childrenField:{type:String,default:"children"},disabledField:{type:String,default:"disabled"},defaultExpandAll:Boolean,defaultExpandedKeys:Array,expandedKeys:Array,value:[String,Number],defaultValue:{type:[String,Number],default:null},mode:{type:String,default:"vertical"},watchProps:{type:Array,default:void 0},disabled:Boolean,show:{type:Boolean,default:!0},inverted:Boolean,"onUpdate:expandedKeys":[Function,Array],onUpdateExpandedKeys:[Function,Array],onUpdateValue:[Function,Array],"onUpdate:value":[Function,Array],expandIcon:Function,renderIcon:Function,renderLabel:Function,renderExtra:Function,dropdownProps:Object,accordion:Boolean,nodeProps:Function,dropdownPlacement:{type:String,default:"bottom"},responsive:Boolean,items:Array,onOpenNamesChange:[Function,Array],onSelect:[Function,Array],onExpandedNamesChange:[Function,Array],expandedNames:Array,defaultExpandedNames:Array}),Lo=M({name:"Menu",inheritAttrs:!1,props:Ko,setup(e){const{mergedClsPrefixRef:r,inlineThemeDisabled:t}=eo(e),i=Ie("Menu","-menu",Ho,oo,e,r),a=K(Po,null),l=b(()=>{var c;const{collapsed:f}=e;if(f!==void 0)return f;if(a){const{collapseModeRef:o,collapsedRef:m}=a;if(o.value==="width")return(c=m.value)!==null&&c!==void 0?c:!1}return!1}),s=b(()=>{const{keyField:c,childrenField:f,disabledField:o}=e;return le(e.items||e.options,{getIgnored(m){return se(m)},getChildren(m){return m[f]},getDisabled(m){return m[o]},getKey(m){var y;return(y=m[c])!==null&&y!==void 0?y:m.name}})}),p=b(()=>new Set(s.value.treeNodes.map(c=>c.key))),{watchProps:u}=e,h=k(null);u?.includes("defaultValue")?Ce(()=>{h.value=e.defaultValue}):h.value=e.defaultValue;const z=ye(e,"value"),_=ze(z,h),d=k([]),I=()=>{d.value=e.defaultExpandAll?s.value.getNonLeafKeys():e.defaultExpandedNames||e.defaultExpandedKeys||s.value.getPath(_.value,{includeSelf:!1}).keyPath};u?.includes("defaultExpandedKeys")?Ce(I):I();const N=to(e,["expandedNames","expandedKeys"]),A=ze(N,d),P=b(()=>s.value.treeNodes),R=b(()=>s.value.getPath(_.value).keyPath);J(U,{props:e,mergedCollapsedRef:l,mergedThemeRef:i,mergedValueRef:_,mergedExpandedKeysRef:A,activePathRef:R,mergedClsPrefixRef:r,isHorizontalRef:b(()=>e.mode==="horizontal"),invertedRef:ye(e,"inverted"),doSelect:oe,toggleExpand:G});function oe(c,f){const{"onUpdate:value":o,onUpdateValue:m,onSelect:y}=e;m&&F(m,c,f),o&&F(o,c,f),y&&F(y,c,f),h.value=c}function j(c){const{"onUpdate:expandedKeys":f,onUpdateExpandedKeys:o,onExpandedNamesChange:m,onOpenNamesChange:y}=e;f&&F(f,c),o&&F(o,c),m&&F(m,c),y&&F(y,c),d.value=c}function G(c){const f=Array.from(A.value),o=f.findIndex(m=>m===c);if(~o)f.splice(o,1);else{if(e.accordion&&p.value.has(c)){const m=f.findIndex(y=>p.value.has(y));m>-1&&f.splice(m,1)}f.push(c)}j(f)}const te=c=>{const f=s.value.getPath(c??_.value,{includeSelf:!1}).keyPath;if(!f.length)return;const o=Array.from(A.value),m=new Set([...o,...f]);e.accordion&&p.value.forEach(y=>{m.has(y)&&!f.includes(y)&&m.delete(y)}),j(Array.from(m))},q=b(()=>{const{inverted:c}=e,{common:{cubicBezierEaseInOut:f},self:o}=i.value,{borderRadius:m,borderColorHorizontal:y,fontSize:Ue,itemHeight:Ge,dividerColor:qe}=o,n={"--n-divider-color":qe,"--n-bezier":f,"--n-font-size":Ue,"--n-border-color-horizontal":y,"--n-border-radius":m,"--n-item-height":Ge};return c?(n["--n-group-text-color"]=o.groupTextColorInverted,n["--n-color"]=o.colorInverted,n["--n-item-text-color"]=o.itemTextColorInverted,n["--n-item-text-color-hover"]=o.itemTextColorHoverInverted,n["--n-item-text-color-active"]=o.itemTextColorActiveInverted,n["--n-item-text-color-child-active"]=o.itemTextColorChildActiveInverted,n["--n-item-text-color-child-active-hover"]=o.itemTextColorChildActiveInverted,n["--n-item-text-color-active-hover"]=o.itemTextColorActiveHoverInverted,n["--n-item-icon-color"]=o.itemIconColorInverted,n["--n-item-icon-color-hover"]=o.itemIconColorHoverInverted,n["--n-item-icon-color-active"]=o.itemIconColorActiveInverted,n["--n-item-icon-color-active-hover"]=o.itemIconColorActiveHoverInverted,n["--n-item-icon-color-child-active"]=o.itemIconColorChildActiveInverted,n["--n-item-icon-color-child-active-hover"]=o.itemIconColorChildActiveHoverInverted,n["--n-item-icon-color-collapsed"]=o.itemIconColorCollapsedInverted,n["--n-item-text-color-horizontal"]=o.itemTextColorHorizontalInverted,n["--n-item-text-color-hover-horizontal"]=o.itemTextColorHoverHorizontalInverted,n["--n-item-text-color-active-horizontal"]=o.itemTextColorActiveHorizontalInverted,n["--n-item-text-color-child-active-horizontal"]=o.itemTextColorChildActiveHorizontalInverted,n["--n-item-text-color-child-active-hover-horizontal"]=o.itemTextColorChildActiveHoverHorizontalInverted,n["--n-item-text-color-active-hover-horizontal"]=o.itemTextColorActiveHoverHorizontalInverted,n["--n-item-icon-color-horizontal"]=o.itemIconColorHorizontalInverted,n["--n-item-icon-color-hover-horizontal"]=o.itemIconColorHoverHorizontalInverted,n["--n-item-icon-color-active-horizontal"]=o.itemIconColorActiveHorizontalInverted,n["--n-item-icon-color-active-hover-horizontal"]=o.itemIconColorActiveHoverHorizontalInverted,n["--n-item-icon-color-child-active-horizontal"]=o.itemIconColorChildActiveHorizontalInverted,n["--n-item-icon-color-child-active-hover-horizontal"]=o.itemIconColorChildActiveHoverHorizontalInverted,n["--n-arrow-color"]=o.arrowColorInverted,n["--n-arrow-color-hover"]=o.arrowColorHoverInverted,n["--n-arrow-color-active"]=o.arrowColorActiveInverted,n["--n-arrow-color-active-hover"]=o.arrowColorActiveHoverInverted,n["--n-arrow-color-child-active"]=o.arrowColorChildActiveInverted,n["--n-arrow-color-child-active-hover"]=o.arrowColorChildActiveHoverInverted,n["--n-item-color-hover"]=o.itemColorHoverInverted,n["--n-item-color-active"]=o.itemColorActiveInverted,n["--n-item-color-active-hover"]=o.itemColorActiveHoverInverted,n["--n-item-color-active-collapsed"]=o.itemColorActiveCollapsedInverted):(n["--n-group-text-color"]=o.groupTextColor,n["--n-color"]=o.color,n["--n-item-text-color"]=o.itemTextColor,n["--n-item-text-color-hover"]=o.itemTextColorHover,n["--n-item-text-color-active"]=o.itemTextColorActive,n["--n-item-text-color-child-active"]=o.itemTextColorChildActive,n["--n-item-text-color-child-active-hover"]=o.itemTextColorChildActiveHover,n["--n-item-text-color-active-hover"]=o.itemTextColorActiveHover,n["--n-item-icon-color"]=o.itemIconColor,n["--n-item-icon-color-hover"]=o.itemIconColorHover,n["--n-item-icon-color-active"]=o.itemIconColorActive,n["--n-item-icon-color-active-hover"]=o.itemIconColorActiveHover,n["--n-item-icon-color-child-active"]=o.itemIconColorChildActive,n["--n-item-icon-color-child-active-hover"]=o.itemIconColorChildActiveHover,n["--n-item-icon-color-collapsed"]=o.itemIconColorCollapsed,n["--n-item-text-color-horizontal"]=o.itemTextColorHorizontal,n["--n-item-text-color-hover-horizontal"]=o.itemTextColorHoverHorizontal,n["--n-item-text-color-active-horizontal"]=o.itemTextColorActiveHorizontal,n["--n-item-text-color-child-active-horizontal"]=o.itemTextColorChildActiveHorizontal,n["--n-item-text-color-child-active-hover-horizontal"]=o.itemTextColorChildActiveHoverHorizontal,n["--n-item-text-color-active-hover-horizontal"]=o.itemTextColorActiveHoverHorizontal,n["--n-item-icon-color-horizontal"]=o.itemIconColorHorizontal,n["--n-item-icon-color-hover-horizontal"]=o.itemIconColorHoverHorizontal,n["--n-item-icon-color-active-horizontal"]=o.itemIconColorActiveHorizontal,n["--n-item-icon-color-active-hover-horizontal"]=o.itemIconColorActiveHoverHorizontal,n["--n-item-icon-color-child-active-horizontal"]=o.itemIconColorChildActiveHorizontal,n["--n-item-icon-color-child-active-hover-horizontal"]=o.itemIconColorChildActiveHoverHorizontal,n["--n-arrow-color"]=o.arrowColor,n["--n-arrow-color-hover"]=o.arrowColorHover,n["--n-arrow-color-active"]=o.arrowColorActive,n["--n-arrow-color-active-hover"]=o.arrowColorActiveHover,n["--n-arrow-color-child-active"]=o.arrowColorChildActive,n["--n-arrow-color-child-active-hover"]=o.arrowColorChildActiveHover,n["--n-item-color-hover"]=o.itemColorHover,n["--n-item-color-active"]=o.itemColorActive,n["--n-item-color-active-hover"]=o.itemColorActiveHover,n["--n-item-color-active-collapsed"]=o.itemColorActiveCollapsed),n}),H=t?no("menu",b(()=>e.inverted?"a":"b"),q,e):void 0,ne=ro(),ge=k(null),Fe=k(null);let xe=!0;const be=()=>{var c;xe?xe=!1:(c=ge.value)===null||c===void 0||c.sync({showAllItemsBeforeCalculate:!0})};function Oe(){return document.getElementById(ne)}const W=k(-1);function Ke(c){W.value=e.options.length-c}function Le(c){c||(W.value=-1)}const Be=b(()=>{const c=W.value;return{children:c===-1?[]:e.options.slice(c)}}),je=b(()=>{const{childrenField:c,disabledField:f,keyField:o}=e;return le([Be.value],{getIgnored(m){return se(m)},getChildren(m){return m[c]},getDisabled(m){return m[f]},getKey(m){var y;return(y=m[o])!==null&&y!==void 0?y:m.name}})}),De=b(()=>le([{}]).treeNodes[0]);function Ve(){var c;if(W.value===-1)return v(de,{root:!0,level:0,key:"__ellpisisGroupPlaceholder__",internalKey:"__ellpisisGroupPlaceholder__",title:"···",tmNode:De.value,domId:ne,isEllipsisPlaceholder:!0});const f=je.value.treeNodes[0],o=R.value,m=!!(!((c=f.children)===null||c===void 0)&&c.some(y=>o.includes(y.key)));return v(de,{level:0,root:!0,key:"__ellpisisGroup__",internalKey:"__ellpisisGroup__",title:"···",virtualChildActive:m,tmNode:f,domId:ne,rawNodes:f.rawNode.children||[],tmNodes:f.children||[],isEllipsisPlaceholder:!0})}return{mergedClsPrefix:r,controlledExpandedKeys:N,uncontrolledExpanededKeys:d,mergedExpandedKeys:A,uncontrolledValue:h,mergedValue:_,activePath:R,tmNodes:P,mergedTheme:i,mergedCollapsed:l,cssVars:t?void 0:q,themeClass:H?.themeClass,overflowRef:ge,counterRef:Fe,updateCounter:()=>{},onResize:be,onUpdateOverflow:Le,onUpdateCount:Ke,renderCounter:Ve,getCounter:Oe,onRender:H?.onRender,showOption:te,deriveResponsiveState:be}},render(){const{mergedClsPrefix:e,mode:r,themeClass:t,onRender:i}=this;i?.();const a=()=>this.tmNodes.map(u=>fe(u,this.$props)),s=r==="horizontal"&&this.responsive,p=()=>v("div",io(this.$attrs,{role:r==="horizontal"?"menubar":"menu",class:[`${e}-menu`,t,`${e}-menu--${r}`,s&&`${e}-menu--responsive`,this.mergedCollapsed&&`${e}-menu--collapsed`],style:this.cssVars}),s?v(Ro,{ref:"overflowRef",onUpdateOverflow:this.onUpdateOverflow,getCounter:this.getCounter,onUpdateCount:this.onUpdateCount,updateCounter:this.updateCounter,style:{width:"100%",display:"flex",overflow:"hidden"}},{default:a,counter:this.renderCounter}):a());return s?v(Xe,{onResize:this.onResize},{default:p}):p()}}),nt={__name:"MenuCollapse",setup(e){const r=ee();return(t,i)=>(T(),V("div",{id:"menu-collapse",class:"f-c-c cursor-pointer rounded-4 auto-bg-hover p-6 text-22 transition-all-300",onClick:i[0]||(i[0]=(...a)=>w(r).switchCollapsed&&w(r).switchCollapsed(...a))},[L("i",{class:Re(w(r).collapsed?"i-line-md-menu-unfold-left":"i-line-md-menu-fold-left")},null,2)]))}},Bo={class:"flex"},jo={__name:"RoleSelect",setup(e,{expose:r}){const t=Se(),i=Ae(),a=k(t.roles||[]),l=k(t.currentRole?.code??a.value[0]?.code??""),[s,p]=lo();function u(_){s.value?.open({..._})}async function h(){try{p.value=!0;const{data:_}=await ce.switchCurrentRole(l.value);await i.switchCurrentRole(_),p.value=!1,$message.success("切换成功"),s.value?.handleOk()}catch(_){return console.error(_),p.value=!1,!1}}async function z(){await ce.logout(),i.logout(),s.value?.close(),$message.success("已退出登录")}return r({open:u}),(_,d)=>{const I=No,N=co,A=Io,P=uo;return T(),Y(w(vo),{ref_key:"modalRef",ref:s,title:"请选择角色",width:"360px",class:"p-12"},{footer:$(()=>[L("div",Bo,[E(P,{class:"flex-1",size:"large",onClick:d[1]||(d[1]=R=>z())},{default:$(()=>[...d[2]||(d[2]=[ie(" 退出登录 ",-1)])]),_:1}),E(P,{loading:w(p),class:"ml-20 flex-1",type:"primary",size:"large",disabled:w(t).currentRole?.code===w(l),onClick:h},{default:$(()=>[...d[3]||(d[3]=[ie(" 确认 ",-1)])]),_:1},8,["loading","disabled"])])]),default:$(()=>[E(A,{value:w(l),"onUpdate:value":d[0]||(d[0]=R=>ao(l)?l.value=R:null),class:"cus-scroll-y max-h-420 w-full py-16"},{default:$(()=>[E(N,{vertical:"",size:24,class:"mx-12"},{default:$(()=>[(T(!0),V(ve,null,so(w(a),R=>(T(),Y(I,{key:R.id,class:Re(["h-36 w-full text-center text-16 leading-36",{"bg-primary! color-white!":R.code===w(l)}]),value:R.code},{default:$(()=>[ie(Q(R.name),1)]),_:2},1032,["class","value"]))),128))]),_:1})]),_:1},8,["value"])]),_:1},512)}}},Do="/assets/isme-D6AR05SU.png",Vo={},Uo={class:"h-32 w-32 rounded-4 bg-primary"};function Go(e,r){return T(),V("div",Uo,[...r[0]||(r[0]=[L("img",{src:Do,alt:"Logo"},null,-1)])])}const qo=zo(Vo,[["render",Go]]),rt={__name:"SideLogo",setup(e){const r="TiHC",t=ee();return(i,a)=>{const l=qo,s=mo("router-link");return T(),Y(s,{class:"h-60 f-c-c",to:"/"},{default:$(()=>[E(l),ho(L("h2",{class:"ml-10 max-w-140 flex-shrink-0 text-16 color-primary font-bold"},Q(w(r)),513),[[po,!w(t).collapsed]])]),_:1})}}},it={__name:"SideMenu",setup(e){const r=Ne(),t=fo(),i=ee(),a=Pe(),l=b(()=>t.meta?.parentKey||t.name),s=k(null);go(t,async()=>{await xo(),s.value?.showOption()});function p(u,h){const z=i.layout==="extension";if(bo(h.originPath))$dialog.confirm({type:"info",title:"请选择打开方式",positiveText:"外链打开",negativeText:"在本站内嵌打开",confirm(){window.open(h.originPath),z&&(i.collapsed=!0)},cancel:()=>{r.push(h.path),z&&(i.collapsed=!0)}});else{if(!h.path)return;r.push(h.path),z&&(i.collapsed=!0)}}return(u,h)=>{const z=Lo;return T(),Y(z,{ref_key:"menu",ref:s,class:"side-menu",accordion:"",indent:18,"collapsed-icon-size":22,"collapsed-width":64,collapsed:w(i).collapsed,options:w(a).menus,value:w(l),"onUpdate:value":p},null,8,["collapsed","options","value"])}}},Wo={id:"user-dropdown",class:"flex cursor-pointer items-center"},Zo={key:0,class:"ml-12 flex-col flex-shrink-0 items-center"},Jo={class:"text-14"},Yo={class:"text-12 opacity-50"},lt={__name:"UserAvatar",setup(e){const r=Ne(),t=Se(),i=ee(),a=Ae(),l=Pe(),s=Co([{label:"个人资料",key:"profile",icon:()=>v("i",{class:"i-material-symbols:person-outline text-14"}),show:b(()=>l.accessRoutes?.some(h=>h.path==="/profile"))},{label:"切换角色",key:"toggleRole",icon:()=>v("i",{class:"i-basil:exchange-solid text-14"}),show:b(()=>t.roles.length>1)},{label:"退出登录",key:"logout",icon:()=>v("i",{class:"i-mdi:exit-to-app text-14"})}]),p=k(null);function u(h){switch(h){case"profile":r.push("/profile"),i.layout==="extension"&&(i.collapsed=!0);break;case"toggleRole":p.value?.open({onOk(){location.reload()}});break;case"logout":$dialog.confirm({title:"提示",type:"info",content:"确认退出？",async confirm(){try{await ce.logout()}catch(z){console.error(z)}a.logout(),$message.success("已退出登录")}})}}return(h,z)=>{const _=So,d=He;return T(),V(ve,null,[E(d,{options:w(s),onSelect:u},{default:$(()=>[L("div",Wo,[E(_,{round:"",size:36,src:w(t).avatar},null,8,["src"]),w(t).userInfo?(T(),V("div",Zo,[L("span",Jo,Q(w(t).nickName??w(t).username),1),L("span",Yo,"["+Q(w(t).currentRole?.name)+"]",1)])):yo("",!0)])]),_:1},8,["options"]),E(w(jo),{ref_key:"roleSelectRef",ref:p},null,512)],64)}}};export{rt as _,it as a,lt as b,nt as c};
