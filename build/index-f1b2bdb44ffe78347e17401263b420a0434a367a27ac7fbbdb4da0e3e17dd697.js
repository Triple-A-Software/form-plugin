var __create = Object.create;
var __defProp = Object.defineProperty;
var __getProtoOf = Object.getPrototypeOf;
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

// /home/ychop/Dokumente/form-plugin/node_modules/@cms-local/plugin-interface/index.ts
function definePlugin(def) {
  return def;
}

// index.js
var import_dayjs = __toESM(require_dayjs_min(), 1);

// settings/overview.ts
function formOverviewPage(actionHandler) {
  return {
    label: { de: "Formulare", en: "Forms" },
    type: "list",
    getDataHandler: actionHandler.toActionHandler("GET"),
    columns: [
      { key: "name", label: { de: "Name", en: "Name" } },
      { key: "email", label: { de: "Empf\xE4nger", en: "Recipient" } },
      { key: "actions", label: { de: "Aktionen", en: "Actions" } }
    ],
    actions: [
      {
        icon: "i-tabler-edit",
        to: "/form-plugin/create?id={{record.id}}"
      },
      {
        icon: "i-tabler-trash",
        handler: {
          url: "/api/plugins/form-plugin/forms?id={{record.id}}",
          method: "DELETE"
        }
      }
    ]
  };
}

// settings/submissions.ts
function formSubmissionsPage(submissionHandler) {
  return {
    label: { de: "Einsendungen", en: "Submissions" },
    type: "list",
    getDataHandler: submissionHandler.toActionHandler("GET"),
    columns: [
      { key: "id", label: { de: "ID", en: "ID" } },
      { key: "name", label: { de: "Formular", en: "Form" } },
      { key: "recipient", label: { de: "Empf\xE4nger", en: "Recipient" } },
      { key: "date", label: { de: "Gesendet am", en: "Recived at" } },
      { key: "actions", label: { de: "Aktionen", en: "Actions" } }
    ],
    actions: [
      {
        icon: "i-tabler-eye",
        showModal: true
      },
      {
        icon: "i-tabler-trash",
        handler: {
          url: "/api/plugins/form-plugin/submit/{{record.id}}?formId={{record.formId}}",
          method: "DELETE"
        }
      }
    ]
  };
}

// helper/formStructs.ts
var formBuilderItems = [
  {
    id: "text",
    label: {
      de: "text",
      en: "text"
    },
    icon: "i-tabler-letter-case",
    props: [
      {
        id: "label",
        type: "text",
        label: { de: "Label", en: "Label" }
      },
      {
        id: "placeholder",
        type: "text",
        label: { de: "Platzhalter", en: "Placeholder" }
      },
      {
        id: "required",
        type: "checkbox",
        label: { de: "Pflichtfeld", en: "Required" },
        defaultValue: false
      },
      {
        id: "class",
        type: "text",
        label: { de: "CSS Klassen", en: "CSS Classes" }
      }
    ],
    data: {
      label: "",
      placeholder: "",
      required: false,
      class: "grid p-2"
    }
  },
  {
    id: "checkbox",
    label: {
      de: "Kontrollk\xE4stchen",
      en: "Checkbox"
    },
    icon: "i-tabler-checkbox",
    props: [
      {
        id: "label",
        type: "text",
        label: { de: "Label", en: "Label" }
      },
      {
        id: "required",
        type: "checkbox",
        label: { de: "Pflichtfeld", en: "Required" },
        defaultValue: false
      },
      {
        id: "class",
        type: "text",
        label: { de: "CSS Klassen", en: "CSS Classes" }
      }
    ],
    data: {
      label: "",
      placeholder: "",
      required: false,
      class: "p-2"
    }
  },
  {
    id: "textarea",
    label: {
      de: "Textfeld",
      en: "Textarea"
    },
    props: [
      {
        id: "label",
        type: "text",
        label: { de: "Label", en: "Label" }
      },
      {
        id: "required",
        type: "checkbox",
        label: { de: "Pflichtfeld", en: "Required" },
        defaultValue: false
      },
      {
        id: "class",
        type: "text",
        label: { de: "CSS Klassen", en: "CSS Classes" }
      }
    ],
    data: {
      label: "",
      placeholder: "",
      required: false,
      class: "grid p-2"
    }
  },
  {
    id: "select",
    label: {
      de: "Auswahl",
      en: "Select"
    },
    icon: "i-tabler-select",
    props: [
      {
        id: "label",
        type: "text",
        label: { de: "Label", en: "Label" }
      },
      {
        id: "required",
        type: "checkbox",
        label: { de: "Pflichtfeld", en: "Required" },
        defaultValue: false
      },
      {
        id: "class",
        type: "text",
        label: { de: "CSS Klassen", en: "CSS Classes" }
      }
    ],
    data: {
      label: "",
      placeholder: "",
      required: false,
      class: "grid p-2",
      options: []
    }
  }
];

// settings/form.ts
function createFormPage(actionHandler) {
  return {
    type: "form",
    getDataHandler: {
      method: "GET",
      url: "/api/plugins/form-plugin/form?id={{id}}"
    },
    label: {
      de: "Formular erstellen",
      en: "Create Form"
    },
    form: {
      label: {
        de: "Formular erstellen",
        en: "Create Form"
      },
      properties: [
        {
          id: "name",
          type: "text",
          label: {
            de: "Name des Formulars",
            en: "Name of the form"
          }
        },
        {
          id: "email",
          type: "text",
          label: {
            de: "Empf\xE4nger E-Mail",
            en: "Recipient"
          }
        },
        {
          id: "builder",
          type: "builder",
          label: {
            de: "Baue dein neues Formular",
            en: "Build your new Form"
          },
          items: formBuilderItems
        }
      ]
    },
    actions: [
      {
        primary: true,
        label: { de: "Speichern", en: "Save" },
        handler: {
          method: "POST",
          url: "/api/plugins/form-plugin/form?id={{id}}"
        }
      }
    ]
  };
}

// helper/utils.ts
function textInput(data, index) {
  return `
        <div class="${data.class}">
            <label htmlFor="${data.label + index}">
			${data.label}
			</label>
            <input type="text"
				   name="${data.label + index}"
				   required="${data.required}" 
				   placeholder="${data.placeholder}" />
        </div>`;
}
function checkboxInput(data, index) {
  return `
        <div class="${data.class}">
            <label htmlFor="${data.label + index}">
			${data.label}
			</label>
            <input type="checkbox"
				   name="${data.label + index}"
				   required="${data.required}" 
				   />
        </div>`;
}
function textAreaInput(data, index) {
  return `
        <div class="${data.class}">
            <label htmlFor="${data.label + index}">
			${data.label}
			</label>
            <textarea 
				name="${data.label + index}"
				required="${data.required}" 
				placeholder="${data.placeholder}" >
			</textarea>
        </div>`;
}

// index.js
var form_plugin_default = definePlugin(async (ctx) => {
  const editHandler = ctx.registerHandler(["POST", "GET"], "/form", async (req) => {
    if (req.method === "POST") {
      if (req.query.id) {
        console.log("UPDATE", req.body);
        await ctx.storage.updateOne(req.query.id, {
          form: {
            ...req.body
          }
        });
      } else {
        console.log("CREATE", req.body);
        await ctx.storage.createOne({
          form: {
            ...req.body
          }
        });
      }
      return new Response(null, {
        status: 200
      });
    }
    if (req.method === "GET") {
      if (req.query.id) {
        const dbData = await ctx.storage.getOne(req.query.id);
        return dbData.data.form;
      }
      return {};
    }
    throw Error(`Method not allowed ${req.method}`);
  });
  const formHandler = ctx.registerHandler(["GET", "DELETE"], "/forms", async (req) => {
    if (req.method === "GET") {
      const dbData = await ctx.storage.getAll();
      const allForms = dbData.map((item) => ({
        ...item.data.form,
        id: item.id,
        pluginId: item.pluginId
      }));
      return allForms;
    }
    if (req.method === "DELETE") {
      const res = await ctx.storage.deleteOne(req.query.id);
      if (res.ok) {
        return new Response(null, { status: 200 });
      }
      throw Error("Error on Deleting Record", req.query.id);
    }
    throw Error(`Method not allowed ${req.method}`);
  });
  const submitHandler = ctx.registerHandler(["POST", "GET", "DELETE"], "/submit/:formId", async (req) => {
    if (req.method === "POST") {
      if (!req.params.formId) {
        return new Response("Form ID is missing in the URL", {
          status: 400
        });
      }
      await ctx.storage.createOne({
        submission: {
          ...req.body
        }
      }, req.params.formId);
      const form2 = await ctx.storage.getOne(req.params.formId);
      ctx.sendMail(`[Neue Einsendung] ${req.params.formId} vom ${import_dayjs.default(new Date).format("DD.MM.YYYY")}`, form2.data.email, `<div style="padding:1rem;">
                    <table style="width:100%;border: solid 1px;padding:1rem">
                        <tr style="border-bottom:solid 1px;text-align:center">
                            <th>Feld</th>
                            <th></th>
                        </tr>
						${Object.entries(req.body).map(([key, value]) => `
						<tr>
                            <td style="border-right:solid 1px;padding:1rem">
                                ${String(key).slice(0, -1)}
                            </td>
                            <td style="padding:1rem">
                                ${value}
                            </td>
                        </tr>
						`)}
                    </table>
                </div>`);
      return new Response(null, {
        status: 302,
        headers: {
          Location: "/"
        }
      });
    }
    if (req.method === "DELETE") {
      const res = await ctx.storage.deleteOne(req.params.formId, req.query.formId);
      if (res) {
        return new Response(null, { status: 200 });
      }
      throw Error("Error on Deleting Record");
    }
    if (req.method === "GET") {
      const allForms = await ctx.storage.getAll();
      const allSubmissions = [];
      for (let i = 0;i < allForms.length; i++) {
        const submission = await ctx.storage.getAll(allForms[i].id);
        for (let k = 0;k < submission.length; k++) {
          allSubmissions.push({
            formId: allForms[i].id,
            id: submission[k].id,
            pluginId: submission[k].pluginId,
            name: allForms[i].data.form.name,
            recipient: allForms[i].data.form.email,
            date: import_dayjs.default(submission[k].createdAt).format("HH:mm, DD.MM.YYYY"),
            data: submission[k].data.submission
          });
        }
      }
      return allSubmissions;
    }
  });
  ctx.registerWidget("formSubmissionsWidget", formSubmissionsPage(submitHandler));
  ctx.registerSettingsPage("/submissions", formSubmissionsPage(submitHandler));
  ctx.registerSettingsPage("/overview", formOverviewPage(formHandler));
  ctx.registerSettingsPage("/create", createFormPage(editHandler));
  ctx.onRewrite(async (rewriter) => {
    const forms = await ctx.storage.getAll();
    if (!forms)
      return;
    for (const form2 of forms) {
      rewriter.on(`form[id='${form2.id}']`, {
        element(el) {
          el.setInnerContent(`${form2.data.form.builder.map((input, index) => {
            const { id, data } = input;
            switch (id) {
              case "text":
                return textInput(data, index);
              case "textarea":
                return textAreaInput(data, index);
              case "checkbox":
                return checkboxInput(data, index);
              default:
                return "";
            }
          }).join("")}<button type="submit">Submit</button>`, { html: true });
        }
      });
    }
  });
  ctx.registerElementType({
    name: "Form-Picker",
    icon: "i-tabler-forms",
    template: "<form id='{{ data.select-form }}' action='/api/plugins/form-plugin/submit/{{ data.select-form }}' class='{{data.css-classes}}' method='POST'></form>",
    style: "",
    script: "",
    form: {
      label: {
        de: "Form-Picker",
        en: "Form-Picker"
      },
      properties: [
        {
          label: {
            de: "Formular Ausw\xE4hlen",
            end: "Select Form"
          },
          id: "select-form",
          type: "select",
          options: "/api/plugins/form-plugin/forms"
        },
        {
          label: {
            de: "CSS Klassen",
            end: "CSS Classes"
          },
          id: "css-classes",
          type: "text"
        }
      ]
    }
  });
});
export {
  form_plugin_default as default
};
