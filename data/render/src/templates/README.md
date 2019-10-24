{% for tool in tools %}
- [{{ tool.name }}]({{ tool.url | safe }})
  {%- if tool.proprietary %}:copyright:{% endif %} {% if tool.deprecated %}:warning:{% endif %} - {{ tool.description }}
{% endfor %}
