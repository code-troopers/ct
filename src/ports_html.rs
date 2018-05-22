pub const INDEX : &str = r##"<!DOCTYPE html>
    <html>
    <head>
    <title>CT port list</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="//cdnjs.cloudflare.com/ajax/libs/twitter-bootstrap/3.2.0/css/bootstrap.min.css" rel="stylesheet" media="screen">
    <link href="//cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css" rel="stylesheet" media="screen">
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
<tfoot>
<tr>
<th></th>
<th></th>
<th></th>
<th><i class="fa fa-globe">&nbsp;</i>: HTTP | <i class="fa fa-lock">&nbsp;</i>: HTTPS</th>
</tfoot>
</table>
</div>
<footer class="text-center" role="contentinfo">
<div class="container">
<p>Made with ❤︎ From Tours, France by <a href="http://www.code-troopers.com" target="_blank">Code-Troopers</a>.</p>
<pre style="font-size:4px">$(banner)</pre>
</div>
</footer>
<!-- jQuery (necessary for Bootstrap's JavaScript plugins) -->
<script src="//cdnjs.cloudflare.com/ajax/libs/jquery/2.1.1/jquery.min.js"></script>
<!-- Include all compiled plugins (below), or include individual files as needed -->
<script src="//cdnjs.cloudflare.com/ajax/libs/twitter-bootstrap/3.2.0/js/bootstrap.min.js"></script>
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
json[i].listen
    .map((p) => p.replace("*", window.location.hostname).replace("localhost", window.location.hostname))
    .map((p) => `${p} - <a href="http://${p}" target="_blank"><i class="fa fa-globe">&nbsp;</i></a> <a href="https://${p}" target="_blank"><i class="fa fa-lock">&nbsp;</i></a>`)
    .join("<br>")
+"</td>" +
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