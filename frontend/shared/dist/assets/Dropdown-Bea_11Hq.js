import{aV as ze,cj as Ie,ck as $e,bi as Ke,a6 as ee,cl as Be,a5 as oe,I as se,g as O,p as V,q as s,m as re,C as q,al as be,x as T,an as ue,a_ as Q,s as ie,ar as W,i as I,k as $,l as C,j as A,az as ne,aA as Fe,bR as Oe,t as Y,c8 as De,w as Z,bJ as Te,y as x,ao as M,z as ce,av as te,ac as ge,au as fe,cm as Ae,bB as je,ae as Ee,af as Ve,ag as Le,ai as Me,cn as me,F as Ue,bZ as He,co as Ge,cp as qe,cq as We,ad as Xe,bX as Ze,a$ as Je,cr as Qe,c6 as we}from"./index-CQZOGmuN.js";import{h as he,c as Ye}from"./create-CfPrN9OH.js";function eo(e={},o){const r=ze({ctrl:!1,command:!1,win:!1,shift:!1,tab:!1}),{keydown:i,keyup:t}=e,n=a=>{switch(a.key){case"Control":r.ctrl=!0;break;case"Meta":r.command=!0,r.win=!0;break;case"Shift":r.shift=!0;break;case"Tab":r.tab=!0;break}i!==void 0&&Object.keys(i).forEach(v=>{if(v!==a.key)return;const f=i[v];if(typeof f=="function")f(a);else{const{stop:R=!1,prevent:g=!1}=f;R&&a.stopPropagation(),g&&a.preventDefault(),f.handler(a)}})},d=a=>{switch(a.key){case"Control":r.ctrl=!1;break;case"Meta":r.command=!1,r.win=!1;break;case"Shift":r.shift=!1;break;case"Tab":r.tab=!1;break}t!==void 0&&Object.keys(t).forEach(v=>{if(v!==a.key)return;const f=t[v];if(typeof f=="function")f(a);else{const{stop:R=!1,prevent:g=!1}=f;R&&a.stopPropagation(),g&&a.preventDefault(),f.handler(a)}})},l=()=>{(o===void 0||o.value)&&(oe("keydown",document,n),oe("keyup",document,d)),o!==void 0&&se(o,a=>{a?(oe("keydown",document,n),oe("keyup",document,d)):(ee("keydown",document,n),ee("keyup",document,d))})};return Ie()?($e(l),Ke(()=>{(o===void 0||o.value)&&(ee("keydown",document,n),ee("keyup",document,d))})):l(),Be(r)}function oo(e,o,r){const i=O(e.value);let t=null;return se(e,n=>{t!==null&&window.clearTimeout(t),n===!0?r&&!r.value?i.value=!0:t=window.setTimeout(()=>{i.value=!0},o):i.value=!1}),i}function no(e){return o=>{o?e.value=o.$el:e.value=null}}const to=V({name:"ChevronRight",render(){return s("svg",{viewBox:"0 0 16 16",fill:"none",xmlns:"http://www.w3.org/2000/svg"},s("path",{d:"M5.64645 3.14645C5.45118 3.34171 5.45118 3.65829 5.64645 3.85355L9.79289 8L5.64645 12.1464C5.45118 12.3417 5.45118 12.6583 5.64645 12.8536C5.84171 13.0488 6.15829 13.0488 6.35355 12.8536L10.8536 8.35355C11.0488 8.15829 11.0488 7.84171 10.8536 7.64645L6.35355 3.14645C6.15829 2.95118 5.84171 2.95118 5.64645 3.14645Z",fill:"currentColor"}))}}),Ro={name:String,value:{type:[String,Number,Boolean],default:"on"},checked:{type:Boolean,default:void 0},defaultChecked:Boolean,disabled:{type:Boolean,default:void 0},label:String,size:String,onUpdateChecked:[Function,Array],"onUpdate:checked":[Function,Array],checkedValue:{type:Boolean,default:void 0}},ye=re("n-radio-group");function ko(e){const o=q(ye,null),r=be(e,{mergedSize(b){const{size:_}=e;if(_!==void 0)return _;if(o){const{mergedSizeRef:{value:S}}=o;if(S!==void 0)return S}return b?b.mergedSize.value:"medium"},mergedDisabled(b){return!!(e.disabled||o?.disabledRef.value||b?.disabled.value)}}),{mergedSizeRef:i,mergedDisabledRef:t}=r,n=O(null),d=O(null),l=O(e.defaultChecked),a=T(e,"checked"),v=ue(a,l),f=Q(()=>o?o.valueRef.value===e.value:v.value),R=Q(()=>{const{name:b}=e;if(b!==void 0)return b;if(o)return o.nameRef.value}),g=O(!1);function P(){if(o){const{doUpdateValue:b}=o,{value:_}=e;W(b,_)}else{const{onUpdateChecked:b,"onUpdate:checked":_}=e,{nTriggerFormInput:S,nTriggerFormChange:z}=r;b&&W(b,!0),_&&W(_,!0),S(),z(),l.value=!0}}function N(){t.value||f.value||P()}function D(){N(),n.value&&(n.value.checked=f.value)}function k(){g.value=!1}function K(){g.value=!0}return{mergedClsPrefix:o?o.mergedClsPrefixRef:ie(e).mergedClsPrefixRef,inputRef:n,labelRef:d,mergedName:R,mergedDisabled:t,renderSafeChecked:f,focus:g,mergedSize:i,handleRadioInputChange:D,handleRadioInputBlur:k,handleRadioInputFocus:K}}const ro=I("radio-group",`
 display: inline-block;
 font-size: var(--n-font-size);
`,[$("splitor",`
 display: inline-block;
 vertical-align: bottom;
 width: 1px;
 transition:
 background-color .3s var(--n-bezier),
 opacity .3s var(--n-bezier);
 background: var(--n-button-border-color);
 `,[C("checked",{backgroundColor:"var(--n-button-border-color-active)"}),C("disabled",{opacity:"var(--n-opacity-disabled)"})]),C("button-group",`
 white-space: nowrap;
 height: var(--n-height);
 line-height: var(--n-height);
 `,[I("radio-button",{height:"var(--n-height)",lineHeight:"var(--n-height)"}),$("splitor",{height:"var(--n-height)"})]),I("radio-button",`
 vertical-align: bottom;
 outline: none;
 position: relative;
 user-select: none;
 -webkit-user-select: none;
 display: inline-block;
 box-sizing: border-box;
 padding-left: 14px;
 padding-right: 14px;
 white-space: nowrap;
 transition:
 background-color .3s var(--n-bezier),
 opacity .3s var(--n-bezier),
 border-color .3s var(--n-bezier),
 color .3s var(--n-bezier);
 background: var(--n-button-color);
 color: var(--n-button-text-color);
 border-top: 1px solid var(--n-button-border-color);
 border-bottom: 1px solid var(--n-button-border-color);
 `,[I("radio-input",`
 pointer-events: none;
 position: absolute;
 border: 0;
 border-radius: inherit;
 left: 0;
 right: 0;
 top: 0;
 bottom: 0;
 opacity: 0;
 z-index: 1;
 `),$("state-border",`
 z-index: 1;
 pointer-events: none;
 position: absolute;
 box-shadow: var(--n-button-box-shadow);
 transition: box-shadow .3s var(--n-bezier);
 left: -1px;
 bottom: -1px;
 right: -1px;
 top: -1px;
 `),A("&:first-child",`
 border-top-left-radius: var(--n-button-border-radius);
 border-bottom-left-radius: var(--n-button-border-radius);
 border-left: 1px solid var(--n-button-border-color);
 `,[$("state-border",`
 border-top-left-radius: var(--n-button-border-radius);
 border-bottom-left-radius: var(--n-button-border-radius);
 `)]),A("&:last-child",`
 border-top-right-radius: var(--n-button-border-radius);
 border-bottom-right-radius: var(--n-button-border-radius);
 border-right: 1px solid var(--n-button-border-color);
 `,[$("state-border",`
 border-top-right-radius: var(--n-button-border-radius);
 border-bottom-right-radius: var(--n-button-border-radius);
 `)]),ne("disabled",`
 cursor: pointer;
 `,[A("&:hover",[$("state-border",`
 transition: box-shadow .3s var(--n-bezier);
 box-shadow: var(--n-button-box-shadow-hover);
 `),ne("checked",{color:"var(--n-button-text-color-hover)"})]),C("focus",[A("&:not(:active)",[$("state-border",{boxShadow:"var(--n-button-box-shadow-focus)"})])])]),C("checked",`
 background: var(--n-button-color-active);
 color: var(--n-button-text-color-active);
 border-color: var(--n-button-border-color-active);
 `),C("disabled",`
 cursor: not-allowed;
 opacity: var(--n-opacity-disabled);
 `)])]);function io(e,o,r){var i;const t=[];let n=!1;for(let d=0;d<e.length;++d){const l=e[d],a=(i=l.type)===null||i===void 0?void 0:i.name;a==="RadioButton"&&(n=!0);const v=l.props;if(a!=="RadioButton"){t.push(l);continue}if(d===0)t.push(l);else{const f=t[t.length-1].props,R=o===f.value,g=f.disabled,P=o===v.value,N=v.disabled,D=(R?2:0)+(g?0:1),k=(P?2:0)+(N?0:1),K={[`${r}-radio-group__splitor--disabled`]:g,[`${r}-radio-group__splitor--checked`]:R},b={[`${r}-radio-group__splitor--disabled`]:N,[`${r}-radio-group__splitor--checked`]:P},_=D<k?b:K;t.push(s("div",{class:[`${r}-radio-group__splitor`,_]}),l)}}return{children:t,isButtonGroup:n}}const ao=Object.assign(Object.assign({},Y.props),{name:String,value:[String,Number,Boolean],defaultValue:{type:[String,Number,Boolean],default:null},size:String,disabled:{type:Boolean,default:void 0},"onUpdate:value":[Function,Array],onUpdateValue:[Function,Array]}),So=V({name:"RadioGroup",props:ao,setup(e){const o=O(null),{mergedSizeRef:r,mergedDisabledRef:i,nTriggerFormChange:t,nTriggerFormInput:n,nTriggerFormBlur:d,nTriggerFormFocus:l}=be(e),{mergedClsPrefixRef:a,inlineThemeDisabled:v,mergedRtlRef:f}=ie(e),R=Y("Radio","-radio-group",ro,De,e,a),g=O(e.defaultValue),P=T(e,"value"),N=ue(P,g);function D(z){const{onUpdateValue:B,"onUpdate:value":U}=e;B&&W(B,z),U&&W(U,z),g.value=z,t(),n()}function k(z){const{value:B}=o;B&&(B.contains(z.relatedTarget)||l())}function K(z){const{value:B}=o;B&&(B.contains(z.relatedTarget)||d())}Z(ye,{mergedClsPrefixRef:a,nameRef:T(e,"name"),valueRef:N,disabledRef:i,mergedSizeRef:r,doUpdateValue:D});const b=Te("Radio",f,a),_=x(()=>{const{value:z}=r,{common:{cubicBezierEaseInOut:B},self:{buttonBorderColor:U,buttonBorderColorActive:J,buttonBorderRadius:H,buttonBoxShadow:G,buttonBoxShadowFocus:E,buttonBoxShadowHover:u,buttonColor:m,buttonColorActive:h,buttonTextColor:p,buttonTextColorActive:F,buttonTextColorHover:c,opacityDisabled:w,[M("buttonHeight",z)]:j,[M("fontSize",z)]:L}}=R.value;return{"--n-font-size":L,"--n-bezier":B,"--n-button-border-color":U,"--n-button-border-color-active":J,"--n-button-border-radius":H,"--n-button-box-shadow":G,"--n-button-box-shadow-focus":E,"--n-button-box-shadow-hover":u,"--n-button-color":m,"--n-button-color-active":h,"--n-button-text-color":p,"--n-button-text-color-hover":c,"--n-button-text-color-active":F,"--n-height":j,"--n-opacity-disabled":w}}),S=v?ce("radio-group",x(()=>r.value[0]),_,e):void 0;return{selfElRef:o,rtlEnabled:b,mergedClsPrefix:a,mergedValue:N,handleFocusout:K,handleFocusin:k,cssVars:v?void 0:_,themeClass:S?.themeClass,onRender:S?.onRender}},render(){var e;const{mergedValue:o,mergedClsPrefix:r,handleFocusin:i,handleFocusout:t}=this,{children:n,isButtonGroup:d}=io(Fe(Oe(this)),o,r);return(e=this.onRender)===null||e===void 0||e.call(this),s("div",{onFocusin:i,onFocusout:t,ref:"selfElRef",class:[`${r}-radio-group`,this.rtlEnabled&&`${r}-radio-group--rtl`,this.themeClass,d&&`${r}-radio-group--button-group`],style:this.cssVars},n)}}),pe=re("n-dropdown-menu"),ae=re("n-dropdown"),ve=re("n-dropdown-option"),xe=V({name:"DropdownDivider",props:{clsPrefix:{type:String,required:!0}},render(){return s("div",{class:`${this.clsPrefix}-dropdown-divider`})}}),lo=V({name:"DropdownGroupHeader",props:{clsPrefix:{type:String,required:!0},tmNode:{type:Object,required:!0}},setup(){const{showIconRef:e,hasSubmenuRef:o}=q(pe),{renderLabelRef:r,labelFieldRef:i,nodePropsRef:t,renderOptionRef:n}=q(ae);return{labelField:i,showIcon:e,hasSubmenu:o,renderLabel:r,nodeProps:t,renderOption:n}},render(){var e;const{clsPrefix:o,hasSubmenu:r,showIcon:i,nodeProps:t,renderLabel:n,renderOption:d}=this,{rawNode:l}=this.tmNode,a=s("div",Object.assign({class:`${o}-dropdown-option`},t?.(l)),s("div",{class:`${o}-dropdown-option-body ${o}-dropdown-option-body--group`},s("div",{"data-dropdown-option":!0,class:[`${o}-dropdown-option-body__prefix`,i&&`${o}-dropdown-option-body__prefix--show-icon`]},te(l.icon)),s("div",{class:`${o}-dropdown-option-body__label`,"data-dropdown-option":!0},n?n(l):te((e=l.title)!==null&&e!==void 0?e:l[this.labelField])),s("div",{class:[`${o}-dropdown-option-body__suffix`,r&&`${o}-dropdown-option-body__suffix--has-submenu`],"data-dropdown-option":!0})));return d?d({node:a,option:l}):a}}),so=I("icon",`
 height: 1em;
 width: 1em;
 line-height: 1em;
 text-align: center;
 display: inline-block;
 position: relative;
 fill: currentColor;
`,[C("color-transition",{transition:"color .3s var(--n-bezier)"}),C("depth",{color:"var(--n-color)"},[A("svg",{opacity:"var(--n-opacity)",transition:"opacity .3s var(--n-bezier)"})]),A("svg",{height:"1em",width:"1em"})]),uo=Object.assign(Object.assign({},Y.props),{depth:[String,Number],size:[Number,String],color:String,component:[Object,Function]}),co=V({_n_icon__:!0,name:"Icon",inheritAttrs:!1,props:uo,setup(e){const{mergedClsPrefixRef:o,inlineThemeDisabled:r}=ie(e),i=Y("Icon","-icon",so,Ae,e,o),t=x(()=>{const{depth:d}=e,{common:{cubicBezierEaseInOut:l},self:a}=i.value;if(d!==void 0){const{color:v,[`opacity${d}Depth`]:f}=a;return{"--n-bezier":l,"--n-color":v,"--n-opacity":f}}return{"--n-bezier":l,"--n-color":"","--n-opacity":""}}),n=r?ce("icon",x(()=>`${e.depth||"d"}`),t,e):void 0;return{mergedClsPrefix:o,mergedStyle:x(()=>{const{size:d,color:l}=e;return{fontSize:je(d),color:l}}),cssVars:r?void 0:t,themeClass:n?.themeClass,onRender:n?.onRender}},render(){var e;const{$parent:o,depth:r,mergedClsPrefix:i,component:t,onRender:n,themeClass:d}=this;return!((e=o?.$options)===null||e===void 0)&&e._n_icon__&&ge("icon","don't wrap `n-icon` inside `n-icon`"),n?.(),s("i",fe(this.$attrs,{role:"img",class:[`${i}-icon`,d,{[`${i}-icon--depth`]:r,[`${i}-icon--color-transition`]:r!==void 0}],style:[this.cssVars,this.mergedStyle]}),t?s(t):this.$slots)}});function le(e,o){return e.type==="submenu"||e.type===void 0&&e[o]!==void 0}function fo(e){return e.type==="group"}function Re(e){return e.type==="divider"}function po(e){return e.type==="render"}const ke=V({name:"DropdownOption",props:{clsPrefix:{type:String,required:!0},tmNode:{type:Object,required:!0},parentKey:{type:[String,Number],default:null},placement:{type:String,default:"right-start"},props:Object,scrollable:Boolean},setup(e){const o=q(ae),{hoverKeyRef:r,keyboardKeyRef:i,lastToggledSubmenuKeyRef:t,pendingKeyPathRef:n,activeKeyPathRef:d,animatedRef:l,mergedShowRef:a,renderLabelRef:v,renderIconRef:f,labelFieldRef:R,childrenFieldRef:g,renderOptionRef:P,nodePropsRef:N,menuPropsRef:D}=o,k=q(ve,null),K=q(pe),b=q(me),_=x(()=>e.tmNode.rawNode),S=x(()=>{const{value:c}=g;return le(e.tmNode.rawNode,c)}),z=x(()=>{const{disabled:c}=e.tmNode;return c}),B=x(()=>{if(!S.value)return!1;const{key:c,disabled:w}=e.tmNode;if(w)return!1;const{value:j}=r,{value:L}=i,{value:de}=t,{value:X}=n;return j!==null?X.includes(c):L!==null?X.includes(c)&&X[X.length-1]!==c:de!==null?X.includes(c):!1}),U=x(()=>i.value===null&&!l.value),J=oo(B,300,U),H=x(()=>!!k?.enteringSubmenuRef.value),G=O(!1);Z(ve,{enteringSubmenuRef:G});function E(){G.value=!0}function u(){G.value=!1}function m(){const{parentKey:c,tmNode:w}=e;w.disabled||a.value&&(t.value=c,i.value=null,r.value=w.key)}function h(){const{tmNode:c}=e;c.disabled||a.value&&r.value!==c.key&&m()}function p(c){if(e.tmNode.disabled||!a.value)return;const{relatedTarget:w}=c;w&&!he({target:w},"dropdownOption")&&!he({target:w},"scrollbarRail")&&(r.value=null)}function F(){const{value:c}=S,{tmNode:w}=e;a.value&&!c&&!w.disabled&&(o.doSelect(w.key,w.rawNode),o.doUpdateShow(!1))}return{labelField:R,renderLabel:v,renderIcon:f,siblingHasIcon:K.showIconRef,siblingHasSubmenu:K.hasSubmenuRef,menuProps:D,popoverBody:b,animated:l,mergedShowSubmenu:x(()=>J.value&&!H.value),rawNode:_,hasSubmenu:S,pending:Q(()=>{const{value:c}=n,{key:w}=e.tmNode;return c.includes(w)}),childActive:Q(()=>{const{value:c}=d,{key:w}=e.tmNode,j=c.findIndex(L=>w===L);return j===-1?!1:j<c.length-1}),active:Q(()=>{const{value:c}=d,{key:w}=e.tmNode,j=c.findIndex(L=>w===L);return j===-1?!1:j===c.length-1}),mergedDisabled:z,renderOption:P,nodeProps:N,handleClick:F,handleMouseMove:h,handleMouseEnter:m,handleMouseLeave:p,handleSubmenuBeforeEnter:E,handleSubmenuAfterEnter:u}},render(){var e,o;const{animated:r,rawNode:i,mergedShowSubmenu:t,clsPrefix:n,siblingHasIcon:d,siblingHasSubmenu:l,renderLabel:a,renderIcon:v,renderOption:f,nodeProps:R,props:g,scrollable:P}=this;let N=null;if(t){const b=(e=this.menuProps)===null||e===void 0?void 0:e.call(this,i,i.children);N=s(Se,Object.assign({},b,{clsPrefix:n,scrollable:this.scrollable,tmNodes:this.tmNode.children,parentKey:this.tmNode.key}))}const D={class:[`${n}-dropdown-option-body`,this.pending&&`${n}-dropdown-option-body--pending`,this.active&&`${n}-dropdown-option-body--active`,this.childActive&&`${n}-dropdown-option-body--child-active`,this.mergedDisabled&&`${n}-dropdown-option-body--disabled`],onMousemove:this.handleMouseMove,onMouseenter:this.handleMouseEnter,onMouseleave:this.handleMouseLeave,onClick:this.handleClick},k=R?.(i),K=s("div",Object.assign({class:[`${n}-dropdown-option`,k?.class],"data-dropdown-option":!0},k),s("div",fe(D,g),[s("div",{class:[`${n}-dropdown-option-body__prefix`,d&&`${n}-dropdown-option-body__prefix--show-icon`]},[v?v(i):te(i.icon)]),s("div",{"data-dropdown-option":!0,class:`${n}-dropdown-option-body__label`},a?a(i):te((o=i[this.labelField])!==null&&o!==void 0?o:i.title)),s("div",{"data-dropdown-option":!0,class:[`${n}-dropdown-option-body__suffix`,l&&`${n}-dropdown-option-body__suffix--has-submenu`]},this.hasSubmenu?s(co,null,{default:()=>s(to,null)}):null)]),this.hasSubmenu?s(Ee,null,{default:()=>[s(Ve,null,{default:()=>s("div",{class:`${n}-dropdown-offset-container`},s(Le,{show:this.mergedShowSubmenu,placement:this.placement,to:P&&this.popoverBody||void 0,teleportDisabled:!P},{default:()=>s("div",{class:`${n}-dropdown-menu-wrapper`},r?s(Me,{onBeforeEnter:this.handleSubmenuBeforeEnter,onAfterEnter:this.handleSubmenuAfterEnter,name:"fade-in-scale-up-transition",appear:!0},{default:()=>N}):N)}))})]}):null);return f?f({node:K,option:i}):K}}),ho=V({name:"NDropdownGroup",props:{clsPrefix:{type:String,required:!0},tmNode:{type:Object,required:!0},parentKey:{type:[String,Number],default:null}},render(){const{tmNode:e,parentKey:o,clsPrefix:r}=this,{children:i}=e;return s(Ue,null,s(lo,{clsPrefix:r,tmNode:e,key:e.key}),i?.map(t=>{const{rawNode:n}=t;return n.show===!1?null:Re(n)?s(xe,{clsPrefix:r,key:t.key}):t.isGroup?(ge("dropdown","`group` node is not allowed to be put in `group` node."),null):s(ke,{clsPrefix:r,tmNode:t,parentKey:o,key:t.key})}))}}),vo=V({name:"DropdownRenderOption",props:{tmNode:{type:Object,required:!0}},render(){const{rawNode:{render:e,props:o}}=this.tmNode;return s("div",o,[e?.()])}}),Se=V({name:"DropdownMenu",props:{scrollable:Boolean,showArrow:Boolean,arrowStyle:[String,Object],clsPrefix:{type:String,required:!0},tmNodes:{type:Array,default:()=>[]},parentKey:{type:[String,Number],default:null}},setup(e){const{renderIconRef:o,childrenFieldRef:r}=q(ae);Z(pe,{showIconRef:x(()=>{const t=o.value;return e.tmNodes.some(n=>{var d;if(n.isGroup)return(d=n.children)===null||d===void 0?void 0:d.some(({rawNode:a})=>t?t(a):a.icon);const{rawNode:l}=n;return t?t(l):l.icon})}),hasSubmenuRef:x(()=>{const{value:t}=r;return e.tmNodes.some(n=>{var d;if(n.isGroup)return(d=n.children)===null||d===void 0?void 0:d.some(({rawNode:a})=>le(a,t));const{rawNode:l}=n;return le(l,t)})})});const i=O(null);return Z(qe,null),Z(We,null),Z(me,i),{bodyRef:i}},render(){const{parentKey:e,clsPrefix:o,scrollable:r}=this,i=this.tmNodes.map(t=>{const{rawNode:n}=t;return n.show===!1?null:po(n)?s(vo,{tmNode:t,key:t.key}):Re(n)?s(xe,{clsPrefix:o,key:t.key}):fo(n)?s(ho,{clsPrefix:o,tmNode:t,parentKey:e,key:t.key}):s(ke,{clsPrefix:o,tmNode:t,parentKey:e,key:t.key,props:n.props,scrollable:r})});return s("div",{class:[`${o}-dropdown-menu`,r&&`${o}-dropdown-menu--scrollable`],ref:"bodyRef"},r?s(He,{contentClass:`${o}-dropdown-menu__content`},{default:()=>i}):i,this.showArrow?Ge({clsPrefix:o,arrowStyle:this.arrowStyle,arrowClass:void 0,arrowWrapperClass:void 0,arrowWrapperStyle:void 0}):null)}}),bo=I("dropdown-menu",`
 transform-origin: var(--v-transform-origin);
 background-color: var(--n-color);
 border-radius: var(--n-border-radius);
 box-shadow: var(--n-box-shadow);
 position: relative;
 transition:
 background-color .3s var(--n-bezier),
 box-shadow .3s var(--n-bezier);
`,[Xe(),I("dropdown-option",`
 position: relative;
 `,[A("a",`
 text-decoration: none;
 color: inherit;
 outline: none;
 `,[A("&::before",`
 content: "";
 position: absolute;
 left: 0;
 right: 0;
 top: 0;
 bottom: 0;
 `)]),I("dropdown-option-body",`
 display: flex;
 cursor: pointer;
 position: relative;
 height: var(--n-option-height);
 line-height: var(--n-option-height);
 font-size: var(--n-font-size);
 color: var(--n-option-text-color);
 transition: color .3s var(--n-bezier);
 `,[A("&::before",`
 content: "";
 position: absolute;
 top: 0;
 bottom: 0;
 left: 4px;
 right: 4px;
 transition: background-color .3s var(--n-bezier);
 border-radius: var(--n-border-radius);
 `),ne("disabled",[C("pending",`
 color: var(--n-option-text-color-hover);
 `,[$("prefix, suffix",`
 color: var(--n-option-text-color-hover);
 `),A("&::before","background-color: var(--n-option-color-hover);")]),C("active",`
 color: var(--n-option-text-color-active);
 `,[$("prefix, suffix",`
 color: var(--n-option-text-color-active);
 `),A("&::before","background-color: var(--n-option-color-active);")]),C("child-active",`
 color: var(--n-option-text-color-child-active);
 `,[$("prefix, suffix",`
 color: var(--n-option-text-color-child-active);
 `)])]),C("disabled",`
 cursor: not-allowed;
 opacity: var(--n-option-opacity-disabled);
 `),C("group",`
 font-size: calc(var(--n-font-size) - 1px);
 color: var(--n-group-header-text-color);
 `,[$("prefix",`
 width: calc(var(--n-option-prefix-width) / 2);
 `,[C("show-icon",`
 width: calc(var(--n-option-icon-prefix-width) / 2);
 `)])]),$("prefix",`
 width: var(--n-option-prefix-width);
 display: flex;
 justify-content: center;
 align-items: center;
 color: var(--n-prefix-color);
 transition: color .3s var(--n-bezier);
 z-index: 1;
 `,[C("show-icon",`
 width: var(--n-option-icon-prefix-width);
 `),I("icon",`
 font-size: var(--n-option-icon-size);
 `)]),$("label",`
 white-space: nowrap;
 flex: 1;
 z-index: 1;
 `),$("suffix",`
 box-sizing: border-box;
 flex-grow: 0;
 flex-shrink: 0;
 display: flex;
 justify-content: flex-end;
 align-items: center;
 min-width: var(--n-option-suffix-width);
 padding: 0 8px;
 transition: color .3s var(--n-bezier);
 color: var(--n-suffix-color);
 z-index: 1;
 `,[C("has-submenu",`
 width: var(--n-option-icon-suffix-width);
 `),I("icon",`
 font-size: var(--n-option-icon-size);
 `)]),I("dropdown-menu","pointer-events: all;")]),I("dropdown-offset-container",`
 pointer-events: none;
 position: absolute;
 left: 0;
 right: 0;
 top: -4px;
 bottom: -4px;
 `)]),I("dropdown-divider",`
 transition: background-color .3s var(--n-bezier);
 background-color: var(--n-divider-color);
 height: 1px;
 margin: 4px 0;
 `),I("dropdown-menu-wrapper",`
 transform-origin: var(--v-transform-origin);
 width: fit-content;
 `),A(">",[I("scrollbar",`
 height: inherit;
 max-height: inherit;
 `)]),ne("scrollable",`
 padding: var(--n-padding);
 `),C("scrollable",[$("content",`
 padding: var(--n-padding);
 `)])]),go={animated:{type:Boolean,default:!0},keyboard:{type:Boolean,default:!0},size:{type:String,default:"medium"},inverted:Boolean,placement:{type:String,default:"bottom"},onSelect:[Function,Array],options:{type:Array,default:()=>[]},menuProps:Function,showArrow:Boolean,renderLabel:Function,renderIcon:Function,renderOption:Function,nodeProps:Function,labelField:{type:String,default:"label"},keyField:{type:String,default:"key"},childrenField:{type:String,default:"children"},value:[String,Number]},mo=Object.keys(we),wo=Object.assign(Object.assign(Object.assign({},we),go),Y.props),Co=V({name:"Dropdown",inheritAttrs:!1,props:wo,setup(e){const o=O(!1),r=ue(T(e,"show"),o),i=x(()=>{const{keyField:u,childrenField:m}=e;return Ye(e.options,{getKey(h){return h[u]},getDisabled(h){return h.disabled===!0},getIgnored(h){return h.type==="divider"||h.type==="render"},getChildren(h){return h[m]}})}),t=x(()=>i.value.treeNodes),n=O(null),d=O(null),l=O(null),a=x(()=>{var u,m,h;return(h=(m=(u=n.value)!==null&&u!==void 0?u:d.value)!==null&&m!==void 0?m:l.value)!==null&&h!==void 0?h:null}),v=x(()=>i.value.getPath(a.value).keyPath),f=x(()=>i.value.getPath(e.value).keyPath),R=Q(()=>e.keyboard&&r.value);eo({keydown:{ArrowUp:{prevent:!0,handler:z},ArrowRight:{prevent:!0,handler:S},ArrowDown:{prevent:!0,handler:B},ArrowLeft:{prevent:!0,handler:_},Enter:{prevent:!0,handler:U},Escape:b}},R);const{mergedClsPrefixRef:g,inlineThemeDisabled:P}=ie(e),N=Y("Dropdown","-dropdown",bo,Qe,e,g);Z(ae,{labelFieldRef:T(e,"labelField"),childrenFieldRef:T(e,"childrenField"),renderLabelRef:T(e,"renderLabel"),renderIconRef:T(e,"renderIcon"),hoverKeyRef:n,keyboardKeyRef:d,lastToggledSubmenuKeyRef:l,pendingKeyPathRef:v,activeKeyPathRef:f,animatedRef:T(e,"animated"),mergedShowRef:r,nodePropsRef:T(e,"nodeProps"),renderOptionRef:T(e,"renderOption"),menuPropsRef:T(e,"menuProps"),doSelect:D,doUpdateShow:k}),se(r,u=>{!e.animated&&!u&&K()});function D(u,m){const{onSelect:h}=e;h&&W(h,u,m)}function k(u){const{"onUpdate:show":m,onUpdateShow:h}=e;m&&W(m,u),h&&W(h,u),o.value=u}function K(){n.value=null,d.value=null,l.value=null}function b(){k(!1)}function _(){H("left")}function S(){H("right")}function z(){H("up")}function B(){H("down")}function U(){const u=J();u?.isLeaf&&r.value&&(D(u.key,u.rawNode),k(!1))}function J(){var u;const{value:m}=i,{value:h}=a;return!m||h===null?null:(u=m.getNode(h))!==null&&u!==void 0?u:null}function H(u){const{value:m}=a,{value:{getFirstAvailableNode:h}}=i;let p=null;if(m===null){const F=h();F!==null&&(p=F.key)}else{const F=J();if(F){let c;switch(u){case"down":c=F.getNext();break;case"up":c=F.getPrev();break;case"right":c=F.getChild();break;case"left":c=F.getParent();break}c&&(p=c.key)}}p!==null&&(n.value=null,d.value=p)}const G=x(()=>{const{size:u,inverted:m}=e,{common:{cubicBezierEaseInOut:h},self:p}=N.value,{padding:F,dividerColor:c,borderRadius:w,optionOpacityDisabled:j,[M("optionIconSuffixWidth",u)]:L,[M("optionSuffixWidth",u)]:de,[M("optionIconPrefixWidth",u)]:X,[M("optionPrefixWidth",u)]:Ce,[M("fontSize",u)]:Pe,[M("optionHeight",u)]:Ne,[M("optionIconSize",u)]:_e}=p,y={"--n-bezier":h,"--n-font-size":Pe,"--n-padding":F,"--n-border-radius":w,"--n-option-height":Ne,"--n-option-prefix-width":Ce,"--n-option-icon-prefix-width":X,"--n-option-suffix-width":de,"--n-option-icon-suffix-width":L,"--n-option-icon-size":_e,"--n-divider-color":c,"--n-option-opacity-disabled":j};return m?(y["--n-color"]=p.colorInverted,y["--n-option-color-hover"]=p.optionColorHoverInverted,y["--n-option-color-active"]=p.optionColorActiveInverted,y["--n-option-text-color"]=p.optionTextColorInverted,y["--n-option-text-color-hover"]=p.optionTextColorHoverInverted,y["--n-option-text-color-active"]=p.optionTextColorActiveInverted,y["--n-option-text-color-child-active"]=p.optionTextColorChildActiveInverted,y["--n-prefix-color"]=p.prefixColorInverted,y["--n-suffix-color"]=p.suffixColorInverted,y["--n-group-header-text-color"]=p.groupHeaderTextColorInverted):(y["--n-color"]=p.color,y["--n-option-color-hover"]=p.optionColorHover,y["--n-option-color-active"]=p.optionColorActive,y["--n-option-text-color"]=p.optionTextColor,y["--n-option-text-color-hover"]=p.optionTextColorHover,y["--n-option-text-color-active"]=p.optionTextColorActive,y["--n-option-text-color-child-active"]=p.optionTextColorChildActive,y["--n-prefix-color"]=p.prefixColor,y["--n-suffix-color"]=p.suffixColor,y["--n-group-header-text-color"]=p.groupHeaderTextColor),y}),E=P?ce("dropdown",x(()=>`${e.size[0]}${e.inverted?"i":""}`),G,e):void 0;return{mergedClsPrefix:g,mergedTheme:N,tmNodes:t,mergedShow:r,handleAfterLeave:()=>{e.animated&&K()},doUpdateShow:k,cssVars:P?void 0:G,themeClass:E?.themeClass,onRender:E?.onRender}},render(){const e=(i,t,n,d,l)=>{var a;const{mergedClsPrefix:v,menuProps:f}=this;(a=this.onRender)===null||a===void 0||a.call(this);const R=f?.(void 0,this.tmNodes.map(P=>P.rawNode))||{},g={ref:no(t),class:[i,`${v}-dropdown`,this.themeClass],clsPrefix:v,tmNodes:this.tmNodes,style:[...n,this.cssVars],showArrow:this.showArrow,arrowStyle:this.arrowStyle,scrollable:this.scrollable,onMouseenter:d,onMouseleave:l};return s(Se,fe(this.$attrs,g,R))},{mergedTheme:o}=this,r={show:this.mergedShow,theme:o.peers.Popover,themeOverrides:o.peerOverrides.Popover,internalOnAfterLeave:this.handleAfterLeave,internalRenderBody:e,onUpdateShow:this.doUpdateShow,"onUpdate:show":void 0};return s(Ze,Object.assign({},Je(this.$props,mo),r),{trigger:()=>{var i,t;return(t=(i=this.$slots).default)===null||t===void 0?void 0:t.call(i)}})}});export{to as C,Co as _,So as a,no as c,Ro as r,ko as s};
