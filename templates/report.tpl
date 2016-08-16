{{> header }}
<div class="text-center">
    <h1>
        {{#report}}{{name}}{{/report}}
    </h1>
</div>
<table class="table table-striped" id="res_table">
    <thead>
        <tr><td>Частота, МГц</td><td>Мощность, дБм</td></tr>
        {{#report}}
        {{#values}}
            <tr><td>{{ freq }}</td><td>{{ pow }}</td></tr>
        {{/values}}
        {{/report}}
    </thead>
    <tbody>

    </tbody>
</table>
<h3>
     <image src="/calibration/graph/{{#report}}{{id}}{{/report}}"/>
</h3>
{{> footer }}
