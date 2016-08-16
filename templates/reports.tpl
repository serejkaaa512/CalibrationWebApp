{{> header }}
<div class="text-center">
    <h1>
        Отчеты по калибровке.
    </h1>
</div>
<div class="text-center">
    <form method="post" id="rep_form" >
        {{#reps}}
        <h2>
            <div class="gen_item">
                <label class="btn btn-default">
                <input type="radio" value="{{name}}" name="rep_name" >
                </label>
                <label class="label label-info">{{name}}</label>
                <br/>
            </div>
        </h2>
        {{/reps}}
        {{^reps}}
        <label class="label label-warning" id="id_empty">Список пуст!!</label>
        {{/reps}}
        <div id="rep_send"/>
    </form>
</div>
<div align="center">
    <form method="get" name="rep_list_form" action="/calibration/report">
        <div align="center">
            <input class="btn btn-lg btn-success" type="button" id="id_rep_btn" value="Открыть">
        </div>
        <input type="hidden" value="" name="name" id="name">
    </form>
</div>

<script>
    $( "#id_rep_btn").on('click',function(e){
        var name = $('input[name=rep_name]:checked', '#rep_form').val();
        
        if (typeof name === 'undefined') {
            alert("Не выбран отчет!");
            return;
        }
        $('#name').val(name);
        document.forms.rep_list_form.submit();
    });
</script>
{{> footer }}
