pub const INDEX : &str = r##"<!DOCTYPE html>
    <html>
    <head>
    <title>CT port list</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="http://cdnjs.cloudflare.com/ajax/libs/twitter-bootstrap/3.2.0/css/bootstrap.min.css" rel="stylesheet" media="screen">
</head>
<body>
<h1 class="page-header">Listening ports <button id="refreshBtn" class="btn btn-primary pull-right">Refresh</button></h1>
<div class="table-responsive">
<table class="table table-striped">
<thead>
<tr>
<th>#</th>
<th>Working dir</th>
<th>Command</th>
<th>URI</th>
</tr>
</thead>
<tbody id="content">

</tbody>
</table>
</div>
<footer class="text-center" role="contentinfo">
<div class="container">
<p>Made with <3 From Tours, France by <a href="http://www.code-troopers.com" target="_blank">Code-Troopers</a>.</p>
<pre style="font-size:4px">$(banner)</pre>
</div>
</footer>
<!-- jQuery (necessary for Bootstrap's JavaScript plugins) -->
<script src="http://cdnjs.cloudflare.com/ajax/libs/jquery/2.1.1/jquery.min.js"></script>
<!-- Include all compiled plugins (below), or include individual files as needed -->
<script src="http://cdnjs.cloudflare.com/ajax/libs/twitter-bootstrap/3.2.0/js/bootstrap.min.js"></script>
<script src="//cdnjs.cloudflare.com/ajax/libs/spin.js/2.0.1/spin.min.js"></script>
<script>
$(document).ready(function(){
var $content = $("#content");
var spinner = new Spinner().spin();
var scan = function(){
document.body.appendChild(spinner.el);
$.get('/scan').then(function(data){
$content.empty();
var json = JSON.parse(data.replace(/'/g, '"'));
for (var i in json){
if (null === json[i]) continue;
//var relativeAddr = json[i].address.replace('127\.0\.0\.1', window.location.hostname);
$content.append("<tr>"+
"<td>"+json[i].pid+"</td>"+
"<td>"+json[i].cwd+"</td>"+
"<td>"+json[i].name+"</td>"+
"<td>"+
json[i].listen.map((p) => p.replace("*", window.location.hostname).replace("localhost", window.location.hostname))
+"</td>" +
//<a href='http"+(json[i].secure?"s":"")+"://"+relativeAddr+"'>"+relativeAddr+"</a></td>"+
"</tr>");
}
document.body.removeChild(spinner.el);
});
}
scan();
$("#refreshBtn").click(scan);
});
</script>
</body>
</html>
"##;