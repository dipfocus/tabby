(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[8098],{7404:function(e,t,n){"use strict";n.d(t,{j:function(){return i}});let r=e=>"boolean"==typeof e?"".concat(e):0===e?"0":e,s=function(){for(var e=arguments.length,t=Array(e),n=0;n<e;n++)t[n]=arguments[n];return t.flat(1/0).filter(Boolean).join(" ")},i=(e,t)=>n=>{var i;if((null==t?void 0:t.variants)==null)return s(e,null==n?void 0:n.class,null==n?void 0:n.className);let{variants:a,defaultVariants:l}=t,o=Object.keys(a).map(e=>{let t=null==n?void 0:n[e],s=null==l?void 0:l[e];if(null===t)return null;let i=r(t)||r(s);return a[e][i]}),c=n&&Object.entries(n).reduce((e,t)=>{let[n,r]=t;return void 0===r||(e[n]=r),e},{}),u=null==t?void 0:null===(i=t.compoundVariants)||void 0===i?void 0:i.reduce((e,t)=>{let{class:n,className:r,...s}=t;return Object.entries(s).every(e=>{let[t,n]=e;return Array.isArray(n)?n.includes({...l,...c}[t]):({...l,...c})[t]===n})?[...e,n,r]:e},[]);return s(e,o,u,null==n?void 0:n.class,null==n?void 0:n.className)}},76846:function(e,t,n){Promise.resolve().then(n.bind(n,61004))},61004:function(e,t,n){"use strict";n.r(t),n.d(t,{default:function(){return b}});var r=n(57437),s=n(2265),i=n(24033),a=n(1589),l=n(58001),o=n(1592),c=n(84168),u=n(38110),d=n(61865),f=n(74578),m=n(61985),x=n(58835),h=n(7820),v=n(39311),p=n(93023),j=n(41315),g=n(51908);let N=(0,x.BX)("\n  mutation tokenAuth($email: String!, $password: String!) {\n    tokenAuth(email: $email, password: $password) {\n      accessToken\n      refreshToken\n    }\n  }\n"),w=f.Ry({email:f.Z_().email("Invalid email address"),password:f.Z_()});function y(e){let{className:t,invitationCode:n,...a}=e,o=(0,d.cI)({resolver:(0,u.F)(w)}),f=(0,i.useRouter)(),{status:x}=(0,l.kP)();s.useEffect(()=>{"authenticated"===x&&f.replace("/")},[x]);let y=(0,l.zq)(),{isSubmitting:b}=o.formState,k=(0,h.Db)(N,{onCompleted(e){y(e.tokenAuth)},form:o});return(0,r.jsx)("div",{className:(0,v.cn)("grid gap-6",t),...a,children:(0,r.jsxs)(j.l0,{...o,children:[(0,r.jsxs)("form",{className:"grid gap-2",onSubmit:o.handleSubmit(k),children:[(0,r.jsx)(j.Wi,{control:o.control,name:"email",render:e=>{let{field:t}=e;return(0,r.jsxs)(j.xJ,{children:[(0,r.jsx)(j.lX,{children:"Email"}),(0,r.jsx)(j.NI,{children:(0,r.jsx)(g.I,{placeholder:m.o,type:"email",autoCapitalize:"none",autoComplete:"email",autoCorrect:"off",...t})}),(0,r.jsx)(j.zG,{})]})}}),(0,r.jsx)(j.Wi,{control:o.control,name:"password",render:e=>{let{field:t}=e;return(0,r.jsxs)(j.xJ,{children:[(0,r.jsx)(j.lX,{children:"Password"}),(0,r.jsx)(j.NI,{children:(0,r.jsx)(g.I,{type:"password",...t})}),(0,r.jsx)(j.zG,{})]})}}),(0,r.jsxs)(p.z,{type:"submit",className:"mt-1",disabled:b,children:[b&&(0,r.jsx)(c.IconSpinner,{className:"mr-2 h-4 w-4 animate-spin"}),"Login"]})]}),(0,r.jsx)(j.zG,{className:"text-center"})]})})}function b(){let e=(0,i.useRouter)(),t=(0,i.useSearchParams)(),n=t.get("error_message"),u=t.get("access_token"),d=t.get("refresh_token"),[f,m]=(0,s.useState)(13),x=!!u&&!!d,h=x&&!n,v=(0,l.zq)(),{data:p}=(0,a.Z)(x?null:"/oauth/providers",o.Z);return((0,s.useEffect)(()=>{!n&&u&&d&&v({accessToken:u,refreshToken:d}).then(()=>m(100))},[t]),(0,s.useEffect)(()=>{100===f&&setTimeout(()=>{e.replace("/")},200)},[f]),h)?(0,r.jsx)(c.IconSpinner,{className:"h-8 w-8 animate-spin"}):(0,r.jsxs)(r.Fragment,{children:[(0,r.jsxs)("div",{className:"w-[350px] space-y-6",children:[(0,r.jsxs)("div",{className:"flex flex-col space-y-2 text-center",children:[(0,r.jsx)("h1",{className:"text-2xl font-semibold tracking-tight",children:"Sign In"}),(0,r.jsx)("p",{className:"text-sm text-muted-foreground",children:"Enter credentials to login to your account"})]}),(0,r.jsx)(y,{})]}),!!(null==p?void 0:p.length)&&(0,r.jsxs)("div",{className:"relative mt-10 flex w-[350px] items-center py-5",children:[(0,r.jsx)("div",{className:"grow border-t "}),(0,r.jsx)("span",{className:"mx-4 shrink text-sm text-muted-foreground",children:"Or Signin with"}),(0,r.jsx)("div",{className:"grow border-t "})]}),(0,r.jsxs)("div",{className:"mx-auto flex items-center gap-6",children:[(null==p?void 0:p.includes("github"))&&(0,r.jsx)("a",{href:"/oauth/signin?provider=github",children:(0,r.jsx)(c.IconGithub,{className:"h-8 w-8"})}),(null==p?void 0:p.includes("google"))&&(0,r.jsx)("a",{href:"/oauth/signin?provider=google",children:(0,r.jsx)(c.IconGoogle,{className:"h-8 w-8"})})]}),!!n&&(0,r.jsx)("div",{className:"mt-4 text-destructive",children:n})]})}},41315:function(e,t,n){"use strict";n.d(t,{NI:function(){return v},Wi:function(){return d},l0:function(){return c},lX:function(){return h},pf:function(){return p},xJ:function(){return x},zG:function(){return j}});var r=n(57437),s=n(2265),i=n(67256),a=n(61865),l=n(39311),o=n(66672);let c=a.RV,u=s.createContext({}),d=e=>{let{...t}=e;return(0,r.jsx)(u.Provider,{value:{name:t.name},children:(0,r.jsx)(a.Qr,{...t})})},f=()=>{let e=s.useContext(u),t=s.useContext(m),{getFieldState:n,formState:r}=(0,a.Gc)(),i=e.name||"root",l=n(i,r);if(!r)throw Error("useFormField should be used within <Form>");let{id:o}=t;return{id:o,name:i,formItemId:"".concat(o,"-form-item"),formDescriptionId:"".concat(o,"-form-item-description"),formMessageId:"".concat(o,"-form-item-message"),...l}},m=s.createContext({}),x=s.forwardRef((e,t)=>{let{className:n,...i}=e,a=s.useId();return(0,r.jsx)(m.Provider,{value:{id:a},children:(0,r.jsx)("div",{ref:t,className:(0,l.cn)("space-y-2",n),...i})})});x.displayName="FormItem";let h=s.forwardRef((e,t)=>{let{className:n,required:s,...i}=e,{error:a,formItemId:c}=f();return(0,r.jsx)(o._,{ref:t,className:(0,l.cn)(a&&"text-destructive",s&&'after:ml-0.5 after:text-destructive after:content-["*"]',n),htmlFor:c,...i})});h.displayName="FormLabel";let v=s.forwardRef((e,t)=>{let{...n}=e,{error:s,formItemId:a,formDescriptionId:l,formMessageId:o}=f();return(0,r.jsx)(i.g7,{ref:t,id:a,"aria-describedby":s?"".concat(l," ").concat(o):"".concat(l),"aria-invalid":!!s,...n})});v.displayName="FormControl";let p=s.forwardRef((e,t)=>{let{className:n,...s}=e,{formDescriptionId:i}=f();return(0,r.jsx)("p",{ref:t,id:i,className:(0,l.cn)("text-sm text-muted-foreground",n),...s})});p.displayName="FormDescription";let j=s.forwardRef((e,t)=>{let{className:n,children:s,...i}=e,{error:a,formMessageId:o}=f(),c=a?String(null==a?void 0:a.message):s;return c?(0,r.jsx)("p",{ref:t,id:o,className:(0,l.cn)("text-sm font-medium text-destructive",n),...i,children:c}):null});j.displayName="FormMessage"},66672:function(e,t,n){"use strict";n.d(t,{_:function(){return c}});var r=n(57437),s=n(2265),i=n(36743),a=n(7404),l=n(39311);let o=(0,a.j)("text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"),c=s.forwardRef((e,t)=>{let{className:n,...s}=e;return(0,r.jsx)(i.f,{ref:t,className:(0,l.cn)(o(),n),...s})});c.displayName=i.f.displayName},61985:function(e,t,n){"use strict";n.d(t,{L:function(){return s},o:function(){return r}});let r="name@yourcompany.com",s=20},1592:function(e,t,n){"use strict";n.d(t,{Z:function(){return o}});var r=n(34084),s=n(58001),i=n(7820);let a=!1,l=[];async function o(e,t){let n=await fetch(e,c(t));if(401!==n.status)return(null==t?void 0:t.format)==="text"?n.text():n.json();{var r,i;if(a)return new Promise(n=>{l.push({url:e,init:t,resolve:n})});let n=null===(r=(0,s.bW)())||void 0===r?void 0:r.refreshToken;if(!n){(0,s.Rn)();return}a=!0;let o=await u(n),c=null==o?void 0:null===(i=o.data)||void 0===i?void 0:i.refreshToken;if(c){for((0,s.pC)({accessToken:c.accessToken,refreshToken:c.refreshToken}),a=!1;l.length;){let e=l.shift();null==e||e.resolve(d(e.url,e.init))}return d(e,t)}a=!1,l.length=0,(0,s.Rn)()}}function c(e){var t;let n=new Headers(null==e?void 0:e.headers);return n.append("authorization","Bearer ".concat(null===(t=(0,s.bW)())||void 0===t?void 0:t.accessToken)),{...e||{},headers:n}}async function u(e){let t=i.Lp.createRequestOperation("mutation",(0,r.h)(s.Dp,{refreshToken:e}));return i.Lp.executeMutation(t)}function d(e,t){return fetch(e,c(t)).then(e=>(null==t?void 0:t.format)==="text"?e.text():e.json())}},9381:function(e,t,n){"use strict";n.d(t,{WV:function(){return l},jH:function(){return o}});var r=n(13428),s=n(2265),i=n(54887),a=n(67256);let l=["a","button","div","form","h2","h3","img","input","label","li","nav","ol","p","span","svg","ul"].reduce((e,t)=>{let n=(0,s.forwardRef)((e,n)=>{let{asChild:i,...l}=e,o=i?a.g7:t;return(0,s.useEffect)(()=>{window[Symbol.for("radix-ui")]=!0},[]),(0,s.createElement)(o,(0,r.Z)({},l,{ref:n}))});return n.displayName=`Primitive.${t}`,{...e,[t]:n}},{});function o(e,t){e&&(0,i.flushSync)(()=>e.dispatchEvent(t))}},1589:function(e,t,n){"use strict";n.d(t,{Z:function(){return i}});var r=n(30713),s=n(44796);let i=(0,s.xD)(r.ZP,e=>(t,n,r)=>(r.revalidateOnFocus=!1,r.revalidateIfStale=!1,r.revalidateOnReconnect=!1,e(t,n,r)))}},function(e){e.O(0,[1091,584,9233,713,4402,4168,633,2971,7864,1744],function(){return e(e.s=76846)}),_N_E=e.O()}]);