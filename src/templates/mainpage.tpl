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
                <input name="generator_ip" id="id_generator_ip" value="10.10.0.7" />
            </td>
            <td class="col">
                port:
            </td>
            <td class="col">
                <input name="generator_port" id="id_generator_port" value="3333" />
            </td>
            <td class="col">
                <input type="submit" value="+" name="add_generator">
            </td>
        </tr>
    </table>
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
                <input name="powermeter_ip" id="id_powermeter_ip" value="10.10.0.7" />
            </td>
            <td class="col">
                port:
            </td>
            <td class="col">
                <input name="powermeter_port" id="id_powermeter_port" value="4444" />
            </td>
            <td class="col">
                <input type="submit" value="+" name="add_powermeter">
            </td>
        </tr>
    </table>
    <br/>
    <input type="submit" value="Соединиться">
</form>
{{> footer }}
