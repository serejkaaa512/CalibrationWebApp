<div>
    <input type="radio" value="{{id}}" name="pm_value">
    <label id="id_{{ip}}">{{ip}}</label>
    <label>:</label>
    <label>{{port}}</label>
    {{#busy}}
    <label>Занят!</label>
    {{/busy}}
    {{^busy}}
    <input type="button" value="-" name="rem_powermeter" 
        onclick="{RemPowerMeter(this, {{id}})}">
    {{/busy}}
    <br/>
</div>
