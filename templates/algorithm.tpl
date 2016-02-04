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
    <thead>
        <tr><td>Частота, МГц</td><td>Мощность, дБм</td></tr>
    </thead>
    <tbody>

    </tbody>
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
    var trs = null;

    $(document).ready(function() {
        $('#id_error').hide();
        while (fcur <= fmax) {
            $('#res_table').append('<tr><td>'+fcur+'</td><td class="respP">?</td></tr>');
            fcur += fstep;
            fcur = Number(fcur.toFixed(3));
        };
        Start();
    });


    function Start() {
        table = $('#res_table').children()[1];
        trs = $(table).children();

        if (!Generator_set_power(gen_id, pgen))
            return;
        if (!Generator_turn_on(gen_id))
            return;

        var set_freq = function(i) {
            i = i | 0;
            if (i < trs.length)
            {
                if (Generator_set_freq(i)) {
                    setTimeout(get_power.bind(null, i), 300);
                    return;
                }
                return;
            }

            Generator_turn_off(gen_id);
            Save_report();
        }

        var get_power = function(i) {
            if (PowerMeter_get_power(i)) {
                i++;
                setTimeout(set_freq.bind(null, i), 300);
            }
        }

        setTimeout(set_freq.bind(null), 300);

    }

    function Save_report () {
        values = []
        for (var i = 0; i < trs.length; i++) {
            var freq = $($(trs[i]).children()[0]).html();
            var pow = $($(trs[i]).children()[1]).html();
            item = {}
            item ["freq"] = freq;
            item ["pow"] = pow;
            values.push(item);
        };

        data = {};
        var rep_name = "{{table_name}}";
        data['rep_name'] = rep_name;
        data['values'] = values;

        jQuery.ajax({
            'type':'POST',
            'url':'/report/add',
            'dataType' : 'json',
            'data' : JSON.stringify(data),
            'contentType': "application/json; charset=utf-8",
            'cache':false,
            'async': false,
            'success':function(response){
                flag = true;
            },
            'error':function(response, status, xhr){
                $('#id_error').show();
                $('#id_error').html(response.responseText);
            }
        });

    }


    function Generator_set_freq (i) {
        var flag = false;
        jQuery.ajax({
            'type':'POST',
            'url':'/generator/'+gen_id+'/set_freq/'+ $($(trs[i]).children()[0]).html(),
            'cache':false,
            'async': false,
            'success':function(response){
                flag = true;
            },
            'error':function(response, status, xhr){
                $('#id_error').show();
                $('#id_error').html(response.responseText);
            }
        });

        return flag;
    }

    function PowerMeter_get_power (i) {
        console.log(trs[i]);
        var flag = false;
        jQuery.ajax({
            'type':'GET',
            'url':'/powermeter/'+pm_id+'/'+pchannel+'/power',
            'cache':false,
            'async': false,
            'success':function(response){
                $($(trs[i]).children()[1]).html(response);
                flag = true;
            },
            'error':function(response, status, xhr){
                $('#id_error').show();
                $('#id_error').html(response.responseText);
            }
        });

        return flag;
    }

    function Generator_set_power (gen_id, pgen) {
        var flag = false;
        jQuery.ajax({
            'type':'POST',
            'url':'/generator/'+gen_id+'/set_power/'+ pgen,
            'cache':false,
            'async': false,
            'success':function(response){
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
