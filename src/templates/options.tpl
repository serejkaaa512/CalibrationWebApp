{{> header }}
<div class="text-center">
    <h1>
        Настройки алгоритма калибровки.
    </h1>
</div>
{{#generator}}
<div class="text-center">
    <h3>Выбран генератор - {{ip}}:{{port}}.</h3>
</div>
{{/generator}}
{{#powermeter}}
<div class="text-center">
    <h3>Выбран измеритель мощности - {{ip}}:{{port}}.</h3>
</div>
{{/powermeter}}
<div>
    <form method="get" action="/calibration/algorithm/">
        <div class="text-center">

            <table class="table">
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
                        Fmin, Гц:
                    </td>
                    <td class="col">
                        <input name="fmin" id="id_fmin" placeholder="10"/>
                    </td>
                </tr>
                <tr class="row">
                    <td class="col">
                        Fшаг, Гц:
                    </td>
                    <td class="col">
                        <input name="fstep" id="id_fstep" placeholder="10"/>
                    </td>
                </tr>
                <tr class="row">
                    <td class="col">
                        Fmax, Гц:
                    </td>
                    <td class="col">
                        <input name="fmax" id="id_fmax" placeholder="100" />
                    </td>
                </tr>
                <tr class="row">
                    <td class="col">
                        P, дБм:
                    </td>
                    <td class="col">
                        <input name="pgen" id="id_pgen" placeholder="0"/>
                    </td>
                </tr>
                <tr class="row">
                    <td class="col">
                        Канал ИМ:
                    </td>
                    <td class="col">
                        <input name="pchannel" id="id_pchannel" value="1"/>
                    </td>
                </tr>
            </table>
            <input class="btn btn-lg btn-success" type="submit" id="id_start" value="Старт" disabled>
            <input type="hidden" value="{{#generator}}{{id}}{{/generator}}" name="gen_id" id="gen_id">
            <input type="hidden" value="{{#powermeter}}{{id}}{{/powermeter}}" name="pm_id" id="pm_id">
        </div>
    </form>
</div>
<script>
    $( "#id_fstep, #id_pgen, #id_name, #id_fmin, #id_fmax, #id_pchannel ").on('input',function(e){
        if (    $('#id_fstep').val() == ""
            ||  $('#id_pgen').val() == ""
            ||  $('#id_name').val() == ""
            ||  $('#id_fmin').val() == ""
            ||  $('#id_fmax').val() == ""
            ||  $('#id_pchannel').val() == ""
            )
            $( "#id_start" ).attr("disabled", "disabled");
        else
            $( "#id_start" ).removeAttr("disabled");
    });
    $(function() {
        $( "#id_fstep, #id_pgen, #id_fmin, #id_fmax, #id_pchannel ").on('keydown',function(e){
            -1!==$.inArray(e.keyCode,[46,8,9,27,13,110,190])||/65|67|86|88/.test(e.keyCode)&&(!0===e.ctrlKey||!0===e.metaKey)||35<=e.keyCode&&40>=e.keyCode||(e.shiftKey||48>e.keyCode||57<e.keyCode)&&(96>e.keyCode||105<e.keyCode)&&e.preventDefault()});
    });
</script>
{{> footer }}
