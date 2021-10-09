-- Your SQL goes here
create table trace_sqls
(
    id              bigint unsigned                           not null primary key auto_increment,
    app_uuid        char(32)     default ''                   not null comment 'sql执行所在生命周期的trace_id',
    sql_uuid        char(32)     default ''                   not null comment '当前sql的trace_id',
    db_host         varchar(32)  default ''                   not null comment '数据库地址',
    run_host        varchar(32)  default ''                   not null comment 'sql执行所在机器',
    run_ms          int unsigned default 0                    not null comment 'sql执行毫秒时间',
    pid             int unsigned default 0                    not null comment '程序的PID',
    run_mode        varchar(16)  default ''                   not null comment '运行模式:job,api...',
    request_uri     varchar(256) default ''                   not null comment 'api模式=REQUEST_URI; job模式=$argv',
    referer         varchar(256) default ''                   not null comment '仅在api模式下，页面来源',
    trace_sql_md5   char(32)     default ''                   not null comment '执行的sql',
    trace_sql       text                                      not null comment '执行的sql',
    trace_sql_binds varchar(512) default ''                   not null comment '参数绑定值',
    created_at      datetime(3)  default CURRENT_TIMESTAMP(3) not null comment '创建时间',
    INDEX idx_app_uuid (`app_uuid`),
    UNIQUE INDEX uk_sql_uuid (`sql_uuid`),
    INDEX idx_created_at (`created_at`)
) ENGINE = InnoDB
  DEFAULT charset = utf8mb4
  COMMENT = 'sql记录，同一个app生命周期记录一到多个sql日志';

create table trace_sql_files
(
    id              bigint unsigned                           not null primary key auto_increment,
    app_uuid        char(32)     default ''                   not null comment 'sql执行所在生命周期的trace_id',
    sql_uuid        char(32)     default ''                   not null comment '当前sql的trace_id',
    trace_file      varchar(128) default ''                   not null comment '所在文件',
    trace_line      int unsigned default 0                    not null comment '所在行数',
    trace_class     varchar(128) default ''                   not null comment '类&函数',
    created_at      datetime(3)  default CURRENT_TIMESTAMP(3) not null comment '创建时间',
    INDEX idx_app_uuid (`app_uuid`),
    INDEX idx_sql_uuid (`sql_uuid`),
    INDEX idx_created_at (`created_at`)
) ENGINE = InnoDB
  DEFAULT charset = utf8mb4
  COMMENT = 'sql调用链路，用于查询执行路径';

create table trace_sql_code
(
    id            bigint unsigned                           not null primary key auto_increment,
    app_uuid      char(16)     default ''                   not null comment 'sql执行所在生命周期的trace_id',
    sql_uuid      char(16)     default ''                   not null comment '当前sql的trace_id',
    file_id       bigint unsigned                           not null comment 'trace_sql_files.id',
    source_code   text                                      not null comment '源码',
    created_at    datetime(3)  default CURRENT_TIMESTAMP(3) not null comment '创建时间',
    INDEX idx_app_uuid (`app_uuid`),
    INDEX idx_sql_uuid (`sql_uuid`),
    INDEX idx_created_at (`created_at`)
) ENGINE = InnoDB
  DEFAULT charset = utf8mb4
  COMMENT = 'sql执行时刻，代码缓存记录';