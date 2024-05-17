"use strict";(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[7957],{59039:function(e,n,t){t.d(n,{Z:function(){return v}});var r,a,s=t(57437),o=t(2265),i=t(62067),l=t.n(i),d=t(39311),u=t(93023),c=t(81291),f=t(17158),m=t(64022);function p(e){let{className:n,classNames:t,showOutsideDays:r=!0,...a}=e;return(0,s.jsx)(m._W,{showOutsideDays:r,className:(0,d.cn)("p-3",n),classNames:{months:"flex flex-col sm:flex-row space-y-4 sm:space-x-4 sm:space-y-0",month:"space-y-4",caption:"flex justify-center pt-1 relative items-center",caption_label:"text-sm font-medium",nav:"space-x-1 flex items-center",nav_button:(0,d.cn)((0,u.d)({variant:"outline"}),"h-7 w-7 bg-transparent p-0 opacity-50 hover:opacity-100"),nav_button_previous:"absolute left-1",nav_button_next:"absolute right-1",table:"w-full border-collapse space-y-1",head_row:"flex",head_cell:"text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]",row:"flex w-full mt-2",cell:"h-9 w-9 text-center text-sm p-0 relative [&:has([aria-selected].day-range-end)]:rounded-r-md [&:has([aria-selected].day-outside)]:bg-accent/50 [&:has([aria-selected])]:bg-accent first:[&:has([aria-selected])]:rounded-l-md last:[&:has([aria-selected])]:rounded-r-md focus-within:relative focus-within:z-20",day:(0,d.cn)((0,u.d)({variant:"ghost"}),"h-9 w-9 p-0 font-normal aria-selected:opacity-100"),day_range_end:"day-range-end",day_selected:"bg-primary text-primary-foreground hover:bg-primary hover:text-primary-foreground focus:bg-primary focus:text-primary-foreground",day_today:"bg-accent text-accent-foreground",day_outside:"day-outside text-muted-foreground opacity-50 aria-selected:bg-accent/50 aria-selected:text-muted-foreground aria-selected:opacity-30",day_disabled:"text-muted-foreground opacity-50",day_range_middle:"aria-selected:bg-accent aria-selected:text-accent-foreground",day_hidden:"invisible",...t},components:{IconLeft:e=>{let{...n}=e;return(0,s.jsx)(c.Z,{className:"h-4 w-4"})},IconRight:e=>{let{...n}=e;return(0,s.jsx)(f.Z,{className:"h-4 w-4"})}},...a})}p.displayName="Calendar";var g=t(86110),h=t(84168),x=t(58094),y=t(16775);(r=a||(a={})).TODAY="today",r.YESTERDAY="yesterday",r.CUSTOM_DATE="custom_date",r.CUSTOM_RANGE="custom_range";let b=e=>({number:parseInt(e,10)||-1,unit:e[e.length-1]||"h"});function v(e){let{options:n,onSelect:t,defaultValue:r,hasToday:i,hasYesterday:c,className:f}=e;r=r||n[0].value;let m=b(r),[v,$]=o.useState({from:l()().add(m.number,m.unit).toDate(),to:l()().toDate()}),[j,N]=o.useState(!1),[w,C]=o.useState(r),[I,S]=o.useState(!1),[D,_]=o.useState({from:l()().add(m.number,m.unit).toDate(),to:l()().toDate()}),[P,R]=o.useState(!1),[O,T]=o.useState(l()().toDate()),A=e=>{t&&t(e),$(e)};return(0,s.jsxs)("div",{className:"relative",children:[(0,s.jsxs)(x.Ph,{value:w,onValueChange:e=>{switch(e){case a.TODAY:A({from:l()().startOf("day").toDate(),to:l()().toDate()});break;case a.YESTERDAY:A({from:l()().subtract(1,"d").startOf("day").toDate(),to:l()().subtract(1,"d").endOf("day").toDate()});break;default:{let{unit:n,number:t}=b(e),r=l()().add(t,n);["h","s","ms"].includes(n)||(r=r.startOf("day")),A({from:r.toDate(),to:l()().toDate()})}}C(e)},open:j,onOpenChange:e=>{e||I||P||N(!1)},children:[(0,s.jsx)(x.i4,{className:(0,d.cn)("w-[240px]",f),onClick:()=>N(!j),children:(0,s.jsx)(x.ki,{placeholder:"Date range"})}),(0,s.jsxs)(x.Bw,{align:"end",children:[i&&(0,s.jsx)(x.Ql,{value:a.TODAY,children:"Today"}),c&&(0,s.jsx)(x.Ql,{value:a.YESTERDAY,children:"Yesterday"}),n.map(e=>(0,s.jsx)(x.Ql,{value:e.value,children:e.label},e.value)),(0,s.jsxs)(x.Ql,{value:a.CUSTOM_DATE,className:"hidden",children:[l()(null==v?void 0:v.from).format("ll")," - Now"]}),(0,s.jsxs)(x.Ql,{value:a.CUSTOM_RANGE,className:"hidden",children:[l()(null==v?void 0:v.from).format("ll"),(null==v?void 0:v.to)?" - ".concat(l()(v.to).format("ll")):""]}),(0,s.jsx)(x.U$,{}),(0,s.jsxs)("div",{className:"relative cursor-default py-1.5 pl-8 text-sm hover:bg-accent hover:text-accent-foreground",onClick:()=>R(!0),children:[w===a.CUSTOM_DATE&&(0,s.jsx)("div",{className:"absolute inset-y-0 left-2 flex items-center",children:(0,s.jsx)(h.IconCheck,{})}),"Custom date until now"]}),(0,s.jsxs)("div",{className:"relative cursor-default py-1.5 pl-8 text-sm hover:bg-accent hover:text-accent-foreground",onClick:()=>S(!0),children:[w===a.CUSTOM_RANGE&&(0,s.jsx)("div",{className:"absolute inset-y-0 left-2 flex items-center",children:(0,s.jsx)(h.IconCheck,{})}),"Custom date range"]})]})]}),(0,s.jsx)(g.Zb,{className:(0,d.cn)("absolute right-0 mt-1",{"opacity-0 z-0 pointer-events-none":!P,"opacity-100 pointer-events-auto":P}),style:P&&{zIndex:99}||{},children:(0,s.jsxs)(g.aY,{className:"w-auto pb-0",children:[(0,s.jsx)(p,{initialFocus:!0,mode:"single",selected:O,onSelect:T,disabled:e=>e>new Date}),(0,s.jsx)(y.Z,{}),(0,s.jsxs)("div",{className:"flex items-center justify-end gap-x-3 py-4",children:[(0,s.jsx)(u.z,{variant:"ghost",onClick:()=>R(!1),children:"Cancel"}),(0,s.jsx)(u.z,{onClick:()=>{N(!1),R(!1),C(a.CUSTOM_DATE),A({from:l()(O).startOf("day").toDate(),to:l()().toDate()})},children:"Apply"})]})]})}),(0,s.jsx)(g.Zb,{className:(0,d.cn)("absolute right-0 mt-1",{"opacity-0 z-0 pointer-events-none":!I,"opacity-100 pointer-events-auto":I}),style:I&&{zIndex:99}||{},children:(0,s.jsxs)(g.aY,{className:"w-auto pb-0",children:[(0,s.jsx)(p,{initialFocus:!0,mode:"range",defaultMonth:l()(null==D?void 0:D.from).subtract(1,"month").toDate(),selected:D,onSelect:_,numberOfMonths:2,disabled:e=>e>new Date}),(0,s.jsx)(y.Z,{}),(0,s.jsxs)("div",{className:"flex items-center justify-end gap-x-3 py-4",children:[(0,s.jsx)(u.z,{variant:"ghost",onClick:()=>S(!1),children:"Cancel"}),(0,s.jsx)(u.z,{onClick:()=>{N(!1),S(!1),C(a.CUSTOM_RANGE),A({from:null==D?void 0:D.from,to:l()(null==D?void 0:D.to).endOf("day").toDate()})},children:"Apply"})]})]})})]})}},70825:function(e,n,t){var r=t(57437),a=t(2265),s=t(12897),o=t(93111);n.Z=e=>{let{loading:n,fallback:t,delay:i,children:l}=e,[d,u]=a.useState(!n),[c]=(0,s.n)(d,null!=i?i:200);return(a.useEffect(()=>{n||d||u(!0)},[n]),c)?l:t||(0,r.jsx)(o.cg,{})}},86110:function(e,n,t){t.d(n,{Ol:function(){return i},Zb:function(){return o},aY:function(){return u},eW:function(){return c},ll:function(){return l}});var r=t(57437),a=t(2265),s=t(39311);let o=a.forwardRef((e,n)=>{let{className:t,...a}=e;return(0,r.jsx)("div",{ref:n,className:(0,s.cn)("rounded-lg border bg-card text-card-foreground shadow-sm",t),...a})});o.displayName="Card";let i=a.forwardRef((e,n)=>{let{className:t,...a}=e;return(0,r.jsx)("div",{ref:n,className:(0,s.cn)("flex flex-col space-y-1.5 p-6",t),...a})});i.displayName="CardHeader";let l=a.forwardRef((e,n)=>{let{className:t,...a}=e;return(0,r.jsx)("h3",{ref:n,className:(0,s.cn)("text-2xl font-semibold leading-none tracking-tight",t),...a})});l.displayName="CardTitle";let d=a.forwardRef((e,n)=>{let{className:t,...a}=e;return(0,r.jsx)("p",{ref:n,className:(0,s.cn)("text-sm text-muted-foreground",t),...a})});d.displayName="CardDescription";let u=a.forwardRef((e,n)=>{let{className:t,...a}=e;return(0,r.jsx)("div",{ref:n,className:(0,s.cn)("p-6 pt-0",t),...a})});u.displayName="CardContent";let c=a.forwardRef((e,n)=>{let{className:t,...a}=e;return(0,r.jsx)("div",{ref:n,className:(0,s.cn)("flex items-center p-6 pt-0",t),...a})});c.displayName="CardFooter"},58094:function(e,n,t){t.d(n,{Bw:function(){return f},DI:function(){return d},Ph:function(){return l},Ql:function(){return p},U$:function(){return g},i4:function(){return c},ki:function(){return u}});var r=t(57437),a=t(2265),s=t(6155),o=t(39311),i=t(84168);let l=s.fC,d=s.ZA,u=s.B4,c=a.forwardRef((e,n)=>{let{className:t,children:a,...l}=e;return(0,r.jsxs)(s.xz,{ref:n,className:(0,o.cn)("flex h-9 w-full items-center justify-between rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",t),...l,children:[a,(0,r.jsx)(s.JO,{asChild:!0,children:(0,r.jsx)(i.IconChevronUpDown,{className:"opacity-50"})})]})});c.displayName=s.xz.displayName;let f=a.forwardRef((e,n)=>{let{className:t,children:a,position:i="popper",...l}=e;return(0,r.jsx)(s.h_,{children:(0,r.jsx)(s.VY,{ref:n,className:(0,o.cn)("relative z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md animate-in fade-in-80","popper"===i&&"translate-y-1",t),position:i,...l,children:(0,r.jsx)(s.l_,{className:(0,o.cn)("p-1","popper"===i&&"h-[var(--radix-select-trigger-height)] w-full min-w-[var(--radix-select-trigger-width)]"),children:a})})})});f.displayName=s.VY.displayName;let m=a.forwardRef((e,n)=>{let{className:t,...a}=e;return(0,r.jsx)(s.__,{ref:n,className:(0,o.cn)("py-1.5 pl-8 pr-2 text-sm font-semibold",t),...a})});m.displayName=s.__.displayName;let p=a.forwardRef((e,n)=>{let{className:t,children:a,isPlaceHolder:l,...d}=e;return(0,r.jsxs)(s.ck,{ref:n,className:(0,o.cn)("relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",t),...d,children:[!l&&(0,r.jsx)("span",{className:"absolute left-2 flex h-3.5 w-3.5 items-center justify-center",children:(0,r.jsx)(s.wU,{children:(0,r.jsx)(i.IconCheck,{className:"h-4 w-4"})})}),(0,r.jsx)(s.eT,{children:a})]})});p.displayName=s.ck.displayName;let g=a.forwardRef((e,n)=>{let{className:t,...a}=e;return(0,r.jsx)(s.Z0,{ref:n,className:(0,o.cn)("-mx-1 my-1 h-px bg-muted",t),...a})});g.displayName=s.Z0.displayName},61985:function(e,n,t){t.d(n,{$6:function(){return s},L8:function(){return a},o0:function(){return r}});let r="name@yourcompany.com",a=20,s={DEMO_AUTO_LOGIN:"_tabby_demo_autologin"}},48723:function(e,n,t){t.d(n,{d:function(){return i}});var r=t(2265),a=t(99109),s=t(61985),o=t(57166);function i(){let[e,n]=(0,r.useState)({first:s.L8}),[t,i]=(0,r.useState)([]),[l,d]=(0,r.useState)(!1),[{data:u,fetching:c}]=(0,a.aM)({query:o.yw,variables:e});return(0,r.useEffect)(()=>{if(!l&&!c&&u){let e=null==u?void 0:u.users.edges.map(e=>({id:e.node.id,email:e.node.email})),r=(null==u?void 0:u.users.pageInfo.endCursor)||"",a=null==u?void 0:u.users.pageInfo.hasNextPage,o=[...t];i(o.concat(e)),a?n({first:s.L8,after:r}):d(!0)}},[e,c]),[t]}},57166:function(e,n,t){t.d(n,{Cl:function(){return c},DQ:function(){return l},GG:function(){return o},S1:function(){return s},Vt:function(){return m},Y6:function(){return u},Zr:function(){return g},cO:function(){return p},hC:function(){return f},lE:function(){return a},yB:function(){return i},yw:function(){return d}});var r=t(60106);let a=(0,r.BX)("\n  query ListInvitations(\n    $after: String\n    $before: String\n    $first: Int\n    $last: Int\n  ) {\n    invitations(after: $after, before: $before, first: $first, last: $last) {\n      edges {\n        node {\n          id\n          email\n          code\n          createdAt\n        }\n        cursor\n      }\n      pageInfo {\n        hasNextPage\n        hasPreviousPage\n        startCursor\n        endCursor\n      }\n    }\n  }\n"),s=(0,r.BX)("\n  query gitRepositories(\n    $after: String\n    $before: String\n    $first: Int\n    $last: Int\n  ) {\n    gitRepositories(\n      after: $after\n      before: $before\n      first: $first\n      last: $last\n    ) {\n      edges {\n        node {\n          id\n          name\n          gitUrl\n        }\n        cursor\n      }\n      pageInfo {\n        hasNextPage\n        hasPreviousPage\n        startCursor\n        endCursor\n      }\n    }\n  }\n"),o=(0,r.BX)("\n  query ListJobRuns(\n    $ids: [ID!]\n    $jobs: [String!]\n    $after: String\n    $before: String\n    $first: Int\n    $last: Int\n  ) {\n    jobRuns(\n      ids: $ids\n      jobs: $jobs\n      after: $after\n      before: $before\n      first: $first\n      last: $last\n    ) {\n      edges {\n        node {\n          id\n          job\n          createdAt\n          finishedAt\n          exitCode\n          stdout\n          stderr\n        }\n        cursor\n      }\n      pageInfo {\n        hasNextPage\n        hasPreviousPage\n        startCursor\n        endCursor\n      }\n    }\n  }\n"),i=(0,r.BX)("\n  query GetJobRunStats($jobs: [String!]) {\n    jobRunStats(jobs: $jobs) {\n      success\n      failed\n      pending\n    }\n  }\n"),l=(0,r.BX)("\n  query ListJobs {\n    jobs\n  }\n"),d=(0,r.BX)("\n  query ListUsers($after: String, $before: String, $first: Int, $last: Int) {\n    users(after: $after, before: $before, first: $first, last: $last) {\n      edges {\n        node {\n          id\n          email\n          isAdmin\n          isOwner\n          createdAt\n          active\n        }\n        cursor\n      }\n      pageInfo {\n        hasNextPage\n        hasPreviousPage\n        startCursor\n        endCursor\n      }\n    }\n  }\n"),u=(0,r.BX)("\n  query DailyStatsInPastYear($users: [ID!]) {\n    dailyStatsInPastYear(users: $users) {\n      start\n      end\n      completions\n      selects\n      views\n    }\n  }\n"),c=(0,r.BX)("\n  query DailyStats(\n    $start: DateTime!\n    $end: DateTime!\n    $users: [ID!]\n    $languages: [Language!]\n  ) {\n    dailyStats(start: $start, end: $end, users: $users, languages: $languages) {\n      start\n      end\n      completions\n      selects\n      views\n      language\n    }\n  }\n"),f=(0,r.BX)("\n  query ListGithubRepositoryProviders(\n    $ids: [ID!]\n    $after: String\n    $before: String\n    $first: Int\n    $last: Int\n  ) {\n    githubRepositoryProviders(\n      ids: $ids\n      after: $after\n      before: $before\n      first: $first\n      last: $last\n    ) {\n      edges {\n        node {\n          id\n          displayName\n          status\n        }\n        cursor\n      }\n      pageInfo {\n        hasNextPage\n        hasPreviousPage\n        startCursor\n        endCursor\n      }\n    }\n  }\n"),m=(0,r.BX)("\n  query ListGithubRepositories(\n    $providerIds: [ID!]!\n    $active: Boolean\n    $after: String\n    $before: String\n    $first: Int\n    $last: Int\n  ) {\n    githubRepositories(\n      providerIds: $providerIds\n      active: $active\n      after: $after\n      before: $before\n      first: $first\n      last: $last\n    ) {\n      edges {\n        node {\n          id\n          vendorId\n          githubRepositoryProviderId\n          name\n          gitUrl\n          active\n        }\n        cursor\n      }\n      pageInfo {\n        hasNextPage\n        hasPreviousPage\n        startCursor\n        endCursor\n      }\n    }\n  }\n"),p=(0,r.BX)("\n  query ListGitlabRepositoryProviders(\n    $ids: [ID!]\n    $after: String\n    $before: String\n    $first: Int\n    $last: Int\n  ) {\n    gitlabRepositoryProviders(\n      ids: $ids\n      after: $after\n      before: $before\n      first: $first\n      last: $last\n    ) {\n      edges {\n        node {\n          id\n          displayName\n          status\n        }\n        cursor\n      }\n      pageInfo {\n        hasNextPage\n        hasPreviousPage\n        startCursor\n        endCursor\n      }\n    }\n  }\n"),g=(0,r.BX)("\n  query ListGitlabRepositories(\n    $providerIds: [ID!]!\n    $active: Boolean\n    $after: String\n    $before: String\n    $first: Int\n    $last: Int\n  ) {\n    gitlabRepositories(\n      providerIds: $providerIds\n      active: $active\n      after: $after\n      before: $before\n      first: $first\n      last: $last\n    ) {\n      edges {\n        node {\n          id\n          vendorId\n          gitlabRepositoryProviderId\n          name\n          gitUrl\n          active\n        }\n        cursor\n      }\n      pageInfo {\n        hasNextPage\n        hasPreviousPage\n        startCursor\n        endCursor\n      }\n    }\n  }\n")}}]);