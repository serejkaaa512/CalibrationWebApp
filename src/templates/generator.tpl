<div>
    <input type="radio" value="{{id}}" name="gen_value">
    <label id="id_{{ip}}">{{ip}}</label>
    <label>:</label>
    <label>{{port}}</label>
    {{#busy}}
    <label>Занят!</label>
    {{/busy}}
    {{^busy}}
    <input type="button" value="-" name="rem_generator" 
        onclick="{RemGenerator(this, {{id}})}">
    {{/busy}}
    <br/>
</div>
