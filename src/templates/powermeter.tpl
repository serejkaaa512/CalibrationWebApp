<h2>
    <div class="pm_item">
        <label class="btn btn-default">
            <input type="radio" value="{{id}}" name="pm_value" {{#busy}}disabled{{/busy}}>
        </label>
        <label class="label label-info" id="id_{{ip}}">{{ip}}</label>
        <label>:</label>
        <label class="label label-info">{{port}}</label>
        {{#busy}}
        <label class="label label-warning">Занят!</label>
        {{/busy}}
        {{^busy}}
        <input class="btn btn-danger" type="button" value="-" name="rem_powermeter" onclick="{RemPowerMeter(this, {{id}})}">
        {{/busy}}
        <br/>
    </div>
</h2>
