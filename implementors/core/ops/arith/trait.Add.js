(function() {var implementors = {};
implementors["chrono"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"chrono/offset/struct.FixedOffset.html\" title=\"struct chrono::offset::FixedOffset\">FixedOffset</a>&gt; for <a class=\"struct\" href=\"chrono/naive/struct.NaiveTime.html\" title=\"struct chrono::naive::NaiveTime\">NaiveTime</a>",synthetic:false,types:["chrono::naive::time::NaiveTime"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"chrono/offset/struct.FixedOffset.html\" title=\"struct chrono::offset::FixedOffset\">FixedOffset</a>&gt; for <a class=\"struct\" href=\"chrono/naive/struct.NaiveDateTime.html\" title=\"struct chrono::naive::NaiveDateTime\">NaiveDateTime</a>",synthetic:false,types:["chrono::naive::datetime::NaiveDateTime"]},{text:"impl&lt;Tz:&nbsp;<a class=\"trait\" href=\"chrono/offset/trait.TimeZone.html\" title=\"trait chrono::offset::TimeZone\">TimeZone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"chrono/offset/struct.FixedOffset.html\" title=\"struct chrono::offset::FixedOffset\">FixedOffset</a>&gt; for <a class=\"struct\" href=\"chrono/struct.DateTime.html\" title=\"struct chrono::DateTime\">DateTime</a>&lt;Tz&gt;",synthetic:false,types:["chrono::datetime::DateTime"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"chrono/struct.Duration.html\" title=\"struct chrono::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"chrono/naive/struct.NaiveDate.html\" title=\"struct chrono::naive::NaiveDate\">NaiveDate</a>",synthetic:false,types:["chrono::naive::date::NaiveDate"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"chrono/struct.Duration.html\" title=\"struct chrono::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"chrono/naive/struct.NaiveTime.html\" title=\"struct chrono::naive::NaiveTime\">NaiveTime</a>",synthetic:false,types:["chrono::naive::time::NaiveTime"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"chrono/struct.Duration.html\" title=\"struct chrono::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"chrono/naive/struct.NaiveDateTime.html\" title=\"struct chrono::naive::NaiveDateTime\">NaiveDateTime</a>",synthetic:false,types:["chrono::naive::datetime::NaiveDateTime"]},{text:"impl&lt;Tz:&nbsp;<a class=\"trait\" href=\"chrono/offset/trait.TimeZone.html\" title=\"trait chrono::offset::TimeZone\">TimeZone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"chrono/struct.Duration.html\" title=\"struct chrono::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"chrono/struct.Date.html\" title=\"struct chrono::Date\">Date</a>&lt;Tz&gt;",synthetic:false,types:["chrono::date::Date"]},{text:"impl&lt;Tz:&nbsp;<a class=\"trait\" href=\"chrono/offset/trait.TimeZone.html\" title=\"trait chrono::offset::TimeZone\">TimeZone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"chrono/struct.Duration.html\" title=\"struct chrono::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"chrono/struct.DateTime.html\" title=\"struct chrono::DateTime\">DateTime</a>&lt;Tz&gt;",synthetic:false,types:["chrono::datetime::DateTime"]},];
implementors["diesel"] = [{text:"impl&lt;Rhs&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Rhs&gt; for <a class=\"struct\" href=\"diesel/dsl/struct.now.html\" title=\"struct diesel::dsl::now\">now</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Rhs: <a class=\"trait\" href=\"diesel/expression/trait.AsExpression.html\" title=\"trait diesel::expression::AsExpression\">AsExpression</a>&lt;&lt;&lt;<a class=\"struct\" href=\"diesel/dsl/struct.now.html\" title=\"struct diesel::dsl::now\">now</a> as <a class=\"trait\" href=\"diesel/expression/trait.Expression.html\" title=\"trait diesel::expression::Expression\">Expression</a>&gt;::<a class=\"type\" href=\"diesel/expression/trait.Expression.html#associatedtype.SqlType\" title=\"type diesel::expression::Expression::SqlType\">SqlType</a> as <a class=\"trait\" href=\"diesel/sql_types/ops/trait.Add.html\" title=\"trait diesel::sql_types::ops::Add\">Add</a>&gt;::<a class=\"type\" href=\"diesel/sql_types/ops/trait.Add.html#associatedtype.Rhs\" title=\"type diesel::sql_types::ops::Add::Rhs\">Rhs</a>&gt;,&nbsp;</span>",synthetic:false,types:["diesel::expression::functions::date_and_time::now"]},{text:"impl&lt;ST, T, __Rhs&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;__Rhs&gt; for <a class=\"struct\" href=\"diesel/expression/struct.SqlLiteral.html\" title=\"struct diesel::expression::SqlLiteral\">SqlLiteral</a>&lt;ST, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: <a class=\"trait\" href=\"diesel/expression/trait.Expression.html\" title=\"trait diesel::expression::Expression\">Expression</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self::<a class=\"type\" href=\"diesel/expression/trait.Expression.html#associatedtype.SqlType\" title=\"type diesel::expression::Expression::SqlType\">SqlType</a>: <a class=\"trait\" href=\"diesel/sql_types/ops/trait.Add.html\" title=\"trait diesel::sql_types::ops::Add\">Add</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;__Rhs: <a class=\"trait\" href=\"diesel/expression/trait.AsExpression.html\" title=\"trait diesel::expression::AsExpression\">AsExpression</a>&lt;&lt;Self::<a class=\"type\" href=\"diesel/expression/trait.Expression.html#associatedtype.SqlType\" title=\"type diesel::expression::Expression::SqlType\">SqlType</a> as <a class=\"trait\" href=\"diesel/sql_types/ops/trait.Add.html\" title=\"trait diesel::sql_types::ops::Add\">Add</a>&gt;::<a class=\"type\" href=\"diesel/sql_types/ops/trait.Add.html#associatedtype.Rhs\" title=\"type diesel::sql_types::ops::Add::Rhs\">Rhs</a>&gt;,&nbsp;</span>",synthetic:false,types:["diesel::expression::sql_literal::SqlLiteral"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"diesel/pg/data_types/struct.PgInterval.html\" title=\"struct diesel::pg::data_types::PgInterval\">PgInterval</a>&gt; for <a class=\"struct\" href=\"diesel/pg/data_types/struct.PgInterval.html\" title=\"struct diesel::pg::data_types::PgInterval\">PgInterval</a>",synthetic:false,types:["diesel::pg::types::date_and_time::PgInterval"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"diesel/pg/data_types/struct.Cents.html\" title=\"struct diesel::pg::data_types::Cents\">PgMoney</a>&gt; for <a class=\"struct\" href=\"diesel/pg/data_types/struct.Cents.html\" title=\"struct diesel::pg::data_types::Cents\">PgMoney</a>",synthetic:false,types:["diesel::pg::types::money::PgMoney"]},];
implementors["openssl"] = [{text:"impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;&amp;'b <a class=\"struct\" href=\"openssl/bn/struct.BigNumRef.html\" title=\"struct openssl::bn::BigNumRef\">BigNumRef</a>&gt; for &amp;'a <a class=\"struct\" href=\"openssl/bn/struct.BigNumRef.html\" title=\"struct openssl::bn::BigNumRef\">BigNumRef</a>",synthetic:false,types:["openssl::bn::BigNumRef"]},{text:"impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;&amp;'b <a class=\"struct\" href=\"openssl/bn/struct.BigNum.html\" title=\"struct openssl::bn::BigNum\">BigNum</a>&gt; for &amp;'a <a class=\"struct\" href=\"openssl/bn/struct.BigNumRef.html\" title=\"struct openssl::bn::BigNumRef\">BigNumRef</a>",synthetic:false,types:["openssl::bn::BigNumRef"]},{text:"impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;&amp;'b <a class=\"struct\" href=\"openssl/bn/struct.BigNumRef.html\" title=\"struct openssl::bn::BigNumRef\">BigNumRef</a>&gt; for &amp;'a <a class=\"struct\" href=\"openssl/bn/struct.BigNum.html\" title=\"struct openssl::bn::BigNum\">BigNum</a>",synthetic:false,types:["openssl::bn::BigNum"]},{text:"impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;&amp;'b <a class=\"struct\" href=\"openssl/bn/struct.BigNum.html\" title=\"struct openssl::bn::BigNum\">BigNum</a>&gt; for &amp;'a <a class=\"struct\" href=\"openssl/bn/struct.BigNum.html\" title=\"struct openssl::bn::BigNum\">BigNum</a>",synthetic:false,types:["openssl::bn::BigNum"]},];
implementors["time"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>",synthetic:false,types:["time::duration::Duration"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Timespec.html\" title=\"struct time::Timespec\">Timespec</a>",synthetic:false,types:["time::Timespec"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.SteadyTime.html\" title=\"struct time::SteadyTime\">SteadyTime</a>",synthetic:false,types:["time::SteadyTime"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;<a class=\"struct\" href=\"time/struct.Duration.html\" title=\"struct time::Duration\">Duration</a>&gt; for <a class=\"struct\" href=\"time/struct.Tm.html\" title=\"struct time::Tm\">Tm</a>",synthetic:false,types:["time::Tm"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()