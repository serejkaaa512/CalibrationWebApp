{{> header }}
<div class="text-center">
    <h1>
        {{name}}
    </h1>
</div>
<table class="table table-striped" id="res_table">
    <thead>
        <tr><td>Частота, МГц</td><td>Мощность, дБм</td></tr>
        {{#results}}
            <tr><td>{{ freq }}</td><td>{{ pow }}</td></tr>
        {{/results}}
    </thead>
    <tbody>

    </tbody>
</table>
<h3>
     <image src="/graph.bmp"/>
</h3>
{{> footer }}
