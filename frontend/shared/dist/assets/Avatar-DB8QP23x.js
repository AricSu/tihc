import{i as $,o as M}from"./utils-C_7EMGUW.js";import{t as W}from"./Tag-Dxmn7TKr.js";import{m as K,i as T,cg as N,ch as V,j as P,k,p as A,D,q as p,aB as G,aC as q,s as U,g as y,C as _,y as O,t as B,cC as X,ao as Y,z as J,cD as Q,h as Z,a7 as ee,bi as re,I as oe}from"./index-CQZOGmuN.js";const te=K("n-avatar-group"),ne=T("avatar",`
 width: var(--n-merged-size);
 height: var(--n-merged-size);
 color: #FFF;
 font-size: var(--n-font-size);
 display: inline-flex;
 position: relative;
 overflow: hidden;
 text-align: center;
 border: var(--n-border);
 border-radius: var(--n-border-radius);
 --n-merged-color: var(--n-color);
 background-color: var(--n-merged-color);
 transition:
 border-color .3s var(--n-bezier),
 background-color .3s var(--n-bezier),
 color .3s var(--n-bezier);
`,[N(P("&","--n-merged-color: var(--n-color-modal);")),V(P("&","--n-merged-color: var(--n-color-popover);")),P("img",`
 width: 100%;
 height: 100%;
 `),k("text",`
 white-space: nowrap;
 display: inline-block;
 position: absolute;
 left: 50%;
 top: 50%;
 `),T("icon",`
 vertical-align: bottom;
 font-size: calc(var(--n-merged-size) - 6px);
 `),k("text","line-height: 1.25")]),ae=Object.assign(Object.assign({},B.props),{size:[String,Number],src:String,circle:{type:Boolean,default:void 0},objectFit:String,round:{type:Boolean,default:void 0},bordered:{type:Boolean,default:void 0},onError:Function,fallbackSrc:String,intersectionObserverOptions:Object,lazy:Boolean,onLoad:Function,renderPlaceholder:Function,renderFallback:Function,imgProps:Object,color:String}),de=A({name:"Avatar",props:ae,slots:Object,setup(o){const{mergedClsPrefixRef:l,inlineThemeDisabled:m}=U(o),g=y(!1);let d=null;const c=y(null),s=y(null),x=()=>{const{value:e}=c;if(e&&(d===null||d!==e.innerHTML)){d=e.innerHTML;const{value:r}=s;if(r){const{offsetWidth:t,offsetHeight:a}=r,{offsetWidth:n,offsetHeight:L}=e,R=.9,S=Math.min(t/n*R,a/L*R,1);e.style.transform=`translateX(-50%) translateY(-50%) scale(${S})`}}},b=_(te,null),i=O(()=>{const{size:e}=o;if(e)return e;const{size:r}=b||{};return r||"medium"}),u=B("Avatar","-avatar",ne,X,o,l),z=_(W,null),f=O(()=>{if(b)return!0;const{round:e,circle:r}=o;return e!==void 0||r!==void 0?e||r:z?z.roundRef.value:!1}),v=O(()=>b?!0:o.bordered||!1),C=O(()=>{const e=i.value,r=f.value,t=v.value,{color:a}=o,{self:{borderRadius:n,fontSize:L,color:R,border:S,colorModal:H,colorPopover:w},common:{cubicBezierEaseInOut:I}}=u.value;let E;return typeof e=="number"?E=`${e}px`:E=u.value.self[Y("height",e)],{"--n-font-size":L,"--n-border":t?S:"none","--n-border-radius":r?"50%":n,"--n-color":a||R,"--n-color-modal":a||H,"--n-color-popover":a||w,"--n-bezier":I,"--n-merged-size":`var(--n-avatar-size-override, ${E})`}}),h=m?J("avatar",O(()=>{const e=i.value,r=f.value,t=v.value,{color:a}=o;let n="";return e&&(typeof e=="number"?n+=`a${e}`:n+=e[0]),r&&(n+="b"),t&&(n+="c"),a&&(n+=Q(a)),n}),C,o):void 0,j=y(!o.lazy);Z(()=>{if(o.lazy&&o.intersectionObserverOptions){let e;const r=ee(()=>{e?.(),e=void 0,o.lazy&&(e=M(s.value,o.intersectionObserverOptions,j))});re(()=>{r(),e?.()})}}),oe(()=>{var e;return o.src||((e=o.imgProps)===null||e===void 0?void 0:e.src)},()=>{g.value=!1});const F=y(!o.lazy);return{textRef:c,selfRef:s,mergedRoundRef:f,mergedClsPrefix:l,fitTextTransform:x,cssVars:m?void 0:C,themeClass:h?.themeClass,onRender:h?.onRender,hasLoadError:g,shouldStartLoading:j,loaded:F,mergedOnError:e=>{if(!j.value)return;g.value=!0;const{onError:r,imgProps:{onError:t}={}}=o;r?.(e),t?.(e)},mergedOnLoad:e=>{const{onLoad:r,imgProps:{onLoad:t}={}}=o;r?.(e),t?.(e),F.value=!0}}},render(){var o,l;const{$slots:m,src:g,mergedClsPrefix:d,lazy:c,onRender:s,loaded:x,hasLoadError:b,imgProps:i={}}=this;s?.();let u;const z=!x&&!b&&(this.renderPlaceholder?this.renderPlaceholder():(l=(o=this.$slots).placeholder)===null||l===void 0?void 0:l.call(o));return this.hasLoadError?u=this.renderFallback?this.renderFallback():D(m.fallback,()=>[p("img",{src:this.fallbackSrc,style:{objectFit:this.objectFit}})]):u=G(m.default,f=>{if(f)return p(q,{onResize:this.fitTextTransform},{default:()=>p("span",{ref:"textRef",class:`${d}-avatar__text`},f)});if(g||i.src){const v=this.src||i.src;return p("img",Object.assign(Object.assign({},i),{loading:$&&!this.intersectionObserverOptions&&c?"lazy":"eager",src:c&&this.intersectionObserverOptions?this.shouldStartLoading?v:void 0:v,"data-image-src":v,onLoad:this.mergedOnLoad,onError:this.mergedOnError,style:[i.style||"",{objectFit:this.objectFit},z?{height:"0",width:"0",visibility:"hidden",position:"absolute"}:""]}))}}),p("span",{ref:"selfRef",class:[`${d}-avatar`,this.themeClass],style:this.cssVars},u,c&&z)}});export{de as _};
