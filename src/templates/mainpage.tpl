{{> header }}
<h1>
    Привет!!! Это Калибровка!!!!
</h1>
<form method="post">
    <table>
        <tr class="row">
            <td class="col">
                IP Генератора:
            </td>
            <td class="col">
                <input name="generator_ip" id="id_generator_ip" value="10.10.0.7" />
            </td>
        </tr>
        <tr class="row">
            <td class="col">
                Port Генератора:
            </td>
            <td class="col">
                <input name="generator_port" id="id_generator_port" value="3333" />
            </td>
        </tr>
        <tr class="row">
            <td class="col">
                IP Измерителя мощности:
            </td>
            <td class="col">
                <input name="powermeter_ip" id="id_powermeter_ip" value="10.10.0.7" />
            </td>
        </tr>
        <tr class="row">
            <td class="col">
                Port Измерителя мощности:
            </td>
            <td class="col">
                <input name="powermeter_port" id="id_powermeter_port" value="5025" />
            </td>
        </tr>
    </table>
    {{#generators}}
        <input type="radio" value="{{value}}" name="gen_value">
       <label>{{name}}</label>
       <br/>
    {{/generators}}

    <input type="submit" value="Соединиться">
    <!-- <input type="submit" value="Ok" style="display:none"> -->
</form>
{{> footer }}
