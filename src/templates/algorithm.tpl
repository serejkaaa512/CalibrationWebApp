{{> header }}
<div class="text-center">
    <h1>
        Поехали!!!!
    </h1>
</div>
<table class="table table-striped">
    <tr class="row">
        <td class="col">
            Название: {{name}}
        </td>
        <td class="col">
            Fmin: {{fmin}}
        </td>
        <td class="col">
            Fшаг: {{fstep}}
        </td>
        <td class="col">
            Fmax: {{fmax}}
        </td>
        <td class="col">
            P: {{pgen}}
        </td>
    </tr>
</table>
<div class="text-center">
    <h3>
        Получаемые значения:
    </h3>
</div>
<table class="table table-striped" id="res_table">
    <tr class="row">
        <td class="col">
            Частота
        </td>
        <td class="col">
            Мощность
        </td>
    </tr>
</table>
<script>
    var fmin = {{fmin}};
    var fmax = {{fmax}};
    var fstep = {{fstep}};
    var pgen = {{pgen}};
    var gen_id = {{gen_id}};
    var pm_id = {{pm_id}};
    var fcur = fmin;


    $(document).ready(function() {
        if (!Generator_set_power(gen_id, pgen))
            return;
        if (!Generator_turn_on(gen_id))
            return;
        while (fcur < fmax) {
            if (!Generator_set_freq(gen_id, fcur))
                return;
            if (!PowerMeter_get_power(pm_id))
                return;
            fcur += fstep;
        }
        Generator_turn_off(gen_id);
    });


    function Generator_set_power (gen_id, pgen) {
        var flag = false;
        jQuery.ajax({
            'type':'POST',
            'url':'/generator/'+gen_id+'/set_power/'+pgen,
            'cache':false,
            'async': false,
            'success':function(response){
                console.log(response);
                flag = true;
            },
            'error':function(response, status, xhr){
                alert(status);
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
                alert(status);
            }
        });
        return flag;
    }

    function Generator_set_freq (gen_id, freq) {
        var flag = false;
        jQuery.ajax({
            'type':'POST',
            'url':'/generator/'+gen_id+'/set_freq/'+freq,
            'cache':false,
            'async': false,
            'success':function(response){
                console.log(response);
                flag = true;
            },
            'error':function(response, status, xhr){
                alert(status);
            }
        });
        return flag;
    }
    function PowerMeter_get_power (pm_id) {
        var flag = false;
        jQuery.ajax({
            'type':'GET',
            'url':'/powermeter/'+pm_id+'/'+0+'/power',
            'cache':false,
            'async': false,
            'success':function(response){
                $('#res_table').append("<tr><td>"+fcur+"</td><td>"+response+"</td></tr>")
                console.log(response);
                flag = true;
            },
            'error':function(response, status, xhr){
                alert(status);
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
                alert(status);
            }
        });
        return flag;
    }
</script>
{{> footer }}
