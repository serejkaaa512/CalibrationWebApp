{{> header }}
<div class="text-center">
    <h1>
        Привет!!! Это Калибровка!!!!
    </h1>
</div>
<div class="text-center">
    <h2>
        <label>Выбери генератор.</label>
    </h2>
</div>
<div class="text-center">
    <form method="post" id="gen_form" >
        {{#generators}}
        {{> generator }}
        {{/generators}}
        {{^generators}}
        <label class="label label-warning" id="id_empty_gen">Список пуст!!</label>
        {{/generators}}
        <div id="generator_send"/>
    </form>
</div>
<div >
    <form method="post" >
        <div >
            <table class="table"> 
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
                        <input  class="btn btn-lg btn-success"  type="button" id="id_generator_add" value="+" name="add_generator" onclick="{AddGenerator($('#id_generator_ip').val(), $('#id_generator_port').val())}" disabled>
                    </td>
                </tr>
            </table>
        </div>
    </form>
</div>

<div class="text-center">
    <h2>
        <label>Выбери измеритель мощности.</label>
    </h2>
</div>
<div class="text-center">
    <form method="post" id="pm_form">
        {{#powermeters}}
        {{> powermeter }}
        {{/powermeters}}
        {{^powermeters}}
        <label class="label label-warning" id="id_empty_pm">Список пуст!!</label>
        {{/powermeters}}
        <div id="powermeter_send"/>
    </form>
</div>

<div class="text-center">
    <form method="post" >
        <div class="col-md-6">
            <table class="table">
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
                        <input class="btn btn-lg btn-success" type="button" id="id_powermeter_add" value="+" name="add_powermeter" onclick="{AddPowerMeter($('#id_powermeter_ip').val(), $('#id_powermeter_port').val())}" disabled>
                    </td>
                </tr>
            </table>
        </div>
    </form>
</div>
<br/>
<div class="text-center">
    <form method="get" name="connection_form" action="/calibration/options">
        <input class="btn btn-lg btn-success" type="button" id="id_connection_btn" value="К настройкам алгоритма">
        <input type="hidden" value="" name="generator_id" id="generator_id">
        <input type="hidden" value="" name="powermeter_id" id="powermeter_id">
    </form>
</div>
<script>
    function RemPowerMeter (el, id) {
        jQuery.ajax({
            'type':'DELETE',
            'url':'/powermeter/'+id,
            'cache':false,
            'async': true,
            'success':function(response){
                console.log(response);
                if (response == "removed")
                    $(el).parent().remove();
                else
                    alert("Couldn't remove !!!!");
                var length = $('.pm_item').length;
                if (length == 0)
                    $('#id_empty_pm').show();
            },
            'error':function(response, status, xhr){
                alert(status);
            }
        });
    }

    function RemGenerator (el, id) {
        jQuery.ajax({
            'type':'DELETE',
            'url':'/generator/'+id,
            'cache':false,
            'async': true,
            'success':function(response){
                console.log(response);
                if (response == "removed")
                    $(el).parent().remove();
                else
                    alert("Couldn't remove !!!!");
                var length = $('.gen_item').length;
                if (length == 0)
                    $('#id_empty_gen').show();
            },
            'error':function(response, status, xhr){
                alert(status);
            }
        });
    }

    
    function AddPowerMeter (ip, port) {
        var data = {
            'ip': ip,
            'port': port
        };
        jQuery.ajax({
            'type':'POST',
            'url':'/powermeter/add',
            'cache':false,
            'async': true,
            'data':data,
            'success':function(response){
                $('#powermeter_send').prepend(response);
                $('#id_empty_pm').hide();
            },
            'error':function(response, status, xhr){
                alert(status);
            }
        });
    }

    function AddGenerator (ip, port) {
        var data = {
            'ip': ip,
            'port': port
        };
        jQuery.ajax({
            'type':'POST',
            'url':'/generator/add',
            'cache':false,
            'async': true,
            'data':data,
            'success':function(response){
                $('#generator_send').prepend(response);
                $('#id_empty_gen').hide();
            },
            'error':function(response, status, xhr){
                alert(status);
            }
        });
    }
    
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
    $( "#id_connection_btn").on('click',function(e){
        var gen_id = $('input[name=gen_value]:checked', '#gen_form').val();
        var pm_id = $('input[name=pm_value]:checked', '#pm_form').val();
        
        if (typeof gen_id === 'undefined') {
            alert("Не выбран генератор!");
            return;
        }

        if (typeof pm_id === 'undefined') {
         alert("Не выбран измеритель мощности!");
         return;
     }

     $('#generator_id').val(gen_id);
     $('#powermeter_id').val(pm_id);
     document.forms.connection_form.submit();
 });
</script>

{{> footer }}
