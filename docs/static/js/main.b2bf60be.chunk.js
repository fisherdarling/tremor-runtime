(window.webpackJsonp=window.webpackJsonp||[]).push([[0],{207:function(e,n,t){e.exports=t(499)},212:function(e,n,t){},213:function(e,n,t){},498:function(e,n,t){},499:function(e,n,t){"use strict";t.r(n);var a=t(5),r=t.n(a),o=t(59),i=t.n(o),c=(t(212),t(89)),u=t(90),l=t(92),d=t(91),s=t(93),f=(t(213),t(38)),p=t(24),m=t(206),g=t(120),v=t(119),h={width:60,height:60};function E(e){var n=v.cloneDeep(e);return function(e){var n=function(e){return e.nodes.map(function(e){return{id:e.id,metadata:Object(m.a)({},h,{id:e.id})}})}(e),t=function(e){return e.links.map(function(e){return{from:e.source,to:e.target}}).filter(function(n){return e.nodes.find(function(e){return e.id===n.from})&&e.nodes.find(function(e){return e.id===n.to})})}(e),a=new g.graphlib.Graph;return a.setGraph({rankdir:"LR",align:"UL",edgesep:50,ranksep:50}),a.setDefaultEdgeLabel(function(){return{}}),n.forEach(function(e){a.setNode(e.id,e.metadata)}),t.forEach(function(e){e.from&&e.to&&a.setEdge(e.from,e.to)}),g.layout(a),a.nodes().map(function(e){return a.node(e)})}(n).forEach(function(e){var t=n.nodes.find(function(n){return n.id===e.id});t.x=e.x-e.width/2,t.y=e.y-e.height/2}),n}var b=t(205),w=t(203),D=t.n(w),k=t(121),y=t.n(k);function O(e){return new p.DefaultNodeModel(e,"rgb(0,192,255)")}t(420);var j=0;function M(e,n){var t=new p.DiagramModel,a=n.pipeline[0],r={};a.interface.inputs.forEach(function(e){var n=O(e);r[e]=n,t.addNode(n)}),a.nodes&&a.nodes.forEach(function(e){var n=O(e.id);r[e.id]=n,t.addNode(n)}),a.interface.outputs.forEach(function(e){var n=O(e);r[e]=n,t.addNode(n)});var o=1;return a.links&&Object.keys(a.links).forEach(function(e){var n=r[e];a.links[e].forEach(function(e){var a=r[e];t.addLink(function(e,n){j++;var t=e.addPort(new p.DefaultPortModel(!1,"".concat(e.name,"-out-").concat(j),"Out")),a=n.addPort(new p.DefaultPortModel(!0,"".concat(e.name,"-to-").concat(j),"In"));return t.link(a)}(n,a))}),n.x=50*o,o++}),C(e,t)}var L=function(e){function n(e){var t;return Object(c.a)(this,n),(t=Object(l.a)(this,Object(d.a)(n).call(this,e))).onDrop=function(e){var n=t.props.engine,a=Object(f.a)(t);e.forEach(function(e){var t=new FileReader;t.onload=function(){var e=t.result.toString(),r=D.a.parse(e),o=M(n,r);n.setDiagramModel(o),a.forceUpdate()},t.onabort=function(){return console.log("file reading was aborted")},t.onerror=function(){return console.log("file reading has failed")},t.readAsText(e)})},t.onLoadYaml=t.onLoadYaml.bind(Object(f.a)(t)),t.onLayout=t.onLayout.bind(Object(f.a)(t)),t.onClear=t.onClear.bind(Object(f.a)(t)),t}return Object(s.a)(n,e),Object(u.a)(n,[{key:"onLoadYaml",value:function(){var e=this.props.engine,n=M(e,{pipeline:[]});e.setDiagramModel(n),this.forceUpdate()}},{key:"onClear",value:function(){var e=this.props.engine,n=new p.DiagramModel;e.setDiagramModel(n),this.forceUpdate()}},{key:"onLayout",value:function(){var e=this.props.engine,n=e.getDiagramModel(),t=C(e,n);e.setDiagramModel(t),this.forceUpdate()}},{key:"render",value:function(){var e=this.props.engine;return a.createElement("div",null,a.createElement(y.a,{variant:"contained",onClick:this.onLayout},"Layout"),a.createElement(y.a,{variant:"contained",onClick:this.onClear},"Clear"),a.createElement(b.a,{accept:".yaml",onDrop:this.onDrop},function(e){var n=e.getRootProps,t=e.getInputProps,r=e.isDragActive;return a.createElement("div",n(),a.createElement("input",t()),r?a.createElement("p",null,"Drop files here..."):a.createElement("p",null,"Try dropping some files here, or click to select files to upload."))}),a.createElement("div",null,a.createElement(p.DiagramWidget,{className:"pipeline-canvas",diagramEngine:e})))}}]),n}(a.Component);function C(e,n){var t=E(n.serializeDiagram()),a=new p.DiagramModel;return a.deSerializeDiagram(t,e),a}t(498);var N=new p.DiagramEngine;N.installDefaultFactories();var P=new p.DiagramModel;N.setDiagramModel(P);var x=function(e){function n(){return Object(c.a)(this,n),Object(l.a)(this,Object(d.a)(n).apply(this,arguments))}return Object(s.a)(n,e),Object(u.a)(n,[{key:"render",value:function(){return r.a.createElement("div",{className:"App"},r.a.createElement(L,{engine:N}))}}]),n}(a.Component);Boolean("localhost"===window.location.hostname||"[::1]"===window.location.hostname||window.location.hostname.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/));i.a.render(r.a.createElement(x,null),document.getElementById("root")),"serviceWorker"in navigator&&navigator.serviceWorker.ready.then(function(e){e.unregister()})}},[[207,1,2]]]);
//# sourceMappingURL=main.b2bf60be.chunk.js.map