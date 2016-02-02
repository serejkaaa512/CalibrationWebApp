{{> header }}
<h1>
    Введи настройки!!!!
</h1>
{{#generator}}
<h2>Генератор: {{ip}}:{{port}}</h2>
{{/generator}}
{{#powermeter}}
<h2>Измеритель мощности: {{ip}}:{{port}}</h2>
{{/powermeter}}
<h2>Настройка измерений:</h2>
<div>
    <form method="get" action="/calibration/algorithm/">
        <table>
            <tr class="row">
                <td class="col">
                    Название:
                </td>
                <td class="col">
                    <input name="name" id="id_name" placeholder="Калибровка кабеля 5м" />
                </td>
            </tr>
            <tr class="row">
                <td class="col">
                    Fmin:
                </td>
                <td class="col">
                    <input name="fmin" id="id_fmin" placeholder="10"/>
                </td>
                <td class="col">
                    Fшаг:
                </td>
                <td class="col">
                    <input name="fstep" id="id_fstep" placeholder="10"/>
                </td>
                <td class="col">
                    Fmax:
                </td>
                <td class="col">
                    <input name="fmax" id="id_fmax" placeholder="100" />
                </td>
            </tr>
            <tr class="row">
                <td class="col">
                    P:
                </td>
                <td class="col">
                    <input name="pgen" id="id_pgen" placeholder="0"/>
                </td>
            </tr>
            <tr class="row">
                <td class="col">
                    <input type="submit" id="id_start" value="save" name="save" disabled>
                </td>
            </tr>
        </table>
        <input type="hidden" value="{{#generator}}{{id}}{{/generator}}" name="gen_id" id="gen_id">
        <input type="hidden" value="{{#powermeter}}{{id}}{{/powermeter}}" name="pm_id" id="pm_id">
    </form>
</div>
<script>
    $( "#id_fstep, #id_pgen, #id_name, #id_fmin, #id_fmax ").on('input',function(e){
        if (    $('#id_fstep').val() == ""
            ||  $('#id_pgen').val() == ""
            ||  $('#id_name').val() == ""
            ||  $('#id_fmin').val() == ""
            ||  $('#id_fmax').val() == ""
            )
            $( "#id_start" ).attr("disabled", "disabled");
        else
            $( "#id_start" ).removeAttr("disabled");
    });
    $(function() {
        $( "#id_fstep, #id_pgen, #id_fmin, #id_fmax ").on('keydown',function(e){
            -1!==$.inArray(e.keyCode,[46,8,9,27,13,110,190])||/65|67|86|88/.test(e.keyCode)&&(!0===e.ctrlKey||!0===e.metaKey)||35<=e.keyCode&&40>=e.keyCode||(e.shiftKey||48>e.keyCode||57<e.keyCode)&&(96>e.keyCode||105<e.keyCode)&&e.preventDefault()});
    });
</script>
{{> footer }}
