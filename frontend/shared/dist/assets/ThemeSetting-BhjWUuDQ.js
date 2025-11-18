import{p as G,q as l,g as I,N as xn,O as zt,P as yn,Q as wn,R as Ye,S as kn,i as d,j as S,k as R,s as wt,T as Sn,U as ne,V as se,W as Se,X as Ke,Y as le,Z as fe,_ as he,$ as Re,a0 as Ce,a1 as Vt,a2 as pt,a3 as ft,a4 as bt,y as j,a5 as ze,a6 as Pe,m as Dt,C as kt,a7 as St,a8 as Ve,a9 as ht,aa as gt,ab as vt,ac as Cn,ad as $n,l as x,ae as Rn,af as zn,ag as Pn,ah as mt,ai as Wt,aj as Ht,ak as _n,al as An,t as et,am as In,w as Ot,x as K,an as xt,I as Ze,ao as Y,z as Lt,ap as Tn,aq as Bn,ar as be,as as Me,at as De,au as Un,av as Mn,F as Ct,aw as En,ax as Vn,ay as Dn,az as Wn,aA as st,aB as Pt,aC as lt,aD as Hn,aE as _t,h as On,aF as Ln,aG as je,aH as Fn,aI as jn,aJ as Nn,aK as qn,u as Ft,aL as Gn,c as ge,o as J,n as jt,d as V,aM as Xn,e as Je,J as q,f as Ee,b as N,aN as ae,L as yt,r as Ne,aO as Kn,aP as Zn,aQ as At,a as ie,K as ke,aR as Nt,aS as Yn,aT as Jn,aU as qt,G as Qn,E as eo,aV as to,M as no,aW as oo,aX as ro}from"./index-CQZOGmuN.js";import{_ as ao}from"./Dropdown-Bea_11Hq.js";import"./UserAvatar-C97dN2pm.js";import{_ as io}from"./_plugin-vue_export-helper-DlAUqK2U.js";import{A as so}from"./Add-C3g-VeQk.js";import{_ as lo}from"./Input-BM48kO3D.js";import{u as co}from"./Eye-CMZyCEZw.js";const uo=zt(".v-x-scroll",{overflow:"auto",scrollbarWidth:"none"},[zt("&::-webkit-scrollbar",{width:0,height:0})]),po=G({name:"XScroll",props:{disabled:Boolean,onScroll:Function},setup(){const e=I(null);function t(o){!(o.currentTarget.offsetWidth<o.currentTarget.scrollWidth)||o.deltaY===0||(o.currentTarget.scrollLeft+=o.deltaY+o.deltaX,o.preventDefault())}const n=xn();return uo.mount({id:"vueuc/x-scroll",head:!0,anchorMetaName:yn,ssr:n}),Object.assign({selfRef:e,handleWheel:t},{scrollTo(...o){var a;(a=e.value)===null||a===void 0||a.scrollTo(...o)}})},render(){return l("div",{ref:"selfRef",onScroll:this.onScroll,onWheel:this.disabled?void 0:this.handleWheel,class:"v-x-scroll"},this.$slots)}});var fo=/\s/;function bo(e){for(var t=e.length;t--&&fo.test(e.charAt(t)););return t}var ho=/^\s+/;function go(e){return e&&e.slice(0,bo(e)+1).replace(ho,"")}var It=NaN,vo=/^[-+]0x[0-9a-f]+$/i,mo=/^0b[01]+$/i,xo=/^0o[0-7]+$/i,yo=parseInt;function Tt(e){if(typeof e=="number")return e;if(wn(e))return It;if(Ye(e)){var t=typeof e.valueOf=="function"?e.valueOf():e;e=Ye(t)?t+"":t}if(typeof e!="string")return e===0?e:+e;e=go(e);var n=mo.test(e);return n||xo.test(e)?yo(e.slice(2),n?2:8):vo.test(e)?It:+e}var dt=function(){return kn.Date.now()},wo="Expected a function",ko=Math.max,So=Math.min;function Co(e,t,n){var r,o,a,c,p,g,h=0,v=!1,w=!1,C=!0;if(typeof e!="function")throw new TypeError(wo);t=Tt(t)||0,Ye(n)&&(v=!!n.leading,w="maxWait"in n,a=w?ko(Tt(n.maxWait)||0,t):a,C="trailing"in n?!!n.trailing:C);function y(m){var D=r,E=o;return r=o=void 0,h=m,c=e.apply(E,D),c}function z(m){return h=m,p=setTimeout(_,t),v?y(m):c}function T(m){var D=m-g,E=m-h,F=t-D;return w?So(F,a-E):F}function H(m){var D=m-g,E=m-h;return g===void 0||D>=t||D<0||w&&E>=a}function _(){var m=dt();if(H(m))return B(m);p=setTimeout(_,T(m))}function B(m){return p=void 0,C&&r?y(m):(r=o=void 0,c)}function L(){p!==void 0&&clearTimeout(p),h=0,r=g=o=p=void 0}function O(){return p===void 0?c:B(dt())}function P(){var m=dt(),D=H(m);if(r=arguments,o=this,g=m,D){if(p===void 0)return z(g);if(w)return clearTimeout(p),p=setTimeout(_,t),y(g)}return p===void 0&&(p=setTimeout(_,t)),c}return P.cancel=L,P.flush=O,P}var $o="Expected a function";function Ro(e,t,n){var r=!0,o=!0;if(typeof e!="function")throw new TypeError($o);return Ye(n)&&(r="leading"in n?!!n.leading:r,o="trailing"in n?!!n.trailing:o),Co(e,t,{leading:r,maxWait:t,trailing:o})}const zo=d("input-group",`
 display: inline-flex;
 width: 100%;
 flex-wrap: nowrap;
 vertical-align: bottom;
`,[S(">",[d("input",[S("&:not(:last-child)",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `),S("&:not(:first-child)",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 margin-left: -1px!important;
 `)]),d("button",[S("&:not(:last-child)",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `,[R("state-border, border",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `)]),S("&:not(:first-child)",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `,[R("state-border, border",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `)])]),S("*",[S("&:not(:last-child)",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `,[S(">",[d("input",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `),d("base-selection",[d("base-selection-label",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `),d("base-selection-tags",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `),R("box-shadow, border, state-border",`
 border-top-right-radius: 0!important;
 border-bottom-right-radius: 0!important;
 `)])])]),S("&:not(:first-child)",`
 margin-left: -1px!important;
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `,[S(">",[d("input",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `),d("base-selection",[d("base-selection-label",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `),d("base-selection-tags",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `),R("box-shadow, border, state-border",`
 border-top-left-radius: 0!important;
 border-bottom-left-radius: 0!important;
 `)])])])])])]),Po={},_o=G({name:"InputGroup",props:Po,setup(e){const{mergedClsPrefixRef:t}=wt(e);return Sn("-input-group",zo,t),{mergedClsPrefix:t}},render(){const{mergedClsPrefix:e}=this;return l("div",{class:`${e}-input-group`},this.$slots)}});function Ao(e,t){switch(e[0]){case"hex":return t?"#000000FF":"#000000";case"rgb":return t?"rgba(0, 0, 0, 1)":"rgb(0, 0, 0)";case"hsl":return t?"hsla(0, 0%, 0%, 1)":"hsl(0, 0%, 0%)";case"hsv":return t?"hsva(0, 0%, 0%, 1)":"hsv(0, 0%, 0%)"}return"#000000"}function We(e){return e===null?null:/^ *#/.test(e)?"hex":e.includes("rgb")?"rgb":e.includes("hsl")?"hsl":e.includes("hsv")?"hsv":null}function Io(e,t=[255,255,255],n="AA"){const[r,o,a,c]=ne(se(e));if(c===1){const y=qe([r,o,a]),z=qe(t);return(Math.max(y,z)+.05)/(Math.min(y,z)+.05)>=(n==="AA"?4.5:7)}const p=Math.round(r*c+t[0]*(1-c)),g=Math.round(o*c+t[1]*(1-c)),h=Math.round(a*c+t[2]*(1-c)),v=qe([p,g,h]),w=qe(t);return(Math.max(v,w)+.05)/(Math.min(v,w)+.05)>=(n==="AA"?4.5:7)}function qe(e){const[t,n,r]=e.map(o=>(o/=255,o<=.03928?o/12.92:Math.pow((o+.055)/1.055,2.4)));return .2126*t+.7152*n+.0722*r}function To(e){return e=Math.round(e),e>=360?359:e<0?0:e}function Bo(e){return e=Math.round(e*100)/100,e>1?1:e<0?0:e}const Uo={rgb:{hex(e){return he(ne(e))},hsl(e){const[t,n,r,o]=ne(e);return se([...bt(t,n,r),o])},hsv(e){const[t,n,r,o]=ne(e);return Ce([...ft(t,n,r),o])}},hex:{rgb(e){return le(ne(e))},hsl(e){const[t,n,r,o]=ne(e);return se([...bt(t,n,r),o])},hsv(e){const[t,n,r,o]=ne(e);return Ce([...ft(t,n,r),o])}},hsl:{hex(e){const[t,n,r,o]=Re(e);return he([...pt(t,n,r),o])},rgb(e){const[t,n,r,o]=Re(e);return le([...pt(t,n,r),o])},hsv(e){const[t,n,r,o]=Re(e);return Ce([...Vt(t,n,r),o])}},hsv:{hex(e){const[t,n,r,o]=Se(e);return he([...fe(t,n,r),o])},rgb(e){const[t,n,r,o]=Se(e);return le([...fe(t,n,r),o])},hsl(e){const[t,n,r,o]=Se(e);return se([...Ke(t,n,r),o])}}};function Gt(e,t,n){return n=n||We(e),n?n===t?e:Uo[n][t](e):null}const Ue="12px",Mo=12,ye="6px",Eo=G({name:"AlphaSlider",props:{clsPrefix:{type:String,required:!0},rgba:{type:Array,default:null},alpha:{type:Number,default:0},onUpdateAlpha:{type:Function,required:!0},onComplete:Function},setup(e){const t=I(null);function n(a){!t.value||!e.rgba||(ze("mousemove",document,r),ze("mouseup",document,o),r(a))}function r(a){const{value:c}=t;if(!c)return;const{width:p,left:g}=c.getBoundingClientRect(),h=(a.clientX-g)/(p-Mo);e.onUpdateAlpha(Bo(h))}function o(){var a;Pe("mousemove",document,r),Pe("mouseup",document,o),(a=e.onComplete)===null||a===void 0||a.call(e)}return{railRef:t,railBackgroundImage:j(()=>{const{rgba:a}=e;return a?`linear-gradient(to right, rgba(${a[0]}, ${a[1]}, ${a[2]}, 0) 0%, rgba(${a[0]}, ${a[1]}, ${a[2]}, 1) 100%)`:""}),handleMouseDown:n}},render(){const{clsPrefix:e}=this;return l("div",{class:`${e}-color-picker-slider`,ref:"railRef",style:{height:Ue,borderRadius:ye},onMousedown:this.handleMouseDown},l("div",{style:{borderRadius:ye,position:"absolute",left:0,right:0,top:0,bottom:0,overflow:"hidden"}},l("div",{class:`${e}-color-picker-checkboard`}),l("div",{class:`${e}-color-picker-slider__image`,style:{backgroundImage:this.railBackgroundImage}})),this.rgba&&l("div",{style:{position:"absolute",left:ye,right:ye,top:0,bottom:0}},l("div",{class:`${e}-color-picker-handle`,style:{left:`calc(${this.alpha*100}% - ${ye})`,borderRadius:ye,width:Ue,height:Ue}},l("div",{class:`${e}-color-picker-handle__fill`,style:{backgroundColor:le(this.rgba),borderRadius:ye,width:Ue,height:Ue}}))))}}),$t=Dt("n-color-picker");function Vo(e){return/^\d{1,3}\.?\d*$/.test(e.trim())?Math.max(0,Math.min(Number.parseInt(e),255)):!1}function Do(e){return/^\d{1,3}\.?\d*$/.test(e.trim())?Math.max(0,Math.min(Number.parseInt(e),360)):!1}function Wo(e){return/^\d{1,3}\.?\d*$/.test(e.trim())?Math.max(0,Math.min(Number.parseInt(e),100)):!1}function Ho(e){const t=e.trim();return/^#[0-9a-fA-F]+$/.test(t)?[4,5,7,9].includes(t.length):!1}function Oo(e){return/^\d{1,3}\.?\d*%$/.test(e.trim())?Math.max(0,Math.min(Number.parseInt(e)/100,100)):!1}const Lo={paddingSmall:"0 4px"},Bt=G({name:"ColorInputUnit",props:{label:{type:String,required:!0},value:{type:[Number,String],default:null},showAlpha:Boolean,onUpdateValue:{type:Function,required:!0}},setup(e){const t=I(""),{themeRef:n}=kt($t,null);St(()=>{t.value=r()});function r(){const{value:c}=e;if(c===null)return"";const{label:p}=e;return p==="HEX"?c:p==="A"?`${Math.floor(c*100)}%`:String(Math.floor(c))}function o(c){t.value=c}function a(c){let p,g;switch(e.label){case"HEX":g=Ho(c),g&&e.onUpdateValue(c),t.value=r();break;case"H":p=Do(c),p===!1?t.value=r():e.onUpdateValue(p);break;case"S":case"L":case"V":p=Wo(c),p===!1?t.value=r():e.onUpdateValue(p);break;case"A":p=Oo(c),p===!1?t.value=r():e.onUpdateValue(p);break;case"R":case"G":case"B":p=Vo(c),p===!1?t.value=r():e.onUpdateValue(p);break}}return{mergedTheme:n,inputValue:t,handleInputChange:a,handleInputUpdateValue:o}},render(){const{mergedTheme:e}=this;return l(lo,{size:"small",placeholder:this.label,theme:e.peers.Input,themeOverrides:e.peerOverrides.Input,builtinThemeOverrides:Lo,value:this.inputValue,onUpdateValue:this.handleInputUpdateValue,onChange:this.handleInputChange,style:this.label==="A"?"flex-grow: 1.25;":""})}}),Fo=G({name:"ColorInput",props:{clsPrefix:{type:String,required:!0},mode:{type:String,required:!0},modes:{type:Array,required:!0},showAlpha:{type:Boolean,required:!0},value:{type:String,default:null},valueArr:{type:Array,default:null},onUpdateValue:{type:Function,required:!0},onUpdateMode:{type:Function,required:!0}},setup(e){return{handleUnitUpdateValue(t,n){const{showAlpha:r}=e;if(e.mode==="hex"){e.onUpdateValue((r?he:Ve)(n));return}let o;switch(e.valueArr===null?o=[0,0,0,0]:o=Array.from(e.valueArr),e.mode){case"hsv":o[t]=n,e.onUpdateValue((r?Ce:vt)(o));break;case"rgb":o[t]=n,e.onUpdateValue((r?le:gt)(o));break;case"hsl":o[t]=n,e.onUpdateValue((r?se:ht)(o));break}}}},render(){const{clsPrefix:e,modes:t}=this;return l("div",{class:`${e}-color-picker-input`},l("div",{class:`${e}-color-picker-input__mode`,onClick:this.onUpdateMode,style:{cursor:t.length===1?"":"pointer"}},this.mode.toUpperCase()+(this.showAlpha?"A":"")),l(_o,null,{default:()=>{const{mode:n,valueArr:r,showAlpha:o}=this;if(n==="hex"){let a=null;try{a=r===null?null:(o?he:Ve)(r)}catch{}return l(Bt,{label:"HEX",showAlpha:o,value:a,onUpdateValue:c=>{this.handleUnitUpdateValue(0,c)}})}return(n+(o?"a":"")).split("").map((a,c)=>l(Bt,{label:a.toUpperCase(),value:r===null?null:r[c],onUpdateValue:p=>{this.handleUnitUpdateValue(c,p)}}))}}))}});function jo(e,t){if(t==="hsv"){const[n,r,o,a]=Se(e);return le([...fe(n,r,o),a])}return e}function No(e){const t=document.createElement("canvas").getContext("2d");return t?(t.fillStyle=e,t.fillStyle):"#000000"}const qo=G({name:"ColorPickerSwatches",props:{clsPrefix:{type:String,required:!0},mode:{type:String,required:!0},swatches:{type:Array,required:!0},onUpdateColor:{type:Function,required:!0}},setup(e){const t=j(()=>e.swatches.map(a=>{const c=We(a);return{value:a,mode:c,legalValue:jo(a,c)}}));function n(a){const{mode:c}=e;let{value:p,mode:g}=a;return g||(g="hex",/^[a-zA-Z]+$/.test(p)?p=No(p):(Cn("color-picker",`color ${p} in swatches is invalid.`),p="#000000")),g===c?p:Gt(p,c,g)}function r(a){e.onUpdateColor(n(a))}function o(a,c){a.key==="Enter"&&r(c)}return{parsedSwatchesRef:t,handleSwatchSelect:r,handleSwatchKeyDown:o}},render(){const{clsPrefix:e}=this;return l("div",{class:`${e}-color-picker-swatches`},this.parsedSwatchesRef.map(t=>l("div",{class:`${e}-color-picker-swatch`,tabindex:0,onClick:()=>{this.handleSwatchSelect(t)},onKeydown:n=>{this.handleSwatchKeyDown(n,t)}},l("div",{class:`${e}-color-picker-swatch__fill`,style:{background:t.legalValue}}))))}}),Go=G({name:"ColorPickerTrigger",slots:Object,props:{clsPrefix:{type:String,required:!0},value:{type:String,default:null},hsla:{type:Array,default:null},disabled:Boolean,onClick:Function},setup(e){const{colorPickerSlots:t,renderLabelRef:n}=kt($t,null);return()=>{const{hsla:r,value:o,clsPrefix:a,onClick:c,disabled:p}=e,g=t.label||n.value;return l("div",{class:[`${a}-color-picker-trigger`,p&&`${a}-color-picker-trigger--disabled`],onClick:p?void 0:c},l("div",{class:`${a}-color-picker-trigger__fill`},l("div",{class:`${a}-color-picker-checkboard`}),l("div",{style:{position:"absolute",left:0,right:0,top:0,bottom:0,backgroundColor:r?se(r):""}}),o&&r?l("div",{class:`${a}-color-picker-trigger__value`,style:{color:Io(r)?"white":"black"}},g?g(o):o):null))}}}),Xo=G({name:"ColorPreview",props:{clsPrefix:{type:String,required:!0},mode:{type:String,required:!0},color:{type:String,default:null,validator:e=>{const t=We(e);return!!(!e||t&&t!=="hsv")}},onUpdateColor:{type:Function,required:!0}},setup(e){function t(n){var r;const o=n.target.value;(r=e.onUpdateColor)===null||r===void 0||r.call(e,Gt(o.toUpperCase(),e.mode,"hex")),n.stopPropagation()}return{handleChange:t}},render(){const{clsPrefix:e}=this;return l("div",{class:`${e}-color-picker-preview__preview`},l("span",{class:`${e}-color-picker-preview__fill`,style:{background:this.color||"#000000"}}),l("input",{class:`${e}-color-picker-preview__input`,type:"color",value:this.color,onChange:this.handleChange}))}}),$e="12px",Ko=12,we="6px",Zo=6,Yo="linear-gradient(90deg,red,#ff0 16.66%,#0f0 33.33%,#0ff 50%,#00f 66.66%,#f0f 83.33%,red)",Jo=G({name:"HueSlider",props:{clsPrefix:{type:String,required:!0},hue:{type:Number,required:!0},onUpdateHue:{type:Function,required:!0},onComplete:Function},setup(e){const t=I(null);function n(a){t.value&&(ze("mousemove",document,r),ze("mouseup",document,o),r(a))}function r(a){const{value:c}=t;if(!c)return;const{width:p,left:g}=c.getBoundingClientRect(),h=To((a.clientX-g-Zo)/(p-Ko)*360);e.onUpdateHue(h)}function o(){var a;Pe("mousemove",document,r),Pe("mouseup",document,o),(a=e.onComplete)===null||a===void 0||a.call(e)}return{railRef:t,handleMouseDown:n}},render(){const{clsPrefix:e}=this;return l("div",{class:`${e}-color-picker-slider`,style:{height:$e,borderRadius:we}},l("div",{ref:"railRef",style:{boxShadow:"inset 0 0 2px 0 rgba(0, 0, 0, .24)",boxSizing:"border-box",backgroundImage:Yo,height:$e,borderRadius:we,position:"relative"},onMousedown:this.handleMouseDown},l("div",{style:{position:"absolute",left:we,right:we,top:0,bottom:0}},l("div",{class:`${e}-color-picker-handle`,style:{left:`calc((${this.hue}%) / 359 * 100 - ${we})`,borderRadius:we,width:$e,height:$e}},l("div",{class:`${e}-color-picker-handle__fill`,style:{backgroundColor:`hsl(${this.hue}, 100%, 50%)`,borderRadius:we,width:$e,height:$e}})))))}}),Ge="12px",Xe="6px",Qo=G({name:"Pallete",props:{clsPrefix:{type:String,required:!0},rgba:{type:Array,default:null},displayedHue:{type:Number,required:!0},displayedSv:{type:Array,required:!0},onUpdateSV:{type:Function,required:!0},onComplete:Function},setup(e){const t=I(null);function n(a){t.value&&(ze("mousemove",document,r),ze("mouseup",document,o),r(a))}function r(a){const{value:c}=t;if(!c)return;const{width:p,height:g,left:h,bottom:v}=c.getBoundingClientRect(),w=(v-a.clientY)/g,C=(a.clientX-h)/p,y=100*(C>1?1:C<0?0:C),z=100*(w>1?1:w<0?0:w);e.onUpdateSV(y,z)}function o(){var a;Pe("mousemove",document,r),Pe("mouseup",document,o),(a=e.onComplete)===null||a===void 0||a.call(e)}return{palleteRef:t,handleColor:j(()=>{const{rgba:a}=e;return a?`rgb(${a[0]}, ${a[1]}, ${a[2]})`:""}),handleMouseDown:n}},render(){const{clsPrefix:e}=this;return l("div",{class:`${e}-color-picker-pallete`,onMousedown:this.handleMouseDown,ref:"palleteRef"},l("div",{class:`${e}-color-picker-pallete__layer`,style:{backgroundImage:`linear-gradient(90deg, white, hsl(${this.displayedHue}, 100%, 50%))`}}),l("div",{class:`${e}-color-picker-pallete__layer ${e}-color-picker-pallete__layer--shadowed`,style:{backgroundImage:"linear-gradient(180deg, rgba(0, 0, 0, 0%), rgba(0, 0, 0, 100%))"}}),this.rgba&&l("div",{class:`${e}-color-picker-handle`,style:{width:Ge,height:Ge,borderRadius:Xe,left:`calc(${this.displayedSv[0]}% - ${Xe})`,bottom:`calc(${this.displayedSv[1]}% - ${Xe})`}},l("div",{class:`${e}-color-picker-handle__fill`,style:{backgroundColor:this.handleColor,borderRadius:Xe,width:Ge,height:Ge}})))}}),er=S([d("color-picker",`
 display: inline-block;
 box-sizing: border-box;
 height: var(--n-height);
 font-size: var(--n-font-size);
 width: 100%;
 position: relative;
 `),d("color-picker-panel",`
 margin: 4px 0;
 width: 240px;
 font-size: var(--n-panel-font-size);
 color: var(--n-text-color);
 background-color: var(--n-color);
 transition:
 box-shadow .3s var(--n-bezier),
 color .3s var(--n-bezier),
 background-color .3s var(--n-bezier);
 border-radius: var(--n-border-radius);
 box-shadow: var(--n-box-shadow);
 `,[$n(),d("input",`
 text-align: center;
 `)]),d("color-picker-checkboard",`
 background: white; 
 position: absolute;
 left: 0;
 right: 0;
 top: 0;
 bottom: 0;
 `,[S("&::after",`
 background-image: linear-gradient(45deg, #DDD 25%, #0000 25%), linear-gradient(-45deg, #DDD 25%, #0000 25%), linear-gradient(45deg, #0000 75%, #DDD 75%), linear-gradient(-45deg, #0000 75%, #DDD 75%);
 background-size: 12px 12px;
 background-position: 0 0, 0 6px, 6px -6px, -6px 0px;
 background-repeat: repeat;
 content: "";
 position: absolute;
 left: 0;
 right: 0;
 top: 0;
 bottom: 0;
 `)]),d("color-picker-slider",`
 margin-bottom: 8px;
 position: relative;
 box-sizing: border-box;
 `,[R("image",`
 position: absolute;
 left: 0;
 right: 0;
 top: 0;
 bottom: 0;
 `),S("&::after",`
 content: "";
 position: absolute;
 border-radius: inherit;
 left: 0;
 right: 0;
 top: 0;
 bottom: 0;
 box-shadow: inset 0 0 2px 0 rgba(0, 0, 0, .24);
 pointer-events: none;
 `)]),d("color-picker-handle",`
 z-index: 1;
 box-shadow: 0 0 2px 0 rgba(0, 0, 0, .45);
 position: absolute;
 background-color: white;
 overflow: hidden;
 `,[R("fill",`
 box-sizing: border-box;
 border: 2px solid white;
 `)]),d("color-picker-pallete",`
 height: 180px;
 position: relative;
 margin-bottom: 8px;
 cursor: crosshair;
 `,[R("layer",`
 position: absolute;
 left: 0;
 right: 0;
 top: 0;
 bottom: 0;
 `,[x("shadowed",`
 box-shadow: inset 0 0 2px 0 rgba(0, 0, 0, .24);
 `)])]),d("color-picker-preview",`
 display: flex;
 `,[R("sliders",`
 flex: 1 0 auto;
 `),R("preview",`
 position: relative;
 height: 30px;
 width: 30px;
 margin: 0 0 8px 6px;
 border-radius: 50%;
 box-shadow: rgba(0, 0, 0, .15) 0px 0px 0px 1px inset;
 overflow: hidden;
 `),R("fill",`
 display: block;
 width: 30px;
 height: 30px;
 `),R("input",`
 position: absolute;
 top: 0;
 left: 0;
 width: 30px;
 height: 30px;
 opacity: 0;
 z-index: 1;
 `)]),d("color-picker-input",`
 display: flex;
 align-items: center;
 `,[d("input",`
 flex-grow: 1;
 flex-basis: 0;
 `),R("mode",`
 width: 72px;
 text-align: center;
 `)]),d("color-picker-control",`
 padding: 12px;
 `),d("color-picker-action",`
 display: flex;
 margin-top: -4px;
 border-top: 1px solid var(--n-divider-color);
 padding: 8px 12px;
 justify-content: flex-end;
 `,[d("button","margin-left: 8px;")]),d("color-picker-trigger",`
 border: var(--n-border);
 height: 100%;
 box-sizing: border-box;
 border-radius: var(--n-border-radius);
 transition: border-color .3s var(--n-bezier);
 cursor: pointer;
 `,[R("value",`
 white-space: nowrap;
 position: relative;
 `),R("fill",`
 border-radius: var(--n-border-radius);
 position: absolute;
 display: flex;
 align-items: center;
 justify-content: center;
 left: 4px;
 right: 4px;
 top: 4px;
 bottom: 4px;
 `),x("disabled","cursor: not-allowed"),d("color-picker-checkboard",`
 border-radius: var(--n-border-radius);
 `,[S("&::after",`
 --n-block-size: calc((var(--n-height) - 8px) / 3);
 background-size: calc(var(--n-block-size) * 2) calc(var(--n-block-size) * 2);
 background-position: 0 0, 0 var(--n-block-size), var(--n-block-size) calc(-1 * var(--n-block-size)), calc(-1 * var(--n-block-size)) 0px; 
 `)])]),d("color-picker-swatches",`
 display: grid;
 grid-gap: 8px;
 flex-wrap: wrap;
 position: relative;
 grid-template-columns: repeat(auto-fill, 18px);
 margin-top: 10px;
 `,[d("color-picker-swatch",`
 width: 18px;
 height: 18px;
 background-image: linear-gradient(45deg, #DDD 25%, #0000 25%), linear-gradient(-45deg, #DDD 25%, #0000 25%), linear-gradient(45deg, #0000 75%, #DDD 75%), linear-gradient(-45deg, #0000 75%, #DDD 75%);
 background-size: 8px 8px;
 background-position: 0px 0, 0px 4px, 4px -4px, -4px 0px;
 background-repeat: repeat;
 `,[R("fill",`
 position: relative;
 width: 100%;
 height: 100%;
 border-radius: 3px;
 box-shadow: rgba(0, 0, 0, .15) 0px 0px 0px 1px inset;
 cursor: pointer;
 `),S("&:focus",`
 outline: none;
 `,[R("fill",[S("&::after",`
 position: absolute;
 top: 0;
 right: 0;
 bottom: 0;
 left: 0;
 background: inherit;
 filter: blur(2px);
 content: "";
 `)])])])])]),tr=Object.assign(Object.assign({},et.props),{value:String,show:{type:Boolean,default:void 0},defaultShow:Boolean,defaultValue:String,modes:{type:Array,default:()=>["rgb","hex","hsl"]},placement:{type:String,default:"bottom-start"},to:mt.propTo,showAlpha:{type:Boolean,default:!0},showPreview:Boolean,swatches:Array,disabled:{type:Boolean,default:void 0},actions:{type:Array,default:null},internalActions:Array,size:String,renderLabel:Function,onComplete:Function,onConfirm:Function,onClear:Function,"onUpdate:show":[Function,Array],onUpdateShow:[Function,Array],"onUpdate:value":[Function,Array],onUpdateValue:[Function,Array]}),nr=G({name:"ColorPicker",props:tr,slots:Object,setup(e,{slots:t}){const n=I(null);let r=null;const o=An(e),{mergedSizeRef:a,mergedDisabledRef:c}=o,{localeRef:p}=co("global"),{mergedClsPrefixRef:g,namespaceRef:h,inlineThemeDisabled:v}=wt(e),w=et("ColorPicker","-color-picker",er,In,e,g);Ot($t,{themeRef:w,renderLabelRef:K(e,"renderLabel"),colorPickerSlots:t});const C=I(e.defaultShow),y=xt(K(e,"show"),C);function z(u){const{onUpdateShow:k,"onUpdate:show":i}=e;k&&be(k,u),i&&be(i,u),C.value=u}const{defaultValue:T}=e,H=I(T===void 0?Ao(e.modes,e.showAlpha):T),_=xt(K(e,"value"),H),B=I([_.value]),L=I(0),O=j(()=>We(_.value)),{modes:P}=e,m=I(We(_.value)||P[0]||"rgb");function D(){const{modes:u}=e,{value:k}=m,i=u.findIndex(s=>s===k);~i?m.value=u[(i+1)%u.length]:m.value="rgb"}let E,F,oe,Q,ee,X,Z,M;const ve=j(()=>{const{value:u}=_;if(!u)return null;switch(O.value){case"hsv":return Se(u);case"hsl":return[E,F,oe,M]=Re(u),[...Vt(E,F,oe),M];case"rgb":case"hex":return[ee,X,Z,M]=ne(u),[...ft(ee,X,Z),M]}}),re=j(()=>{const{value:u}=_;if(!u)return null;switch(O.value){case"rgb":case"hex":return ne(u);case"hsv":return[E,F,Q,M]=Se(u),[...fe(E,F,Q),M];case"hsl":return[E,F,oe,M]=Re(u),[...pt(E,F,oe),M]}}),_e=j(()=>{const{value:u}=_;if(!u)return null;switch(O.value){case"hsl":return Re(u);case"hsv":return[E,F,Q,M]=Se(u),[...Ke(E,F,Q),M];case"rgb":case"hex":return[ee,X,Z,M]=ne(u),[...bt(ee,X,Z),M]}}),He=j(()=>{switch(m.value){case"rgb":case"hex":return re.value;case"hsv":return ve.value;case"hsl":return _e.value}}),me=I(0),Ae=I(1),Ie=I([0,0]);function tt(u,k){const{value:i}=ve,s=me.value,f=i?i[3]:1;Ie.value=[u,k];const{showAlpha:b}=e;switch(m.value){case"hsv":U((b?Ce:vt)([s,u,k,f]),"cursor");break;case"hsl":U((b?se:ht)([...Ke(s,u,k),f]),"cursor");break;case"rgb":U((b?le:gt)([...fe(s,u,k),f]),"cursor");break;case"hex":U((b?he:Ve)([...fe(s,u,k),f]),"cursor");break}}function Oe(u){me.value=u;const{value:k}=ve;if(!k)return;const[,i,s,f]=k,{showAlpha:b}=e;switch(m.value){case"hsv":U((b?Ce:vt)([u,i,s,f]),"cursor");break;case"rgb":U((b?le:gt)([...fe(u,i,s),f]),"cursor");break;case"hex":U((b?he:Ve)([...fe(u,i,s),f]),"cursor");break;case"hsl":U((b?se:ht)([...Ke(u,i,s),f]),"cursor");break}}function de(u){switch(m.value){case"hsv":[E,F,Q]=ve.value,U(Ce([E,F,Q,u]),"cursor");break;case"rgb":[ee,X,Z]=re.value,U(le([ee,X,Z,u]),"cursor");break;case"hex":[ee,X,Z]=re.value,U(he([ee,X,Z,u]),"cursor");break;case"hsl":[E,F,oe]=_e.value,U(se([E,F,oe,u]),"cursor");break}Ae.value=u}function U(u,k){k==="cursor"?r=u:r=null;const{nTriggerFormChange:i,nTriggerFormInput:s}=o,{onUpdateValue:f,"onUpdate:value":b}=e;f&&be(f,u),b&&be(b,u),i(),s(),H.value=u}function Le(u){U(u,"input"),De(ce)}function ce(u=!0){const{value:k}=_;if(k){const{nTriggerFormChange:i,nTriggerFormInput:s}=o,{onComplete:f}=e;f&&f(k);const{value:b}=B,{value:$}=L;u&&(b.splice($+1,b.length,k),L.value=$+1),i(),s()}}function nt(){const{value:u}=L;u-1<0||(U(B.value[u-1],"input"),ce(!1),L.value=u-1)}function ue(){const{value:u}=L;u<0||u+1>=B.value.length||(U(B.value[u+1],"input"),ce(!1),L.value=u+1)}function ot(){U(null,"input");const{onClear:u}=e;u&&u(),z(!1)}function rt(){const{value:u}=_,{onConfirm:k}=e;k&&k(u),z(!1)}const at=j(()=>L.value>=1),Te=j(()=>{const{value:u}=B;return u.length>1&&L.value<u.length-1});Ze(y,u=>{u||(B.value=[_.value],L.value=0)}),St(()=>{if(!(r&&r===_.value)){const{value:u}=ve;u&&(me.value=u[0],Ae.value=u[3],Ie.value=[u[1],u[2]])}r=null});const Be=j(()=>{const{value:u}=a,{common:{cubicBezierEaseInOut:k},self:{textColor:i,color:s,panelFontSize:f,boxShadow:b,border:$,borderRadius:A,dividerColor:W,[Y("height",u)]:pe,[Y("fontSize",u)]:xe}}=w.value;return{"--n-bezier":k,"--n-text-color":i,"--n-color":s,"--n-panel-font-size":f,"--n-font-size":xe,"--n-box-shadow":b,"--n-border":$,"--n-border-radius":A,"--n-height":pe,"--n-divider-color":W}}),te=v?Lt("color-picker",j(()=>a.value[0]),Be,e):void 0;function it(){var u;const{value:k}=re,{value:i}=me,{internalActions:s,modes:f,actions:b}=e,{value:$}=w,{value:A}=g;return l("div",{class:[`${A}-color-picker-panel`,te?.themeClass.value],onDragstart:W=>{W.preventDefault()},style:v?void 0:Be.value},l("div",{class:`${A}-color-picker-control`},l(Qo,{clsPrefix:A,rgba:k,displayedHue:i,displayedSv:Ie.value,onUpdateSV:tt,onComplete:ce}),l("div",{class:`${A}-color-picker-preview`},l("div",{class:`${A}-color-picker-preview__sliders`},l(Jo,{clsPrefix:A,hue:i,onUpdateHue:Oe,onComplete:ce}),e.showAlpha?l(Eo,{clsPrefix:A,rgba:k,alpha:Ae.value,onUpdateAlpha:de,onComplete:ce}):null),e.showPreview?l(Xo,{clsPrefix:A,mode:m.value,color:re.value&&Ve(re.value),onUpdateColor:W=>{U(W,"input")}}):null),l(Fo,{clsPrefix:A,showAlpha:e.showAlpha,mode:m.value,modes:f,onUpdateMode:D,value:_.value,valueArr:He.value,onUpdateValue:Le}),((u=e.swatches)===null||u===void 0?void 0:u.length)&&l(qo,{clsPrefix:A,mode:m.value,swatches:e.swatches,onUpdateColor:W=>{U(W,"input")}})),b?.length?l("div",{class:`${A}-color-picker-action`},b.includes("confirm")&&l(Me,{size:"small",onClick:rt,theme:$.peers.Button,themeOverrides:$.peerOverrides.Button},{default:()=>p.value.confirm}),b.includes("clear")&&l(Me,{size:"small",onClick:ot,disabled:!_.value,theme:$.peers.Button,themeOverrides:$.peerOverrides.Button},{default:()=>p.value.clear})):null,t.action?l("div",{class:`${A}-color-picker-action`},{default:t.action}):s?l("div",{class:`${A}-color-picker-action`},s.includes("undo")&&l(Me,{size:"small",onClick:nt,disabled:!at.value,theme:$.peers.Button,themeOverrides:$.peerOverrides.Button},{default:()=>p.value.undo}),s.includes("redo")&&l(Me,{size:"small",onClick:ue,disabled:!Te.value,theme:$.peers.Button,themeOverrides:$.peerOverrides.Button},{default:()=>p.value.redo})):null)}return{mergedClsPrefix:g,namespace:h,selfRef:n,hsla:_e,rgba:re,mergedShow:y,mergedDisabled:c,isMounted:Tn(),adjustedTo:mt(e),mergedValue:_,handleTriggerClick(){z(!0)},handleClickOutside(u){var k;!((k=n.value)===null||k===void 0)&&k.contains(Bn(u))||z(!1)},renderPanel:it,cssVars:v?void 0:Be,themeClass:te?.themeClass,onRender:te?.onRender}},render(){const{mergedClsPrefix:e,onRender:t}=this;return t?.(),l("div",{class:[this.themeClass,`${e}-color-picker`],ref:"selfRef",style:this.cssVars},l(Rn,null,{default:()=>[l(zn,null,{default:()=>l(Go,{clsPrefix:e,value:this.mergedValue,hsla:this.hsla,disabled:this.mergedDisabled,onClick:this.handleTriggerClick})}),l(Pn,{placement:this.placement,show:this.mergedShow,containerClass:this.namespace,teleportDisabled:this.adjustedTo===mt.tdkey,to:this.adjustedTo},{default:()=>l(Wt,{name:"fade-in-scale-up-transition",appear:this.isMounted},{default:()=>this.mergedShow?Ht(this.renderPanel(),[[_n,this.handleClickOutside,void 0,{capture:!0}]]):null})})]}))}}),Xt=Dt("n-tabs"),or={tab:[String,Number,Object,Function],name:{type:[String,Number],required:!0},disabled:Boolean,displayDirective:{type:String,default:"if"},closable:{type:Boolean,default:void 0},tabProps:Object,label:[String,Number,Object,Function]},rr=Object.assign({internalLeftPadded:Boolean,internalAddable:Boolean,internalCreatedByPane:Boolean},Dn(or,["displayDirective"])),Qe=G({__TAB__:!0,inheritAttrs:!1,name:"Tab",props:rr,setup(e){const{mergedClsPrefixRef:t,valueRef:n,typeRef:r,closableRef:o,tabStyleRef:a,addTabStyleRef:c,tabClassRef:p,addTabClassRef:g,tabChangeIdRef:h,onBeforeLeaveRef:v,triggerRef:w,handleAdd:C,activateTab:y,handleClose:z}=kt(Xt);return{trigger:w,mergedClosable:j(()=>{if(e.internalAddable)return!1;const{closable:T}=e;return T===void 0?o.value:T}),style:a,addStyle:c,tabClass:p,addTabClass:g,clsPrefix:t,value:n,type:r,handleClose(T){T.stopPropagation(),!e.disabled&&z(e.name)},activateTab(){if(e.disabled)return;if(e.internalAddable){C();return}const{name:T}=e,H=++h.id;if(T!==n.value){const{value:_}=v;_?Promise.resolve(_(e.name,n.value)).then(B=>{B&&h.id===H&&y(T)}):y(T)}}}},render(){const{internalAddable:e,clsPrefix:t,name:n,disabled:r,label:o,tab:a,value:c,mergedClosable:p,trigger:g,$slots:{default:h}}=this,v=o??a;return l("div",{class:`${t}-tabs-tab-wrapper`},this.internalLeftPadded?l("div",{class:`${t}-tabs-tab-pad`}):null,l("div",Object.assign({key:n,"data-name":n,"data-disabled":r?!0:void 0},Un({class:[`${t}-tabs-tab`,c===n&&`${t}-tabs-tab--active`,r&&`${t}-tabs-tab--disabled`,p&&`${t}-tabs-tab--closable`,e&&`${t}-tabs-tab--addable`,e?this.addTabClass:this.tabClass],onClick:g==="click"?this.activateTab:void 0,onMouseenter:g==="hover"?this.activateTab:void 0,style:e?this.addStyle:this.style},this.internalCreatedByPane?this.tabProps||{}:this.$attrs)),l("span",{class:`${t}-tabs-tab__label`},e?l(Ct,null,l("div",{class:`${t}-tabs-tab__height-placeholder`}," "),l(En,{clsPrefix:t},{default:()=>l(so,null)})):h?h():typeof v=="object"?v:Mn(v??n)),p&&this.type==="card"?l(Vn,{clsPrefix:t,class:`${t}-tabs-tab__close`,onClick:this.handleClose,disabled:r}):null))}}),ar=d("tabs",`
 box-sizing: border-box;
 width: 100%;
 display: flex;
 flex-direction: column;
 transition:
 background-color .3s var(--n-bezier),
 border-color .3s var(--n-bezier);
`,[x("segment-type",[d("tabs-rail",[S("&.transition-disabled",[d("tabs-capsule",`
 transition: none;
 `)])])]),x("top",[d("tab-pane",`
 padding: var(--n-pane-padding-top) var(--n-pane-padding-right) var(--n-pane-padding-bottom) var(--n-pane-padding-left);
 `)]),x("left",[d("tab-pane",`
 padding: var(--n-pane-padding-right) var(--n-pane-padding-bottom) var(--n-pane-padding-left) var(--n-pane-padding-top);
 `)]),x("left, right",`
 flex-direction: row;
 `,[d("tabs-bar",`
 width: 2px;
 right: 0;
 transition:
 top .2s var(--n-bezier),
 max-height .2s var(--n-bezier),
 background-color .3s var(--n-bezier);
 `),d("tabs-tab",`
 padding: var(--n-tab-padding-vertical); 
 `)]),x("right",`
 flex-direction: row-reverse;
 `,[d("tab-pane",`
 padding: var(--n-pane-padding-left) var(--n-pane-padding-top) var(--n-pane-padding-right) var(--n-pane-padding-bottom);
 `),d("tabs-bar",`
 left: 0;
 `)]),x("bottom",`
 flex-direction: column-reverse;
 justify-content: flex-end;
 `,[d("tab-pane",`
 padding: var(--n-pane-padding-bottom) var(--n-pane-padding-right) var(--n-pane-padding-top) var(--n-pane-padding-left);
 `),d("tabs-bar",`
 top: 0;
 `)]),d("tabs-rail",`
 position: relative;
 padding: 3px;
 border-radius: var(--n-tab-border-radius);
 width: 100%;
 background-color: var(--n-color-segment);
 transition: background-color .3s var(--n-bezier);
 display: flex;
 align-items: center;
 `,[d("tabs-capsule",`
 border-radius: var(--n-tab-border-radius);
 position: absolute;
 pointer-events: none;
 background-color: var(--n-tab-color-segment);
 box-shadow: 0 1px 3px 0 rgba(0, 0, 0, .08);
 transition: transform 0.3s var(--n-bezier);
 `),d("tabs-tab-wrapper",`
 flex-basis: 0;
 flex-grow: 1;
 display: flex;
 align-items: center;
 justify-content: center;
 `,[d("tabs-tab",`
 overflow: hidden;
 border-radius: var(--n-tab-border-radius);
 width: 100%;
 display: flex;
 align-items: center;
 justify-content: center;
 `,[x("active",`
 font-weight: var(--n-font-weight-strong);
 color: var(--n-tab-text-color-active);
 `),S("&:hover",`
 color: var(--n-tab-text-color-hover);
 `)])])]),x("flex",[d("tabs-nav",`
 width: 100%;
 position: relative;
 `,[d("tabs-wrapper",`
 width: 100%;
 `,[d("tabs-tab",`
 margin-right: 0;
 `)])])]),d("tabs-nav",`
 box-sizing: border-box;
 line-height: 1.5;
 display: flex;
 transition: border-color .3s var(--n-bezier);
 `,[R("prefix, suffix",`
 display: flex;
 align-items: center;
 `),R("prefix","padding-right: 16px;"),R("suffix","padding-left: 16px;")]),x("top, bottom",[S(">",[d("tabs-nav",[d("tabs-nav-scroll-wrapper",[S("&::before",`
 top: 0;
 bottom: 0;
 left: 0;
 width: 20px;
 `),S("&::after",`
 top: 0;
 bottom: 0;
 right: 0;
 width: 20px;
 `),x("shadow-start",[S("&::before",`
 box-shadow: inset 10px 0 8px -8px rgba(0, 0, 0, .12);
 `)]),x("shadow-end",[S("&::after",`
 box-shadow: inset -10px 0 8px -8px rgba(0, 0, 0, .12);
 `)])])])])]),x("left, right",[d("tabs-nav-scroll-content",`
 flex-direction: column;
 `),S(">",[d("tabs-nav",[d("tabs-nav-scroll-wrapper",[S("&::before",`
 top: 0;
 left: 0;
 right: 0;
 height: 20px;
 `),S("&::after",`
 bottom: 0;
 left: 0;
 right: 0;
 height: 20px;
 `),x("shadow-start",[S("&::before",`
 box-shadow: inset 0 10px 8px -8px rgba(0, 0, 0, .12);
 `)]),x("shadow-end",[S("&::after",`
 box-shadow: inset 0 -10px 8px -8px rgba(0, 0, 0, .12);
 `)])])])])]),d("tabs-nav-scroll-wrapper",`
 flex: 1;
 position: relative;
 overflow: hidden;
 `,[d("tabs-nav-y-scroll",`
 height: 100%;
 width: 100%;
 overflow-y: auto; 
 scrollbar-width: none;
 `,[S("&::-webkit-scrollbar, &::-webkit-scrollbar-track-piece, &::-webkit-scrollbar-thumb",`
 width: 0;
 height: 0;
 display: none;
 `)]),S("&::before, &::after",`
 transition: box-shadow .3s var(--n-bezier);
 pointer-events: none;
 content: "";
 position: absolute;
 z-index: 1;
 `)]),d("tabs-nav-scroll-content",`
 display: flex;
 position: relative;
 min-width: 100%;
 min-height: 100%;
 width: fit-content;
 box-sizing: border-box;
 `),d("tabs-wrapper",`
 display: inline-flex;
 flex-wrap: nowrap;
 position: relative;
 `),d("tabs-tab-wrapper",`
 display: flex;
 flex-wrap: nowrap;
 flex-shrink: 0;
 flex-grow: 0;
 `),d("tabs-tab",`
 cursor: pointer;
 white-space: nowrap;
 flex-wrap: nowrap;
 display: inline-flex;
 align-items: center;
 color: var(--n-tab-text-color);
 font-size: var(--n-tab-font-size);
 background-clip: padding-box;
 padding: var(--n-tab-padding);
 transition:
 box-shadow .3s var(--n-bezier),
 color .3s var(--n-bezier),
 background-color .3s var(--n-bezier),
 border-color .3s var(--n-bezier);
 `,[x("disabled",{cursor:"not-allowed"}),R("close",`
 margin-left: 6px;
 transition:
 background-color .3s var(--n-bezier),
 color .3s var(--n-bezier);
 `),R("label",`
 display: flex;
 align-items: center;
 z-index: 1;
 `)]),d("tabs-bar",`
 position: absolute;
 bottom: 0;
 height: 2px;
 border-radius: 1px;
 background-color: var(--n-bar-color);
 transition:
 left .2s var(--n-bezier),
 max-width .2s var(--n-bezier),
 opacity .3s var(--n-bezier),
 background-color .3s var(--n-bezier);
 `,[S("&.transition-disabled",`
 transition: none;
 `),x("disabled",`
 background-color: var(--n-tab-text-color-disabled)
 `)]),d("tabs-pane-wrapper",`
 position: relative;
 overflow: hidden;
 transition: max-height .2s var(--n-bezier);
 `),d("tab-pane",`
 color: var(--n-pane-text-color);
 width: 100%;
 transition:
 color .3s var(--n-bezier),
 background-color .3s var(--n-bezier),
 opacity .2s var(--n-bezier);
 left: 0;
 right: 0;
 top: 0;
 `,[S("&.next-transition-leave-active, &.prev-transition-leave-active, &.next-transition-enter-active, &.prev-transition-enter-active",`
 transition:
 color .3s var(--n-bezier),
 background-color .3s var(--n-bezier),
 transform .2s var(--n-bezier),
 opacity .2s var(--n-bezier);
 `),S("&.next-transition-leave-active, &.prev-transition-leave-active",`
 position: absolute;
 `),S("&.next-transition-enter-from, &.prev-transition-leave-to",`
 transform: translateX(32px);
 opacity: 0;
 `),S("&.next-transition-leave-to, &.prev-transition-enter-from",`
 transform: translateX(-32px);
 opacity: 0;
 `),S("&.next-transition-leave-from, &.next-transition-enter-to, &.prev-transition-leave-from, &.prev-transition-enter-to",`
 transform: translateX(0);
 opacity: 1;
 `)]),d("tabs-tab-pad",`
 box-sizing: border-box;
 width: var(--n-tab-gap);
 flex-grow: 0;
 flex-shrink: 0;
 `),x("line-type, bar-type",[d("tabs-tab",`
 font-weight: var(--n-tab-font-weight);
 box-sizing: border-box;
 vertical-align: bottom;
 `,[S("&:hover",{color:"var(--n-tab-text-color-hover)"}),x("active",`
 color: var(--n-tab-text-color-active);
 font-weight: var(--n-tab-font-weight-active);
 `),x("disabled",{color:"var(--n-tab-text-color-disabled)"})])]),d("tabs-nav",[x("line-type",[x("top",[R("prefix, suffix",`
 border-bottom: 1px solid var(--n-tab-border-color);
 `),d("tabs-nav-scroll-content",`
 border-bottom: 1px solid var(--n-tab-border-color);
 `),d("tabs-bar",`
 bottom: -1px;
 `)]),x("left",[R("prefix, suffix",`
 border-right: 1px solid var(--n-tab-border-color);
 `),d("tabs-nav-scroll-content",`
 border-right: 1px solid var(--n-tab-border-color);
 `),d("tabs-bar",`
 right: -1px;
 `)]),x("right",[R("prefix, suffix",`
 border-left: 1px solid var(--n-tab-border-color);
 `),d("tabs-nav-scroll-content",`
 border-left: 1px solid var(--n-tab-border-color);
 `),d("tabs-bar",`
 left: -1px;
 `)]),x("bottom",[R("prefix, suffix",`
 border-top: 1px solid var(--n-tab-border-color);
 `),d("tabs-nav-scroll-content",`
 border-top: 1px solid var(--n-tab-border-color);
 `),d("tabs-bar",`
 top: -1px;
 `)]),R("prefix, suffix",`
 transition: border-color .3s var(--n-bezier);
 `),d("tabs-nav-scroll-content",`
 transition: border-color .3s var(--n-bezier);
 `),d("tabs-bar",`
 border-radius: 0;
 `)]),x("card-type",[R("prefix, suffix",`
 transition: border-color .3s var(--n-bezier);
 `),d("tabs-pad",`
 flex-grow: 1;
 transition: border-color .3s var(--n-bezier);
 `),d("tabs-tab-pad",`
 transition: border-color .3s var(--n-bezier);
 `),d("tabs-tab",`
 font-weight: var(--n-tab-font-weight);
 border: 1px solid var(--n-tab-border-color);
 background-color: var(--n-tab-color);
 box-sizing: border-box;
 position: relative;
 vertical-align: bottom;
 display: flex;
 justify-content: space-between;
 font-size: var(--n-tab-font-size);
 color: var(--n-tab-text-color);
 `,[x("addable",`
 padding-left: 8px;
 padding-right: 8px;
 font-size: 16px;
 justify-content: center;
 `,[R("height-placeholder",`
 width: 0;
 font-size: var(--n-tab-font-size);
 `),Wn("disabled",[S("&:hover",`
 color: var(--n-tab-text-color-hover);
 `)])]),x("closable","padding-right: 8px;"),x("active",`
 background-color: #0000;
 font-weight: var(--n-tab-font-weight-active);
 color: var(--n-tab-text-color-active);
 `),x("disabled","color: var(--n-tab-text-color-disabled);")])]),x("left, right",`
 flex-direction: column; 
 `,[R("prefix, suffix",`
 padding: var(--n-tab-padding-vertical);
 `),d("tabs-wrapper",`
 flex-direction: column;
 `),d("tabs-tab-wrapper",`
 flex-direction: column;
 `,[d("tabs-tab-pad",`
 height: var(--n-tab-gap-vertical);
 width: 100%;
 `)])]),x("top",[x("card-type",[d("tabs-scroll-padding","border-bottom: 1px solid var(--n-tab-border-color);"),R("prefix, suffix",`
 border-bottom: 1px solid var(--n-tab-border-color);
 `),d("tabs-tab",`
 border-top-left-radius: var(--n-tab-border-radius);
 border-top-right-radius: var(--n-tab-border-radius);
 `,[x("active",`
 border-bottom: 1px solid #0000;
 `)]),d("tabs-tab-pad",`
 border-bottom: 1px solid var(--n-tab-border-color);
 `),d("tabs-pad",`
 border-bottom: 1px solid var(--n-tab-border-color);
 `)])]),x("left",[x("card-type",[d("tabs-scroll-padding","border-right: 1px solid var(--n-tab-border-color);"),R("prefix, suffix",`
 border-right: 1px solid var(--n-tab-border-color);
 `),d("tabs-tab",`
 border-top-left-radius: var(--n-tab-border-radius);
 border-bottom-left-radius: var(--n-tab-border-radius);
 `,[x("active",`
 border-right: 1px solid #0000;
 `)]),d("tabs-tab-pad",`
 border-right: 1px solid var(--n-tab-border-color);
 `),d("tabs-pad",`
 border-right: 1px solid var(--n-tab-border-color);
 `)])]),x("right",[x("card-type",[d("tabs-scroll-padding","border-left: 1px solid var(--n-tab-border-color);"),R("prefix, suffix",`
 border-left: 1px solid var(--n-tab-border-color);
 `),d("tabs-tab",`
 border-top-right-radius: var(--n-tab-border-radius);
 border-bottom-right-radius: var(--n-tab-border-radius);
 `,[x("active",`
 border-left: 1px solid #0000;
 `)]),d("tabs-tab-pad",`
 border-left: 1px solid var(--n-tab-border-color);
 `),d("tabs-pad",`
 border-left: 1px solid var(--n-tab-border-color);
 `)])]),x("bottom",[x("card-type",[d("tabs-scroll-padding","border-top: 1px solid var(--n-tab-border-color);"),R("prefix, suffix",`
 border-top: 1px solid var(--n-tab-border-color);
 `),d("tabs-tab",`
 border-bottom-left-radius: var(--n-tab-border-radius);
 border-bottom-right-radius: var(--n-tab-border-radius);
 `,[x("active",`
 border-top: 1px solid #0000;
 `)]),d("tabs-tab-pad",`
 border-top: 1px solid var(--n-tab-border-color);
 `),d("tabs-pad",`
 border-top: 1px solid var(--n-tab-border-color);
 `)])])])]),ct=Ro,ir=Object.assign(Object.assign({},et.props),{value:[String,Number],defaultValue:[String,Number],trigger:{type:String,default:"click"},type:{type:String,default:"bar"},closable:Boolean,justifyContent:String,size:{type:String,default:"medium"},placement:{type:String,default:"top"},tabStyle:[String,Object],tabClass:String,addTabStyle:[String,Object],addTabClass:String,barWidth:Number,paneClass:String,paneStyle:[String,Object],paneWrapperClass:String,paneWrapperStyle:[String,Object],addable:[Boolean,Object],tabsPadding:{type:Number,default:0},animated:Boolean,onBeforeLeave:Function,onAdd:Function,"onUpdate:value":[Function,Array],onUpdateValue:[Function,Array],onClose:[Function,Array],labelSize:String,activeName:[String,Number],onActiveNameChange:[Function,Array]}),sr=G({name:"Tabs",props:ir,slots:Object,setup(e,{slots:t}){var n,r,o,a;const{mergedClsPrefixRef:c,inlineThemeDisabled:p}=wt(e),g=et("Tabs","-tabs",ar,Hn,e,c),h=I(null),v=I(null),w=I(null),C=I(null),y=I(null),z=I(null),T=I(!0),H=I(!0),_=_t(e,["labelSize","size"]),B=_t(e,["activeName","value"]),L=I((r=(n=B.value)!==null&&n!==void 0?n:e.defaultValue)!==null&&r!==void 0?r:t.default?(a=(o=st(t.default())[0])===null||o===void 0?void 0:o.props)===null||a===void 0?void 0:a.name:null),O=xt(B,L),P={id:0},m=j(()=>{if(!(!e.justifyContent||e.type==="card"))return{display:"flex",justifyContent:e.justifyContent}});Ze(O,()=>{P.id=0,Q(),ee()});function D(){var i;const{value:s}=O;return s===null?null:(i=h.value)===null||i===void 0?void 0:i.querySelector(`[data-name="${s}"]`)}function E(i){if(e.type==="card")return;const{value:s}=v;if(!s)return;const f=s.style.opacity==="0";if(i){const b=`${c.value}-tabs-bar--disabled`,{barWidth:$,placement:A}=e;if(i.dataset.disabled==="true"?s.classList.add(b):s.classList.remove(b),["top","bottom"].includes(A)){if(oe(["top","maxHeight","height"]),typeof $=="number"&&i.offsetWidth>=$){const W=Math.floor((i.offsetWidth-$)/2)+i.offsetLeft;s.style.left=`${W}px`,s.style.maxWidth=`${$}px`}else s.style.left=`${i.offsetLeft}px`,s.style.maxWidth=`${i.offsetWidth}px`;s.style.width="8192px",f&&(s.style.transition="none"),s.offsetWidth,f&&(s.style.transition="",s.style.opacity="1")}else{if(oe(["left","maxWidth","width"]),typeof $=="number"&&i.offsetHeight>=$){const W=Math.floor((i.offsetHeight-$)/2)+i.offsetTop;s.style.top=`${W}px`,s.style.maxHeight=`${$}px`}else s.style.top=`${i.offsetTop}px`,s.style.maxHeight=`${i.offsetHeight}px`;s.style.height="8192px",f&&(s.style.transition="none"),s.offsetHeight,f&&(s.style.transition="",s.style.opacity="1")}}}function F(){if(e.type==="card")return;const{value:i}=v;i&&(i.style.opacity="0")}function oe(i){const{value:s}=v;if(s)for(const f of i)s.style[f]=""}function Q(){if(e.type==="card")return;const i=D();i?E(i):F()}function ee(){var i;const s=(i=y.value)===null||i===void 0?void 0:i.$el;if(!s)return;const f=D();if(!f)return;const{scrollLeft:b,offsetWidth:$}=s,{offsetLeft:A,offsetWidth:W}=f;b>A?s.scrollTo({top:0,left:A,behavior:"smooth"}):A+W>b+$&&s.scrollTo({top:0,left:A+W-$,behavior:"smooth"})}const X=I(null);let Z=0,M=null;function ve(i){const s=X.value;if(s){Z=i.getBoundingClientRect().height;const f=`${Z}px`,b=()=>{s.style.height=f,s.style.maxHeight=f};M?(b(),M(),M=null):M=b}}function re(i){const s=X.value;if(s){const f=i.getBoundingClientRect().height,b=()=>{document.body.offsetHeight,s.style.maxHeight=`${f}px`,s.style.height=`${Math.max(Z,f)}px`};M?(M(),M=null,b()):M=b}}function _e(){const i=X.value;if(i){i.style.maxHeight="",i.style.height="";const{paneWrapperStyle:s}=e;if(typeof s=="string")i.style.cssText=s;else if(s){const{maxHeight:f,height:b}=s;f!==void 0&&(i.style.maxHeight=f),b!==void 0&&(i.style.height=b)}}}const He={value:[]},me=I("next");function Ae(i){const s=O.value;let f="next";for(const b of He.value){if(b===s)break;if(b===i){f="prev";break}}me.value=f,Ie(i)}function Ie(i){const{onActiveNameChange:s,onUpdateValue:f,"onUpdate:value":b}=e;s&&be(s,i),f&&be(f,i),b&&be(b,i),L.value=i}function tt(i){const{onClose:s}=e;s&&be(s,i)}function Oe(){const{value:i}=v;if(!i)return;const s="transition-disabled";i.classList.add(s),Q(),i.classList.remove(s)}const de=I(null);function U({transitionDisabled:i}){const s=h.value;if(!s)return;i&&s.classList.add("transition-disabled");const f=D();f&&de.value&&(de.value.style.width=`${f.offsetWidth}px`,de.value.style.height=`${f.offsetHeight}px`,de.value.style.transform=`translateX(${f.offsetLeft-Fn(getComputedStyle(s).paddingLeft)}px)`,i&&de.value.offsetWidth),i&&s.classList.remove("transition-disabled")}Ze([O],()=>{e.type==="segment"&&De(()=>{U({transitionDisabled:!1})})}),On(()=>{e.type==="segment"&&U({transitionDisabled:!0})});let Le=0;function ce(i){var s;if(i.contentRect.width===0&&i.contentRect.height===0||Le===i.contentRect.width)return;Le=i.contentRect.width;const{type:f}=e;if((f==="line"||f==="bar")&&Oe(),f!=="segment"){const{placement:b}=e;Te((b==="top"||b==="bottom"?(s=y.value)===null||s===void 0?void 0:s.$el:z.value)||null)}}const nt=ct(ce,64);Ze([()=>e.justifyContent,()=>e.size],()=>{De(()=>{const{type:i}=e;(i==="line"||i==="bar")&&Oe()})});const ue=I(!1);function ot(i){var s;const{target:f,contentRect:{width:b,height:$}}=i,A=f.parentElement.parentElement.offsetWidth,W=f.parentElement.parentElement.offsetHeight,{placement:pe}=e;if(!ue.value)pe==="top"||pe==="bottom"?A<b&&(ue.value=!0):W<$&&(ue.value=!0);else{const{value:xe}=C;if(!xe)return;pe==="top"||pe==="bottom"?A-b>xe.$el.offsetWidth&&(ue.value=!1):W-$>xe.$el.offsetHeight&&(ue.value=!1)}Te(((s=y.value)===null||s===void 0?void 0:s.$el)||null)}const rt=ct(ot,64);function at(){const{onAdd:i}=e;i&&i(),De(()=>{const s=D(),{value:f}=y;!s||!f||f.scrollTo({left:s.offsetLeft,top:0,behavior:"smooth"})})}function Te(i){if(!i)return;const{placement:s}=e;if(s==="top"||s==="bottom"){const{scrollLeft:f,scrollWidth:b,offsetWidth:$}=i;T.value=f<=0,H.value=f+$>=b}else{const{scrollTop:f,scrollHeight:b,offsetHeight:$}=i;T.value=f<=0,H.value=f+$>=b}}const Be=ct(i=>{Te(i.target)},64);Ot(Xt,{triggerRef:K(e,"trigger"),tabStyleRef:K(e,"tabStyle"),tabClassRef:K(e,"tabClass"),addTabStyleRef:K(e,"addTabStyle"),addTabClassRef:K(e,"addTabClass"),paneClassRef:K(e,"paneClass"),paneStyleRef:K(e,"paneStyle"),mergedClsPrefixRef:c,typeRef:K(e,"type"),closableRef:K(e,"closable"),valueRef:O,tabChangeIdRef:P,onBeforeLeaveRef:K(e,"onBeforeLeave"),activateTab:Ae,handleClose:tt,handleAdd:at}),Ln(()=>{Q(),ee()}),St(()=>{const{value:i}=w;if(!i)return;const{value:s}=c,f=`${s}-tabs-nav-scroll-wrapper--shadow-start`,b=`${s}-tabs-nav-scroll-wrapper--shadow-end`;T.value?i.classList.remove(f):i.classList.add(f),H.value?i.classList.remove(b):i.classList.add(b)});const te={syncBarPosition:()=>{Q()}},it=()=>{U({transitionDisabled:!0})},u=j(()=>{const{value:i}=_,{type:s}=e,f={card:"Card",bar:"Bar",line:"Line",segment:"Segment"}[s],b=`${i}${f}`,{self:{barColor:$,closeIconColor:A,closeIconColorHover:W,closeIconColorPressed:pe,tabColor:xe,tabBorderColor:Kt,paneTextColor:Zt,tabFontWeight:Yt,tabBorderRadius:Jt,tabFontWeightActive:Qt,colorSegment:en,fontWeightStrong:tn,tabColorSegment:nn,closeSize:on,closeIconSize:rn,closeColorHover:an,closeColorPressed:sn,closeBorderRadius:ln,[Y("panePadding",i)]:Fe,[Y("tabPadding",b)]:dn,[Y("tabPaddingVertical",b)]:cn,[Y("tabGap",b)]:un,[Y("tabGap",`${b}Vertical`)]:pn,[Y("tabTextColor",s)]:fn,[Y("tabTextColorActive",s)]:bn,[Y("tabTextColorHover",s)]:hn,[Y("tabTextColorDisabled",s)]:gn,[Y("tabFontSize",i)]:vn},common:{cubicBezierEaseInOut:mn}}=g.value;return{"--n-bezier":mn,"--n-color-segment":en,"--n-bar-color":$,"--n-tab-font-size":vn,"--n-tab-text-color":fn,"--n-tab-text-color-active":bn,"--n-tab-text-color-disabled":gn,"--n-tab-text-color-hover":hn,"--n-pane-text-color":Zt,"--n-tab-border-color":Kt,"--n-tab-border-radius":Jt,"--n-close-size":on,"--n-close-icon-size":rn,"--n-close-color-hover":an,"--n-close-color-pressed":sn,"--n-close-border-radius":ln,"--n-close-icon-color":A,"--n-close-icon-color-hover":W,"--n-close-icon-color-pressed":pe,"--n-tab-color":xe,"--n-tab-font-weight":Yt,"--n-tab-font-weight-active":Qt,"--n-tab-padding":dn,"--n-tab-padding-vertical":cn,"--n-tab-gap":un,"--n-tab-gap-vertical":pn,"--n-pane-padding-left":je(Fe,"left"),"--n-pane-padding-right":je(Fe,"right"),"--n-pane-padding-top":je(Fe,"top"),"--n-pane-padding-bottom":je(Fe,"bottom"),"--n-font-weight-strong":tn,"--n-tab-color-segment":nn}}),k=p?Lt("tabs",j(()=>`${_.value[0]}${e.type[0]}`),u,e):void 0;return Object.assign({mergedClsPrefix:c,mergedValue:O,renderedNames:new Set,segmentCapsuleElRef:de,tabsPaneWrapperRef:X,tabsElRef:h,barElRef:v,addTabInstRef:C,xScrollInstRef:y,scrollWrapperElRef:w,addTabFixed:ue,tabWrapperStyle:m,handleNavResize:nt,mergedSize:_,handleScroll:Be,handleTabsResize:rt,cssVars:p?void 0:u,themeClass:k?.themeClass,animationDirection:me,renderNameListRef:He,yScrollElRef:z,handleSegmentResize:it,onAnimationBeforeLeave:ve,onAnimationEnter:re,onAnimationAfterEnter:_e,onRender:k?.onRender},te)},render(){const{mergedClsPrefix:e,type:t,placement:n,addTabFixed:r,addable:o,mergedSize:a,renderNameListRef:c,onRender:p,paneWrapperClass:g,paneWrapperStyle:h,$slots:{default:v,prefix:w,suffix:C}}=this;p?.();const y=v?st(v()).filter(P=>P.type.__TAB_PANE__===!0):[],z=v?st(v()).filter(P=>P.type.__TAB__===!0):[],T=!z.length,H=t==="card",_=t==="segment",B=!H&&!_&&this.justifyContent;c.value=[];const L=()=>{const P=l("div",{style:this.tabWrapperStyle,class:`${e}-tabs-wrapper`},B?null:l("div",{class:`${e}-tabs-scroll-padding`,style:n==="top"||n==="bottom"?{width:`${this.tabsPadding}px`}:{height:`${this.tabsPadding}px`}}),T?y.map((m,D)=>(c.value.push(m.props.name),ut(l(Qe,Object.assign({},m.props,{internalCreatedByPane:!0,internalLeftPadded:D!==0&&(!B||B==="center"||B==="start"||B==="end")}),m.children?{default:m.children.tab}:void 0)))):z.map((m,D)=>(c.value.push(m.props.name),ut(D!==0&&!B?Et(m):m))),!r&&o&&H?Mt(o,(T?y.length:z.length)!==0):null,B?null:l("div",{class:`${e}-tabs-scroll-padding`,style:{width:`${this.tabsPadding}px`}}));return l("div",{ref:"tabsElRef",class:`${e}-tabs-nav-scroll-content`},H&&o?l(lt,{onResize:this.handleTabsResize},{default:()=>P}):P,H?l("div",{class:`${e}-tabs-pad`}):null,H?null:l("div",{ref:"barElRef",class:`${e}-tabs-bar`}))},O=_?"top":n;return l("div",{class:[`${e}-tabs`,this.themeClass,`${e}-tabs--${t}-type`,`${e}-tabs--${a}-size`,B&&`${e}-tabs--flex`,`${e}-tabs--${O}`],style:this.cssVars},l("div",{class:[`${e}-tabs-nav--${t}-type`,`${e}-tabs-nav--${O}`,`${e}-tabs-nav`]},Pt(w,P=>P&&l("div",{class:`${e}-tabs-nav__prefix`},P)),_?l(lt,{onResize:this.handleSegmentResize},{default:()=>l("div",{class:`${e}-tabs-rail`,ref:"tabsElRef"},l("div",{class:`${e}-tabs-capsule`,ref:"segmentCapsuleElRef"},l("div",{class:`${e}-tabs-wrapper`},l("div",{class:`${e}-tabs-tab`}))),T?y.map((P,m)=>(c.value.push(P.props.name),l(Qe,Object.assign({},P.props,{internalCreatedByPane:!0,internalLeftPadded:m!==0}),P.children?{default:P.children.tab}:void 0))):z.map((P,m)=>(c.value.push(P.props.name),m===0?P:Et(P))))}):l(lt,{onResize:this.handleNavResize},{default:()=>l("div",{class:`${e}-tabs-nav-scroll-wrapper`,ref:"scrollWrapperElRef"},["top","bottom"].includes(O)?l(po,{ref:"xScrollInstRef",onScroll:this.handleScroll},{default:L}):l("div",{class:`${e}-tabs-nav-y-scroll`,onScroll:this.handleScroll,ref:"yScrollElRef"},L()))}),r&&o&&H?Mt(o,!0):null,Pt(C,P=>P&&l("div",{class:`${e}-tabs-nav__suffix`},P))),T&&(this.animated&&(O==="top"||O==="bottom")?l("div",{ref:"tabsPaneWrapperRef",style:h,class:[`${e}-tabs-pane-wrapper`,g]},Ut(y,this.mergedValue,this.renderedNames,this.onAnimationBeforeLeave,this.onAnimationEnter,this.onAnimationAfterEnter,this.animationDirection)):Ut(y,this.mergedValue,this.renderedNames)))}});function Ut(e,t,n,r,o,a,c){const p=[];return e.forEach(g=>{const{name:h,displayDirective:v,"display-directive":w}=g.props,C=z=>v===z||w===z,y=t===h;if(g.key!==void 0&&(g.key=h),y||C("show")||C("show:lazy")&&n.has(h)){n.has(h)||n.add(h);const z=!C("if");p.push(z?Ht(g,[[jn,y]]):g)}}),c?l(Nn,{name:`${c}-transition`,onBeforeLeave:r,onEnter:o,onAfterEnter:a},{default:()=>p}):p}function Mt(e,t){return l(Qe,{ref:"addTabInstRef",key:"__addable",name:"__addable",internalCreatedByPane:!0,internalAddable:!0,internalLeftPadded:t,disabled:typeof e=="object"&&e.disabled})}function Et(e){const t=qn(e);return t.props?t.props.internalLeftPadded=!0:t.props={internalLeftPadded:!0},t}function ut(e){return Array.isArray(e.dynamicProps)?e.dynamicProps.includes("internalLeftPadded")||e.dynamicProps.push("internalLeftPadded"):e.dynamicProps=["internalLeftPadded"],e}const Ir={__name:"ToggleTheme",setup(e){const t=Ft(),n=Gn();async function r({clientX:o,clientY:a}){function c(){t.toggleDark(),Xn(n)()}if(!document.startViewTransition)return c();const p=[`circle(0px at ${o}px ${a}px)`,`circle(${Math.hypot(Math.max(o,window.innerWidth-o),Math.max(a,window.innerHeight-a))}px at ${o}px ${a}px)`];await document.startViewTransition(c).ready,document.documentElement.animate({clipPath:n.value?p.reverse():p},{duration:500,easing:"ease-in",pseudoElement:`::view-transition-${n.value?"old":"new"}(root)`,fill:"both"})}return(o,a)=>(J(),ge("i",{id:"toggleTheme",class:jt(["mr-16 cursor-pointer",V(n)?"i-fe:moon":"i-fe:sun"]),onClick:r},null,2))}};var lr=void 0;const dr=(e,t)=>{let n=null,r=!0;return function(){if(!r)return;r=!1;for(var o=arguments.length,a=new Array(o),c=0;c<o;c++)a[c]=arguments[c];let p=a;n&&clearTimeout(n),n=setTimeout(()=>{r=!0,e.apply(lr,p)},t)}};var Rt=G({name:"Vue3IntroStep",props:{show:{type:Boolean,required:!0},config:{type:Object,required:!0}},emits:["update:show"],data(){return{originalBox:{left:250,top:250,width:200,height:100},tipBoxPosition:"bottom",currentIndex:0}},watch:{config:{deep:!0,handler(){this.currentIndex=0},immediate:!0},show(e){e?this.setBoxInfo():document.body.style.overflow="auto"}},computed:{tipBoxStyle(){if(this.tipBoxPosition==="right")return{left:`${this.originalBox.left+this.originalBox.width}px`,top:`${this.originalBox.top}px`};if(this.tipBoxPosition==="left")return{right:`${window.innerWidth-this.originalBox.left}px`,top:`${this.originalBox.top}px`};if(this.tipBoxPosition==="top")return{left:`${this.originalBox.left}px`,bottom:`${window.innerHeight-this.originalBox.top}px`};if(this.tipBoxPosition==="bottom")return{left:`${this.originalBox.left>window.innerWidth-300?window.innerWidth-300:this.originalBox.left}px`,top:`${this.originalBox.top+this.originalBox.height}px`}}},created(){this.init()},mounted(){window.onresize=dr(()=>{this.show&&this.setBoxInfo()},100)},beforeUnmount(){window.onresize=null},methods:{async prev(){let e=!0;if(this.config.tips[this.currentIndex]&&this.config.tips[this.currentIndex].onPrev&&(e=await this.config.tips[this.currentIndex].onPrev()),!e)throw new Error("onPrev 需要 Promise.resolve(true) 才可以继续往下走");this.setBoxInfo(this.currentIndex-1)},async next(){let e=!0;if(this.config.tips[this.currentIndex]&&this.config.tips[this.currentIndex].onNext&&(e=await this.config.tips[this.currentIndex].onNext()),!e)throw new Error("onNext 需要 Promise.resolve(true) 才可以继续往下走");this.setBoxInfo(this.currentIndex+1)},done(){this.$emit("update:show",!1)},async setBoxInfo(e){try{e===void 0&&(e=this.currentIndex),this.show&&(document.body.style.overflow="hidden");let t=this.config.tips[e].el,n=document.querySelector(t);if(!n)throw new Error("没有找到相应的元素");let r=n.getBoundingClientRect();this.originalBox={left:r.left,top:r.top,width:r.width,height:r.height},this.tipBoxPosition=this.config.tips[e].tipPosition,this.currentIndex=e}catch(t){throw new Error(t.message)}},init(){const{tips:e}=this.config;let t=null;if(e&&Array.isArray(e))if(e.length>0){this.currentIndex=0;try{let n=document.querySelector(e[0].el);t=setInterval(()=>{n=document.querySelector(e[0].el),n&&(this.setBoxInfo(0),clearInterval(t))},0)}catch(n){throw new Error(n.message)}}else throw new Error("tips数组不能为空");else throw new Error("config中的tips不存在或者不是数组")}}});const cr=e=>(Kn("data-v-5d3b253c"),e=e(),Zn(),e),ur={key:0,id:"intro_box"},pr=cr(()=>N("div",{class:"round round-flicker"},null,-1)),fr=[pr],br={class:"tip-content"},hr={class:"action",style:{justifyContent:"center"}};function gr(e,t,n,r,o,a){return J(),Je(Wt,{name:"custom-classes-transition","enter-active-class":"animate__animated animate__fadeIn animate__faster","leave-active-class":"animate__animated animate__fadeOut animate__faster"},{default:q(()=>[e.show?(J(),ge("div",ur,[N("div",{class:"top",style:ae({height:`${e.originalBox.top}px`,backgroundColor:`rgba(0, 0, 0, ${e.config.backgroundOpacity?e.config.backgroundOpacity:.9})`})},null,4),N("div",{class:"content",style:ae({height:`${e.originalBox.height}px`})},[N("div",{class:"left",style:ae({top:`${e.originalBox.top}px`,width:`${e.originalBox.left}px`,height:`${e.originalBox.height}px`,backgroundColor:`rgba(0, 0, 0, ${e.config.backgroundOpacity?e.config.backgroundOpacity:.9})`})},null,4),N("div",{class:"original-box",style:ae({top:`${e.originalBox.top}px`,left:`${e.originalBox.left}px`,width:`${e.originalBox.width}px`,height:`${e.originalBox.height}px`})},fr,4),N("div",{class:"tip-box",style:ae(e.tipBoxStyle)},[N("div",br,[e.config.tips[e.currentIndex].title?(J(),ge("div",{key:0,class:"title",style:ae({textAlign:e.config.titleStyle&&e.config.titleStyle.textAlign?e.config.titleStyle.textAlign:"center",fontSize:e.config.titleStyle&&e.config.titleStyle.fontSize?e.config.titleStyle.fontSize:"19px"})},yt(e.config.tips[e.currentIndex].title),5)):Ee("",!0),N("div",{class:"content",style:ae({textAlign:e.config.contentStyle&&e.config.contentStyle.textAlign?e.config.contentStyle.textAlign:"center",fontSize:e.config.contentStyle&&e.config.contentStyle.fontSize?e.config.contentStyle.fontSize:"15px"})},yt(e.config.tips[e.currentIndex].content),5),N("div",hr,[e.currentIndex!==0?Ne(e.$slots,"prev",{key:0,index:e.currentIndex,tipItem:e.config.tips[e.currentIndex]},()=>[N("div",{class:"item prev",onClick:t[0]||(t[0]=function(){return e.prev&&e.prev(...arguments)})},"上一步")]):Ee("",!0),e.currentIndex!==e.config.tips.length-1?Ne(e.$slots,"next",{key:1,index:e.currentIndex,tipItem:e.config.tips[e.currentIndex]},()=>[N("div",{class:"item next",onClick:t[1]||(t[1]=function(){return e.next&&e.next(...arguments)})},"下一步")]):Ee("",!0),e.currentIndex===e.config.tips.length-1?Ne(e.$slots,"done",{key:2,index:e.currentIndex,tipItem:e.config.tips[e.currentIndex]},()=>[N("div",{class:"item done",onClick:t[2]||(t[2]=function(){return e.done&&e.done(...arguments)})},"完成")]):Ne(e.$slots,"skip",{key:3,index:e.currentIndex,tipItem:e.config.tips[e.currentIndex]},()=>[N("div",{class:"item skip",onClick:t[3]||(t[3]=function(){return e.done&&e.done(...arguments)})},"跳过")])])])],4),N("div",{class:"right",style:ae({top:`${e.originalBox.top}px`,left:`${e.originalBox.left+e.originalBox.width}px`,width:`calc(100% - ${e.originalBox.left+e.originalBox.width}px)`,height:`${e.originalBox.height}px`,backgroundColor:`rgba(0, 0, 0, ${e.config.backgroundOpacity?e.config.backgroundOpacity:.9})`}),ref:"tip_box"},null,4)],4),N("div",{class:"bottom",style:ae({height:`calc(100% - ${e.originalBox.top}px - ${e.originalBox.height}px)`,backgroundColor:`rgba(0, 0, 0, ${e.config.backgroundOpacity?e.config.backgroundOpacity:.9})`})},null,4)])):Ee("",!0)]),_:3})}function vr(e,t){t===void 0&&(t={});var n=t.insertAt;if(!(typeof document>"u")){var r=document.head||document.getElementsByTagName("head")[0],o=document.createElement("style");o.type="text/css",n==="top"&&r.firstChild?r.insertBefore(o,r.firstChild):r.appendChild(o),o.styleSheet?o.styleSheet.cssText=e:o.appendChild(document.createTextNode(e))}}var mr=`
#intro_box[data-v-5d3b253c] {
  position: fixed;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  z-index: 99999;
}
#intro_box > .top[data-v-5d3b253c] {
  width: 100%;
}
#intro_box > .content[data-v-5d3b253c] {
  width: 100%;
}
#intro_box > .content > .left[data-v-5d3b253c] {
  position: absolute;
  left: 0;
}
#intro_box > .content > .original-box[data-v-5d3b253c] {
  position: absolute;
  background-color: transparent;
  transition: all 0.3s cubic-bezier(0, 0, 0.58, 1);
}
#intro_box > .content > .original-box .round[data-v-5d3b253c] {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  width: 10px;
  height: 10px;
  border-radius: 50%;
  opacity: 0.65;
  background-color: #9900ff;
}
#intro_box > .content > .original-box .round-flicker[data-v-5d3b253c]:before,
#intro_box > .content > .original-box .round-flicker[data-v-5d3b253c]:after {
  content: '';
  width: 100%;
  height: 100%;
  position: absolute;
  left: -1px;
  top: -1px;
  box-shadow: #9900ff 0px 0px 2px 2px;
  border: 1px solid rgba(153, 0, 255, 0.5);
  border-radius: 50%;
  animation: warn-5d3b253c 2s linear 0s infinite;
}
@keyframes warn-5d3b253c {
0% {
    transform: scale(0.5);
    opacity: 1;
}
25% {
    transform: scale(1);
    opacity: 0.75;
}
50% {
    transform: scale(1.5);
    opacity: 0.5;
}
75% {
    transform: scale(2);
    opacity: 0.25;
}
100% {
    transform: scale(2.5);
    opacity: 0;
}
}
#intro_box > .content > .tip-box[data-v-5d3b253c] {
  position: absolute;
  /*宽度应为内容宽*/
  width: fit-content;
  max-width: 300px;
  box-sizing: border-box;
  /*高度应为内容高度*/
  height: fit-content;
  transition: all 0.3s;
  z-index: 99999;
  padding: 12px;
  font-size: 15px;
}
#intro_box > .content > .tip-box > .tip-content[data-v-5d3b253c] {
  border-radius: 10px;
  overflow: hidden;
  padding: 10px;
  color: #fff;
}
#intro_box > .content > .tip-box > .tip-content > .title[data-v-5d3b253c] {
  font-weight: bold;
  margin-bottom: 10px;
}
#intro_box > .content > .tip-box > .tip-content > .content[data-v-5d3b253c] {
  white-space: normal;
  overflow-wrap: break-word;
  line-height: 1.5;
}
#intro_box > .content > .tip-box > .tip-content > .action[data-v-5d3b253c] {
  margin-top: 15px;
  width: 100%;
  display: flex;
}
#intro_box > .content > .tip-box > .tip-content > .action > .item[data-v-5d3b253c] {
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
  border-radius: 15px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.3s;
  padding: 5px 15px;
  color: #fff;
  font-weight: bold;
  border: 1px solid #ccc;
  margin: 5px;
}
#intro_box > .content > .tip-box > .tip-content > .action > .item.prev[data-v-5d3b253c] {
  color: #ccc;
}
#intro_box > .content > .tip-box > .tip-content > .action > .item.next[data-v-5d3b253c] {
  color: #ccc;
}
#intro_box > .content > .tip-box > .tip-content > .action > .item.done[data-v-5d3b253c] {
  color: #ccc;
}
#intro_box > .content > .tip-box > .tip-content > .action > .item.skip[data-v-5d3b253c] {
  color: #ccc;
}
#intro_box > .content > .right[data-v-5d3b253c] {
  position: absolute;
  background-color: rgba(0, 0, 0, 0.9);
}
#intro_box > .bottom[data-v-5d3b253c] {
  width: 100%;
  background-color: rgba(0, 0, 0, 0.9);
}
`;vr(mr);Rt.render=gr;Rt.__scopeId="data-v-5d3b253c";var xr=(()=>{const e=Rt;return e.install=t=>{t.component("Vue3IntroStep",e)},e})();const Tr={__name:"BeginnerGuide",setup(e){const t=At(null),n=At(!1),r={backgroundOpacity:.8,titleStyle:{textAlign:"left",fontSize:"18px"},contentStyle:{textAlign:"left",fontSize:"14px"},tips:[{el:"#toggleTheme",tipPosition:"bottom",title:"切换系统主题",content:"一键开启护眼模式"},{el:"#fullscreen",tipPosition:"bottom",title:"全屏/退出全屏",content:"一键开启全屏"},{el:"#theme-setting",tipPosition:"bottom",title:"设置主题色",content:"调整为你喜欢的主题色"},{el:"#user-dropdown",tipPosition:"bottom",title:"个人中心",content:"查看个人资料和退出系统"},{el:"#menu-collapse",tipPosition:"bottom",title:"展开/收起菜单",content:"一键展开/收起菜单"},{el:"#top-tab",tipPosition:"bottom",title:"标签栏",content:"鼠标滚轮滑动可调整至最佳视野"},{el:"#layout-setting",tipPosition:"left",title:"调整系统布局",content:"将系统布局调整为你喜欢的样子"}]};function o(){n.value=!1}function a(){n.value=!1}function c(){t.value.next()}function p(){t.value.prev()}return(g,h)=>{const v=Nt,w=Me;return J(),ge(Ct,null,[ie(v,{trigger:"hover"},{trigger:q(()=>[N("i",{class:"i-fe:beginner mr-16 cursor-pointer text-20",onClick:h[0]||(h[0]=C=>n.value=!0)})]),default:q(()=>[h[2]||(h[2]=ke(" 操作指引 ",-1))]),_:1}),ie(V(xr),{ref_key:"myIntroStep",ref:t,show:V(n),"onUpdate:show":h[1]||(h[1]=C=>Yn(n)?n.value=C:null),config:r},{prev:q(({tipItem:C,index:y})=>[ie(w,{class:"mr-12",type:"primary",color:"#fff","text-color":"#fff",ghost:"",round:"",size:"small",onClick:z=>p(C,y)},{default:q(()=>[...h[3]||(h[3]=[ke(" 上一步 ",-1)])]),_:1},8,["onClick"])]),next:q(({tipItem:C})=>[ie(w,{class:"mr-12",type:"primary",color:"#fff","text-color":"#fff",ghost:"",round:"",size:"small",onClick:y=>c(C)},{default:q(()=>[...h[4]||(h[4]=[ke(" 下一步 ",-1)])]),_:1},8,["onClick"])]),skip:q(()=>[ie(w,{type:"primary",color:"#fff","text-color":"#fff",ghost:"",round:"",size:"small",onClick:o},{default:q(()=>[...h[5]||(h[5]=[ke(" 跳过 ",-1)])]),_:1})]),done:q(()=>[ie(w,{type:"primary",color:"#fff","text-color":"#fff",ghost:"",round:"",size:"small",onClick:a},{default:q(()=>[...h[6]||(h[6]=[ke(" 完成 ",-1)])]),_:1})]),_:1},8,["show"])],64)}}},Br={__name:"Fullscreen",setup(e){const{isFullscreen:t,toggle:n}=Jn();return(r,o)=>(J(),ge("i",{id:"fullscreen",class:jt(["mr-16 cursor-pointer",V(t)?"i-fe:minimize":"i-fe:maximize"]),onClick:o[0]||(o[0]=(...a)=>V(n)&&V(n)(...a))},null,2))}},yr={__name:"ContextMenu",props:{show:{type:Boolean,default:!1},currentPath:{type:String,default:""},x:{type:Number,default:0},y:{type:Number,default:0}},emits:["update:show"],setup(e,{emit:t}){const n=e,r=t,o=qt(),a=j(()=>[{label:"重新加载",key:"reload",disabled:n.currentPath!==o.activeTab,icon:()=>l("i",{class:"i-mdi:refresh text-14"})},{label:"关闭",key:"close",disabled:o.tabs.length<=1,icon:()=>l("i",{class:"i-mdi:close text-14"})},{label:"关闭其他",key:"close-other",disabled:o.tabs.length<=1,icon:()=>l("i",{class:"i-mdi:arrow-expand-horizontal text-14"})},{label:"关闭左侧",key:"close-left",disabled:o.tabs.length<=1||n.currentPath===o.tabs[0].path,icon:()=>l("i",{class:"i-mdi:arrow-expand-left text-14"})},{label:"关闭右侧",key:"close-right",disabled:o.tabs.length<=1||n.currentPath===o.tabs[o.tabs.length-1].path,icon:()=>l("i",{class:"i-mdi:arrow-expand-right text-14"})}]),c=Qn(),p=new Map([["reload",()=>{o.reloadTab(c.fullPath,c.meta?.keepAlive)}],["close",()=>{o.removeTab(n.currentPath)}],["close-other",()=>{o.removeOther(n.currentPath)}],["close-left",()=>{o.removeLeft(n.currentPath)}],["close-right",()=>{o.removeRight(n.currentPath)}]]);function g(){r("update:show",!1)}function h(v){const w=p.get(v);typeof w=="function"&&w(),g()}return(v,w)=>{const C=ao;return J(),Je(C,{show:e.show,options:V(a),x:e.x,y:e.y,placement:"bottom-start",onClickoutside:g,onSelect:h},null,8,["show","options","x","y"])}}},wr={id:"top-tab"},kr={__name:"index",setup(e){const t=eo(),n=qt(),r=to({show:!1,x:0,y:0,currentPath:""});function o(h){n.setActiveTab(h),t.push(h)}function a(){r.show=!0}function c(){r.show=!1}function p(h,v,w){Object.assign(r,{x:h,y:v,currentPath:w})}async function g(h,v){const{clientX:w,clientY:C}=h;c(),p(w,C,v.path),await De(),a()}return(h,v)=>{const w=Qe,C=sr;return J(),ge("div",wr,[ie(C,{value:V(n).activeTab,closable:V(n).tabs.length>1,type:"card",onClose:v[0]||(v[0]=y=>V(n).removeTab(y))},{default:q(()=>[(J(!0),ge(Ct,null,no(V(n).tabs,y=>(J(),Je(w,{key:y.path,name:y.path,onClick:z=>o(y.path),onContextmenu:oo(z=>g(z,y),["prevent"])},{default:q(()=>[ke(yt(y.title),1)]),_:2},1032,["name","onClick","onContextmenu"]))),128))]),_:1},8,["value","closable"]),V(r).show?(J(),Je(yr,{key:0,show:V(r).show,"onUpdate:show":v[1]||(v[1]=y=>V(r).show=y),"current-path":V(r).currentPath,x:V(r).x,y:V(r).y},null,8,["show","current-path","x","y"])):Ee("",!0)])}}},Ur=io(kr,[["__scopeId","data-v-0d851a24"]]),Sr={class:"f-c-c"},Mr={__name:"ThemeSetting",setup(e){const t=Ft(),n=Object.entries(ro.getPresetColors()).map(([,r])=>r.primary);return(r,o)=>{const a=nr,c=Nt;return J(),ge("div",Sr,[ie(c,{trigger:"hover",placement:"bottom"},{trigger:q(()=>[ie(a,{id:"theme-setting",class:"h-32 w-32",value:V(t).primaryColor,swatches:V(n),"on-update:value":p=>V(t).setPrimaryColor(p),"render-label":()=>""},null,8,["value","swatches","on-update:value"])]),default:q(()=>[o[0]||(o[0]=ke(" 设置主题色 ",-1))]),_:1})])}}};export{Ur as A,Tr as _,Ir as a,Br as b,Mr as c};
