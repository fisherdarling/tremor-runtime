CFG='[{"rule":"application=recsvc","class":"applog_recsvc","rate":1250,"dimensions":["application"],"index_key":"wf_index_type"},{"rule":"application=sayl","class":"applog_sayl","rate":2500,"dimensions":["application"],"index_key":"wf_index_type"},{"rule":"wf_index_type=applog_purest","class":"applog_purest","rate":18750,"dimensions":["logger_name"],"index_key":"wf_index_type"},{"rule":"wf_index_type=applog_admin","class":"applog_admin","rate":750,"dimensions":["logger_name"],"index_key":"wf_index_type"},{"rule":"wf_index_type=applog_supply_chain_services","class":"applog_supply_chain_services","rate":18750,"index_key":"wf_index_type"},{"rule":"wf_index_type=applog_logs","class":"applog_logs","rate":4500,"dimensions":["application"],"index_key":"wf_index_type"},{"rule":"wf_index_type=syslog_haproxy","class":"syslog_haproxy","rate":2500,"dimensions":["syslog_hostname"],"index_key":"wf_index_type"},{"rule":"tags:cisco","class":"syslog_cisco","rate":125,"dimensions":["src_ip","dst_ip"],"index_key":"wf_index_type"},{"rule":"wf_index_type=syslog_logs","class":"syslog_logs","rate":1750,"dimensions":["syslog_hostname"],"index_key":"wf_index_type"},{"rule":"wf_index_type=syslog_influxdb","class":"syslog_influxdb","rate":1750,"index_key":"wf_index_type"},{"rule":"wf_index_type=syslog_ftpd","class":"syslog_ftpd","rate":7500,"dimensions":["syslog_hostname"],"index_key":"wf_index_type"},{"rule":"wf_index_type=syslog_hypernova","class":"syslog_hypernova","rate":125,"dimensions":["syslog_hostname"],"index_key":"wf_index_type"},{"rule":"wf_index_type=edilog","class":"edilog","rate":3750,"index_key":"wf_index_type"},{"rule":"wf_index_type=sqlserverlog","class":"sqlserverlog","rate":125,"index_key":"wf_index_type"},{"rule":"type=applog","class":"applog","rate":75,"dimensions":["application"],"index_key":"wf_index_type"},{"class":"default","rate":250,"index_key":"wf_index_type"}]'

target/release/tremor-runtime --on-ramp stdin --on-ramp-config fire-and-forget --parser raw --classifier mimir --classifier-config "${CFG}" --grouping bucket --grouping-config "${CFG}" --off-ramp null
