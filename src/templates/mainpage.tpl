{{> header }}
<h1>
    Привет!!! Это Калибровка!!!!
</h1>
<form method="post">
    <h2>
        <label>Генераторы:</label>
    </h2>
    {{#generators}}
    <input type="radio" value="{{id}}" name="gen_value">
    <label>{{ip}}</label>
    <label>:</label>
    <label>{{port}}</label>
    {{#busy}}
    <label>Занят!</label>
    {{/busy}}
    {{^busy}}
    <input type="submit" value="-" name="rem_generator-{{id}}">
    {{/busy}}
    <br/>
    {{/generators}}
    {{^generators}}
    <label>Список пуст!!</label>
    {{/generators}}
    <table>
        <tr class="row">
            <td class="col">
                ip:
            </td>
            <td class="col">
                <input name="generator_ip" id="id_generator_ip" placeholder="10.10.0.7" />
            </td>
            <td class="col">
                port:
            </td>
            <td class="col" id="staticParent">
                <input name="generator_port" id="id_generator_port" placeholder="3333" maxlength="5" />
            </td>
            <td class="col">
                <input type="submit" id="id_generator_add" value="+" name="add_generator" disabled>
            </td>
        </tr>
    </table>
</form>
<form method="post">
    <h2>
        <label>Измерители мощности:</label>
    </h2>
    {{#powermeters}}
    <input type="radio" value="{{id}}" name="pm_value">
    <label>{{ip}}</label>
    <label>:</label>
    <label>{{port}}</label>
    {{#busy}}
    <label>Занят!</label>
    {{/busy}}
    {{^busy}}
    <input type="submit" value="-" name="rem_powermeter-{{id}}">
    {{/busy}}
    <br/>
    {{/powermeters}}
    {{^powermeters}}
    <label>Список пуст!!</label>
    {{/powermeters}}
    <table>
        <tr class="row">
            <td class="col">
                ip:
            </td>
            <td class="col">
                <input name="powermeter_ip" id="id_powermeter_ip" placeholder="10.10.0.7" />
            </td>
            <td class="col">
                port:
            </td>
            <td class="col">
                <input name="powermeter_port" id="id_powermeter_port" placeholder="4444" maxlength="5"/>
            </td>
            <td class="col">
                <input type="submit" id="id_powermeter_add" value="+" name="add_powermeter" disabled>
            </td>
        </tr>
    </table>
</form>
<br/>
<form method="post">
    <input type="submit" value="Соединиться">
</form>
<div class="form-group has-error">
    <span class="help-block">{{error}}</span>
</div>
<script>
    function ValidateIPaddress(ipaddress)   
    {  
        if (/^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/.test(ipaddress))  
        {  
            return (true)  
        }  
        return (false)  
    } 
    $( "#id_generator_ip, #id_generator_port").on('input',function(e){
        if (    $('#id_generator_port').val() == ""
            ||  !ValidateIPaddress($('#id_generator_ip').val())
            )
            $( "#id_generator_add" ).attr("disabled", "disabled");
        else
            $( "#id_generator_add" ).removeAttr("disabled");
    });
    $( "#id_powermeter_ip, #id_powermeter_port").on('input',function(e){
        if (    $('#id_powermeter_port').val() == ""
            ||  !ValidateIPaddress($('#id_powermeter_ip').val())
            )
            $( "#id_powermeter_add" ).attr("disabled", "disabled");
        else
            $( "#id_powermeter_add" ).removeAttr("disabled");
    });
    $(function() {
        $( "#id_powermeter_port, #id_generator_port").on('keydown',function(e){
            -1!==$.inArray(e.keyCode,[46,8,9,27,13,110,190])||/65|67|86|88/.test(e.keyCode)&&(!0===e.ctrlKey||!0===e.metaKey)||35<=e.keyCode&&40>=e.keyCode||(e.shiftKey||48>e.keyCode||57<e.keyCode)&&(96>e.keyCode||105<e.keyCode)&&e.preventDefault()});
    });
</script>

{{> footer }}
