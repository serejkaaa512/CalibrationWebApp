{{> header }}
<div class="text-center">
    <h1>
        {{name}}
    </h1>
</div>
<table class="table table-striped">
    <tr class="row">
        <td class="col">
            Начальная частота, МГц: {{fmin}}
        </td>
        <td class="col">
            Конечная частота, МГц: {{fmax}}
        </td>
        <td class="col">
            Шаг частоты, МГц: {{fstep}}
        </td>
        <td class="col">
            Мощность генератора, дБм: {{pgen}}
        </td>
        <td class="col">
            Канал ИМ: {{pchannel}}
        </td>
    </tr>
</table>
<div class="text-center">
    <h3>
        Полученные значения:
    </h3>
</div>
<table class="table table-striped" id="res_table">
    <tr><td>Частота, МГц</td><td>Мощность, дБм</td></tr>
</table>

<h3>
    <div class="alert alert-danger" id="id_error">
        
    </div>
</h3>
<script>
    var fmin = {{fmin}};
    var fmax = {{fmax}};
    var fstep = {{fstep}};
    var pgen = {{pgen}};
    var pchannel = {{pchannel}};
    var gen_id = {{gen_id}};
    var pm_id = {{pm_id}};
    var fcur = fmin;
    $('#id_error').hide();

    $(document).ready(function() {
        if (!Generator_set_power(gen_id, pgen))
            return;
        if (!Generator_turn_on(gen_id))
            return;
        while (fcur <= fmax) {
            if (!Generator_set_freq(gen_id, fcur))
                return;
            if (!PowerMeter_get_power(pm_id))
                return;
            fcur += fstep;
            fcur = Number(fcur.toFixed(3));
            console.log(fcur);
        }
        Generator_turn_off(gen_id);
    });


    function Generator_set_power (gen_id, pgen) {
        var flag = false;
        jQuery.ajax({
            'type':'POST',
            'url':'/generator/'+gen_id+'/set_power/'+ pgen,
            'cache':false,
            'async': false,
            'success':function(response){
                console.log(response);
                flag = true;
            },
            'error':function(response, status, xhr){
                $('#id_error').show();
                $('#id_error').html(response.responseText);
            }
        });

        return flag;
    }


    function Generator_turn_on (gen_id) {
        var flag = false;
        jQuery.ajax({
            'type':'POST',
            'url':'/generator/'+gen_id+'/turn_on',
            'cache':false,
            'async': false,
            'success':function(response){
                console.log(response);
                flag = true;
            },
            'error':function(response, status, xhr){
                $('#id_error').show();
                $('#id_error').html(response.responseText);
            }
        });
        return flag;
    }

    function Generator_set_freq (gen_id, freq) {
        var flag = false;
        jQuery.ajax({
            'type':'POST',
            'url':'/generator/'+gen_id+'/set_freq/'+ freq,
            'cache':false,
            'async': false,
            'success':function(response){
                console.log(response);
                flag = true;
            },
            'error':function(response, status, xhr){
                $('#id_error').show();
                $('#id_error').html(response.responseText);
            }
        });
        return flag;
    }
    function PowerMeter_get_power (pm_id) {
        var flag = false;
        jQuery.ajax({
            'type':'GET',
            'url':'/powermeter/'+pm_id+'/'+pchannel+'/power',
            'cache':false,
            'async': false,
            'success':function(response){
                $('#res_table').append("<tr><td>"+fcur+"</td><td>"+response+"</td></tr>")
                console.log(response);
                flag = true;
            },
            'error':function(response, status, xhr){
                $('#id_error').show();
                $('#id_error').html(response.responseText);
            }
        });
        return flag;
    }

    function Generator_turn_off (gen_id) {
        var flag = false;
        jQuery.ajax({
            'type':'POST',
            'url':'/generator/'+gen_id+'/turn_off',
            'cache':false,
            'async': false,
            'success':function(response){
                console.log(response);
                flag = true;
            },
            'error':function(response, status, xhr){
                $('#id_error').show();
                $('#id_error').html(response.responseText);
            }
        });
        return flag;
    }
</script>
{{> footer }}
