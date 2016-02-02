{{> header }}
<h1>
    Поехали!!!!
</h1>
<table>
    <tr class="row">
        <td class="col">
            Название: {{name}}
        </td>
    </tr>
    <tr class="row">
        <td class="col">
            Fmin: {{fmin}}
        </td>
        <td class="col">
            Fшаг: {{fstep}}
        </td>
        <td class="col">
            Fmax: {{fmax}}
        </td>
    </tr>
    <tr class="row">
        <td class="col">
            P: {{pgen}}
        </td>
    </tr>
    <tr class="row">
        <td class="col">
            <input type="button" id="id_start" value="start" name="start">
        </td>
    </tr>
</table>
<h2>
    Результат!!!!
</h2>
<table id="res_table">
    <tr class="row">
        <td class="col">
            Частота:
        </td>
        <td class="col">
            Мощность:
        </td>
    </tr>
</table>
{{> footer }}
