<h2>
    <div class="gen_item">
        <label class="btn btn-default">
            <input type="radio" value="{{id}}" name="gen_value" {{#busy}}disabled{{/busy}}>
        </label>
        <label class="label label-info" id="id_{{ip}}">{{ip}}</label>
        <label>:</label>
        <label class="label label-info">{{port}}</label>
        {{#busy}}
        <label class="label label-warning">Занят!</label>
        {{/busy}}
        {{^busy}}
        <input class="btn btn-danger" type="button" value="-" name="rem_generator" onclick="{RemGenerator(this, {{id}})}">
        {{/busy}}
        <br/>
    </div>
</h2>
