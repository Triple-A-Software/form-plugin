var __create = Object.create;
var __getProtoOf = Object.getPrototypeOf;
var __defProp = Object.defineProperty;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __toESM = (mod, isNodeMode, target) => {
  target = mod != null ? __create(__getProtoOf(mod)) : {};
  const to = isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target;
  for (let key of __getOwnPropNames(mod))
    if (!__hasOwnProp.call(to, key))
      __defProp(to, key, {
        get: () => mod[key],
        enumerable: true
      });
  return to;
};
var __commonJS = (cb, mod) => () => (mod || cb((mod = { exports: {} }).exports, mod), mod.exports);

// node_modules/dayjs/dayjs.min.js
var require_dayjs_min = __commonJS((exports, module) => {
  (function(t, e) {
    typeof exports == "object" && typeof module != "undefined" ? module.exports = e() : typeof define == "function" && define.amd ? define(e) : (t = typeof globalThis != "undefined" ? globalThis : t || self).dayjs = e();
  })(exports, function() {
    var t = 1000, e = 60000, n = 3600000, r = "millisecond", i = "second", s = "minute", u = "hour", a = "day", o = "week", c = "month", f = "quarter", h = "year", d = "date", l = "Invalid Date", $ = /^(\d{4})[-/]?(\d{1,2})?[-/]?(\d{0,2})[Tt\s]*(\d{1,2})?:?(\d{1,2})?:?(\d{1,2})?[.:]?(\d+)?$/, y = /\[([^\]]+)]|Y{1,4}|M{1,4}|D{1,2}|d{1,4}|H{1,2}|h{1,2}|a|A|m{1,2}|s{1,2}|Z{1,2}|SSS/g, M = { name: "en", weekdays: "Sunday_Monday_Tuesday_Wednesday_Thursday_Friday_Saturday".split("_"), months: "January_February_March_April_May_June_July_August_September_October_November_December".split("_"), ordinal: function(t2) {
      var e2 = ["th", "st", "nd", "rd"], n2 = t2 % 100;
      return "[" + t2 + (e2[(n2 - 20) % 10] || e2[n2] || e2[0]) + "]";
    } }, m = function(t2, e2, n2) {
      var r2 = String(t2);
      return !r2 || r2.length >= e2 ? t2 : "" + Array(e2 + 1 - r2.length).join(n2) + t2;
    }, v = { s: m, z: function(t2) {
      var e2 = -t2.utcOffset(), n2 = Math.abs(e2), r2 = Math.floor(n2 / 60), i2 = n2 % 60;
      return (e2 <= 0 ? "+" : "-") + m(r2, 2, "0") + ":" + m(i2, 2, "0");
    }, m: function t(e2, n2) {
      if (e2.date() < n2.date())
        return -t(n2, e2);
      var r2 = 12 * (n2.year() - e2.year()) + (n2.month() - e2.month()), i2 = e2.clone().add(r2, c), s2 = n2 - i2 < 0, u2 = e2.clone().add(r2 + (s2 ? -1 : 1), c);
      return +(-(r2 + (n2 - i2) / (s2 ? i2 - u2 : u2 - i2)) || 0);
    }, a: function(t2) {
      return t2 < 0 ? Math.ceil(t2) || 0 : Math.floor(t2);
    }, p: function(t2) {
      return { M: c, y: h, w: o, d: a, D: d, h: u, m: s, s: i, ms: r, Q: f }[t2] || String(t2 || "").toLowerCase().replace(/s$/, "");
    }, u: function(t2) {
      return t2 === undefined;
    } }, g = "en", D = {};
    D[g] = M;
    var p = "$isDayjsObject", S = function(t2) {
      return t2 instanceof _ || !(!t2 || !t2[p]);
    }, w = function t(e2, n2, r2) {
      var i2;
      if (!e2)
        return g;
      if (typeof e2 == "string") {
        var s2 = e2.toLowerCase();
        D[s2] && (i2 = s2), n2 && (D[s2] = n2, i2 = s2);
        var u2 = e2.split("-");
        if (!i2 && u2.length > 1)
          return t(u2[0]);
      } else {
        var a2 = e2.name;
        D[a2] = e2, i2 = a2;
      }
      return !r2 && i2 && (g = i2), i2 || !r2 && g;
    }, O = function(t2, e2) {
      if (S(t2))
        return t2.clone();
      var n2 = typeof e2 == "object" ? e2 : {};
      return n2.date = t2, n2.args = arguments, new _(n2);
    }, b = v;
    b.l = w, b.i = S, b.w = function(t2, e2) {
      return O(t2, { locale: e2.$L, utc: e2.$u, x: e2.$x, $offset: e2.$offset });
    };
    var _ = function() {
      function M2(t2) {
        this.$L = w(t2.locale, null, true), this.parse(t2), this.$x = this.$x || t2.x || {}, this[p] = true;
      }
      var m2 = M2.prototype;
      return m2.parse = function(t2) {
        this.$d = function(t3) {
          var { date: e2, utc: n2 } = t3;
          if (e2 === null)
            return new Date(NaN);
          if (b.u(e2))
            return new Date;
          if (e2 instanceof Date)
            return new Date(e2);
          if (typeof e2 == "string" && !/Z$/i.test(e2)) {
            var r2 = e2.match($);
            if (r2) {
              var i2 = r2[2] - 1 || 0, s2 = (r2[7] || "0").substring(0, 3);
              return n2 ? new Date(Date.UTC(r2[1], i2, r2[3] || 1, r2[4] || 0, r2[5] || 0, r2[6] || 0, s2)) : new Date(r2[1], i2, r2[3] || 1, r2[4] || 0, r2[5] || 0, r2[6] || 0, s2);
            }
          }
          return new Date(e2);
        }(t2), this.init();
      }, m2.init = function() {
        var t2 = this.$d;
        this.$y = t2.getFullYear(), this.$M = t2.getMonth(), this.$D = t2.getDate(), this.$W = t2.getDay(), this.$H = t2.getHours(), this.$m = t2.getMinutes(), this.$s = t2.getSeconds(), this.$ms = t2.getMilliseconds();
      }, m2.$utils = function() {
        return b;
      }, m2.isValid = function() {
        return !(this.$d.toString() === l);
      }, m2.isSame = function(t2, e2) {
        var n2 = O(t2);
        return this.startOf(e2) <= n2 && n2 <= this.endOf(e2);
      }, m2.isAfter = function(t2, e2) {
        return O(t2) < this.startOf(e2);
      }, m2.isBefore = function(t2, e2) {
        return this.endOf(e2) < O(t2);
      }, m2.$g = function(t2, e2, n2) {
        return b.u(t2) ? this[e2] : this.set(n2, t2);
      }, m2.unix = function() {
        return Math.floor(this.valueOf() / 1000);
      }, m2.valueOf = function() {
        return this.$d.getTime();
      }, m2.startOf = function(t2, e2) {
        var n2 = this, r2 = !!b.u(e2) || e2, f2 = b.p(t2), l2 = function(t3, e3) {
          var i2 = b.w(n2.$u ? Date.UTC(n2.$y, e3, t3) : new Date(n2.$y, e3, t3), n2);
          return r2 ? i2 : i2.endOf(a);
        }, $2 = function(t3, e3) {
          return b.w(n2.toDate()[t3].apply(n2.toDate("s"), (r2 ? [0, 0, 0, 0] : [23, 59, 59, 999]).slice(e3)), n2);
        }, y2 = this.$W, M3 = this.$M, m3 = this.$D, v2 = "set" + (this.$u ? "UTC" : "");
        switch (f2) {
          case h:
            return r2 ? l2(1, 0) : l2(31, 11);
          case c:
            return r2 ? l2(1, M3) : l2(0, M3 + 1);
          case o:
            var g2 = this.$locale().weekStart || 0, D2 = (y2 < g2 ? y2 + 7 : y2) - g2;
            return l2(r2 ? m3 - D2 : m3 + (6 - D2), M3);
          case a:
          case d:
            return $2(v2 + "Hours", 0);
          case u:
            return $2(v2 + "Minutes", 1);
          case s:
            return $2(v2 + "Seconds", 2);
          case i:
            return $2(v2 + "Milliseconds", 3);
          default:
            return this.clone();
        }
      }, m2.endOf = function(t2) {
        return this.startOf(t2, false);
      }, m2.$set = function(t2, e2) {
        var n2, o2 = b.p(t2), f2 = "set" + (this.$u ? "UTC" : ""), l2 = (n2 = {}, n2[a] = f2 + "Date", n2[d] = f2 + "Date", n2[c] = f2 + "Month", n2[h] = f2 + "FullYear", n2[u] = f2 + "Hours", n2[s] = f2 + "Minutes", n2[i] = f2 + "Seconds", n2[r] = f2 + "Milliseconds", n2)[o2], $2 = o2 === a ? this.$D + (e2 - this.$W) : e2;
        if (o2 === c || o2 === h) {
          var y2 = this.clone().set(d, 1);
          y2.$d[l2]($2), y2.init(), this.$d = y2.set(d, Math.min(this.$D, y2.daysInMonth())).$d;
        } else
          l2 && this.$d[l2]($2);
        return this.init(), this;
      }, m2.set = function(t2, e2) {
        return this.clone().$set(t2, e2);
      }, m2.get = function(t2) {
        return this[b.p(t2)]();
      }, m2.add = function(r2, f2) {
        var d2, l2 = this;
        r2 = Number(r2);
        var $2 = b.p(f2), y2 = function(t2) {
          var e2 = O(l2);
          return b.w(e2.date(e2.date() + Math.round(t2 * r2)), l2);
        };
        if ($2 === c)
          return this.set(c, this.$M + r2);
        if ($2 === h)
          return this.set(h, this.$y + r2);
        if ($2 === a)
          return y2(1);
        if ($2 === o)
          return y2(7);
        var M3 = (d2 = {}, d2[s] = e, d2[u] = n, d2[i] = t, d2)[$2] || 1, m3 = this.$d.getTime() + r2 * M3;
        return b.w(m3, this);
      }, m2.subtract = function(t2, e2) {
        return this.add(-1 * t2, e2);
      }, m2.format = function(t2) {
        var e2 = this, n2 = this.$locale();
        if (!this.isValid())
          return n2.invalidDate || l;
        var r2 = t2 || "YYYY-MM-DDTHH:mm:ssZ", i2 = b.z(this), s2 = this.$H, u2 = this.$m, a2 = this.$M, o2 = n2.weekdays, c2 = n2.months, f2 = n2.meridiem, h2 = function(t3, n3, i3, s3) {
          return t3 && (t3[n3] || t3(e2, r2)) || i3[n3].slice(0, s3);
        }, d2 = function(t3) {
          return b.s(s2 % 12 || 12, t3, "0");
        }, $2 = f2 || function(t3, e3, n3) {
          var r3 = t3 < 12 ? "AM" : "PM";
          return n3 ? r3.toLowerCase() : r3;
        };
        return r2.replace(y, function(t3, r3) {
          return r3 || function(t4) {
            switch (t4) {
              case "YY":
                return String(e2.$y).slice(-2);
              case "YYYY":
                return b.s(e2.$y, 4, "0");
              case "M":
                return a2 + 1;
              case "MM":
                return b.s(a2 + 1, 2, "0");
              case "MMM":
                return h2(n2.monthsShort, a2, c2, 3);
              case "MMMM":
                return h2(c2, a2);
              case "D":
                return e2.$D;
              case "DD":
                return b.s(e2.$D, 2, "0");
              case "d":
                return String(e2.$W);
              case "dd":
                return h2(n2.weekdaysMin, e2.$W, o2, 2);
              case "ddd":
                return h2(n2.weekdaysShort, e2.$W, o2, 3);
              case "dddd":
                return o2[e2.$W];
              case "H":
                return String(s2);
              case "HH":
                return b.s(s2, 2, "0");
              case "h":
                return d2(1);
              case "hh":
                return d2(2);
              case "a":
                return $2(s2, u2, true);
              case "A":
                return $2(s2, u2, false);
              case "m":
                return String(u2);
              case "mm":
                return b.s(u2, 2, "0");
              case "s":
                return String(e2.$s);
              case "ss":
                return b.s(e2.$s, 2, "0");
              case "SSS":
                return b.s(e2.$ms, 3, "0");
              case "Z":
                return i2;
            }
            return null;
          }(t3) || i2.replace(":", "");
        });
      }, m2.utcOffset = function() {
        return 15 * -Math.round(this.$d.getTimezoneOffset() / 15);
      }, m2.diff = function(r2, d2, l2) {
        var $2, y2 = this, M3 = b.p(d2), m3 = O(r2), v2 = (m3.utcOffset() - this.utcOffset()) * e, g2 = this - m3, D2 = function() {
          return b.m(y2, m3);
        };
        switch (M3) {
          case h:
            $2 = D2() / 12;
            break;
          case c:
            $2 = D2();
            break;
          case f:
            $2 = D2() / 3;
            break;
          case o:
            $2 = (g2 - v2) / 604800000;
            break;
          case a:
            $2 = (g2 - v2) / 86400000;
            break;
          case u:
            $2 = g2 / n;
            break;
          case s:
            $2 = g2 / e;
            break;
          case i:
            $2 = g2 / t;
            break;
          default:
            $2 = g2;
        }
        return l2 ? $2 : b.a($2);
      }, m2.daysInMonth = function() {
        return this.endOf(c).$D;
      }, m2.$locale = function() {
        return D[this.$L];
      }, m2.locale = function(t2, e2) {
        if (!t2)
          return this.$L;
        var n2 = this.clone(), r2 = w(t2, e2, true);
        return r2 && (n2.$L = r2), n2;
      }, m2.clone = function() {
        return b.w(this.$d, this);
      }, m2.toDate = function() {
        return new Date(this.valueOf());
      }, m2.toJSON = function() {
        return this.isValid() ? this.toISOString() : null;
      }, m2.toISOString = function() {
        return this.$d.toISOString();
      }, m2.toString = function() {
        return this.$d.toUTCString();
      }, M2;
    }(), k = _.prototype;
    return O.prototype = k, [["$ms", r], ["$s", i], ["$m", s], ["$H", u], ["$W", a], ["$M", c], ["$y", h], ["$D", d]].forEach(function(t2) {
      k[t2[1]] = function(e2) {
        return this.$g(e2, t2[0], t2[1]);
      };
    }), O.extend = function(t2, e2) {
      return t2.$i || (t2(e2, _, O), t2.$i = true), O;
    }, O.locale = w, O.isDayjs = S, O.unix = function(t2) {
      return O(1000 * t2);
    }, O.en = D[g], O.Ls = D, O.p = {}, O;
  });
});

// node_modules/@aaa-soft/plugin-interface/index.ts
function definePlugin(def) {
  return def;
}

// node_modules/@aaa-soft/plugin-interface/builder.ts
function label() {
  const label2 = {};
  const proxy = new Proxy({}, {
    get(_target, prop, _receiver) {
      if (prop === "build") {
        return () => label2;
      }
      return (v) => {
        label2[prop] = v;
        return proxy;
      };
    }
  });
  return proxy;
}
function builderItem(item) {
  return item;
}

// settings/overview.ts
function formOverviewPage(actionHandler) {
  return {
    label: label().de("Formulare").en("Forms").build(),
    type: "list",
    getDataHandler: actionHandler.toActionHandler("GET"),
    columns: [
      { key: "name", label: label().de("Name").en("Name").build() },
      { key: "email", label: label().de("Empf\xE4nger").en("Recipient").build() },
      { key: "actions", label: label().de("Aktionen").en("Actions").build() }
    ],
    actions: [
      {
        icon: "i-tabler-edit",
        to: "/form-plugin/create?id={{ record.id }}"
      },
      {
        icon: "i-tabler-trash",
        handler: {
          url: `${actionHandler.toActionHandler("DELETE").url}?id={{ record.id }}`,
          method: "DELETE"
        }
      }
    ]
  };
}

// settings/submissions.ts
function formSubmissionsPage(listHandler, deleteHandler) {
  return {
    label: label().de("Einsendungen").en("Submissions").build(),
    type: "list",
    getDataHandler: listHandler.toActionHandler("GET"),
    columns: [
      { key: "id", label: label().de("ID").en("ID").build() },
      { key: "name", label: label().de("Formular").en("Form").build() },
      {
        key: "recipient",
        label: label().de("Empf\xE4nger").en("Recipient").build()
      },
      {
        key: "date",
        label: label().de("Gesendet am").en("Recived at").build()
      },
      { key: "actions", label: label().de("Aktionen").en("Actions").build() }
    ],
    actions: [
      {
        icon: "i-tabler-eye",
        showModal: true
      },
      {
        icon: "i-tabler-trash",
        handler: {
          url: deleteHandler.toActionHandler("DELETE").url.replace(":submissionId", "{{ record.id }}"),
          method: "DELETE"
        }
      }
    ]
  };
}

// helper/formStructs.ts
var commonProps = [
  {
    id: "label",
    type: "text",
    label: label().en("Label").de("Label").build()
  },
  {
    id: "required",
    type: "checkbox",
    label: label().en("Required").de("Pflichtfeld").build(),
    defaultValue: false
  },
  {
    id: "class",
    type: "text",
    label: label().en("CSS Classes").de("CSS Klassen").build()
  },
  {
    id: "name",
    type: "text",
    label: label().en("Name").de("Name").build()
  }
];
var formBuilderItems = [
  builderItem({
    id: "select",
    label: label().en("Select").de("Auswahl").build(),
    icon: "i-tabler-select",
    props: [
      ...commonProps,
      {
        id: "placeholder",
        type: "text",
        label: label().en("Placeholder").de("Platzhalter").build()
      },
      {
        id: "options",
        type: "builder",
        label: label().en("Options").de("Optionen").build(),
        items: [
          builderItem({
            id: "option",
            label: label().en("Option").de("Option").build(),
            icon: "i-tabler-select",
            props: [
              {
                id: "label",
                type: "text",
                label: label().en("Label").de("Label").build()
              },
              {
                id: "value",
                type: "text",
                label: label().en("Value").de("Value").build()
              }
            ],
            data: {
              label: "",
              value: ""
            },
            template: `<option value="{{ this.data.value }}">{{ render this.data.label }}</option>`
          })
        ]
      }
    ],
    data: {
      label: "",
      placeholder: "",
      required: false,
      name: "",
      class: "grid p-2",
      options: []
    },
    template: `<div class="{{ data.class }}">
            <label for="{{ data.name }}">
                {{ render data.label }}
            </label>
            <select type="text" name="{{ data.name }}" {{#if data.required }}required{{/if}} placeholder="{{ render data.placeholder }}" >
			<option selected disabled value="{{render data.placeholder}}">{{ render data.placeholder }}</option>
			  	{{#each data.options}}
				<option value="{{ this.data.value }}">{{ render this.data.label }}</option>
  				{{/each}}
			</select>
        </div>`
  }),
  builderItem({
    id: "text",
    label: label().en("Text").de("Text").build(),
    icon: "i-tabler-letter-case",
    props: [
      ...commonProps,
      {
        id: "placeholder",
        type: "text",
        label: label().en("Placeholder").de("Platzhalter").build()
      },
      {
        id: "type",
        type: "text",
        label: label().en("Type").de("Typ").build()
      }
    ],
    data: {
      label: "",
      placeholder: "",
      required: false,
      name: "",
      type: "text",
      class: "grid p-2"
    },
    template: `<div class="{{ data.class }}">
            <label for="{{ data.name }}">
                {{ render data.label }}
            </label>
            <input type="{{ data.type }}" name="{{ data.name }}" {{#if data.required }}required{{/if}} placeholder="{{ render data.placeholder }}" />
        </div>`
  }),
  builderItem({
    id: "label",
    label: label().en("Label").de("Anzeige").build(),
    icon: "i-tabler-letter-case",
    props: [
      {
        id: "class",
        type: "text",
        label: label().en("CSS-Classes").de("CSS-Klasses").build()
      },
      {
        id: "text",
        type: "text",
        label: label().en("Text").de("Text").build()
      }
    ],
    data: {
      text: "text",
      class: "p-2"
    },
    template: `<div class="{{ data.class }}">
		{{render data.text }}
        </div>`
  }),
  builderItem({
    id: "checkbox",
    label: label().en("Checkbox").de("Kontrollk\xE4stchen").build(),
    icon: "i-tabler-checkbox",
    props: [...commonProps],
    data: {
      label: "",
      name: "",
      placeholder: "",
      required: false,
      class: "p-2"
    },
    template: `<div class="{{ data.class }}">
            <label for="{{ data.name }}">
                {{ render data.label }}
            </label>
            <input type="checkbox" name="{{ data.name }}" {{#if data.required }}required{{/if}} />
        </div>`
  }),
  builderItem({
    id: "textarea",
    label: label().en("Textarea").de("Textfeld").build(),
    icon: "i-tabler-text-plus",
    props: [...commonProps],
    data: {
      label: "",
      name: "",
      placeholder: "",
      required: false,
      class: "grid p-2"
    },
    template: `<div class="{{ data.class }}">
            <label for="{{ data.name }}">
                {{ render data.label }}
            </label>
            <textarea name="{{ data.name }}" {{#if data.required }}required{{/if}} placeholder="{{ render data.placeholder }}"></textarea>
        </div>`
  }),
  builderItem({
    id: "button",
    label: label().en("Button").de("Button").build(),
    icon: "i-tabler-square-arrow-down",
    props: [
      ...commonProps,
      {
        id: "disabled",
        type: "checkbox",
        label: label().en("Disabled").de("Deaktiviert").build()
      },
      {
        id: "id",
        type: "text",
        label: label().en("Id").de("Id").build()
      },
      {
        id: "type",
        type: "buttongroup",
        label: label().en("Button type").de("Buttontyp").build(),
        options: [
          {
            label: label().en("Submit").de("Abschicken").build(),
            value: "submit"
          },
          {
            label: label().en("Reset").de("Zur\xFCcksetzen").build(),
            value: "reset"
          },
          { label: label().en("Button").de("Button").build(), value: "button" }
        ]
      },
      {
        id: "onclick",
        type: "code",
        language: "js",
        label: label().en("onClick Function").de("onClick Funktion").build()
      }
    ],
    data: {
      label: "Submit",
      class: "",
      type: "submit",
      id: "submit-btn",
      onClick: "",
      disabled: false
    },
    template: `<button name="{{ data.name }}" id="{{data.id}}" type="{{ data.type }}" onclick="{{ data.onclick }}" class="{{ data.class }}" {{#if data.disabled}}disabled{{/if}}>
            {{ render data.label }}
        </button>`
  }),
  builderItem({
    id: "captcha",
    label: label().en("CAPTCHA").de("CAPTCHA").build(),
    icon: "i-tabler-robot-off",
    props: [
      {
        id: "classes",
        type: "text",
        label: label().en("CSS Klassen").de("CSS Klassen").build()
      },
      {
        id: "dataSitekey",
        type: "text",
        label: label().en("data-sitekey").de("data-sitekey").build()
      },
      {
        id: "callback",
        type: "text",
        label: label().en("data-callback").de("data-callback").build()
      },
      {
        id: "expiredCallback",
        type: "text",
        label: label().en("data-expired-callback").de("data-expired-callback").build()
      }
    ],
    data: {
      dataSitekey: "",
      classes: "grid p-2",
      callback: "",
      expiredCallback: ""
    },
    template: `
		<div class="{{ data.classes }}">
          <div class="g-recaptcha" data-sitekey="{{ data.dataSitekey }}" data-callback="{{ data.callback }}"  data-expired-callback="{{ data.expiredCallback }}"></div>
		</div>
		`
  })
];
var builderItemsWithLayout = [...formBuilderItems, builderItem({
  id: "layout",
  label: label().en("Layout").de("Layout").build(),
  icon: "i-tabler-layout-grid",
  props: [
    ...commonProps,
    {
      id: "children",
      type: "builder",
      label: label().en("Child elements").de("Kind Elemente").build(),
      items: formBuilderItems
    }
  ],
  data: {
    label: "",
    placeholder: "",
    required: false,
    name: "",
    class: "grid p-2",
    children: []
  },
  template: `<div class="{{ data.class }}">
 {{ render_builder data.children }}
        </div>`
})];
var formStructs_default = builderItemsWithLayout;

// settings/form.ts
function createFormPage(formHandler) {
  const dataHandler = formHandler.toActionHandler("GET");
  const upsertHandler = formHandler.toActionHandler("POST");
  return {
    type: "form",
    getDataHandler: {
      method: dataHandler.method,
      url: `${dataHandler.url}?id={{id}}`
    },
    label: label().en("Create Form").de("Formular erstellen").build(),
    form: {
      label: label().en("Create Form").de("Formular erstellen").build(),
      properties: [
        {
          id: "name",
          type: "text",
          label: label().en("Name of the form").de("Name des Formulars").build()
        },
        {
          id: "email",
          type: "text",
          label: label().en("Send submissions to this email").de("Formularergebnisse an diese Email schicken").build()
        },
        {
          label: label().en("Activate ReCaptcha").de("Aktivere ReCaptcha").build(),
          id: "recaptcha-active",
          type: "checkbox"
        },
        {
          label: label().en("Activate Cleverreach").de("Aktivere Cleverreach").build(),
          id: "cleverreach-active",
          type: "checkbox"
        },
        {
          label: label().en("Group Ids").de("Group Ids").build(),
          id: "cleverreach_group_id",
          type: "text"
        },
        {
          label: label().en("CSS Classes").de("CSS Klassen").build(),
          id: "cssClasses",
          type: "text"
        },
        {
          id: "builder",
          type: "builder",
          label: label().en("Build your Form").de("Bearbeite dein Formular").build(),
          items: formStructs_default,
          previewTemplate: `<form 
    action=""
    class="{{ properties.cssClasses }}"
    method="POST"
>
    {{ render_builder properties.builder }}
</form>`
        }
      ]
    },
    actions: [
      {
        primary: true,
        label: label().en("Save").de("Speichern").build(),
        handler: {
          method: upsertHandler.method,
          url: `${upsertHandler.url}?id={{id}}`,
          redirect: "/plugins/form-plugin/overview"
        }
      }
    ]
  };
}

// settings/cleverreach.ts
function cleverReachConfig(formHandler) {
  const dataHandler = formHandler.toActionHandler("GET");
  const upsertHandler = formHandler.toActionHandler("POST");
  return {
    type: "form",
    getDataHandler: {
      method: dataHandler.method,
      url: `${dataHandler.url}`
    },
    label: label().en("Cleverreach Settings").de("Cleverreach Einstellungen").build(),
    form: {
      label: label().en("Cleverreach Settings").de("Cleverreach Einstellungen").build(),
      properties: [
        {
          id: "client_id",
          type: "text",
          label: label().en("OAuth Client Id").de("OAuth Client Id").build()
        },
        {
          label: label().en("OAuth Cleverreach password").de("OAuth Cleverreach Passwort").build(),
          id: "login_password",
          type: "password"
        }
      ]
    },
    actions: [
      {
        primary: true,
        label: label().en("Save").de("Speichern").build(),
        handler: {
          method: upsertHandler.method,
          url: `${upsertHandler.url}`
        }
      }
    ]
  };
}

// settings/recaptcha.ts
function recaptchaConfig(formHandler) {
  const dataHandler = formHandler.toActionHandler("GET");
  const upsertHandler = formHandler.toActionHandler("POST");
  return {
    type: "form",
    getDataHandler: {
      method: dataHandler.method,
      url: `${dataHandler.url}`
    },
    label: label().en("ReCaptcha Settings").de("ReCaptcha Einstellungen").build(),
    form: {
      label: label().en("ReCaptcha Settings").de("ReCaptcha Einstellungen").build(),
      properties: [
        {
          label: label().en("Secret Key").de("Secret Key").build(),
          id: "secretKey",
          type: "password"
        }
      ]
    },
    actions: [
      {
        primary: true,
        label: label().en("Save").de("Speichern").build(),
        handler: {
          method: upsertHandler.method,
          url: `${upsertHandler.url}`
        }
      }
    ]
  };
}

// helper/cleverReach.ts
var CR_BASE_URL = "https://rest.cleverreach.com";
var CR_TOKEN_URL = `${CR_BASE_URL}/oauth/token.php`;
var CR_API_URL = `${CR_BASE_URL}/v3`;
var accessToken;
var updateAccessToken = async (client_id, client_secret) => {
  if (!client_id || !client_secret)
    return;
  const response = await fetch(CR_TOKEN_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded"
    },
    body: new URLSearchParams({
      client_id,
      client_secret,
      grant_type: "client_credentials"
    })
  });
  if (response.status !== 200)
    return;
  accessToken = (await response.json()).access_token;
};
var createReceiver = async (client_id, client_secret, groupId, email, firstName, lastName, salutation) => {
  if (!accessToken)
    await updateAccessToken(client_id, client_secret);
  const res = await fetch(`${CR_API_URL}/groups/${groupId}/receivers`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${accessToken}`
    },
    body: JSON.stringify({
      email,
      global_attributes: {
        firstname: firstName,
        lastname: lastName,
        anrede: salutation
      },
      source: ""
    })
  });
};

// index.js
var import_dayjs = __toESM(require_dayjs_min(), 1);
var form_plugin_default = definePlugin(async (ctx) => {
  function dataentryIsForm(dataentry) {
    return dataentry != null && typeof dataentry === "object" && "data" in dataentry && dataentry.data != null && typeof dataentry.data === "object" && "form" in dataentry.data;
  }
  function getMailContent(title, body) {
    return `
		<html lang="de">
		<head>
			<meta charset="utf-8"/>
			<style>
        		table, th, td {
            		padding: 5px 10px;
            		text-align: left;
        		}
    		</style>
		</head>
		<body>
			<b>${title}</b>
			<br/>
			<table> 
				<thead>
					<tr>
						<th>Feld</th>
						<th>Inhalt</th>
					</tr>
				</thead>
				<tbody>
				${Object.entries(body).map(([key, value]) => `
					<tr>
						<td>
							${key}
						</td>
						<td>
							${value}
						</td>
					</tr>`).join("")}
				</tbody>
			</table>
			<br/>
		</body>
		</html>`;
  }
  const cleverReachUpsertHandler = ctx.registerHandler(["POST", "GET"], "/form/cleverreach", async (req) => {
    const config = await ctx.storage.getAllWithFilter("clever_reach");
    if (req.method === "POST") {
      if (config[0]) {
        await ctx.storage.updateOne(config[0].id, {
          clever_reach: {
            ...req.body
          }
        });
      } else {
        await ctx.storage.createOne({
          clever_reach: {
            ...req.body
          }
        });
      }
      await ctx.hooks.callHook("pageCache.updateAll" /* PAGE_CACHE_UPDATE_ALL */);
      return new Response(null, {
        status: 200
      });
    }
    if (req.method === "GET") {
      if (!config[0])
        return new Response("Clever reach config not found", {
          status: 404
        });
      return config[0].data.clever_reach;
    }
    return new Response(`Method not allowed ${req.method}`, {
      status: 405
    });
  }, {
    allowedRoles: ["admin", "editor", "developer"]
  });
  const recaptchaUpsertHandler = ctx.registerHandler(["POST", "GET"], "/form/recaptcha", async (req) => {
    const config = await ctx.storage.getAllWithFilter("recaptcha");
    if (req.method === "POST") {
      if (config[0]) {
        await ctx.storage.updateOne(config[0].id, {
          recaptcha: {
            ...req.body
          }
        });
      } else {
        await ctx.storage.createOne({
          recaptcha: {
            ...req.body
          }
        });
      }
      await ctx.hooks.callHook("pageCache.updateAll" /* PAGE_CACHE_UPDATE_ALL */);
      return new Response(null, {
        status: 200
      });
    }
    if (req.method === "GET") {
      if (!config[0])
        return new Response("Recaptcha config not found", {
          status: 404
        });
      return config[0].data.recaptcha;
    }
    return new Response(`Method not allowed ${req.method}`, {
      status: 405
    });
  }, {
    allowedRoles: ["admin", "editor", "developer"]
  });
  const formCreateUpdateHandler = ctx.registerHandler(["POST", "GET"], "/form", async (req) => {
    if (req.method === "POST") {
      if (req.query.id) {
        await ctx.storage.updateOne(req.query.id, {
          form: {
            ...req.body
          }
        });
      } else {
        await ctx.storage.createOne({
          form: {
            ...req.body
          }
        });
      }
      await ctx.hooks.callHook("pageCache.updateAll" /* PAGE_CACHE_UPDATE_ALL */);
      return new Response(null, {
        status: 200
      });
    }
    if (req.method === "GET") {
      if (req.query.id) {
        const dbData = await ctx.storage.getOne(req.query.id);
        if (!dbData)
          return new Response("Form with this id not found", {
            status: 404
          });
        return dbData.data.form;
      }
      return new Response("FormID not given", {
        status: 400
      });
    }
    return new Response(`Method not allowed ${req.method}`, {
      status: 405
    });
  }, {
    allowedRoles: ["admin", "editor", "author", "developer"]
  });
  const formsHandler = ctx.registerHandler(["GET", "DELETE"], "/forms", async (req) => {
    if (req.method === "GET") {
      const dbData = await ctx.storage.getAllWithFilter("form");
      if (!dbData)
        return [];
      const allForms = dbData.map((item) => ({
        ...item.data.form,
        id: item.id,
        pluginId: item.pluginId
      }));
      return allForms;
    }
    if (req.method === "DELETE") {
      const res = await ctx.storage.deleteOne(req.query.id);
      if (!res)
        return new Response("form not found", {
          status: 404
        });
      if (res?.id) {
        return new Response("Deleted", { status: 200 });
      }
      return new Response(`Error on deleting record ${req.query.id}`, {
        status: 500
      });
    }
    return new Response(`Method not allowed ${req.method}`, {
      status: 405
    });
  }, {
    allowedRoles: ["admin", "developer", "author", "editor"]
  });
  const submitHandler = ctx.registerHandler(["POST"], "/submit/:formId", async (req) => {
    if (!req.params?.formId) {
      return new Response("Form ID is missing in the URL", {
        status: 400
      });
    }
    const form2 = await ctx.storage.getOne(req.params.formId);
    if (dataentryIsForm(form2)) {
      let recaptchaSucess = true;
      if (form2.data.form["recaptcha-active"]) {
        const recaptchaResponse = req.body["g-recaptcha-response"];
        if (!recaptchaResponse) {
          console.log("kein Recaptcha gefunden !");
        }
        try {
          const SECRET_KEY = await ctx.storage.getAllWithFilter("recaptcha");
          const response = await fetch("https://www.google.com/recaptcha/api/siteverify", {
            method: "POST",
            headers: {
              "Content-Type": "application/x-www-form-urlencoded"
            },
            body: `secret=${SECRET_KEY[0].data.recaptcha.secretKey}&response=${recaptchaResponse}`
          });
          const data = await response.json();
          if (!data.success) {
            recaptchaSucess = false;
            console.log("Recaptcha Fehlgeschlagen");
          }
        } catch (error) {
          console.error("Fehler bei der reCAPTCHA-\xDCberpr\xFCfung:", error);
        }
      }
      if (recaptchaSucess) {
        await ctx.storage.createOne({
          submission: {
            data: req.body,
            formId: req.params.formId
          }
        });
        if (form2.data.form.email) {
          const title = `[Neue Einsendung] ${form2.data.form.name} vom ${import_dayjs.default(new Date).format("DD.MM.YYYY")}`;
          ctx.sendMail(title, form2.data.form.email, getMailContent(title, req.body));
        }
        if (form2.data.form["cleverreach-active"] || req.body.subscribe_newsletter) {
          try {
            const config = await ctx.storage.getAllWithFilter("clever_reach");
            await createReceiver(config[0].data.clever_reach.client_id, config[0].data.clever_reach.login_password, form2.data.form.cleverreach_group_id, req.body.email ?? "", req.body.firstName ?? "", req.body.lastName ?? "", req.body.salutation ?? "");
          } catch (e) {
            console.log("error on subscribing newsletter", e);
          }
        }
      }
    } else {
      return new Response("Unknown FormID", {
        status: 404
      });
    }
    return new Response(null, {
      status: 302,
      headers: {
        location: "/"
      }
    });
  });
  const submissionsListHandler = ctx.registerHandler(["GET"], "/submission", async (req) => {
    const allForms = await ctx.storage.getAllWithFilter("form");
    const submissions2 = await ctx.storage.getAllWithFilter("submission");
    const allSubmissions = [];
    if (!submissions2 || !allForms) {
      console.error("No Form or Submissions Found!");
      return [];
    }
    for (const submission of submissions2) {
      const matchingForm = allForms.find((form2) => form2.id === Number(submission.data.submission.formId));
      if (matchingForm) {
        allSubmissions.push({
          formId: submission.data.submission.formId,
          id: submission.id,
          pluginId: submission.pluginId,
          name: matchingForm.data.form.name,
          recipient: matchingForm.data.form.email,
          date: import_dayjs.default(submission.createdAt).format("HH:mm, DD.MM.YYYY"),
          data: submission.data.submission
        });
      }
    }
    return allSubmissions.sort((a, b) => a.id > b.id ? -1 : 1);
  });
  const submissionsDeleteHandler = ctx.registerHandler(["DELETE"], "/submission/:submissionId", async (req) => {
    if (!req.params?.submissionId) {
      return new Response("Submission ID is missing in the URL", {
        status: 400
      });
    }
    const res = await ctx.storage.deleteOne(req.params.submissionId);
    if (res) {
      return new Response(null, { status: 200 });
    }
    return new Response("Error on Deleting Record", {
      status: 500
    });
  }, {
    allowedRoles: ["admin", "editor", "author", "developer"]
  });
  ctx.registerSettingsPage("/submissions", formSubmissionsPage(submissionsListHandler, submissionsDeleteHandler));
  ctx.registerSettingsPage("/overview", formOverviewPage(formsHandler));
  ctx.registerSettingsPage("/create", createFormPage(formCreateUpdateHandler));
  ctx.registerSettingsPage("/cleverreach", cleverReachConfig(cleverReachUpsertHandler));
  ctx.registerSettingsPage("/recaptcha", recaptchaConfig(recaptchaUpsertHandler));
  ctx.registerElementType({
    name: "Form-Picker",
    icon: "i-tabler-forms",
    category: "misc",
    template: `<form 
		id="{{ properties.selectForm.id }}"
    action="${submitHandler.toActionHandler("POST").url.replace(":formId", "{{ properties.selectForm.id }}")}"
    class="{{ relations.selectForm.cssClasses }}"
    method="POST"
>
    {{ render_builder relations.selectForm.builder }}
</form>`,
    style: "",
    script: "",
    form: {
      label: label().en("Form-Picker").de("Form-Picker").build(),
      properties: [
        {
          label: label().en("Select Form").de("Formular Ausw\xE4hlen").build(),
          id: "selectForm",
          type: "pluginData",
          pluginId: ctx.pluginId(),
          filterByKey: "form",
          labelKey: "form.name",
          valueKey: "form"
        },
        {
          label: label().en("CSS Classes").de("CSS Klassen").build(),
          id: "cssClasses",
          type: "text"
        }
      ]
    }
  });
});
export {
  form_plugin_default as default
};
