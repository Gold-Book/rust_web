-- Your SQL goes here
create table cards
(
    card_id           serial                                                    not null,
    name              varchar(255)                                              not null,
    kana              varchar(255)                                              not null,
    company_name      varchar(255)                                              not null,
    department        varchar(255)                                              not null,
    position          varchar(255)                                              not null,
    email             varchar(255)                                              not null,
    cell_phone_number varchar(255)                                              not null,
    phone_number      varchar(255)                                              not null,
    fax               varchar(255)                                              not null,
    zip_code          varchar(255)                                              not null,
    address           varchar(255)                                              not null,
    url               varchar(255)                                              not null,
    owner             varchar(255)                                              not null,
    PRIMARY KEY (card_id)
);

comment on table cards is '名刺一覧';

comment on column cards.card_id is '名刺ID';

comment on column cards.name is '指名';

comment on column cards.kana is 'ふりがな';

comment on column cards.company_name is '会社名';

comment on column cards.department is '部署名';

comment on column cards.position is '部署名';

comment on column cards.email is 'Eメールアドレス';

comment on column cards.cell_phone_number is '電話番号';

comment on column cards.phone_number is '固定電話番号';

comment on column cards.fax is 'FAX';

comment on column cards.zip_code is '郵便番号';

comment on column cards.address is '住所';

comment on column cards.url is '会社URL';

comment on column cards.owner is '名刺所有者';

