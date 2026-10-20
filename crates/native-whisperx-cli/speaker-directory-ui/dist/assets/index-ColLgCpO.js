var e = Object.create,
  t = Object.defineProperty,
  n = Object.getOwnPropertyDescriptor,
  r = Object.getOwnPropertyNames,
  i = Object.getPrototypeOf,
  a = Object.prototype.hasOwnProperty,
  o = (e, t) => () => (
    t || (e((t = { exports: {} }).exports, t, t.exports), (e = null)),
    t.exports
  ),
  s = (e, i, o, s) => {
    if ((i && typeof i == `object`) || typeof i == `function`)
      for (var c = r(i), l = 0, u = c.length, d; l < u; l++)
        ((d = c[l]),
          !a.call(e, d) &&
            d !== o &&
            t(e, d, {
              get: ((e) => i[e]).bind(null, d),
              enumerable: !(s = n(i, d)) || s.enumerable,
            }));
    return e;
  },
  c = (n, r, o) => (
    (o = n == null ? {} : e(i(n))),
    s(
      r || !n || !n.__esModule || !a.call(n, `default`)
        ? t(o, `default`, { value: n, enumerable: !0 })
        : o,
      n,
    )
  );
(function () {
  let e = document.createElement(`link`).relList;
  if (e && e.supports && e.supports(`modulepreload`)) return;
  for (let e of document.querySelectorAll(`link[rel="modulepreload"]`)) n(e);
  new MutationObserver((e) => {
    for (let t of e)
      if (t.type === `childList`)
        for (let e of t.addedNodes) e.tagName === `LINK` && e.rel === `modulepreload` && n(e);
  }).observe(document, { childList: !0, subtree: !0 });
  function t(e) {
    let t = {};
    return (
      e.integrity && (t.integrity = e.integrity),
      e.referrerPolicy && (t.referrerPolicy = e.referrerPolicy),
      (t.credentials =
        e.crossOrigin === `use-credentials`
          ? `include`
          : e.crossOrigin === `anonymous`
            ? `omit`
            : `same-origin`),
      t
    );
  }
  function n(e) {
    if (e.ep) return;
    e.ep = !0;
    let n = t(e);
    fetch(e.href, n);
  }
})();
var l = o((e) => {
    var t = Symbol.for(`react.transitional.element`),
      n = Symbol.for(`react.portal`),
      r = Symbol.for(`react.fragment`),
      i = Symbol.for(`react.strict_mode`),
      a = Symbol.for(`react.profiler`),
      o = Symbol.for(`react.consumer`),
      s = Symbol.for(`react.context`),
      c = Symbol.for(`react.forward_ref`),
      l = Symbol.for(`react.suspense`),
      u = Symbol.for(`react.memo`),
      d = Symbol.for(`react.lazy`),
      f = Symbol.for(`react.activity`),
      p = Symbol.for(`react.view_transition`),
      m = Symbol.iterator;
    function h(e) {
      return typeof e != `object` || !e
        ? null
        : ((e = (m && e[m]) || e[`@@iterator`]), typeof e == `function` ? e : null);
    }
    var g = {
        isMounted: function () {
          return !1;
        },
        enqueueForceUpdate: function () {},
        enqueueReplaceState: function () {},
        enqueueSetState: function () {},
      },
      _ = Object.assign,
      v = {};
    function y(e, t, n) {
      ((this.props = e), (this.context = t), (this.refs = v), (this.updater = n || g));
    }
    ((y.prototype.isReactComponent = {}),
      (y.prototype.setState = function (e, t) {
        if (typeof e != `object` && typeof e != `function` && e != null)
          throw Error(
            `takes an object of state variables to update or a function which returns an object of state variables.`,
          );
        this.updater.enqueueSetState(this, e, t, `setState`);
      }),
      (y.prototype.forceUpdate = function (e) {
        this.updater.enqueueForceUpdate(this, e, `forceUpdate`);
      }));
    function b() {}
    b.prototype = y.prototype;
    function x(e, t, n) {
      ((this.props = e), (this.context = t), (this.refs = v), (this.updater = n || g));
    }
    var S = (x.prototype = new b());
    ((S.constructor = x), _(S, y.prototype), (S.isPureReactComponent = !0));
    var C = Array.isArray;
    function w() {}
    var T = { H: null, A: null, T: null, S: null },
      E = Object.prototype.hasOwnProperty;
    function D(e, n, r) {
      var i = r.ref;
      return { $$typeof: t, type: e, key: n, ref: i === void 0 ? null : i, props: r };
    }
    function O(e, t) {
      return D(e.type, t, e.props);
    }
    function ee(e) {
      return typeof e == `object` && !!e && e.$$typeof === t;
    }
    function te(e) {
      var t = { "=": `=0`, ":": `=2` };
      return (
        `$` +
        e.replace(/[=:]/g, function (e) {
          return t[e];
        })
      );
    }
    var k = /\/+/g;
    function A(e, t) {
      return typeof e == `object` && e && e.key != null ? te(`` + e.key) : t.toString(36);
    }
    function j(e) {
      switch (e.status) {
        case `fulfilled`:
          return e.value;
        case `rejected`:
          throw e.reason;
        default:
          switch (
            (typeof e.status == `string`
              ? e.then(w, w)
              : ((e.status = `pending`),
                e.then(
                  function (t) {
                    e.status === `pending` && ((e.status = `fulfilled`), (e.value = t));
                  },
                  function (t) {
                    e.status === `pending` && ((e.status = `rejected`), (e.reason = t));
                  },
                )),
            e.status)
          ) {
            case `fulfilled`:
              return e.value;
            case `rejected`:
              throw e.reason;
          }
      }
      throw e;
    }
    function ne(e, r, i, a, o) {
      var s = typeof e;
      (s === `undefined` || s === `boolean`) && (e = null);
      var c = !1;
      if (e === null) c = !0;
      else
        switch (s) {
          case `bigint`:
          case `string`:
          case `number`:
            c = !0;
            break;
          case `object`:
            switch (e.$$typeof) {
              case t:
              case n:
                c = !0;
                break;
              case d:
                return ((c = e._init), ne(c(e._payload), r, i, a, o));
            }
        }
      if (c)
        return (
          (o = o(e)),
          (c = a === `` ? `.` + A(e, 0) : a),
          C(o)
            ? ((i = ``),
              c != null && (i = c.replace(k, `$&/`) + `/`),
              ne(o, r, i, ``, function (e) {
                return e;
              }))
            : o != null &&
              (ee(o) &&
                (o = O(
                  o,
                  i +
                    (o.key == null || (e && e.key === o.key)
                      ? ``
                      : (`` + o.key).replace(k, `$&/`) + `/`) +
                    c,
                )),
              r.push(o)),
          1
        );
      c = 0;
      var l = a === `` ? `.` : a + `:`;
      if (C(e))
        for (var u = 0; u < e.length; u++)
          ((a = e[u]), (s = l + A(a, u)), (c += ne(a, r, i, s, o)));
      else if (((u = h(e)), typeof u == `function`))
        for (e = u.call(e), u = 0; !(a = e.next()).done;)
          ((a = a.value), (s = l + A(a, u++)), (c += ne(a, r, i, s, o)));
      else if (s === `object`) {
        if (typeof e.then == `function`) return ne(j(e), r, i, a, o);
        throw (
          (r = String(e)),
          Error(
            `Objects are not valid as a React child (found: ` +
              (r === `[object Object]`
                ? `object with keys {` + Object.keys(e).join(`, `) + `}`
                : r) +
              `). If you meant to render a collection of children, use an array instead.`,
          )
        );
      }
      return c;
    }
    function M(e, t, n) {
      if (e == null) return e;
      var r = [],
        i = 0;
      return (
        ne(e, r, ``, ``, function (e) {
          return t.call(n, e, i++);
        }),
        r
      );
    }
    function N(e) {
      if (e._status === -1) {
        var t = e._result,
          n = t();
        (n.then(
          function (t) {
            (e._status === 0 || e._status === -1) &&
              ((e._status = 1),
              (e._result = t),
              n.status === void 0 && ((n.status = `fulfilled`), (n.value = t)));
          },
          function (t) {
            (e._status === 0 || e._status === -1) &&
              ((e._status = 2),
              (e._result = t),
              n.status === void 0 && ((n.status = `rejected`), (n.reason = t)));
          },
        ),
          e._status === -1 && ((e._status = 0), (e._result = n)));
      }
      if (e._status === 1) return e._result.default;
      throw e._result;
    }
    var P =
      typeof reportError == `function`
        ? reportError
        : function (e) {
            if (typeof window == `object` && typeof window.ErrorEvent == `function`) {
              var t = new window.ErrorEvent(`error`, {
                bubbles: !0,
                cancelable: !0,
                message:
                  typeof e == `object` && e && typeof e.message == `string`
                    ? String(e.message)
                    : String(e),
                error: e,
              });
              if (!window.dispatchEvent(t)) return;
            } else if (typeof process == `object` && typeof process.emit == `function`) {
              process.emit(`uncaughtException`, e);
              return;
            }
            console.error(e);
          };
    function re(e) {
      var t = T.T,
        n = {};
      ((n.types = t === null ? null : t.types), (T.T = n));
      try {
        var r = e(),
          i = T.S;
        (i !== null && i(n, r),
          typeof r == `object` && r && typeof r.then == `function` && r.then(w, P));
      } catch (e) {
        P(e);
      } finally {
        (t !== null && n.types !== null && (t.types = n.types), (T.T = t));
      }
    }
    function F(e) {
      var t = T.T;
      if (t !== null) {
        var n = t.types;
        n === null ? (t.types = [e]) : n.indexOf(e) === -1 && n.push(e);
      } else re(F.bind(null, e));
    }
    var I = {
      map: M,
      forEach: function (e, t, n) {
        M(
          e,
          function () {
            t.apply(this, arguments);
          },
          n,
        );
      },
      count: function (e) {
        var t = 0;
        return (
          M(e, function () {
            t++;
          }),
          t
        );
      },
      toArray: function (e) {
        return (
          M(e, function (e) {
            return e;
          }) || []
        );
      },
      only: function (e) {
        if (!ee(e))
          throw Error(`React.Children.only expected to receive a single React element child.`);
        return e;
      },
    };
    ((e.Activity = f),
      (e.Children = I),
      (e.Component = y),
      (e.Fragment = r),
      (e.Profiler = a),
      (e.PureComponent = x),
      (e.StrictMode = i),
      (e.Suspense = l),
      (e.ViewTransition = p),
      (e.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE = T),
      (e.__COMPILER_RUNTIME = {
        __proto__: null,
        c: function (e) {
          return T.H.useMemoCache(e);
        },
      }),
      (e.addTransitionType = F),
      (e.cache = function (e) {
        return function () {
          return e.apply(null, arguments);
        };
      }),
      (e.cacheSignal = function () {
        return null;
      }),
      (e.cloneElement = function (e, t, n) {
        if (e == null)
          throw Error(`The argument must be a React element, but you passed ` + e + `.`);
        var r = _({}, e.props),
          i = e.key;
        if (t != null)
          for (a in (t.key !== void 0 && (i = `` + t.key), t))
            !E.call(t, a) ||
              a === `key` ||
              a === `__self` ||
              a === `__source` ||
              (a === `ref` && t.ref === void 0) ||
              (r[a] = t[a]);
        var a = arguments.length - 2;
        if (a === 1) r.children = n;
        else if (1 < a) {
          for (var o = Array(a), s = 0; s < a; s++) o[s] = arguments[s + 2];
          r.children = o;
        }
        return D(e.type, i, r);
      }),
      (e.createContext = function (e) {
        return (
          (e = {
            $$typeof: s,
            _currentValue: e,
            _currentValue2: e,
            _threadCount: 0,
            Provider: null,
            Consumer: null,
          }),
          (e.Provider = e),
          (e.Consumer = { $$typeof: o, _context: e }),
          e
        );
      }),
      (e.createElement = function (e, t, n) {
        var r,
          i = {},
          a = null;
        if (t != null)
          for (r in (t.key !== void 0 && (a = `` + t.key), t))
            E.call(t, r) && r !== `key` && r !== `__self` && r !== `__source` && (i[r] = t[r]);
        var o = arguments.length - 2;
        if (o === 1) i.children = n;
        else if (1 < o) {
          for (var s = Array(o), c = 0; c < o; c++) s[c] = arguments[c + 2];
          i.children = s;
        }
        if (e && e.defaultProps)
          for (r in ((o = e.defaultProps), o)) i[r] === void 0 && (i[r] = o[r]);
        return D(e, a, i);
      }),
      (e.createRef = function () {
        return { current: null };
      }),
      (e.forwardRef = function (e) {
        return { $$typeof: c, render: e };
      }),
      (e.isValidElement = ee),
      (e.lazy = function (e) {
        return { $$typeof: d, _payload: { _status: -1, _result: e }, _init: N };
      }),
      (e.memo = function (e, t) {
        return { $$typeof: u, type: e, compare: t === void 0 ? null : t };
      }),
      (e.startTransition = re),
      (e.unstable_useCacheRefresh = function () {
        return T.H.useCacheRefresh();
      }),
      (e.use = function (e) {
        return T.H.use(e);
      }),
      (e.useActionState = function (e, t, n) {
        return T.H.useActionState(e, t, n);
      }),
      (e.useCallback = function (e, t) {
        return T.H.useCallback(e, t);
      }),
      (e.useContext = function (e) {
        return T.H.useContext(e);
      }),
      (e.useDebugValue = function () {}),
      (e.useDeferredValue = function (e, t) {
        return T.H.useDeferredValue(e, t);
      }),
      (e.useEffect = function (e, t) {
        return T.H.useEffect(e, t);
      }),
      (e.useEffectEvent = function (e) {
        return T.H.useEffectEvent(e);
      }),
      (e.useId = function () {
        return T.H.useId();
      }),
      (e.useImperativeHandle = function (e, t, n) {
        return T.H.useImperativeHandle(e, t, n);
      }),
      (e.useInsertionEffect = function (e, t) {
        return T.H.useInsertionEffect(e, t);
      }),
      (e.useLayoutEffect = function (e, t) {
        return T.H.useLayoutEffect(e, t);
      }),
      (e.useMemo = function (e, t) {
        return T.H.useMemo(e, t);
      }),
      (e.useOptimistic = function (e, t) {
        return T.H.useOptimistic(e, t);
      }),
      (e.useReducer = function (e, t, n) {
        return T.H.useReducer(e, t, n);
      }),
      (e.useRef = function (e) {
        return T.H.useRef(e);
      }),
      (e.useState = function (e) {
        return T.H.useState(e);
      }),
      (e.useSyncExternalStore = function (e, t, n) {
        return T.H.useSyncExternalStore(e, t, n);
      }),
      (e.useTransition = function () {
        return T.H.useTransition();
      }),
      (e.version = `19.3.1`));
  }),
  u = o((e, t) => {
    t.exports = l();
  }),
  d = o((e) => {
    function t(e, t) {
      var n = e.length;
      e.push(t);
      a: for (; 0 < n;) {
        var r = (n - 1) >>> 1,
          a = e[r];
        if (0 < i(a, t)) ((e[r] = t), (e[n] = a), (n = r));
        else break a;
      }
    }
    function n(e) {
      return e.length === 0 ? null : e[0];
    }
    function r(e) {
      if (e.length === 0) return null;
      var t = e[0],
        n = e.pop();
      if (n !== t) {
        e[0] = n;
        a: for (var r = 0, a = e.length, o = a >>> 1; r < o;) {
          var s = 2 * (r + 1) - 1,
            c = e[s],
            l = s + 1,
            u = e[l];
          if (0 > i(c, n))
            l < a && 0 > i(u, c)
              ? ((e[r] = u), (e[l] = n), (r = l))
              : ((e[r] = c), (e[s] = n), (r = s));
          else if (l < a && 0 > i(u, n)) ((e[r] = u), (e[l] = n), (r = l));
          else break a;
        }
      }
      return t;
    }
    function i(e, t) {
      var n = e.sortIndex - t.sortIndex;
      return n === 0 ? e.id - t.id : n;
    }
    if (
      ((e.unstable_now = void 0),
      typeof performance == `object` && typeof performance.now == `function`)
    ) {
      var a = performance;
      e.unstable_now = function () {
        return a.now();
      };
    } else {
      var o = Date,
        s = o.now();
      e.unstable_now = function () {
        return o.now() - s;
      };
    }
    var c = [],
      l = [],
      u = 1,
      d = null,
      f = 3,
      p = !1,
      m = !1,
      h = !1,
      g = !1,
      _ = typeof setTimeout == `function` ? setTimeout : null,
      v = typeof clearTimeout == `function` ? clearTimeout : null,
      y = typeof setImmediate < `u` ? setImmediate : null;
    function b(e) {
      for (var i = n(l); i !== null;) {
        if (i.callback === null) r(l);
        else if (i.startTime <= e) (r(l), (i.sortIndex = i.expirationTime), t(c, i));
        else break;
        i = n(l);
      }
    }
    function x(e) {
      if (((h = !1), b(e), !m)) {
        if (n(c) !== null) ((m = !0), S || ((S = !0), O()));
        else {
          var t = n(l);
          t !== null && k(x, t.startTime - e);
        }
      }
    }
    var S = !1,
      C = -1,
      w = 5,
      T = -1;
    function E() {
      return g ? !0 : !(e.unstable_now() - T < w);
    }
    function D() {
      if (((g = !1), S)) {
        var t = e.unstable_now();
        T = t;
        var i = !0;
        try {
          a: {
            ((m = !1), h && ((h = !1), v(C), (C = -1)), (p = !0));
            var a = f;
            try {
              b: {
                for (b(t), d = n(c); d !== null && !(d.expirationTime > t && E());) {
                  var o = d.callback;
                  if (typeof o == `function`) {
                    ((d.callback = null), (f = d.priorityLevel));
                    var s = o(d.expirationTime <= t);
                    if (((t = e.unstable_now()), typeof s == `function`)) {
                      ((d.callback = s), b(t), (i = !0));
                      break b;
                    }
                    (d === n(c) && r(c), b(t));
                  } else r(c);
                  d = n(c);
                }
                if (d !== null) i = !0;
                else {
                  var u = n(l);
                  (u !== null && k(x, u.startTime - t), (i = !1));
                }
              }
              break a;
            } finally {
              ((d = null), (f = a), (p = !1));
            }
            i = void 0;
          }
        } finally {
          i ? O() : (S = !1);
        }
      }
    }
    var O;
    if (typeof y == `function`)
      O = function () {
        y(D);
      };
    else if (typeof MessageChannel < `u`) {
      var ee = new MessageChannel(),
        te = ee.port2;
      ((ee.port1.onmessage = D),
        (O = function () {
          te.postMessage(null);
        }));
    } else
      O = function () {
        _(D, 0);
      };
    function k(t, n) {
      C = _(function () {
        t(e.unstable_now());
      }, n);
    }
    ((e.unstable_IdlePriority = 5),
      (e.unstable_ImmediatePriority = 1),
      (e.unstable_LowPriority = 4),
      (e.unstable_NormalPriority = 3),
      (e.unstable_Profiling = null),
      (e.unstable_UserBlockingPriority = 2),
      (e.unstable_cancelCallback = function (e) {
        e.callback = null;
      }),
      (e.unstable_forceFrameRate = function (e) {
        0 > e || 125 < e
          ? console.error(
              `forceFrameRate takes a positive int between 0 and 125, forcing frame rates higher than 125 fps is not supported`,
            )
          : (w = 0 < e ? Math.floor(1e3 / e) : 5);
      }),
      (e.unstable_getCurrentPriorityLevel = function () {
        return f;
      }),
      (e.unstable_next = function (e) {
        switch (f) {
          case 1:
          case 2:
          case 3:
            var t = 3;
            break;
          default:
            t = f;
        }
        var n = f;
        f = t;
        try {
          return e();
        } finally {
          f = n;
        }
      }),
      (e.unstable_requestPaint = function () {
        g = !0;
      }),
      (e.unstable_runWithPriority = function (e, t) {
        switch (e) {
          case 1:
          case 2:
          case 3:
          case 4:
          case 5:
            break;
          default:
            e = 3;
        }
        var n = f;
        f = e;
        try {
          return t();
        } finally {
          f = n;
        }
      }),
      (e.unstable_scheduleCallback = function (r, i, a) {
        var o = e.unstable_now();
        switch (
          (typeof a == `object` && a
            ? ((a = a.delay), (a = typeof a == `number` && 0 < a ? o + a : o))
            : (a = o),
          r)
        ) {
          case 1:
            var s = -1;
            break;
          case 2:
            s = 250;
            break;
          case 5:
            s = 1073741823;
            break;
          case 4:
            s = 1e4;
            break;
          default:
            s = 5e3;
        }
        return (
          (s = a + s),
          (r = {
            id: u++,
            callback: i,
            priorityLevel: r,
            startTime: a,
            expirationTime: s,
            sortIndex: -1,
          }),
          a > o
            ? ((r.sortIndex = a),
              t(l, r),
              n(c) === null && r === n(l) && (h ? (v(C), (C = -1)) : (h = !0), k(x, a - o)))
            : ((r.sortIndex = s), t(c, r), m || p || ((m = !0), S || ((S = !0), O()))),
          r
        );
      }),
      (e.unstable_shouldYield = E),
      (e.unstable_wrapCallback = function (e) {
        var t = f;
        return function () {
          var n = f;
          f = t;
          try {
            return e.apply(this, arguments);
          } finally {
            f = n;
          }
        };
      }));
  }),
  f = o((e, t) => {
    t.exports = d();
  }),
  p = o((e) => {
    var t = u();
    function n(e) {
      var t = `https://react.dev/errors/` + e;
      if (1 < arguments.length) {
        t += `?args[]=` + encodeURIComponent(arguments[1]);
        for (var n = 2; n < arguments.length; n++)
          t += `&args[]=` + encodeURIComponent(arguments[n]);
      }
      return (
        `Minified React error #` +
        e +
        `; visit ` +
        t +
        ` for the full message or use the non-minified dev environment for full errors and additional helpful warnings.`
      );
    }
    function r() {}
    var i = {
        d: {
          f: r,
          r: function () {
            throw Error(n(522));
          },
          D: r,
          C: r,
          L: r,
          m: r,
          X: r,
          S: r,
          M: r,
        },
        p: 0,
        findDOMNode: null,
      },
      a = Symbol.for(`react.portal`),
      o = Symbol.for(`react.recoverable`),
      s = Symbol.for(`react.optimistic_key`);
    function c(e, t, n) {
      var r = 3 < arguments.length && arguments[3] !== void 0 ? arguments[3] : null;
      return {
        $$typeof: a,
        key: r == null ? null : r === s ? s : `` + r,
        children: e,
        containerInfo: t,
        implementation: n,
      };
    }
    var l = t.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE;
    function d(e, t) {
      if (e === `font`) return ``;
      if (typeof t == `string`) return t === `use-credentials` ? t : ``;
    }
    ((e.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE = i),
      (e.browser = function (e) {
        return { $$typeof: o, _reason: e };
      }),
      (e.createPortal = function (e, t) {
        var r = 2 < arguments.length && arguments[2] !== void 0 ? arguments[2] : null;
        if (!t || (t.nodeType !== 1 && t.nodeType !== 9 && t.nodeType !== 11)) throw Error(n(299));
        return c(e, t, null, r);
      }),
      (e.flushSync = function (e) {
        var t = l.T,
          n = i.p;
        try {
          if (((l.T = null), (i.p = 2), e)) return e();
        } finally {
          ((l.T = t), (i.p = n), i.d.f());
        }
      }),
      (e.preconnect = function (e, t) {
        typeof e == `string` &&
          (t
            ? ((t = t.crossOrigin),
              (t = typeof t == `string` ? (t === `use-credentials` ? t : ``) : void 0))
            : (t = null),
          i.d.C(e, t));
      }),
      (e.prefetchDNS = function (e) {
        typeof e == `string` && i.d.D(e);
      }),
      (e.preinit = function (e, t) {
        if (typeof e == `string` && t && typeof t.as == `string`) {
          var n = t.as,
            r = d(n, t.crossOrigin),
            a = typeof t.integrity == `string` ? t.integrity : void 0,
            o = typeof t.fetchPriority == `string` ? t.fetchPriority : void 0;
          n === `style`
            ? i.d.S(e, typeof t.precedence == `string` ? t.precedence : void 0, {
                crossOrigin: r,
                integrity: a,
                fetchPriority: o,
              })
            : n === `script` &&
              i.d.X(e, {
                crossOrigin: r,
                integrity: a,
                fetchPriority: o,
                nonce: typeof t.nonce == `string` ? t.nonce : void 0,
              });
        }
      }),
      (e.preinitModule = function (e, t) {
        if (typeof e == `string`) {
          if (typeof t == `object` && t) {
            if (t.as == null || t.as === `script`) {
              var n = d(t.as, t.crossOrigin);
              i.d.M(e, {
                crossOrigin: n,
                integrity: typeof t.integrity == `string` ? t.integrity : void 0,
                nonce: typeof t.nonce == `string` ? t.nonce : void 0,
                fetchPriority: typeof t.fetchPriority == `string` ? t.fetchPriority : void 0,
              });
            }
          } else t ?? i.d.M(e);
        }
      }),
      (e.preload = function (e, t) {
        if (typeof e == `string` && typeof t == `object` && t && typeof t.as == `string`) {
          var n = t.as,
            r = d(n, t.crossOrigin);
          i.d.L(e, n, {
            crossOrigin: r,
            integrity: typeof t.integrity == `string` ? t.integrity : void 0,
            nonce: typeof t.nonce == `string` ? t.nonce : void 0,
            type: typeof t.type == `string` ? t.type : void 0,
            fetchPriority: typeof t.fetchPriority == `string` ? t.fetchPriority : void 0,
            referrerPolicy: typeof t.referrerPolicy == `string` ? t.referrerPolicy : void 0,
            imageSrcSet: typeof t.imageSrcSet == `string` ? t.imageSrcSet : void 0,
            imageSizes: typeof t.imageSizes == `string` ? t.imageSizes : void 0,
            media: typeof t.media == `string` ? t.media : void 0,
          });
        }
      }),
      (e.preloadModule = function (e, t) {
        if (typeof e == `string`) {
          if (t) {
            var n = d(t.as, t.crossOrigin);
            i.d.m(e, {
              as: typeof t.as == `string` && t.as !== `script` ? t.as : void 0,
              crossOrigin: n,
              integrity: typeof t.integrity == `string` ? t.integrity : void 0,
              nonce: typeof t.nonce == `string` ? t.nonce : void 0,
              fetchPriority: typeof t.fetchPriority == `string` ? t.fetchPriority : void 0,
            });
          } else i.d.m(e);
        }
      }),
      (e.requestFormReset = function (e) {
        i.d.r(e);
      }),
      (e.unstable_batchedUpdates = function (e, t) {
        return e(t);
      }),
      (e.useFormState = function (e, t, n) {
        return l.H.useFormState(e, t, n);
      }),
      (e.useFormStatus = function () {
        return l.H.useHostTransitionStatus();
      }),
      (e.version = `19.3.1`));
  }),
  m = o((e, t) => {
    function n() {
      if (
        typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ < `u` &&
        typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE == `function`
      )
        try {
          __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(n);
        } catch (e) {
          console.error(e);
        }
    }
    (n(), (t.exports = p()));
  }),
  h = o((e) => {
    var t = f(),
      n = u(),
      r = m();
    function i(e) {
      var t = `https://react.dev/errors/` + e;
      if (1 < arguments.length) {
        t += `?args[]=` + encodeURIComponent(arguments[1]);
        for (var n = 2; n < arguments.length; n++)
          t += `&args[]=` + encodeURIComponent(arguments[n]);
      }
      return (
        `Minified React error #` +
        e +
        `; visit ` +
        t +
        ` for the full message or use the non-minified dev environment for full errors and additional helpful warnings.`
      );
    }
    function a(e) {
      return !(!e || (e.nodeType !== 1 && e.nodeType !== 9 && e.nodeType !== 11));
    }
    function o(e) {
      for (var t = e, n = t; n && !n.alternate;)
        ((t = n), t.flags & 4098 && (e = t.return), (n = t.return));
      for (; t.return;) t = t.return;
      return t.tag === 3 ? e : null;
    }
    function s(e) {
      if (e.tag === 13) {
        var t = e.memoizedState;
        if ((t === null && ((e = e.alternate), e !== null && (t = e.memoizedState)), t !== null))
          return t.dehydrated;
      }
      return null;
    }
    function c(e) {
      if (e.tag === 31) {
        var t = e.memoizedState;
        if ((t === null && ((e = e.alternate), e !== null && (t = e.memoizedState)), t !== null))
          return t.dehydrated;
      }
      return null;
    }
    function l(e) {
      if (o(e) !== e) throw Error(i(188));
    }
    function d(e) {
      var t = e.alternate;
      if (!t) {
        if (((t = o(e)), t === null)) throw Error(i(188));
        return t === e ? e : null;
      }
      for (var n = e, r = t; ;) {
        var a = n.return;
        if (a === null) break;
        var s = a.alternate;
        if (s === null) {
          if (((r = a.return), r !== null)) {
            n = r;
            continue;
          }
          break;
        }
        if (a.child === s.child) {
          for (s = a.child; s;) {
            if (s === n) return (l(a), e);
            if (s === r) return (l(a), t);
            s = s.sibling;
          }
          throw Error(i(188));
        }
        if (n.return !== r.return) ((n = a), (r = s));
        else {
          for (var c = !1, u = a.child; u;) {
            if (u === n) {
              ((c = !0), (n = a), (r = s));
              break;
            }
            if (u === r) {
              ((c = !0), (r = a), (n = s));
              break;
            }
            u = u.sibling;
          }
          if (!c) {
            for (u = s.child; u;) {
              if (u === n) {
                ((c = !0), (n = s), (r = a));
                break;
              }
              if (u === r) {
                ((c = !0), (r = s), (n = a));
                break;
              }
              u = u.sibling;
            }
            if (!c) throw Error(i(189));
          }
        }
        if (n.alternate !== r) throw Error(i(190));
      }
      if (n.tag !== 3) throw Error(i(188));
      return n.stateNode.current === n ? e : t;
    }
    function p(e) {
      var t = e.tag;
      if (t === 5 || t === 26 || t === 27 || t === 6) return e;
      for (e = e.child; e !== null;) {
        if (((t = p(e)), t !== null)) return t;
        e = e.sibling;
      }
      return null;
    }
    function h(e, t, n, r, i, a) {
      for (; e !== null;) {
        if (
          ((e.tag === 5 || e.tag === 27 || e.tag === 6) && n(e, r, i, a)) ||
          ((e.tag !== 22 || e.memoizedState === null) &&
            (t || (e.tag !== 5 && e.tag !== 27)) &&
            h(e.child, t, n, r, i, a))
        )
          return !0;
        e = e.sibling;
      }
      return !1;
    }
    function g(e) {
      for (e = e.return; e !== null;) {
        if (e.tag === 3 || e.tag === 5 || e.tag === 27) return e;
        e = e.return;
      }
      return null;
    }
    function _(e) {
      var t = !1;
      for (
        e = e.return;
        e !== null && (e.tag === 4 && (t = !0), e.tag !== 3 && e.tag !== 5 && e.tag !== 27);
      )
        e = e.return;
      return t;
    }
    function v(e) {
      var t = [null, null],
        n = g(e);
      return (n === null || y(t, e, n.child, { foundSelf: !1 }), t);
    }
    function y(e, t, n, r) {
      for (; n !== null;) {
        if (n === t) r.foundSelf = !0;
        else if (n.tag === 5 || n.tag === 27 || n.tag === 6) {
          if (r.foundSelf) return ((e[1] = n), !0);
          e[0] = n;
        } else if ((n.tag !== 22 || n.memoizedState === null) && y(e, t, n.child, r)) return !0;
        n = n.sibling;
      }
      return !1;
    }
    function b(e) {
      switch (e.tag) {
        case 5:
        case 27:
        case 6:
          return e.stateNode;
        case 3:
          return e.stateNode.containerInfo;
        default:
          throw Error(i(559));
      }
    }
    var x = null,
      S = null;
    function C(e, t, n) {
      return e === n || (e === t && ((x = e), !0));
    }
    function w(e, t, n) {
      return e === n ? ((S = e), !1) : e === t && (S !== null && (x = e), !0);
    }
    function T(e) {
      if (e === null) return null;
      do e = e === null ? null : e.return;
      while (e && e.tag !== 5 && e.tag !== 27 && e.tag !== 3);
      return e || null;
    }
    function E(e, t, n) {
      for (var r = 0, i = e; i; i = n(i)) r++;
      i = 0;
      for (var a = t; a; a = n(a)) i++;
      for (; 0 < r - i;) ((e = n(e)), r--);
      for (; 0 < i - r;) ((t = n(t)), i--);
      for (; r--;) {
        if (e === t || (t !== null && e === t.alternate)) return e;
        ((e = n(e)), (t = n(t)));
      }
      return null;
    }
    var D = Object.assign,
      O = Symbol.for(`react.element`),
      ee = Symbol.for(`react.transitional.element`),
      te = Symbol.for(`react.portal`),
      k = Symbol.for(`react.fragment`),
      A = Symbol.for(`react.strict_mode`),
      j = Symbol.for(`react.profiler`),
      ne = Symbol.for(`react.consumer`),
      M = Symbol.for(`react.context`),
      N = Symbol.for(`react.forward_ref`),
      P = Symbol.for(`react.suspense`),
      re = Symbol.for(`react.suspense_list`),
      F = Symbol.for(`react.memo`),
      I = Symbol.for(`react.lazy`),
      ie = Symbol.for(`react.activity`),
      ae = Symbol.for(`react.legacy_hidden`),
      oe = Symbol.for(`react.memo_cache_sentinel`),
      se = Symbol.for(`react.view_transition`),
      ce = Symbol.for(`react.recoverable`),
      le = Symbol.iterator;
    function ue(e) {
      return typeof e != `object` || !e
        ? null
        : ((e = (le && e[le]) || e[`@@iterator`]), typeof e == `function` ? e : null);
    }
    var de = Symbol.for(`react.client.reference`);
    function fe(e) {
      if (e == null) return null;
      if (typeof e == `function`) return e.$$typeof === de ? null : e.displayName || e.name || null;
      if (typeof e == `string`) return e;
      switch (e) {
        case k:
          return `Fragment`;
        case j:
          return `Profiler`;
        case A:
          return `StrictMode`;
        case P:
          return `Suspense`;
        case re:
          return `SuspenseList`;
        case ie:
          return `Activity`;
        case se:
          return `ViewTransition`;
      }
      if (typeof e == `object`)
        switch (e.$$typeof) {
          case te:
            return `Portal`;
          case M:
            return e.displayName || `Context`;
          case ne:
            return (e._context.displayName || `Context`) + `.Consumer`;
          case N:
            var t = e.render;
            return (
              (e = e.displayName),
              (e ||=
                ((e = t.displayName || t.name || ``),
                e === `` ? `ForwardRef` : `ForwardRef(` + e + `)`)),
              e
            );
          case F:
            return ((t = e.displayName || null), t === null ? fe(e.type) || `Memo` : t);
          case I:
            ((t = e._payload), (e = e._init));
            try {
              return fe(e(t));
            } catch {}
        }
      return null;
    }
    var pe = Array.isArray,
      L = n.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,
      R = r.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,
      me = { pending: !1, data: null, method: null, action: null },
      he = [],
      ge = -1;
    function _e(e) {
      return { current: e };
    }
    function ve(e) {
      0 > ge || ((e.current = he[ge]), (he[ge] = null), ge--);
    }
    function z(e, t) {
      (ge++, (he[ge] = e.current), (e.current = t));
    }
    var ye = _e(null),
      be = _e(null),
      xe = _e(null),
      Se = _e(null);
    function Ce(e, t) {
      switch ((z(xe, t), z(be, e), z(ye, null), t.nodeType)) {
        case 9:
        case 11:
          e = (e = t.documentElement) && (e = e.namespaceURI) ? up(e) : 0;
          break;
        default:
          if (((e = t.tagName), (t = t.namespaceURI))) ((t = up(t)), (e = dp(t, e)));
          else
            switch (e) {
              case `svg`:
                e = 1;
                break;
              case `math`:
                e = 2;
                break;
              default:
                e = 0;
            }
      }
      (ve(ye), z(ye, e));
    }
    function we() {
      (ve(ye), ve(be), ve(xe));
    }
    function Te(e) {
      var t = e.memoizedState;
      (t !== null && ((sh._currentValue = t.memoizedState), z(Se, e)), (t = ye.current));
      var n = dp(t, e.type);
      t !== n && (z(be, e), z(ye, n));
    }
    function Ee(e) {
      (be.current === e && (ve(ye), ve(be)), Se.current === e && (ve(Se), (sh._currentValue = me)));
    }
    var De, Oe;
    function ke(e) {
      if (De === void 0)
        try {
          throw Error();
        } catch (e) {
          var t = e.stack.trim().match(/\n( *(at )?)/);
          ((De = (t && t[1]) || ``),
            (Oe =
              -1 <
              e.stack.indexOf(`
    at`)
                ? ` (<anonymous>)`
                : -1 < e.stack.indexOf(`@`)
                  ? `@unknown:0:0`
                  : ``));
        }
      return (
        `
` +
        De +
        e +
        Oe
      );
    }
    var Ae = !1;
    function je(e, t) {
      if (!e || Ae) return ``;
      Ae = !0;
      var n = Error.prepareStackTrace;
      Error.prepareStackTrace = void 0;
      try {
        var r = {
          DetermineComponentFrameRoot: function () {
            try {
              if (t) {
                var n = function () {
                  throw Error();
                };
                if (
                  (Object.defineProperty(n.prototype, "props", {
                    set: function () {
                      throw Error();
                    },
                  }),
                  typeof Reflect == `object` && Reflect.construct)
                ) {
                  try {
                    Reflect.construct(n, []);
                  } catch (e) {
                    var r = e;
                  }
                  Reflect.construct(e, [], n);
                } else {
                  try {
                    n.call();
                  } catch (e) {
                    r = e;
                  }
                  n = !1;
                  try {
                    var i = Object.getOwnPropertyDescriptor(e.prototype, `props`);
                    (Object.defineProperty(e.prototype, "props", {
                      configurable: !0,
                      set: function () {
                        throw Error();
                      },
                    }),
                      (n = !0),
                      new e());
                  } finally {
                    n &&
                      (i === void 0
                        ? delete e.prototype.props
                        : Object.defineProperty(e.prototype, "props", i));
                  }
                }
              } else {
                try {
                  throw Error();
                } catch (e) {
                  r = e;
                }
                (n = e()) && typeof n.catch == `function` && n.catch(function () {});
              }
            } catch (e) {
              if (e && r && typeof e.stack == `string`) return [e.stack, r.stack];
            }
            return [null, null];
          },
        };
        r.DetermineComponentFrameRoot.displayName = `DetermineComponentFrameRoot`;
        var i = Object.getOwnPropertyDescriptor(r.DetermineComponentFrameRoot, `name`);
        i &&
          i.configurable &&
          Object.defineProperty(r.DetermineComponentFrameRoot, "name", {
            value: `DetermineComponentFrameRoot`,
          });
        var a = r.DetermineComponentFrameRoot(),
          o = a[0],
          s = a[1];
        if (o && s) {
          var c = o.split(`
`),
            l = s.split(`
`);
          for (i = r = 0; r < c.length && !c[r].includes(`DetermineComponentFrameRoot`);) r++;
          for (; i < l.length && !l[i].includes(`DetermineComponentFrameRoot`);) i++;
          if (r === c.length || i === l.length)
            for (r = c.length - 1, i = l.length - 1; 1 <= r && 0 <= i && c[r] !== l[i];) i--;
          for (; 1 <= r && 0 <= i; r--, i--)
            if (c[r] !== l[i]) {
              if (r !== 1 || i !== 1)
                do
                  if ((r--, i--, 0 > i || c[r] !== l[i])) {
                    var u =
                      `
` + c[r].replace(` at new `, ` at `);
                    return (
                      e.displayName &&
                        u.includes(`<anonymous>`) &&
                        (u = u.replace(`<anonymous>`, e.displayName)),
                      u
                    );
                  }
                while (1 <= r && 0 <= i);
              break;
            }
        }
      } finally {
        ((Ae = !1), (Error.prepareStackTrace = n));
      }
      return (n = e ? e.displayName || e.name : ``) ? ke(n) : ``;
    }
    function Me(e, t) {
      switch (e.tag) {
        case 26:
        case 27:
        case 5:
          return ke(e.type);
        case 16:
          return ke(`Lazy`);
        case 13:
          return e.child !== t && t !== null ? ke(`Suspense Fallback`) : ke(`Suspense`);
        case 19:
          return ke(`SuspenseList`);
        case 0:
        case 15:
          return je(e.type, !1);
        case 11:
          return je(e.type.render, !1);
        case 1:
          return je(e.type, !0);
        case 31:
          return ke(`Activity`);
        case 30:
          return ke(`ViewTransition`);
        default:
          return ``;
      }
    }
    function Ne(e) {
      try {
        var t = ``,
          n = null;
        do ((t += Me(e, n)), (n = e), (e = e.return));
        while (e);
        return t;
      } catch (e) {
        return (
          `
Error generating stack: ` +
          e.message +
          `
` +
          e.stack
        );
      }
    }
    var Pe = Object.prototype.hasOwnProperty,
      Fe = t.unstable_scheduleCallback,
      Ie = t.unstable_cancelCallback,
      Le = t.unstable_shouldYield,
      Re = t.unstable_requestPaint,
      ze = t.unstable_now,
      Be = t.unstable_getCurrentPriorityLevel,
      Ve = t.unstable_ImmediatePriority,
      He = t.unstable_UserBlockingPriority,
      Ue = t.unstable_NormalPriority,
      We = t.unstable_LowPriority,
      Ge = t.unstable_IdlePriority,
      Ke = t.log,
      qe = t.unstable_setDisableYieldValue,
      Je = null,
      Ye = null;
    function Xe(e) {
      if ((typeof Ke == `function` && qe(e), Ye && typeof Ye.setStrictMode == `function`))
        try {
          Ye.setStrictMode(Je, e);
        } catch {}
    }
    var Ze = Math.clz32 ? Math.clz32 : et,
      Qe = Math.log,
      $e = Math.LN2;
    function et(e) {
      return ((e >>>= 0), e === 0 ? 32 : (31 - ((Qe(e) / $e) | 0)) | 0);
    }
    var tt = 256,
      nt = 262144,
      rt = 4194304;
    function B(e) {
      var t = e & 42;
      if (t !== 0) return t;
      switch (e & -e) {
        case 1:
          return 1;
        case 2:
          return 2;
        case 4:
          return 4;
        case 8:
          return 8;
        case 16:
          return 16;
        case 32:
          return 32;
        case 64:
          return 64;
        case 128:
          return 128;
        case 256:
        case 512:
        case 1024:
        case 2048:
        case 4096:
        case 8192:
        case 16384:
        case 32768:
        case 65536:
        case 131072:
          return e & -e;
        case 262144:
        case 524288:
        case 1048576:
        case 2097152:
          return e & 3932160;
        case 4194304:
        case 8388608:
        case 16777216:
        case 33554432:
          return e & 62914560;
        case 67108864:
          return 67108864;
        case 134217728:
          return 134217728;
        case 268435456:
          return 268435456;
        case 536870912:
          return 536870912;
        case 1073741824:
          return 0;
        default:
          return e;
      }
    }
    function it(e, t, n) {
      var r = e.pendingLanes;
      if (r === 0) return 0;
      var i = 0,
        a = e.suspendedLanes,
        o = e.pingedLanes;
      e = e.warmLanes;
      var s = r & 134217727;
      return (
        s === 0
          ? ((s = r & ~a),
            s === 0
              ? o === 0
                ? n || ((n = r & ~e), n !== 0 && (i = B(n)))
                : (i = B(o))
              : (i = B(s)))
          : ((r = s & ~a),
            r === 0
              ? ((o &= s), o === 0 ? n || ((n = s & ~e), n !== 0 && (i = B(n))) : (i = B(o)))
              : (i = B(r))),
        i === 0
          ? 0
          : t !== 0 &&
              t !== i &&
              (t & a) === 0 &&
              ((a = i & -i), (n = t & -t), a >= n || (a === 32 && n & 4194048))
            ? t
            : i
      );
    }
    function at(e, t) {
      return (e.pendingLanes & ~(e.suspendedLanes & ~e.pingedLanes) & t) === 0;
    }
    function ot(e, t) {
      t & 8 && (t |= t & 32);
      var n = e.entangledLanes;
      if (n !== 0)
        for (e = e.entanglements, n &= t; 0 < n;) {
          var r = 31 - Ze(n),
            i = 1 << r;
          ((t |= e[r]), (n &= ~i));
        }
      return t;
    }
    function st(e, t) {
      switch (e) {
        case 1:
        case 2:
        case 4:
        case 8:
        case 64:
          return t + 250;
        case 16:
        case 32:
        case 128:
        case 256:
        case 512:
        case 1024:
        case 2048:
        case 4096:
        case 8192:
        case 16384:
        case 32768:
        case 65536:
        case 131072:
        case 262144:
        case 524288:
        case 1048576:
        case 2097152:
          return t + 5e3;
        case 4194304:
        case 8388608:
        case 16777216:
        case 33554432:
          return -1;
        case 67108864:
        case 134217728:
        case 268435456:
        case 536870912:
        case 1073741824:
          return -1;
        default:
          return -1;
      }
    }
    function ct() {
      var e = rt;
      return ((rt <<= 1), !(rt & 62914560) && (rt = 4194304), e);
    }
    function lt(e) {
      for (var t = [], n = 0; 31 > n; n++) t.push(e);
      return t;
    }
    function ut(e, t) {
      ((e.pendingLanes |= t),
        t !== 268435456 && ((e.suspendedLanes = 0), (e.pingedLanes = 0), (e.warmLanes = 0)));
    }
    function dt(e, t, n, r, i, a) {
      var o = e.pendingLanes;
      ((e.pendingLanes = n),
        (e.suspendedLanes = 0),
        (e.pingedLanes = 0),
        (e.warmLanes = 0),
        (e.expiredLanes &= n),
        (e.entangledLanes &= n),
        (e.errorRecoveryDisabledLanes &= n),
        (e.shellSuspendCounter = 0));
      var s = e.entanglements,
        c = e.expirationTimes,
        l = e.hiddenUpdates;
      for (n = o & ~n; 0 < n;) {
        var u = 31 - Ze(n),
          d = 1 << u;
        ((s[u] = 0), (c[u] = -1));
        var f = l[u];
        if (f !== null)
          for (l[u] = null, u = 0; u < f.length; u++) {
            var p = f[u];
            p !== null && (p.lane &= -536870913);
          }
        n &= ~d;
      }
      (r !== 0 && ft(e, r, 0),
        a !== 0 && i === 0 && e.tag !== 0 && (e.suspendedLanes |= a & ~(o & ~t)));
    }
    function ft(e, t, n) {
      ((e.pendingLanes |= t), (e.suspendedLanes &= ~t));
      var r = 31 - Ze(t);
      ((e.entangledLanes |= t),
        (e.entanglements[r] = e.entanglements[r] | 1073741824 | (n & 261930)));
    }
    function pt(e, t) {
      var n = (e.entangledLanes |= t);
      for (e = e.entanglements; n;) {
        var r = 31 - Ze(n),
          i = 1 << r;
        ((i & t) | (e[r] & t) && (e[r] |= t), (n &= ~i));
      }
    }
    function mt(e, t) {
      var n = t & -t;
      return ((n = n & 42 ? 1 : ht(n)), (n & (e.suspendedLanes | t)) === 0 ? n : 0);
    }
    function ht(e) {
      switch (e) {
        case 2:
          e = 1;
          break;
        case 8:
          e = 4;
          break;
        case 32:
          e = 16;
          break;
        case 256:
        case 512:
        case 1024:
        case 2048:
        case 4096:
        case 8192:
        case 16384:
        case 32768:
        case 65536:
        case 131072:
        case 262144:
        case 524288:
        case 1048576:
        case 2097152:
        case 4194304:
        case 8388608:
        case 16777216:
        case 33554432:
          e = 128;
          break;
        case 268435456:
          e = 134217728;
          break;
        default:
          e = 0;
      }
      return e;
    }
    function gt(e) {
      return ((e &= -e), 2 < e ? (8 < e ? (e & 134217727 ? 32 : 268435456) : 8) : 2);
    }
    function _t() {
      var e = R.p;
      return e === 0 ? ((e = window.event), e === void 0 ? 32 : Ch(e.type)) : e;
    }
    function vt(e, t) {
      var n = R.p;
      try {
        return ((R.p = e), t());
      } finally {
        R.p = n;
      }
    }
    var yt = Math.random().toString(36).slice(2),
      bt = `__reactFiber$` + yt,
      xt = `__reactProps$` + yt,
      St = `__reactContainer$` + yt,
      Ct = `__reactEvents$` + yt,
      wt = `__reactListeners$` + yt,
      Tt = `__reactHandles$` + yt,
      Et = `__reactResources$` + yt,
      Dt = `__reactMarker$` + yt,
      Ot = `__reactLoad$` + yt;
    function kt(e) {
      (delete e[bt], delete e[xt], delete e[wt], delete e[Tt]);
    }
    function At(e) {
      var t;
      if ((t = e[bt])) return t;
      for (var n = e.parentNode; n;) {
        if ((t = n[St] || n[bt])) {
          if (((n = t.alternate), t.child !== null || (n !== null && n.child !== null)))
            for (e = fm(e); e !== null;) {
              if ((n = e[bt])) return n;
              e = fm(e);
            }
          return t;
        }
        ((e = n), (n = e.parentNode));
      }
      return null;
    }
    function jt(e) {
      if ((e = e[bt] || e[St])) {
        var t = e.tag;
        if (t === 5 || t === 6 || t === 13 || t === 31 || t === 26 || t === 27 || t === 3) return e;
      }
      return null;
    }
    function Mt(e) {
      var t = e.tag;
      if (t === 5 || t === 26 || t === 27 || t === 6) return e.stateNode;
      throw Error(i(33));
    }
    function Nt(e) {
      var t = e[Et];
      return ((t ||= e[Et] = { hoistableStyles: new Map(), hoistableScripts: new Map() }), t);
    }
    function Pt(e) {
      e[Dt] = !0;
    }
    function Ft(e) {
      e[Ot] = void 0;
    }
    var It = new Set(),
      Lt = {};
    function Rt(e, t) {
      (zt(e, t), zt(e + `Capture`, t));
    }
    function zt(e, t) {
      for (Lt[e] = t, e = 0; e < t.length; e++) It.add(t[e]);
    }
    var Bt = RegExp(
        `^[:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD][:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040]*$`,
      ),
      Vt = {},
      Ht = {};
    function Ut(e) {
      return Pe.call(Ht, e)
        ? !0
        : Pe.call(Vt, e)
          ? !1
          : Bt.test(e)
            ? (Ht[e] = !0)
            : ((Vt[e] = !0), !1);
    }
    var V = !1;
    function Wt() {
      var e = V;
      return ((V = !1), e);
    }
    function Gt(e, t, n) {
      if (Ut(t)) {
        if (n === null) e.removeAttribute(t);
        else {
          switch (typeof n) {
            case `undefined`:
            case `function`:
            case `symbol`:
              e.removeAttribute(t);
              return;
            case `boolean`:
              var r = t.toLowerCase().slice(0, 5);
              if (r !== `data-` && r !== `aria-`) {
                e.removeAttribute(t);
                return;
              }
          }
          e.setAttribute(t, n);
        }
      }
    }
    function Kt(e, t, n) {
      if (n === null) e.removeAttribute(t);
      else {
        switch (typeof n) {
          case `undefined`:
          case `function`:
          case `symbol`:
          case `boolean`:
            e.removeAttribute(t);
            return;
        }
        e.setAttribute(t, n);
      }
    }
    function qt(e, t, n, r) {
      if (r === null) e.removeAttribute(n);
      else {
        switch (typeof r) {
          case `undefined`:
          case `function`:
          case `symbol`:
          case `boolean`:
            e.removeAttribute(n);
            return;
        }
        e.setAttributeNS(t, n, r);
      }
    }
    function Jt(e) {
      switch (typeof e) {
        case `bigint`:
        case `boolean`:
        case `number`:
        case `string`:
        case `undefined`:
          return e;
        case `object`:
          return e;
        default:
          return ``;
      }
    }
    function Yt(e) {
      var t = e.type;
      return (e = e.nodeName) && e.toLowerCase() === `input` && (t === `checkbox` || t === `radio`);
    }
    function Xt(e, t, n) {
      var r = Object.getOwnPropertyDescriptor(e.constructor.prototype, t);
      if (
        !e.hasOwnProperty(t) &&
        r !== void 0 &&
        typeof r.get == `function` &&
        typeof r.set == `function`
      ) {
        var i = r.get,
          a = r.set;
        return (
          Object.defineProperty(e, t, {
            configurable: !0,
            get: function () {
              return i.call(this);
            },
            set: function (e) {
              ((n = `` + e), a.call(this, e));
            },
          }),
          Object.defineProperty(e, t, { enumerable: r.enumerable }),
          {
            getValue: function () {
              return n;
            },
            setValue: function (e) {
              n = `` + e;
            },
            stopTracking: function () {
              ((e._valueTracker = null), delete e[t]);
            },
          }
        );
      }
    }
    function Zt(e) {
      if (!e._valueTracker) {
        var t = Yt(e) ? `checked` : `value`;
        e._valueTracker = Xt(e, t, `` + e[t]);
      }
    }
    function Qt(e) {
      if (!e) return !1;
      var t = e._valueTracker;
      if (!t) return !0;
      var n = t.getValue(),
        r = ``;
      return (
        e && (r = Yt(e) ? (e.checked ? `true` : `false`) : e.value),
        (e = r),
        e !== n && (t.setValue(e), !0)
      );
    }
    var $t = /[\n"\\]/g;
    function en(e) {
      return e.replace($t, function (e) {
        return `\\` + e.charCodeAt(0).toString(16) + ` `;
      });
    }
    function tn(e, t, n, r, i, a, o, s) {
      ((e.name = ``),
        o != null && typeof o != `function` && typeof o != `symbol` && typeof o != `boolean`
          ? (e.type = o)
          : e.removeAttribute(`type`),
        t == null
          ? (o !== `submit` && o !== `reset`) || e.removeAttribute(`value`)
          : o === `number`
            ? ((t === 0 && e.value === ``) || e.value != t) && (e.value = `` + Jt(t))
            : e.value !== `` + Jt(t) && (e.value = `` + Jt(t)),
        t == null
          ? n == null
            ? r != null && e.removeAttribute(`value`)
            : rn(e, Jt(n))
          : o === `number` && e.value == t
            ? rn(e, Jt(e.value))
            : rn(e, Jt(t)),
        i == null && a != null && (e.defaultChecked = !!a),
        i != null && (e.checked = i && typeof i != `function` && typeof i != `symbol`),
        s != null && typeof s != `function` && typeof s != `symbol` && typeof s != `boolean`
          ? (e.name = `` + Jt(s))
          : e.removeAttribute(`name`));
    }
    function nn(e, t, n, r, i, a, o, s) {
      if (
        (a != null &&
          typeof a != `function` &&
          typeof a != `symbol` &&
          typeof a != `boolean` &&
          (e.type = a),
        t != null || n != null)
      ) {
        if (!((a !== `submit` && a !== `reset`) || t != null)) {
          Zt(e);
          return;
        }
        ((n = n == null ? `` : `` + Jt(n)),
          (t = t == null ? n : `` + Jt(t)),
          s || t === e.value || (e.value = t),
          (e.defaultValue = t));
      }
      ((r ??= i),
        (r = typeof r != `function` && typeof r != `symbol` && !!r),
        (e.checked = s ? e.checked : !!r),
        (e.defaultChecked = !!r),
        o != null &&
          typeof o != `function` &&
          typeof o != `symbol` &&
          typeof o != `boolean` &&
          (e.name = o),
        Zt(e));
    }
    function rn(e, t) {
      e.defaultValue !== `` + t && (e.defaultValue = `` + t);
    }
    function an(e, t, n, r) {
      if (((e = e.options), t)) {
        t = {};
        for (var i = 0; i < n.length; i++) t[`$` + n[i]] = !0;
        for (n = 0; n < e.length; n++)
          ((i = t.hasOwnProperty(`$` + e[n].value)),
            e[n].selected !== i && (e[n].selected = i),
            i && r && (e[n].defaultSelected = !0));
      } else {
        for (n = `` + Jt(n), t = null, i = 0; i < e.length; i++) {
          if (e[i].value === n) {
            ((e[i].selected = !0), r && (e[i].defaultSelected = !0));
            return;
          }
          t !== null || e[i].disabled || (t = e[i]);
        }
        t !== null && (t.selected = !0);
      }
    }
    function on(e, t, n) {
      t != null && ((t = `` + Jt(t)), t !== e.value && (e.value = t), n == null)
        ? e.defaultValue !== t && (e.defaultValue = t)
        : (e.defaultValue = n == null ? `` : `` + Jt(n));
    }
    function sn(e, t, n, r) {
      if (t == null) {
        if (r != null) {
          if (n != null) throw Error(i(92));
          if (pe(r)) {
            if (1 < r.length) throw Error(i(93));
            r = r[0];
          }
          n = r;
        }
        ((n ??= ``), (t = n));
      }
      ((n = Jt(t)),
        (e.defaultValue = n),
        (r = e.textContent),
        r === n && r !== `` && r !== null && (e.value = r),
        Zt(e));
    }
    function cn(e, t) {
      if (t) {
        var n = e.firstChild;
        if (n && n === e.lastChild && n.nodeType === 3) {
          n.nodeValue = t;
          return;
        }
      }
      e.textContent = t;
    }
    var ln = new Set(
      `animationIterationCount aspectRatio borderImageOutset borderImageSlice borderImageWidth boxFlex boxFlexGroup boxOrdinalGroup columnCount columns flex flexGrow flexPositive flexShrink flexNegative flexOrder gridArea gridRow gridRowEnd gridRowSpan gridRowStart gridColumn gridColumnEnd gridColumnSpan gridColumnStart fontWeight lineClamp lineHeight opacity order orphans scale tabSize widows zIndex zoom fillOpacity floodOpacity stopOpacity strokeDasharray strokeDashoffset strokeMiterlimit strokeOpacity strokeWidth MozAnimationIterationCount MozBoxFlex MozBoxFlexGroup MozLineClamp msAnimationIterationCount msFlex msZoom msFlexGrow msFlexNegative msFlexOrder msFlexPositive msFlexShrink msGridColumn msGridColumnSpan msGridRow msGridRowSpan WebkitAnimationIterationCount WebkitBoxFlex WebKitBoxFlexGroup WebkitBoxOrdinalGroup WebkitColumnCount WebkitColumns WebkitFlex WebkitFlexGrow WebkitFlexPositive WebkitFlexShrink WebkitLineClamp`.split(
        ` `,
      ),
    );
    function un(e, t, n) {
      var r = t.indexOf(`--`) === 0;
      n == null || typeof n == `boolean` || n === ``
        ? r
          ? e.setProperty(t, ``)
          : t === `float`
            ? (e.cssFloat = ``)
            : (e[t] = ``)
        : r
          ? e.setProperty(t, n)
          : typeof n != `number` || n === 0 || ln.has(t)
            ? t === `float`
              ? (e.cssFloat = n)
              : (e[t] = (`` + n).trim())
            : (e[t] = n + `px`);
    }
    function dn(e, t, n) {
      if (t != null && typeof t != `object`) throw Error(i(62));
      if (((e = e.style), n != null)) {
        for (var r in n)
          !n.hasOwnProperty(r) ||
            (t != null && t.hasOwnProperty(r)) ||
            (r.indexOf(`--`) === 0
              ? e.setProperty(r, ``)
              : r === `float`
                ? (e.cssFloat = ``)
                : (e[r] = ``),
            (V = !0));
        for (var a in t) ((r = t[a]), t.hasOwnProperty(a) && n[a] !== r && (un(e, a, r), (V = !0)));
      } else for (var o in t) t.hasOwnProperty(o) && un(e, o, t[o]);
    }
    function fn(e) {
      if (e.indexOf(`-`) === -1) return !1;
      switch (e) {
        case `annotation-xml`:
        case `color-profile`:
        case `font-face`:
        case `font-face-src`:
        case `font-face-uri`:
        case `font-face-format`:
        case `font-face-name`:
        case `missing-glyph`:
          return !1;
        default:
          return !0;
      }
    }
    var pn = new Map([
        [`acceptCharset`, `accept-charset`],
        [`htmlFor`, `for`],
        [`httpEquiv`, `http-equiv`],
        [`crossOrigin`, `crossorigin`],
        [`accentHeight`, `accent-height`],
        [`alignmentBaseline`, `alignment-baseline`],
        [`arabicForm`, `arabic-form`],
        [`baselineShift`, `baseline-shift`],
        [`capHeight`, `cap-height`],
        [`clipPath`, `clip-path`],
        [`clipRule`, `clip-rule`],
        [`colorInterpolation`, `color-interpolation`],
        [`colorInterpolationFilters`, `color-interpolation-filters`],
        [`colorProfile`, `color-profile`],
        [`colorRendering`, `color-rendering`],
        [`dominantBaseline`, `dominant-baseline`],
        [`enableBackground`, `enable-background`],
        [`fillOpacity`, `fill-opacity`],
        [`fillRule`, `fill-rule`],
        [`floodColor`, `flood-color`],
        [`floodOpacity`, `flood-opacity`],
        [`fontFamily`, `font-family`],
        [`fontSize`, `font-size`],
        [`fontSizeAdjust`, `font-size-adjust`],
        [`fontStretch`, `font-stretch`],
        [`fontStyle`, `font-style`],
        [`fontVariant`, `font-variant`],
        [`fontWeight`, `font-weight`],
        [`glyphName`, `glyph-name`],
        [`glyphOrientationHorizontal`, `glyph-orientation-horizontal`],
        [`glyphOrientationVertical`, `glyph-orientation-vertical`],
        [`horizAdvX`, `horiz-adv-x`],
        [`horizOriginX`, `horiz-origin-x`],
        [`imageRendering`, `image-rendering`],
        [`letterSpacing`, `letter-spacing`],
        [`lightingColor`, `lighting-color`],
        [`markerEnd`, `marker-end`],
        [`markerMid`, `marker-mid`],
        [`markerStart`, `marker-start`],
        [`maskType`, `mask-type`],
        [`overlinePosition`, `overline-position`],
        [`overlineThickness`, `overline-thickness`],
        [`paintOrder`, `paint-order`],
        [`panose-1`, `panose-1`],
        [`pointerEvents`, `pointer-events`],
        [`renderingIntent`, `rendering-intent`],
        [`shapeRendering`, `shape-rendering`],
        [`stopColor`, `stop-color`],
        [`stopOpacity`, `stop-opacity`],
        [`strikethroughPosition`, `strikethrough-position`],
        [`strikethroughThickness`, `strikethrough-thickness`],
        [`strokeDasharray`, `stroke-dasharray`],
        [`strokeDashoffset`, `stroke-dashoffset`],
        [`strokeLinecap`, `stroke-linecap`],
        [`strokeLinejoin`, `stroke-linejoin`],
        [`strokeMiterlimit`, `stroke-miterlimit`],
        [`strokeOpacity`, `stroke-opacity`],
        [`strokeWidth`, `stroke-width`],
        [`textAnchor`, `text-anchor`],
        [`textDecoration`, `text-decoration`],
        [`textRendering`, `text-rendering`],
        [`transformOrigin`, `transform-origin`],
        [`underlinePosition`, `underline-position`],
        [`underlineThickness`, `underline-thickness`],
        [`unicodeBidi`, `unicode-bidi`],
        [`unicodeRange`, `unicode-range`],
        [`unitsPerEm`, `units-per-em`],
        [`vAlphabetic`, `v-alphabetic`],
        [`vHanging`, `v-hanging`],
        [`vIdeographic`, `v-ideographic`],
        [`vMathematical`, `v-mathematical`],
        [`vectorEffect`, `vector-effect`],
        [`vertAdvY`, `vert-adv-y`],
        [`vertOriginX`, `vert-origin-x`],
        [`vertOriginY`, `vert-origin-y`],
        [`wordSpacing`, `word-spacing`],
        [`writingMode`, `writing-mode`],
        [`xmlnsXlink`, `xmlns:xlink`],
        [`xHeight`, `x-height`],
      ]),
      mn =
        /^[\u0000-\u001F ]*j[\r\n\t]*a[\r\n\t]*v[\r\n\t]*a[\r\n\t]*s[\r\n\t]*c[\r\n\t]*r[\r\n\t]*i[\r\n\t]*p[\r\n\t]*t[\r\n\t]*:/i;
    function hn(e) {
      return mn.test(`` + e)
        ? `javascript:throw new Error('React has blocked a javascript: URL as a security precaution.')`
        : e;
    }
    function gn() {}
    var _n = null;
    function vn(e) {
      return (
        (e = e.target || e.srcElement || window),
        e.correspondingUseElement && (e = e.correspondingUseElement),
        e.nodeType === 3 ? e.parentNode : e
      );
    }
    var yn = null,
      bn = null;
    function xn(e) {
      var t = jt(e);
      if (t && (e = t.stateNode)) {
        var n = e[xt] || null;
        a: switch (((e = t.stateNode), t.type)) {
          case `input`:
            if (
              (tn(
                e,
                n.value,
                n.defaultValue,
                n.defaultValue,
                n.checked,
                n.defaultChecked,
                n.type,
                n.name,
              ),
              (t = n.name),
              n.type === `radio` && t != null)
            ) {
              for (n = e; n.parentNode;) n = n.parentNode;
              for (
                n = n.querySelectorAll(`input[name="` + en(`` + t) + `"][type="radio"]`), t = 0;
                t < n.length;
                t++
              ) {
                var r = n[t];
                if (r !== e && r.form === e.form) {
                  var a = r[xt] || null;
                  if (!a) throw Error(i(90));
                  tn(
                    r,
                    a.value,
                    a.defaultValue,
                    a.defaultValue,
                    a.checked,
                    a.defaultChecked,
                    a.type,
                    a.name,
                  );
                }
              }
              for (t = 0; t < n.length; t++) ((r = n[t]), r.form === e.form && Qt(r));
            }
            break a;
          case `textarea`:
            on(e, n.value, n.defaultValue);
            break a;
          case `select`:
            ((t = n.value), t != null && an(e, !!n.multiple, t, !1));
        }
      }
    }
    var Sn = !1;
    function Cn(e, t, n) {
      if (Sn) return e(t, n);
      Sn = !0;
      try {
        return e(t);
      } finally {
        if (
          ((Sn = !1),
          (yn !== null || bn !== null) &&
            (zd(), yn && ((t = yn), (e = bn), (bn = yn = null), xn(t), e)))
        )
          for (t = 0; t < e.length; t++) xn(e[t]);
      }
    }
    function wn(e, t) {
      var n = e.stateNode;
      if (n === null) return null;
      var r = n[xt] || null;
      if (r === null) return null;
      n = r[t];
      a: switch (t) {
        case `onClick`:
        case `onClickCapture`:
        case `onDoubleClick`:
        case `onDoubleClickCapture`:
        case `onMouseDown`:
        case `onMouseDownCapture`:
        case `onMouseMove`:
        case `onMouseMoveCapture`:
        case `onMouseUp`:
        case `onMouseUpCapture`:
        case `onMouseEnter`:
          ((r = !r.disabled) ||
            ((e = e.type),
            (r = e !== `button` && e !== `input` && e !== `select` && e !== `textarea`)),
            (e = !r));
          break a;
        default:
          e = !1;
      }
      if (e) return null;
      if (n && typeof n != `function`) throw Error(i(231, t, typeof n));
      return n;
    }
    var Tn =
        typeof window < `u` &&
        window.document !== void 0 &&
        window.document.createElement !== void 0,
      En = !1;
    if (Tn)
      try {
        var Dn = {};
        (Object.defineProperty(Dn, "passive", {
          get: function () {
            En = !0;
          },
        }),
          window.addEventListener(`test`, Dn, Dn),
          window.removeEventListener(`test`, Dn, Dn));
      } catch {
        En = !1;
      }
    var On = null,
      kn = null,
      An = null;
    function jn() {
      if (An) return An;
      var e,
        t = kn,
        n = t.length,
        r,
        i = `value` in On ? On.value : On.textContent,
        a = i.length;
      for (e = 0; e < n && t[e] === i[e]; e++);
      var o = n - e;
      for (r = 1; r <= o && t[n - r] === i[a - r]; r++);
      return (An = i.slice(e, 1 < r ? 1 - r : void 0));
    }
    function Mn(e) {
      var t = e.keyCode;
      return (
        `charCode` in e ? ((e = e.charCode), e === 0 && t === 13 && (e = 13)) : (e = t),
        e === 10 && (e = 13),
        32 <= e || e === 13 ? e : 0
      );
    }
    function Nn() {
      return !0;
    }
    function Pn() {
      return !1;
    }
    function Fn(e) {
      function t(t, n, r, i, a) {
        for (var o in ((this._reactName = t),
        (this._targetInst = r),
        (this.type = n),
        (this.nativeEvent = i),
        (this.target = a),
        (this.currentTarget = null),
        e))
          e.hasOwnProperty(o) && ((t = e[o]), (this[o] = t ? t(i) : i[o]));
        return (
          (this.isDefaultPrevented = (
            i.defaultPrevented == null ? !1 === i.returnValue : i.defaultPrevented
          )
            ? Nn
            : Pn),
          (this.isPropagationStopped = Pn),
          this
        );
      }
      return (
        D(t.prototype, {
          preventDefault: function () {
            this.defaultPrevented = !0;
            var e = this.nativeEvent;
            e &&
              (e.preventDefault
                ? e.preventDefault()
                : typeof e.returnValue != `unknown` && (e.returnValue = !1),
              (this.isDefaultPrevented = Nn));
          },
          stopPropagation: function () {
            var e = this.nativeEvent;
            e &&
              (e.stopPropagation
                ? e.stopPropagation()
                : typeof e.cancelBubble != `unknown` && (e.cancelBubble = !0),
              (this.isPropagationStopped = Nn));
          },
          persist: function () {},
          isPersistent: Nn,
        }),
        t
      );
    }
    var In = {
        eventPhase: 0,
        bubbles: 0,
        cancelable: 0,
        timeStamp: function (e) {
          return e.timeStamp || Date.now();
        },
        defaultPrevented: 0,
        isTrusted: 0,
      },
      Ln = Fn(In),
      Rn = D({}, In, { view: 0, detail: 0 }),
      zn = Fn(Rn),
      Bn,
      Vn,
      Hn,
      Un = D({}, Rn, {
        screenX: 0,
        screenY: 0,
        clientX: 0,
        clientY: 0,
        pageX: 0,
        pageY: 0,
        ctrlKey: 0,
        shiftKey: 0,
        altKey: 0,
        metaKey: 0,
        getModifierState: er,
        button: 0,
        buttons: 0,
        relatedTarget: function (e) {
          return e.relatedTarget === void 0
            ? e.fromElement === e.srcElement
              ? e.toElement
              : e.fromElement
            : e.relatedTarget;
        },
        movementX: function (e) {
          return `movementX` in e
            ? e.movementX
            : (e !== Hn &&
                (Hn && e.type === `mousemove`
                  ? ((Bn = e.screenX - Hn.screenX), (Vn = e.screenY - Hn.screenY))
                  : (Vn = Bn = 0),
                (Hn = e)),
              Bn);
        },
        movementY: function (e) {
          return `movementY` in e ? e.movementY : Vn;
        },
      }),
      Wn = Fn(Un),
      Gn = Fn(D({}, Un, { dataTransfer: 0 })),
      Kn = Fn(D({}, Rn, { relatedTarget: 0 })),
      qn = Fn(D({}, In, { animationName: 0, elapsedTime: 0, pseudoElement: 0 })),
      Jn = Fn(
        D({}, In, {
          clipboardData: function (e) {
            return `clipboardData` in e ? e.clipboardData : window.clipboardData;
          },
        }),
      ),
      Yn = Fn(D({}, In, { data: 0 })),
      Xn = {
        Esc: `Escape`,
        Spacebar: ` `,
        Left: `ArrowLeft`,
        Up: `ArrowUp`,
        Right: `ArrowRight`,
        Down: `ArrowDown`,
        Del: `Delete`,
        Win: `OS`,
        Menu: `ContextMenu`,
        Apps: `ContextMenu`,
        Scroll: `ScrollLock`,
        MozPrintableKey: `Unidentified`,
      },
      Zn = {
        8: `Backspace`,
        9: `Tab`,
        12: `Clear`,
        13: `Enter`,
        16: `Shift`,
        17: `Control`,
        18: `Alt`,
        19: `Pause`,
        20: `CapsLock`,
        27: `Escape`,
        32: ` `,
        33: `PageUp`,
        34: `PageDown`,
        35: `End`,
        36: `Home`,
        37: `ArrowLeft`,
        38: `ArrowUp`,
        39: `ArrowRight`,
        40: `ArrowDown`,
        45: `Insert`,
        46: `Delete`,
        112: `F1`,
        113: `F2`,
        114: `F3`,
        115: `F4`,
        116: `F5`,
        117: `F6`,
        118: `F7`,
        119: `F8`,
        120: `F9`,
        121: `F10`,
        122: `F11`,
        123: `F12`,
        144: `NumLock`,
        145: `ScrollLock`,
        224: `Meta`,
      },
      Qn = { Alt: `altKey`, Control: `ctrlKey`, Meta: `metaKey`, Shift: `shiftKey` };
    function $n(e) {
      var t = this.nativeEvent;
      return t.getModifierState ? t.getModifierState(e) : (e = Qn[e]) ? !!t[e] : !1;
    }
    function er() {
      return $n;
    }
    var tr = Fn(
        D({}, Rn, {
          key: function (e) {
            if (e.key) {
              var t = Xn[e.key] || e.key;
              if (t !== `Unidentified`) return t;
            }
            return e.type === `keypress`
              ? ((e = Mn(e)), e === 13 ? `Enter` : String.fromCharCode(e))
              : e.type === `keydown` || e.type === `keyup`
                ? Zn[e.keyCode] || `Unidentified`
                : ``;
          },
          code: 0,
          location: 0,
          ctrlKey: 0,
          shiftKey: 0,
          altKey: 0,
          metaKey: 0,
          repeat: 0,
          locale: 0,
          getModifierState: er,
          charCode: function (e) {
            return e.type === `keypress` ? Mn(e) : 0;
          },
          keyCode: function (e) {
            return e.type === `keydown` || e.type === `keyup` ? e.keyCode : 0;
          },
          which: function (e) {
            return e.type === `keypress`
              ? Mn(e)
              : e.type === `keydown` || e.type === `keyup`
                ? e.keyCode
                : 0;
          },
        }),
      ),
      nr = Fn(
        D({}, Un, {
          pointerId: 0,
          width: 0,
          height: 0,
          pressure: 0,
          tangentialPressure: 0,
          tiltX: 0,
          tiltY: 0,
          twist: 0,
          pointerType: 0,
          isPrimary: 0,
        }),
      ),
      rr = Fn(D({}, In, { submitter: 0 })),
      ir = Fn(
        D({}, Rn, {
          touches: 0,
          targetTouches: 0,
          changedTouches: 0,
          altKey: 0,
          metaKey: 0,
          ctrlKey: 0,
          shiftKey: 0,
          getModifierState: er,
        }),
      ),
      ar = Fn(D({}, In, { propertyName: 0, elapsedTime: 0, pseudoElement: 0 })),
      or = Fn(
        D({}, Un, {
          deltaX: function (e) {
            return `deltaX` in e ? e.deltaX : `wheelDeltaX` in e ? -e.wheelDeltaX : 0;
          },
          deltaY: function (e) {
            return `deltaY` in e
              ? e.deltaY
              : `wheelDeltaY` in e
                ? -e.wheelDeltaY
                : `wheelDelta` in e
                  ? -e.wheelDelta
                  : 0;
          },
          deltaZ: 0,
          deltaMode: 0,
        }),
      ),
      sr = Fn(D({}, In, { newState: 0, oldState: 0, source: 0 })),
      cr = [9, 13, 27, 32],
      lr = Tn && `CompositionEvent` in window,
      ur = null;
    Tn && `documentMode` in document && (ur = document.documentMode);
    var dr = Tn && `TextEvent` in window && !ur,
      fr = Tn && (!lr || (ur && 8 < ur && 11 >= ur)),
      pr = ` `,
      mr = !1;
    function hr(e, t) {
      switch (e) {
        case `keyup`:
          return cr.indexOf(t.keyCode) !== -1;
        case `keydown`:
          return t.keyCode !== 229;
        case `keypress`:
        case `mousedown`:
        case `focusout`:
          return !0;
        default:
          return !1;
      }
    }
    function gr(e) {
      return ((e = e.detail), typeof e == `object` && `data` in e ? e.data : null);
    }
    var _r = !1;
    function vr(e, t) {
      switch (e) {
        case `compositionend`:
          return gr(t);
        case `keypress`:
          return t.which === 32 ? ((mr = !0), pr) : null;
        case `textInput`:
          return ((e = t.data), e === pr && mr ? null : e);
        default:
          return null;
      }
    }
    function yr(e, t) {
      if (_r)
        return e === `compositionend` || (!lr && hr(e, t))
          ? ((e = jn()), (An = kn = On = null), (_r = !1), e)
          : null;
      switch (e) {
        case `paste`:
          return null;
        case `keypress`:
          if (!(t.ctrlKey || t.altKey || t.metaKey) || (t.ctrlKey && t.altKey)) {
            if (t.char && 1 < t.char.length) return t.char;
            if (t.which) return String.fromCharCode(t.which);
          }
          return null;
        case `compositionend`:
          return fr && t.locale !== `ko` ? null : t.data;
        default:
          return null;
      }
    }
    var br = {
      color: !0,
      date: !0,
      datetime: !0,
      "datetime-local": !0,
      email: !0,
      month: !0,
      number: !0,
      password: !0,
      range: !0,
      search: !0,
      tel: !0,
      text: !0,
      time: !0,
      url: !0,
      week: !0,
    };
    function xr(e) {
      var t = e && e.nodeName && e.nodeName.toLowerCase();
      return t === `input` ? !!br[e.type] : t === `textarea`;
    }
    function Sr(e, t, n, r) {
      (yn ? (bn ? bn.push(r) : (bn = [r])) : (yn = r),
        (t = Jf(t, `onChange`)),
        0 < t.length &&
          ((n = new Ln(`onChange`, `change`, null, n, r)), e.push({ event: n, listeners: t })));
    }
    var Cr = null,
      H = null;
    function wr(e) {
      Vf(e, 0);
    }
    function Tr(e) {
      if (Qt(Mt(e))) return e;
    }
    function Er(e, t) {
      if (e === `change`) return t;
    }
    var Dr = !1;
    if (Tn) {
      var Or;
      if (Tn) {
        var kr = `oninput` in document;
        if (!kr) {
          var Ar = document.createElement(`div`);
          (Ar.setAttribute(`oninput`, `return;`), (kr = typeof Ar.oninput == `function`));
        }
        Or = kr;
      } else Or = !1;
      Dr = Or && (!document.documentMode || 9 < document.documentMode);
    }
    function jr() {
      Cr && (Cr.detachEvent(`onpropertychange`, Mr), (H = Cr = null));
    }
    function Mr(e) {
      if (e.propertyName === `value` && Tr(H)) {
        var t = [];
        (Sr(t, H, e, vn(e)), Cn(wr, t));
      }
    }
    function Nr(e, t, n) {
      e === `focusin`
        ? (jr(), (Cr = t), (H = n), Cr.attachEvent(`onpropertychange`, Mr))
        : e === `focusout` && jr();
    }
    function Pr(e) {
      if (e === `selectionchange` || e === `keyup` || e === `keydown`) return Tr(H);
    }
    function Fr(e, t) {
      if (e === `click`) return Tr(t);
    }
    function Ir(e, t) {
      if (e === `input` || e === `change`) return Tr(t);
    }
    function Lr(e, t) {
      return (e === t && (e !== 0 || 1 / e == 1 / t)) || (e !== e && t !== t);
    }
    var Rr = typeof Object.is == `function` ? Object.is : Lr;
    function zr(e, t) {
      if (Rr(e, t)) return !0;
      if (typeof e != `object` || !e || typeof t != `object` || !t) return !1;
      var n = Object.keys(e),
        r = Object.keys(t);
      if (n.length !== r.length) return !1;
      for (r = 0; r < n.length; r++) {
        var i = n[r];
        if (!Pe.call(t, i) || !Rr(e[i], t[i])) return !1;
      }
      return !0;
    }
    function Br(e) {
      if (((e ||= typeof document < `u` ? document : void 0), e === void 0)) return null;
      try {
        return e.activeElement || e.body;
      } catch {
        return e.body;
      }
    }
    function Vr(e) {
      for (; e && e.firstChild;) e = e.firstChild;
      return e;
    }
    function Hr(e, t) {
      var n = Vr(e);
      e = 0;
      for (var r; n;) {
        if (n.nodeType === 3) {
          if (((r = e + n.textContent.length), e <= t && r >= t)) return { node: n, offset: t - e };
          e = r;
        }
        a: {
          for (; n;) {
            if (n.nextSibling) {
              n = n.nextSibling;
              break a;
            }
            n = n.parentNode;
          }
          n = void 0;
        }
        n = Vr(n);
      }
    }
    function Ur(e, t) {
      return e && t
        ? e === t
          ? !0
          : e && e.nodeType === 3
            ? !1
            : t && t.nodeType === 3
              ? Ur(e, t.parentNode)
              : `contains` in e
                ? e.contains(t)
                : e.compareDocumentPosition
                  ? !!(e.compareDocumentPosition(t) & 16)
                  : !1
        : !1;
    }
    function Wr(e) {
      e =
        e != null && e.ownerDocument != null && e.ownerDocument.defaultView != null
          ? e.ownerDocument.defaultView
          : window;
      for (var t = Br(e.document); t instanceof e.HTMLIFrameElement;) {
        try {
          var n = typeof t.contentWindow.location.href == `string`;
        } catch {
          n = !1;
        }
        if (n) e = t.contentWindow;
        else break;
        t = Br(e.document);
      }
      return t;
    }
    function Gr(e) {
      var t = e && e.nodeName && e.nodeName.toLowerCase();
      return (
        t &&
        ((t === `input` &&
          (e.type === `text` ||
            e.type === `search` ||
            e.type === `tel` ||
            e.type === `url` ||
            e.type === `password`)) ||
          t === `textarea` ||
          e.contentEditable === `true`)
      );
    }
    var Kr = Tn && `documentMode` in document && 11 >= document.documentMode,
      qr = null,
      Jr = null,
      Yr = null,
      Xr = !1;
    function Zr(e, t, n) {
      var r = n.window === n ? n.document : n.nodeType === 9 ? n : n.ownerDocument;
      Xr ||
        qr == null ||
        qr !== Br(r) ||
        ((r = qr),
        `selectionStart` in r && Gr(r)
          ? (r = { start: r.selectionStart, end: r.selectionEnd })
          : ((r = ((r.ownerDocument && r.ownerDocument.defaultView) || window).getSelection()),
            (r = {
              anchorNode: r.anchorNode,
              anchorOffset: r.anchorOffset,
              focusNode: r.focusNode,
              focusOffset: r.focusOffset,
            })),
        (Yr && zr(Yr, r)) ||
          ((Yr = r),
          (r = Jf(Jr, `onSelect`)),
          0 < r.length &&
            ((t = new Ln(`onSelect`, `select`, null, t, n)),
            e.push({ event: t, listeners: r }),
            (t.target = qr))));
    }
    function Qr(e, t) {
      var n = {};
      return (
        (n[e.toLowerCase()] = t.toLowerCase()),
        (n[`Webkit` + e] = `webkit` + t),
        (n[`Moz` + e] = `moz` + t),
        n
      );
    }
    var $r = {
        animationend: Qr(`Animation`, `AnimationEnd`),
        animationiteration: Qr(`Animation`, `AnimationIteration`),
        animationstart: Qr(`Animation`, `AnimationStart`),
        transitionrun: Qr(`Transition`, `TransitionRun`),
        transitionstart: Qr(`Transition`, `TransitionStart`),
        transitioncancel: Qr(`Transition`, `TransitionCancel`),
        transitionend: Qr(`Transition`, `TransitionEnd`),
      },
      ei = {},
      ti = {};
    Tn &&
      ((ti = document.createElement(`div`).style),
      `AnimationEvent` in window ||
        (delete $r.animationend.animation,
        delete $r.animationiteration.animation,
        delete $r.animationstart.animation),
      `TransitionEvent` in window || delete $r.transitionend.transition);
    function ni(e) {
      if (ei[e]) return ei[e];
      if (!$r[e]) return e;
      var t = $r[e],
        n;
      for (n in t) if (t.hasOwnProperty(n) && n in ti) return (ei[e] = t[n]);
      return e;
    }
    var ri = ni(`animationend`),
      ii = ni(`animationiteration`),
      ai = ni(`animationstart`),
      oi = ni(`transitionrun`),
      si = ni(`transitionstart`),
      ci = ni(`transitioncancel`),
      li = ni(`transitionend`),
      ui = new Map(),
      di =
        `abort auxClick beforeToggle cancel canPlay canPlayThrough click close contextMenu copy cut drag dragEnd dragEnter dragExit dragLeave dragOver dragStart drop durationChange emptied encrypted ended error fullscreenChange fullscreenError gotPointerCapture input invalid keyDown keyPress keyUp load loadedData loadedMetadata loadStart lostPointerCapture mouseDown mouseMove mouseOut mouseOver mouseUp paste pause play playing pointerCancel pointerDown pointerMove pointerOut pointerOver pointerUp progress rateChange reset resize seeked seeking stalled submit suspend timeUpdate touchCancel touchEnd touchStart volumeChange scroll toggle touchMove waiting wheel`.split(
          ` `,
        );
    di.push(`scrollEnd`);
    function fi(e, t) {
      (ui.set(e, t), Rt(t, [e]));
    }
    var pi = 0;
    function mi(e, t) {
      if (e.name != null && e.name !== `auto`) return e.name;
      if (t.autoName !== null) return t.autoName;
      e = bd.identifierPrefix;
      var n = pi++;
      return ((e = `_` + e + `t_` + n.toString(32) + `_`), (t.autoName = e));
    }
    function hi(e) {
      if (e == null || typeof e == `string`) return e;
      var t = null,
        n = Od;
      if (n !== null)
        for (var r = 0; r < n.length; r++) {
          var i = e[n[r]];
          if (i != null) {
            if (i === `none`) return `none`;
            t = t == null ? i : t + (` ` + i);
          }
        }
      return t ?? e.default;
    }
    function gi(e, t) {
      return (
        (e = hi(e)), (t = hi(t)), t == null ? (e === `auto` ? null : e) : t === `auto` ? null : t
      );
    }
    var _i =
        typeof reportError == `function`
          ? reportError
          : function (e) {
              if (typeof window == `object` && typeof window.ErrorEvent == `function`) {
                var t = new window.ErrorEvent(`error`, {
                  bubbles: !0,
                  cancelable: !0,
                  message:
                    typeof e == `object` && e && typeof e.message == `string`
                      ? String(e.message)
                      : String(e),
                  error: e,
                });
                if (!window.dispatchEvent(t)) return;
              } else if (typeof process == `object` && typeof process.emit == `function`) {
                process.emit(`uncaughtException`, e);
                return;
              }
              console.error(e);
            },
      vi = [],
      yi = 0,
      bi = 0;
    function xi() {
      for (var e = yi, t = (bi = yi = 0); t < e;) {
        var n = vi[t];
        vi[t++] = null;
        var r = vi[t];
        vi[t++] = null;
        var i = vi[t];
        vi[t++] = null;
        var a = vi[t];
        if (((vi[t++] = null), r !== null && i !== null)) {
          var o = r.pending;
          (o === null ? (i.next = i) : ((i.next = o.next), (o.next = i)), (r.pending = i));
        }
        a !== 0 && Ti(n, i, a);
      }
    }
    function Si(e, t, n, r) {
      ((vi[yi++] = e),
        (vi[yi++] = t),
        (vi[yi++] = n),
        (vi[yi++] = r),
        (bi |= r),
        (e.lanes |= r),
        (e = e.alternate),
        e !== null && (e.lanes |= r));
    }
    function Ci(e, t, n, r) {
      return (Si(e, t, n, r), Ei(e));
    }
    function wi(e, t) {
      return (Si(e, null, null, t), Ei(e));
    }
    function Ti(e, t, n) {
      e.lanes |= n;
      var r = e.alternate;
      r !== null && (r.lanes |= n);
      for (var i = !1, a = e.return; a !== null;)
        ((a.childLanes |= n),
          (r = a.alternate),
          r !== null && (r.childLanes |= n),
          a.tag === 22 && ((e = a.stateNode), e === null || e._visibility & 1 || (i = !0)),
          (e = a),
          (a = a.return));
      return e.tag === 3
        ? ((a = e.stateNode),
          i &&
            t !== null &&
            ((i = 31 - Ze(n)),
            (e = a.hiddenUpdates),
            (r = e[i]),
            r === null ? (e[i] = [t]) : r.push(t),
            (t.lane = n | 536870912)),
          a)
        : null;
    }
    function Ei(e) {
      if (50 < kd) throw ((kd = 0), (Ad = null), Error(i(185)));
      for (var t = e.return; t !== null;) ((e = t), (t = e.return));
      return e.tag === 3 ? e.stateNode : null;
    }
    var Di = {};
    function Oi(e, t, n, r) {
      ((this.tag = e),
        (this.key = n),
        (this.sibling =
          this.child =
          this.return =
          this.stateNode =
          this.type =
          this.elementType =
            null),
        (this.index = 0),
        (this.refCleanup = this.ref = null),
        (this.pendingProps = t),
        (this.dependencies = this.memoizedState = this.updateQueue = this.memoizedProps = null),
        (this.mode = r),
        (this.subtreeFlags = this.flags = 0),
        (this.deletions = null),
        (this.childLanes = this.lanes = 0),
        (this.alternate = null));
    }
    function ki(e, t, n, r) {
      return new Oi(e, t, n, r);
    }
    function Ai(e) {
      return ((e = e.prototype), !(!e || !e.isReactComponent));
    }
    function ji(e, t) {
      var n = e.alternate;
      return (
        n === null
          ? ((n = ki(e.tag, t, e.key, e.mode)),
            (n.elementType = e.elementType),
            (n.type = e.type),
            (n.stateNode = e.stateNode),
            (n.alternate = e),
            (e.alternate = n))
          : ((n.pendingProps = t),
            (n.type = e.type),
            (n.flags = 0),
            (n.subtreeFlags = 0),
            (n.deletions = null)),
        (n.flags = e.flags & 1206910976),
        (n.childLanes = e.childLanes),
        (n.lanes = e.lanes),
        (n.child = e.child),
        (n.memoizedProps = e.memoizedProps),
        (n.memoizedState = e.memoizedState),
        (n.updateQueue = e.updateQueue),
        (t = e.dependencies),
        (n.dependencies = t === null ? null : { lanes: t.lanes, firstContext: t.firstContext }),
        (n.sibling = e.sibling),
        (n.index = e.index),
        (n.ref = e.ref),
        (n.refCleanup = e.refCleanup),
        n
      );
    }
    function Mi(e, t) {
      e.flags &= 1206910978;
      var n = e.alternate;
      return (
        n === null
          ? ((e.childLanes = 0),
            (e.lanes = t),
            (e.child = null),
            (e.subtreeFlags = 0),
            (e.memoizedProps = null),
            (e.memoizedState = null),
            (e.updateQueue = null),
            (e.dependencies = null),
            (e.stateNode = null))
          : ((e.childLanes = n.childLanes),
            (e.lanes = n.lanes),
            (e.child = n.child),
            (e.subtreeFlags = 0),
            (e.deletions = null),
            (e.memoizedProps = n.memoizedProps),
            (e.memoizedState = n.memoizedState),
            (e.updateQueue = n.updateQueue),
            (e.type = n.type),
            (t = n.dependencies),
            (e.dependencies =
              t === null ? null : { lanes: t.lanes, firstContext: t.firstContext })),
        e
      );
    }
    function Ni(e, t, n, r, a, o) {
      var s = 0;
      if (((r = e), typeof r == `function`)) Ai(r) && (s = 1);
      else if (typeof r == `string`)
        s = qm(e, n, ye.current) ? 26 : e === `html` || e === `head` || e === `body` ? 27 : 5;
      else
        a: switch (r) {
          case ie:
            return ((e = ki(31, n, t, a)), (e.elementType = ie), (e.lanes = o), e);
          case k:
            return Pi(n.children, a, o, t);
          case A:
            ((s = 8), (a |= 24));
            break;
          case j:
            return ((e = ki(12, n, t, a | 2)), (e.elementType = j), (e.lanes = o), e);
          case P:
            return ((e = ki(13, n, t, a)), (e.elementType = P), (e.lanes = o), e);
          case re:
            return ((e = ki(19, n, t, a)), (e.elementType = re), (e.lanes = o), e);
          case ae:
          case se:
            return (
              (e = a | 32),
              (e = ki(30, n, t, e)),
              (e.elementType = se),
              (e.lanes = o),
              (e.stateNode = { autoName: null, paired: null, clones: null, ref: null }),
              e
            );
          default:
            if (typeof r == `object` && r)
              switch (r.$$typeof) {
                case M:
                  s = 10;
                  break a;
                case ne:
                  s = 9;
                  break a;
                case N:
                  s = 11;
                  break a;
                case F:
                  s = 14;
                  break a;
                case I:
                  ((s = 16), (r = null));
                  break a;
              }
            ((s = 29), (n = Error(i(130, e === null ? `null` : typeof e, ``))), (r = null));
        }
      return ((t = ki(s, n, t, a)), (t.elementType = e), (t.type = r), (t.lanes = o), t);
    }
    function Pi(e, t, n, r) {
      return ((e = ki(7, e, r, t)), (e.lanes = n), e);
    }
    function Fi(e, t, n) {
      return ((e = ki(6, e, null, t)), (e.lanes = n), e);
    }
    function Ii(e) {
      var t = ki(18, null, null, 0);
      return ((t.stateNode = e), t);
    }
    function Li(e, t, n) {
      return (
        (t = ki(4, e.children === null ? [] : e.children, e.key, t)),
        (t.lanes = n),
        (t.stateNode = {
          containerInfo: e.containerInfo,
          pendingChildren: null,
          implementation: e.implementation,
        }),
        t
      );
    }
    var Ri = new WeakMap();
    function zi(e, t) {
      if (typeof e == `object` && e) {
        var n = Ri.get(e);
        return n === void 0 ? ((t = { value: e, source: t, stack: Ne(t) }), Ri.set(e, t), t) : n;
      }
      return { value: e, source: t, stack: Ne(t) };
    }
    var Bi = [],
      Vi = 0,
      Hi = null,
      Ui = 0,
      Wi = [],
      Gi = 0,
      Ki = null,
      qi = 1,
      Ji = ``;
    function Yi(e, t) {
      ((Bi[Vi++] = Ui), (Bi[Vi++] = Hi), (Hi = e), (Ui = t));
    }
    function Xi(e, t, n) {
      ((Wi[Gi++] = qi), (Wi[Gi++] = Ji), (Wi[Gi++] = Ki), (Ki = e));
      var r = qi;
      e = Ji;
      var i = 32 - Ze(r) - 1;
      ((r &= ~(1 << i)), (n += 1));
      var a = 32 - Ze(t) + i;
      if (30 < a) {
        var o = i - (i % 5);
        ((a = (r & ((1 << o) - 1)).toString(32)),
          (r >>= o),
          (i -= o),
          (qi = (1 << (32 - Ze(t) + i)) | (n << i) | r),
          (Ji = a + e));
      } else ((qi = (1 << a) | (n << i) | r), (Ji = e));
    }
    function Zi(e) {
      e.return !== null && (Yi(e, 1), Xi(e, 1, 0));
    }
    function Qi(e) {
      for (; e === Hi;) ((Hi = Bi[--Vi]), (Bi[Vi] = null), (Ui = Bi[--Vi]), (Bi[Vi] = null));
      for (; e === Ki;)
        ((Ki = Wi[--Gi]),
          (Wi[Gi] = null),
          (Ji = Wi[--Gi]),
          (Wi[Gi] = null),
          (qi = Wi[--Gi]),
          (Wi[Gi] = null));
    }
    function $i(e, t) {
      ((Wi[Gi++] = qi), (Wi[Gi++] = Ji), (Wi[Gi++] = Ki), (qi = t.id), (Ji = t.overflow), (Ki = e));
    }
    var ea = null,
      ta = null,
      U = !1,
      na = null,
      ra = !1,
      ia = Error(i(519));
    function aa(e) {
      throw (
        da(
          zi(
            Error(
              i(
                418,
                1 < arguments.length && arguments[1] !== void 0 && arguments[1] ? `text` : `HTML`,
                ``,
              ),
            ),
            e,
          ),
        ),
        ia
      );
    }
    function oa(e) {
      var t = e.stateNode,
        n = e.type,
        r = e.memoizedProps;
      switch (((t[bt] = e), (t[xt] = r), n)) {
        case `dialog`:
          (Q(`cancel`, t), Q(`close`, t));
          break;
        case `iframe`:
        case `object`:
        case `embed`:
          Q(`load`, t);
          break;
        case `video`:
        case `audio`:
          for (n = 0; n < zf.length; n++) Q(zf[n], t);
          break;
        case `source`:
          Q(`error`, t);
          break;
        case `img`:
        case `image`:
        case `link`:
          (Q(`error`, t), Q(`load`, t));
          break;
        case `details`:
          Q(`toggle`, t);
          break;
        case `input`:
          (Q(`invalid`, t),
            nn(t, r.value, r.defaultValue, r.checked, r.defaultChecked, r.type, r.name, !0));
          break;
        case `select`:
          Q(`invalid`, t);
          break;
        case `textarea`:
          (Q(`invalid`, t), sn(t, r.value, r.defaultValue, r.children));
      }
      ((n = r.children),
        (typeof n != `string` && typeof n != `number` && typeof n != `bigint`) ||
        t.textContent === `` + n ||
        !0 === r.suppressHydrationWarning ||
        ep(t.textContent, n)
          ? (r.popover != null && (Q(`beforetoggle`, t), Q(`toggle`, t)),
            r.onScroll != null && Q(`scroll`, t),
            r.onScrollEnd != null && Q(`scrollend`, t),
            r.onClick != null && (t.onclick = gn),
            (t = !0))
          : (t = !1),
        t || aa(e, !0));
    }
    function sa(e) {
      for (ea = e.return; ea;)
        switch (ea.tag) {
          case 5:
          case 31:
          case 13:
            ra = !1;
            return;
          case 27:
          case 3:
            ra = !0;
            return;
          default:
            ea = ea.return;
        }
    }
    function ca(e) {
      if (e !== ea) return !1;
      if (!U) return (sa(e), (U = !0), !1);
      var t = e.tag,
        n;
      if (
        ((n = t !== 3 && t !== 27) &&
          ((n = t === 5) &&
            ((n = e.type), (n = n === `form` || n === `button` || pp(e.type, e.memoizedProps))),
          (n = !n)),
        n && ta && aa(e),
        sa(e),
        t === 13)
      ) {
        if (((e = e.memoizedState), (e = e === null ? null : e.dehydrated), !e))
          throw Error(i(317));
        ta = dm(e);
      } else if (t === 31) {
        if (((e = e.memoizedState), (e = e === null ? null : e.dehydrated), !e))
          throw Error(i(317));
        ta = dm(e);
      } else
        t === 27
          ? ((t = ta), Sp(e.type) ? ((e = um), (um = null), (ta = e)) : (ta = t))
          : (ta = ea ? lm(e.stateNode.nextSibling) : null);
      return !0;
    }
    function la() {
      ((ta = ea = null), (U = !1));
    }
    function ua() {
      var e = na;
      return (e !== null && (fd === null ? (fd = e) : fd.push.apply(fd, e), (na = null)), e);
    }
    function da(e) {
      na === null ? (na = [e]) : na.push(e);
    }
    var fa = _e(null),
      pa = null,
      ma = null;
    function ha(e, t, n) {
      (z(fa, t._currentValue), (t._currentValue = n));
    }
    function ga(e) {
      ((e._currentValue = fa.current), ve(fa));
    }
    function _a(e, t, n) {
      for (; e !== null;) {
        var r = e.alternate;
        if (
          ((e.childLanes & t) === t
            ? r !== null && (r.childLanes & t) !== t && (r.childLanes |= t)
            : ((e.childLanes |= t), r !== null && (r.childLanes |= t)),
          e === n)
        )
          break;
        e = e.return;
      }
    }
    function va(e, t, n, r) {
      var a = e.child;
      for (a !== null && (a.return = e); a !== null;) {
        var o = a.dependencies;
        if (o !== null) {
          var s = a.child;
          o = o.firstContext;
          a: for (; o !== null;) {
            var c = o;
            o = a;
            for (var l = 0; l < t.length; l++)
              if (c.context === t[l]) {
                ((o.lanes |= n),
                  (c = o.alternate),
                  c !== null && (c.lanes |= n),
                  _a(o.return, n, e),
                  r || (s = null));
                break a;
              }
            o = c.next;
          }
        } else if (a.tag === 18) {
          if (((s = a.return), s === null)) throw Error(i(341));
          ((s.lanes |= n),
            (o = s.alternate),
            o !== null && (o.lanes |= n),
            _a(s, n, e),
            (s = null));
        } else
          a.tag === 13 && a.memoizedState !== null && a.memoizedState.dehydrated === null
            ? ((a.lanes |= n),
              (s = a.alternate),
              s !== null && (s.lanes |= n),
              _a(a.return, n, e),
              (s = a.child),
              (s = s === null ? null : s.sibling))
            : (s = a.child);
        if (s !== null) s.return = a;
        else
          for (s = a; s !== null;) {
            if (s === e) {
              s = null;
              break;
            }
            if (((a = s.sibling), a !== null)) {
              ((a.return = s.return), (s = a));
              break;
            }
            s = s.return;
          }
        a = s;
      }
    }
    function ya(e, t, n, r) {
      e = null;
      for (var a = t, o = !1; a !== null;) {
        if (!o) {
          if (a.flags & 524288) o = !0;
          else if (a.flags & 262144) break;
        }
        if (a.tag === 10) {
          var s = a.alternate;
          if (s === null) throw Error(i(387));
          if (((s = s.memoizedProps), s !== null)) {
            var c = a.type;
            Rr(a.pendingProps.value, s.value) || (e === null ? (e = [c]) : e.push(c));
          }
        } else if (a === Se.current) {
          if (((s = a.alternate), s === null)) throw Error(i(387));
          s.memoizedState.memoizedState !== a.memoizedState.memoizedState &&
            (e === null ? (e = [sh]) : e.push(sh));
        }
        a = a.return;
      }
      return (e !== null && va(t, e, n, r), (t.flags |= 262144), e !== null);
    }
    function ba(e) {
      for (e = e.firstContext; e !== null;) {
        if (!Rr(e.context._currentValue, e.memoizedValue)) return !0;
        e = e.next;
      }
      return !1;
    }
    function xa(e) {
      ((pa = e), (ma = null), (e = e.dependencies), e !== null && (e.firstContext = null));
    }
    function Sa(e) {
      return wa(pa, e);
    }
    function Ca(e, t) {
      return (pa === null && xa(e), wa(e, t));
    }
    function wa(e, t) {
      var n = t._currentValue;
      if (((t = { context: t, memoizedValue: n, next: null }), ma === null)) {
        if (e === null) throw Error(i(308));
        ((ma = t), (e.dependencies = { lanes: 0, firstContext: t }), (e.flags |= 524288));
      } else ma = ma.next = t;
      return n;
    }
    var Ta =
        typeof AbortController < `u`
          ? AbortController
          : function () {
              var e = [],
                t = (this.signal = {
                  aborted: !1,
                  addEventListener: function (t, n) {
                    e.push(n);
                  },
                });
              this.abort = function () {
                ((t.aborted = !0),
                  e.forEach(function (e) {
                    return e();
                  }));
              };
            },
      Ea = t.unstable_scheduleCallback,
      Da = t.unstable_NormalPriority,
      Oa = {
        $$typeof: M,
        Consumer: null,
        Provider: null,
        _currentValue: null,
        _currentValue2: null,
        _threadCount: 0,
      };
    function ka() {
      return { controller: new Ta(), data: new Map(), refCount: 0 };
    }
    function Aa(e) {
      (e.refCount--,
        e.refCount === 0 &&
          Ea(Da, function () {
            e.controller.abort();
          }));
    }
    function ja(e, t) {
      if (e.pendingLanes & 4194048) {
        var n = e.transitionTypes;
        for (n === null && (n = e.transitionTypes = []), e = 0; e < t.length; e++) {
          var r = t[e];
          n.indexOf(r) === -1 && n.push(r);
        }
      }
    }
    var Ma = null;
    function Na(e) {
      var t = e.transitionTypes;
      return ((e.transitionTypes = null), t);
    }
    var Pa = null,
      Fa = 0,
      Ia = 0,
      La = null;
    function Ra(e, t) {
      if (Pa === null) {
        var n = (Pa = []);
        ((Fa = 0),
          (Ia = Pf()),
          (La = {
            status: `pending`,
            value: void 0,
            then: function (e) {
              n.push(e);
            },
          }));
      }
      return (Fa++, t.then(za, za), t);
    }
    function za() {
      if (--Fa === 0 && ((Ma = null), Pa !== null)) {
        La !== null && (La.status = `fulfilled`);
        var e = Pa;
        ((Pa = null), (Ia = 0), (La = null));
        for (var t = 0; t < e.length; t++) (0, e[t])();
      }
    }
    function Ba(e, t) {
      var n = [],
        r = {
          status: `pending`,
          value: null,
          reason: null,
          then: function (e) {
            n.push(e);
          },
        };
      return (
        e.then(
          function () {
            ((r.status = `fulfilled`), (r.value = t));
            for (var e = 0; e < n.length; e++) (0, n[e])(t);
          },
          function (e) {
            for (r.status = `rejected`, r.reason = e, e = 0; e < n.length; e++) (0, n[e])(void 0);
          },
        ),
        r
      );
    }
    var Va = L.S;
    L.S = function (e, t) {
      if (
        ((hd = ze()),
        typeof t == `object` && t && typeof t.then == `function` && Ra(e, t),
        Ma !== null)
      )
        for (var n = bf; n !== null;) (ja(n, Ma), (n = n.next));
      if (((n = e.types), n !== null)) {
        for (var r = bf; r !== null;) (ja(r, n), (r = r.next));
        if (Ia !== 0) {
          ((r = Ma), r === null && (r = Ma = []));
          for (var i = 0; i < n.length; i++) {
            var a = n[i];
            r.indexOf(a) === -1 && r.push(a);
          }
        }
      }
      Va !== null && Va(e, t);
    };
    var Ha = _e(null);
    function Ua() {
      var e = Ha.current;
      return e === null ? $u.pooledCache : e;
    }
    function Wa(e, t) {
      t === null ? z(Ha, Ha.current) : z(Ha, t.pool);
    }
    function Ga() {
      var e = Ua();
      return e === null ? null : { parent: Oa._currentValue, pool: e };
    }
    var Ka = Error(i(460)),
      qa = Error(i(474)),
      Ja = Error(i(542)),
      Ya = { then: function () {} };
    function Xa(e) {
      return ((e = e.status), e === `fulfilled` || e === `rejected`);
    }
    function Za(e, t, n) {
      switch (
        ((n = e[n]), n === void 0 ? e.push(t) : n !== t && (t.then(gn, gn), (t = n)), t.status)
      ) {
        case `fulfilled`:
          return t.value;
        case `rejected`:
          throw ((e = t.reason), to(e), e === void 0 && !(`reason` in t) ? Error(i(600)) : e);
        default:
          if (typeof t.status == `string`) t.then(gn, gn);
          else {
            if (((e = $u), e !== null && 100 < e.shellSuspendCounter)) throw Error(i(482));
            ((e = t),
              (e.status = `pending`),
              e.then(
                function (e) {
                  if (t.status === `pending`) {
                    var n = t;
                    ((n.status = `fulfilled`), (n.value = e));
                  }
                },
                function (e) {
                  if (t.status === `pending`) {
                    var n = t;
                    ((n.status = `rejected`), (n.reason = e));
                  }
                },
              ));
          }
          switch (t.status) {
            case `fulfilled`:
              return t.value;
            case `rejected`:
              throw ((e = t.reason), to(e), e);
          }
          throw (($a = t), Ka);
      }
    }
    function Qa(e) {
      try {
        var t = e._init;
        return t(e._payload);
      } catch (e) {
        throw typeof e == `object` && e && typeof e.then == `function` ? (($a = e), Ka) : e;
      }
    }
    var $a = null;
    function eo() {
      if ($a === null) throw Error(i(459));
      var e = $a;
      return (($a = null), e);
    }
    function to(e) {
      if (e === Ka || e === Ja) throw Error(i(483));
    }
    var no = null,
      ro = 0;
    function io(e) {
      var t = ro;
      return ((ro += 1), no === null && (no = []), Za(no, e, t));
    }
    function ao(e, t) {
      ((t = t.props.ref), (e.ref = t === void 0 ? null : t));
    }
    function oo(e, t) {
      throw t.$$typeof === O
        ? Error(i(525))
        : ((e = Object.prototype.toString.call(t)),
          Error(
            i(
              31,
              e === `[object Object]` ? `object with keys {` + Object.keys(t).join(`, `) + `}` : e,
            ),
          ));
    }
    function so(e) {
      function t(t, n) {
        if (e) {
          var r = t.deletions;
          r === null ? ((t.deletions = [n]), (t.flags |= 16)) : r.push(n);
        }
      }
      function n(n, r) {
        if (!e) return null;
        for (; r !== null;) (t(n, r), (r = r.sibling));
        return null;
      }
      function r(e) {
        for (var t = new Map(); e !== null;)
          (e.key === null ? t.set(e.index, e) : t.set(e.key, e), (e = e.sibling));
        return t;
      }
      function a(e, t) {
        return ((e = ji(e, t)), (e.index = 0), (e.sibling = null), e);
      }
      function o(t, n, r) {
        return (
          (t.index = r),
          e
            ? ((r = t.alternate),
              r === null
                ? ((t.flags |= 134217730), n)
                : ((r = r.index), r < n ? ((t.flags |= 2), n) : r))
            : ((t.flags |= 1048576), n)
        );
      }
      function s(t) {
        return (e && t.alternate === null && (t.flags |= 134217730), t);
      }
      function c(e, t, n, r) {
        return t === null || t.tag !== 6
          ? ((t = Fi(n, e.mode, r)), (t.return = e), t)
          : ((t = a(t, n)), (t.return = e), t);
      }
      function l(e, t, n, r) {
        var i = n.type;
        return i === k
          ? ((e = d(e, t, n.props.children, r, n.key)), ao(e, n), e)
          : t !== null &&
              (t.elementType === i ||
                (typeof i == `object` && i && i.$$typeof === I && Qa(i) === t.type))
            ? ((t = a(t, n.props)), ao(t, n), (t.return = e), t)
            : ((t = Ni(n.type, n.key, n.props, null, e.mode, r)), ao(t, n), (t.return = e), t);
      }
      function u(e, t, n, r) {
        return t === null ||
          t.tag !== 4 ||
          t.stateNode.containerInfo !== n.containerInfo ||
          t.stateNode.implementation !== n.implementation
          ? ((t = Li(n, e.mode, r)), (t.return = e), t)
          : ((t = a(t, n.children || [])), (t.return = e), t);
      }
      function d(e, t, n, r, i) {
        return t === null || t.tag !== 7
          ? ((t = Pi(n, e.mode, r, i)), (t.return = e), t)
          : ((t = a(t, n)), (t.return = e), t);
      }
      function f(e, t, n) {
        if ((typeof t == `string` && t !== ``) || typeof t == `number` || typeof t == `bigint`)
          return ((t = Fi(`` + t, e.mode, n)), (t.return = e), t);
        if (typeof t == `object` && t) {
          switch (t.$$typeof) {
            case ee:
              return (
                (n = Ni(t.type, t.key, t.props, null, e.mode, n)), ao(n, t), (n.return = e), n
              );
            case te:
              return ((t = Li(t, e.mode, n)), (t.return = e), t);
            case I:
              return ((t = Qa(t)), f(e, t, n));
          }
          if (pe(t) || ue(t)) return ((t = Pi(t, e.mode, n, null)), (t.return = e), t);
          if (typeof t.then == `function`) return f(e, io(t), n);
          if (t.$$typeof === M) return f(e, Ca(e, t), n);
          oo(e, t);
        }
        return null;
      }
      function p(e, t, n, r) {
        var i = t === null ? null : t.key;
        if ((typeof n == `string` && n !== ``) || typeof n == `number` || typeof n == `bigint`)
          return i === null ? c(e, t, `` + n, r) : null;
        if (typeof n == `object` && n) {
          switch (n.$$typeof) {
            case ee:
              return n.key === i ? l(e, t, n, r) : null;
            case te:
              return n.key === i ? u(e, t, n, r) : null;
            case I:
              return ((n = Qa(n)), p(e, t, n, r));
          }
          if (pe(n) || ue(n)) return i === null ? d(e, t, n, r, null) : null;
          if (typeof n.then == `function`) return p(e, t, io(n), r);
          if (n.$$typeof === M) return p(e, t, Ca(e, n), r);
          oo(e, n);
        }
        return null;
      }
      function m(e, t, n, r, i) {
        if ((typeof r == `string` && r !== ``) || typeof r == `number` || typeof r == `bigint`)
          return ((e = e.get(n) || null), c(t, e, `` + r, i));
        if (typeof r == `object` && r) {
          switch (r.$$typeof) {
            case ee:
              return ((e = e.get(r.key === null ? n : r.key) || null), l(t, e, r, i));
            case te:
              return ((e = e.get(r.key === null ? n : r.key) || null), u(t, e, r, i));
            case I:
              return ((r = Qa(r)), m(e, t, n, r, i));
          }
          if (pe(r) || ue(r)) return ((e = e.get(n) || null), d(t, e, r, i, null));
          if (typeof r.then == `function`) return m(e, t, n, io(r), i);
          if (r.$$typeof === M) return m(e, t, n, Ca(t, r), i);
          oo(t, r);
        }
        return null;
      }
      function h(i, a, s, c) {
        for (
          var l = null, u = null, d = a, h = (a = 0), g = null;
          d !== null && h < s.length;
          h++
        ) {
          d.index > h ? ((g = d), (d = null)) : (g = d.sibling);
          var _ = p(i, d, s[h], c);
          if (_ === null) {
            d === null && (d = g);
            break;
          }
          (e && d && _.alternate === null && t(i, d),
            (a = o(_, a, h)),
            u === null ? (l = _) : (u.sibling = _),
            (u = _),
            (d = g));
        }
        if (h === s.length) return (n(i, d), U && Yi(i, h), l);
        if (d === null) {
          for (; h < s.length; h++)
            ((d = f(i, s[h], c)),
              d !== null && ((a = o(d, a, h)), u === null ? (l = d) : (u.sibling = d), (u = d)));
          return (U && Yi(i, h), l);
        }
        for (d = r(d); h < s.length; h++)
          ((g = m(d, i, h, s[h], c)),
            g !== null &&
              (e && ((_ = g.alternate), _ !== null && d.delete(_.key === null ? h : _.key)),
              (a = o(g, a, h)),
              u === null ? (l = g) : (u.sibling = g),
              (u = g)));
        return (
          e &&
            d.forEach(function (e) {
              return t(i, e);
            }),
          U && Yi(i, h),
          l
        );
      }
      function g(a, s, c, l) {
        if (c == null) throw Error(i(151));
        for (
          var u = null, d = null, h = s, g = (s = 0), _ = null, v = c.next();
          h !== null && !v.done;
          g++, v = c.next()
        ) {
          h.index > g ? ((_ = h), (h = null)) : (_ = h.sibling);
          var y = p(a, h, v.value, l);
          if (y === null) {
            h === null && (h = _);
            break;
          }
          (e && h && y.alternate === null && t(a, h),
            (s = o(y, s, g)),
            d === null ? (u = y) : (d.sibling = y),
            (d = y),
            (h = _));
        }
        if (v.done) return (n(a, h), U && Yi(a, g), u);
        if (h === null) {
          for (; !v.done; g++, v = c.next())
            ((v = f(a, v.value, l)),
              v !== null && ((s = o(v, s, g)), d === null ? (u = v) : (d.sibling = v), (d = v)));
          return (U && Yi(a, g), u);
        }
        for (h = r(h); !v.done; g++, v = c.next())
          ((v = m(h, a, g, v.value, l)),
            v !== null &&
              (e && ((_ = v.alternate), _ !== null && h.delete(_.key === null ? g : _.key)),
              (s = o(v, s, g)),
              d === null ? (u = v) : (d.sibling = v),
              (d = v)));
        return (
          e &&
            h.forEach(function (e) {
              return t(a, e);
            }),
          U && Yi(a, g),
          u
        );
      }
      function _(e, r, o, c) {
        if (
          (typeof o == `object` &&
            o &&
            o.type === k &&
            o.key === null &&
            o.props.ref === void 0 &&
            (o = o.props.children),
          typeof o == `object` && o)
        ) {
          switch (o.$$typeof) {
            case ee:
              a: {
                for (var l = o.key; r !== null;) {
                  if (r.key === l) {
                    if (((l = o.type), l === k)) {
                      if (r.tag === 7) {
                        (n(e, r.sibling),
                          (c = a(r, o.props.children)),
                          ao(c, o),
                          (c.return = e),
                          (e = c));
                        break a;
                      }
                    } else if (
                      r.elementType === l ||
                      (typeof l == `object` && l && l.$$typeof === I && Qa(l) === r.type)
                    ) {
                      (n(e, r.sibling), (c = a(r, o.props)), ao(c, o), (c.return = e), (e = c));
                      break a;
                    }
                    n(e, r);
                    break;
                  }
                  (t(e, r), (r = r.sibling));
                }
                o.type === k
                  ? ((c = Pi(o.props.children, e.mode, c, o.key)),
                    ao(c, o),
                    (c.return = e),
                    (e = c))
                  : ((c = Ni(o.type, o.key, o.props, null, e.mode, c)),
                    ao(c, o),
                    (c.return = e),
                    (e = c));
              }
              return s(e);
            case te:
              a: {
                for (l = o.key; r !== null;) {
                  if (r.key === l) {
                    if (
                      r.tag === 4 &&
                      r.stateNode.containerInfo === o.containerInfo &&
                      r.stateNode.implementation === o.implementation
                    ) {
                      (n(e, r.sibling), (c = a(r, o.children || [])), (c.return = e), (e = c));
                      break a;
                    }
                    n(e, r);
                    break;
                  }
                  (t(e, r), (r = r.sibling));
                }
                ((c = Li(o, e.mode, c)), (c.return = e), (e = c));
              }
              return s(e);
            case I:
              return ((o = Qa(o)), _(e, r, o, c));
          }
          if (pe(o)) return h(e, r, o, c);
          if (ue(o)) {
            if (((l = ue(o)), typeof l != `function`)) throw Error(i(150));
            return ((o = l.call(o)), g(e, r, o, c));
          }
          if (typeof o.then == `function`) return _(e, r, io(o), c);
          if (o.$$typeof === M) return _(e, r, Ca(e, o), c);
          oo(e, o);
        }
        return (typeof o == `string` && o !== ``) || typeof o == `number` || typeof o == `bigint`
          ? ((o = `` + o),
            r !== null && r.tag === 6
              ? (n(e, r.sibling), (c = a(r, o)), (c.return = e), (e = c))
              : (n(e, r), (c = Fi(o, e.mode, c)), (c.return = e), (e = c)),
            s(e))
          : n(e, r);
      }
      return function (e, t, n, r) {
        try {
          ro = 0;
          var i = _(e, t, n, r);
          return ((no = null), i);
        } catch (t) {
          if (t === Ka || t === Ja) throw t;
          var a = ki(29, t, null, e.mode);
          return ((a.lanes = r), (a.return = e), a);
        }
      };
    }
    var co = so(!0),
      lo = so(!1),
      uo = !1;
    function fo(e) {
      e.updateQueue = {
        baseState: e.memoizedState,
        firstBaseUpdate: null,
        lastBaseUpdate: null,
        shared: { pending: null, lanes: 0, hiddenCallbacks: null },
        callbacks: null,
      };
    }
    function po(e, t) {
      ((e = e.updateQueue),
        t.updateQueue === e &&
          (t.updateQueue = {
            baseState: e.baseState,
            firstBaseUpdate: e.firstBaseUpdate,
            lastBaseUpdate: e.lastBaseUpdate,
            shared: e.shared,
            callbacks: null,
          }));
    }
    function mo(e) {
      return { lane: e, tag: 0, payload: null, callback: null, next: null };
    }
    function ho(e, t, n) {
      var r = e.updateQueue;
      if (r === null) return null;
      if (((r = r.shared), q & 2)) {
        var i = r.pending;
        return (
          i === null ? (t.next = t) : ((t.next = i.next), (i.next = t)),
          (r.pending = t),
          (t = Ei(e)),
          Ti(e, null, n),
          t
        );
      }
      return (Si(e, r, t, n), Ei(e));
    }
    function go(e, t, n) {
      if (((t = t.updateQueue), t !== null && ((t = t.shared), n & 4194048))) {
        var r = t.lanes;
        ((r &= e.pendingLanes), (n |= r), (t.lanes = n), pt(e, n));
      }
    }
    function _o(e, t) {
      var n = e.updateQueue,
        r = e.alternate;
      if (r !== null && ((r = r.updateQueue), n === r)) {
        var i = null,
          a = null;
        if (((n = n.firstBaseUpdate), n !== null)) {
          do {
            var o = { lane: n.lane, tag: n.tag, payload: n.payload, callback: null, next: null };
            (a === null ? (i = a = o) : (a = a.next = o), (n = n.next));
          } while (n !== null);
          a === null ? (i = a = t) : (a = a.next = t);
        } else i = a = t;
        ((n = {
          baseState: r.baseState,
          firstBaseUpdate: i,
          lastBaseUpdate: a,
          shared: r.shared,
          callbacks: r.callbacks,
        }),
          (e.updateQueue = n));
      } else
        ((e = n.lastBaseUpdate),
          e === null ? (n.firstBaseUpdate = t) : (e.next = t),
          (n.lastBaseUpdate = t));
    }
    var vo = !1;
    function yo() {
      if (vo) {
        var e = La;
        if (e !== null) throw e;
      }
    }
    function bo(e, t, n, r) {
      vo = !1;
      var i = e.updateQueue;
      uo = !1;
      var a = i.firstBaseUpdate,
        o = i.lastBaseUpdate,
        s = i.shared.pending;
      if (s !== null) {
        i.shared.pending = null;
        var c = s,
          l = c.next;
        ((c.next = null), o === null ? (a = l) : (o.next = l), (o = c));
        var u = e.alternate;
        u !== null &&
          ((u = u.updateQueue),
          (s = u.lastBaseUpdate),
          s !== o && (s === null ? (u.firstBaseUpdate = l) : (s.next = l), (u.lastBaseUpdate = c)));
      }
      if (a !== null) {
        var d = i.baseState;
        ((o = 0), (u = l = c = null), (s = a));
        do {
          var f = s.lane & -536870913,
            p = f !== s.lane;
          if (p ? (Y & f) === f : (r & f) === f) {
            (f !== 0 && f === Ia && (vo = !0),
              u !== null &&
                (u = u.next =
                  { lane: 0, tag: s.tag, payload: s.payload, callback: null, next: null }));
            a: {
              var m = e,
                h = s;
              f = t;
              var g = n;
              switch (h.tag) {
                case 1:
                  if (((m = h.payload), typeof m == `function`)) {
                    d = m.call(g, d, f);
                    break a;
                  }
                  d = m;
                  break a;
                case 3:
                  m.flags = (m.flags & -65537) | 128;
                case 0:
                  if (
                    ((m = h.payload), (f = typeof m == `function` ? m.call(g, d, f) : m), f == null)
                  )
                    break a;
                  d = D({}, d, f);
                  break a;
                case 2:
                  uo = !0;
              }
            }
            ((f = s.callback),
              f !== null &&
                ((e.flags |= 64),
                p && (e.flags |= 8192),
                (p = i.callbacks),
                p === null ? (i.callbacks = [f]) : p.push(f)));
          } else
            ((p = { lane: f, tag: s.tag, payload: s.payload, callback: s.callback, next: null }),
              u === null ? ((l = u = p), (c = d)) : (u = u.next = p),
              (o |= f));
          if (((s = s.next), s === null)) {
            if (((s = i.shared.pending), s === null)) break;
            ((p = s),
              (s = p.next),
              (p.next = null),
              (i.lastBaseUpdate = p),
              (i.shared.pending = null));
          }
        } while (1);
        (u === null && (c = d),
          (i.baseState = c),
          (i.firstBaseUpdate = l),
          (i.lastBaseUpdate = u),
          a === null && (i.shared.lanes = 0),
          (od |= o),
          (e.lanes = o),
          (e.memoizedState = d));
      }
    }
    function xo(e, t) {
      if (typeof e != `function`) throw Error(i(191, e));
      e.call(t);
    }
    function So(e, t) {
      var n = e.callbacks;
      if (n !== null) for (e.callbacks = null, e = 0; e < n.length; e++) xo(n[e], t);
    }
    var Co = _e(null),
      wo = _e(0);
    function To(e, t) {
      ((e = id), z(wo, e), z(Co, t), (id = e | t.baseLanes));
    }
    function Eo() {
      (z(wo, id), z(Co, Co.current));
    }
    function Do() {
      ((id = wo.current), ve(Co), ve(wo));
    }
    var Oo = _e(null),
      ko = null;
    function Ao(e) {
      var t = e.alternate;
      (z(Fo, Fo.current & 1),
        z(Oo, e),
        ko === null && (t === null || Co.current !== null || t.memoizedState !== null) && (ko = e));
    }
    function jo(e) {
      (z(Fo, Fo.current), z(Oo, e), ko === null && (ko = e));
    }
    function Mo(e) {
      e.tag === 22 ? (z(Fo, Fo.current), z(Oo, e), ko === null && (ko = e)) : No();
    }
    function No() {
      (z(Fo, Fo.current), z(Oo, Oo.current));
    }
    function Po(e) {
      (ve(Oo), ko === e && (ko = null), ve(Fo));
    }
    var Fo = _e(0);
    function Io(e, t) {
      (z(Oo, Oo.current), z(Fo, t));
    }
    function Lo(e) {
      (ve(Fo), ve(Oo), ko === e && (ko = null));
    }
    function Ro(e) {
      for (var t = e; t !== null;) {
        if (t.tag === 13) {
          var n = t.memoizedState;
          if (n !== null && ((n = n.dehydrated), n === null || om(n) || sm(n))) return t;
        } else if (t.tag === 19 && t.memoizedProps.revealOrder !== `independent`) {
          if (t.flags & 128) return t;
        } else if (t.child !== null) {
          ((t.child.return = t), (t = t.child));
          continue;
        }
        if (t === e) break;
        for (; t.sibling === null;) {
          if (t.return === null || t.return === e) return null;
          t = t.return;
        }
        ((t.sibling.return = t.return), (t = t.sibling));
      }
      return null;
    }
    var zo = 0,
      W = null,
      G = null,
      Bo = null,
      Vo = !1,
      Ho = !1,
      Uo = !1,
      Wo = 0,
      Go = 0,
      Ko = null,
      qo = 0;
    function Jo() {
      throw Error(i(321));
    }
    function Yo(e, t) {
      if (t === null) return !1;
      for (var n = 0; n < t.length && n < e.length; n++) if (!Rr(e[n], t[n])) return !1;
      return !0;
    }
    function Xo(e, t, n, r, i, a) {
      return (
        (zo = a),
        (W = t),
        (t.memoizedState = null),
        (t.updateQueue = null),
        (t.lanes = 0),
        (L.H = e === null || e.memoizedState === null ? pc : mc),
        (Uo = !1),
        (a = n(r, i)),
        (Uo = !1),
        Ho && (a = Qo(t, n, r, i)),
        Zo(e),
        a
      );
    }
    function Zo(e) {
      L.H = fc;
      var t = G !== null && G.next !== null;
      if (((zo = 0), (Bo = G = W = null), (Vo = !1), (Go = 0), (Ko = null), t)) throw Error(i(300));
      e === null || jc || ((e = e.dependencies), e !== null && ba(e) && (jc = !0));
    }
    function Qo(e, t, n, r) {
      W = e;
      var a = 0;
      do {
        if ((Ho && (Ko = null), (Go = 0), (Ho = !1), 25 <= a)) throw Error(i(301));
        if (((a += 1), (Bo = G = null), e.updateQueue != null)) {
          var o = e.updateQueue;
          ((o.lastEffect = null),
            (o.events = null),
            (o.stores = null),
            o.memoCache != null && (o.memoCache.index = 0));
        }
        ((L.H = hc), (o = t(n, r)));
      } while (Ho);
      return o;
    }
    function $o() {
      var e = L.H,
        t = e.useState()[0];
      return (
        (t = typeof t.then == `function` ? os(t) : t),
        (e = e.useState()[0]),
        (G === null ? null : G.memoizedState) !== e && (W.flags |= 1024),
        t
      );
    }
    function es() {
      var e = Wo !== 0;
      return ((Wo = 0), e);
    }
    function ts(e, t, n) {
      ((t.updateQueue = e.updateQueue), (t.flags &= -2053), (e.lanes &= ~n));
    }
    function ns(e) {
      if (Vo) {
        for (e = e.memoizedState; e !== null;) {
          var t = e.queue;
          (t !== null && (t.pending = null), (e = e.next));
        }
        Vo = !1;
      }
      ((zo = 0), (Bo = G = W = null), (Ho = !1), (Go = Wo = 0), (Ko = null));
    }
    function rs() {
      var e = { memoizedState: null, baseState: null, baseQueue: null, queue: null, next: null };
      return (Bo === null ? (W.memoizedState = Bo = e) : (Bo = Bo.next = e), Bo);
    }
    function is() {
      if (G === null) {
        var e = W.alternate;
        e = e === null ? null : e.memoizedState;
      } else e = G.next;
      var t = Bo === null ? W.memoizedState : Bo.next;
      if (t !== null) ((Bo = t), (G = e));
      else {
        if (e === null) throw W.alternate === null ? Error(i(467)) : Error(i(310));
        ((G = e),
          (e = {
            memoizedState: G.memoizedState,
            baseState: G.baseState,
            baseQueue: G.baseQueue,
            queue: G.queue,
            next: null,
          }),
          Bo === null ? (W.memoizedState = Bo = e) : (Bo = Bo.next = e));
      }
      return Bo;
    }
    function as() {
      return { lastEffect: null, events: null, stores: null, memoCache: null };
    }
    function os(e) {
      var t = Go;
      return (
        (Go += 1),
        Ko === null && (Ko = []),
        (e = Za(Ko, e, t)),
        (t = W),
        (Bo === null ? t.memoizedState : Bo.next) === null &&
          ((t = t.alternate), (L.H = t === null || t.memoizedState === null ? pc : mc)),
        e
      );
    }
    function ss(e) {
      if (typeof e == `object` && e) {
        if (typeof e.then == `function`) return os(e);
        if (e.$$typeof === ce) return;
        if (e.$$typeof === M) return Sa(e);
      }
      throw Error(i(438, String(e)));
    }
    function cs(e) {
      var t = null,
        n = W.updateQueue;
      if ((n !== null && (t = n.memoCache), t == null)) {
        var r = W.alternate;
        r !== null &&
          ((r = r.updateQueue),
          r !== null &&
            ((r = r.memoCache),
            r != null &&
              (t = {
                data: r.data.map(function (e) {
                  return e.slice();
                }),
                index: 0,
              })));
      }
      if (
        ((t ??= { data: [], index: 0 }),
        n === null && ((n = as()), (W.updateQueue = n)),
        (n.memoCache = t),
        (n = t.data[t.index]),
        n === void 0)
      )
        for (n = t.data[t.index] = Array(e), r = 0; r < e; r++) n[r] = oe;
      return (t.index++, n);
    }
    function ls(e, t) {
      return typeof t == `function` ? t(e) : t;
    }
    function us(e) {
      return ds(is(), G, e);
    }
    function ds(e, t, n) {
      var r = e.queue;
      if (r === null) throw Error(i(311));
      r.lastRenderedReducer = n;
      var a = e.baseQueue,
        o = r.pending;
      if (o !== null) {
        if (a !== null) {
          var s = a.next;
          ((a.next = o.next), (o.next = s));
        }
        ((t.baseQueue = a = o), (r.pending = null));
      }
      if (((o = e.baseState), a === null)) e.memoizedState = o;
      else {
        t = a.next;
        var c = (s = null),
          l = null,
          u = t,
          d = !1;
        do {
          var f = u.lane & -536870913;
          if (f === u.lane ? (zo & f) === f : (Y & f) === f) {
            var p = u.revertLane;
            if (p === 0)
              (l !== null &&
                (l = l.next =
                  {
                    lane: 0,
                    revertLane: 0,
                    gesture: null,
                    action: u.action,
                    hasEagerState: u.hasEagerState,
                    eagerState: u.eagerState,
                    next: null,
                  }),
                f === Ia && (d = !0));
            else if ((zo & p) === p) {
              ((u = u.next), p === Ia && (d = !0));
              continue;
            } else
              ((f = {
                lane: 0,
                revertLane: u.revertLane,
                gesture: null,
                action: u.action,
                hasEagerState: u.hasEagerState,
                eagerState: u.eagerState,
                next: null,
              }),
                l === null ? ((c = l = f), (s = o)) : (l = l.next = f),
                (W.lanes |= p),
                (od |= p));
            ((f = u.action), Uo && n(o, f), (o = u.hasEagerState ? u.eagerState : n(o, f)));
          } else
            ((p = {
              lane: f,
              revertLane: u.revertLane,
              gesture: u.gesture,
              action: u.action,
              hasEagerState: u.hasEagerState,
              eagerState: u.eagerState,
              next: null,
            }),
              l === null ? ((c = l = p), (s = o)) : (l = l.next = p),
              (W.lanes |= f),
              (od |= f));
          u = u.next;
        } while (u !== null && u !== t);
        if (
          (l === null ? (s = o) : (l.next = c),
          !Rr(o, e.memoizedState) && ((jc = !0), d && ((n = La), n !== null)))
        )
          throw n;
        ((e.memoizedState = o), (e.baseState = s), (e.baseQueue = l), (r.lastRenderedState = o));
      }
      return (a === null && (r.lanes = 0), [e.memoizedState, r.dispatch]);
    }
    function fs(e) {
      var t = is(),
        n = t.queue;
      if (n === null) throw Error(i(311));
      n.lastRenderedReducer = e;
      var r = n.dispatch,
        a = n.pending,
        o = t.memoizedState;
      if (a !== null) {
        n.pending = null;
        var s = (a = a.next);
        do ((o = e(o, s.action)), (s = s.next));
        while (s !== a);
        (Rr(o, t.memoizedState) || (jc = !0),
          (t.memoizedState = o),
          t.baseQueue === null && (t.baseState = o),
          (n.lastRenderedState = o));
      }
      return [o, r];
    }
    function ps(e, t, n) {
      var r = W,
        a = is(),
        o = U;
      if (o) {
        if (n === void 0) throw Error(i(407));
        n = n();
      } else n = t();
      var s = !Rr((G || a).memoizedState, n);
      if (
        (s && ((a.memoizedState = n), (jc = !0)),
        (a = a.queue),
        Rs(gs.bind(null, r, a, e), [e]),
        (e = a.getSnapshot !== t || s || (Bo !== null && !!(Bo.memoizedState.tag & 1))),
        Ns(e ? 9 : 8, { destroy: void 0 }, hs.bind(null, r, a, n, t), null),
        e)
      ) {
        if (((r.flags |= 2048), $u === null)) throw Error(i(349));
        o || zo & 127 || ms(r, t, n);
      }
      return n;
    }
    function ms(e, t, n) {
      ((e.flags |= 16384),
        (e = { getSnapshot: t, value: n }),
        (t = W.updateQueue),
        t === null
          ? ((t = as()), (W.updateQueue = t), (t.stores = [e]))
          : ((n = t.stores), n === null ? (t.stores = [e]) : n.push(e)));
    }
    function hs(e, t, n, r) {
      ((t.value = n), (t.getSnapshot = r), _s(t) && vs(e));
    }
    function gs(e, t, n) {
      return n(function () {
        _s(t) && vs(e);
      });
    }
    function _s(e) {
      var t = e.getSnapshot;
      e = e.value;
      try {
        var n = t();
        return !Rr(e, n);
      } catch {
        return !0;
      }
    }
    function vs(e) {
      var t = wi(e, 2);
      t !== null && Pd(t, e, 2);
    }
    function ys(e) {
      var t = rs();
      if (typeof e == `function`) {
        var n = e;
        if (((e = n()), Uo)) {
          Xe(!0);
          try {
            n();
          } finally {
            Xe(!1);
          }
        }
      }
      return (
        (t.memoizedState = t.baseState = e),
        (t.queue = {
          pending: null,
          lanes: 0,
          dispatch: null,
          lastRenderedReducer: ls,
          lastRenderedState: e,
        }),
        t
      );
    }
    function bs(e, t, n, r) {
      return ((e.baseState = n), ds(e, G, typeof r == `function` ? r : ls));
    }
    function xs(e, t, n, r, a) {
      if (lc(e)) throw Error(i(485));
      if (((e = t.action), e !== null)) {
        var o = {
          payload: a,
          action: e,
          next: null,
          isTransition: !0,
          status: `pending`,
          value: null,
          reason: null,
          listeners: [],
          then: function (e) {
            o.listeners.push(e);
          },
        };
        (L.T === null ? (o.isTransition = !1) : n(!0),
          r(o),
          (n = t.pending),
          n === null
            ? ((o.next = t.pending = o), Ss(t, o))
            : ((o.next = n.next), (t.pending = n.next = o)));
      }
    }
    function Ss(e, t) {
      var n = t.action,
        r = t.payload,
        i = e.state;
      if (t.isTransition) {
        var a = L.T,
          o = {};
        ((o.types = a === null ? null : a.types), (L.T = o));
        try {
          var s = n(i, r),
            c = L.S;
          (c !== null && c(o, s), Cs(e, t, s));
        } catch (n) {
          Ts(e, t, n);
        } finally {
          (a !== null && o.types !== null && (a.types = o.types), (L.T = a));
        }
      } else
        try {
          ((a = n(i, r)), Cs(e, t, a));
        } catch (n) {
          Ts(e, t, n);
        }
    }
    function Cs(e, t, n) {
      typeof n == `object` && n && typeof n.then == `function`
        ? n.then(
            function (n) {
              ws(e, t, n);
            },
            function (n) {
              return Ts(e, t, n);
            },
          )
        : ws(e, t, n);
    }
    function ws(e, t, n) {
      ((t.status = `fulfilled`),
        (t.value = n),
        Es(t),
        (e.state = n),
        (t = e.pending),
        t !== null &&
          ((n = t.next), n === t ? (e.pending = null) : ((n = n.next), (t.next = n), Ss(e, n))));
    }
    function Ts(e, t, n) {
      var r = e.pending;
      if (((e.pending = null), r !== null)) {
        r = r.next;
        do ((t.status = `rejected`), (t.reason = n), Es(t), (t = t.next));
        while (t !== r);
      }
      e.action = null;
    }
    function Es(e) {
      e = e.listeners;
      for (var t = 0; t < e.length; t++) (0, e[t])();
    }
    function Ds(e, t) {
      return t;
    }
    function Os(e, t) {
      if (U) {
        var n = $u.formState;
        if (n !== null) {
          a: {
            var r = W;
            if (U) {
              if (ta) {
                b: {
                  for (var i = ta, a = ra; i.nodeType !== 8;)
                    if (!a || ((i = lm(i.nextSibling)), i === null)) {
                      i = null;
                      break b;
                    }
                  ((a = i.data), (i = a === `F!` || a === `F` ? i : null));
                }
                if (i) {
                  ((ta = lm(i.nextSibling)), (r = i.data === `F!`));
                  break a;
                }
              }
              aa(r);
            }
            r = !1;
          }
          r && (t = n[0]);
        }
      }
      return (
        (n = rs()),
        (n.memoizedState = n.baseState = t),
        (r = {
          pending: null,
          lanes: 0,
          dispatch: null,
          lastRenderedReducer: Ds,
          lastRenderedState: t,
        }),
        (n.queue = r),
        (n = oc.bind(null, W, r)),
        (r.dispatch = n),
        (r = ys(!1)),
        (a = cc.bind(null, W, !1, r.queue)),
        (r = rs()),
        (i = { state: t, dispatch: null, action: e, pending: null }),
        (r.queue = i),
        (n = xs.bind(null, W, i, a, n)),
        (i.dispatch = n),
        (r.memoizedState = e),
        [t, n, !1]
      );
    }
    function ks(e) {
      return As(is(), G, e);
    }
    function As(e, t, n) {
      if (
        ((t = ds(e, t, Ds)[0]),
        (e = us(ls)[0]),
        typeof t == `object` && t && typeof t.then == `function`)
      )
        try {
          var r = os(t);
        } catch (e) {
          throw e === Ka ? Ja : e;
        }
      else r = t;
      t = is();
      var i = t.queue,
        a = i.dispatch;
      return (
        n !== t.memoizedState &&
          ((W.flags |= 2048), Ns(9, { destroy: void 0 }, js.bind(null, i, n), null)),
        [r, a, e]
      );
    }
    function js(e, t) {
      e.action = t;
    }
    function Ms(e) {
      var t = is(),
        n = G;
      if (n !== null) return As(t, n, e);
      (is(), (t = t.memoizedState), (n = is()));
      var r = n.queue.dispatch;
      return ((n.memoizedState = e), [t, r, !1]);
    }
    function Ns(e, t, n, r) {
      return (
        (e = { tag: e, create: n, deps: r, inst: t, next: null }),
        (t = W.updateQueue),
        t === null && ((t = as()), (W.updateQueue = t)),
        (n = t.lastEffect),
        n === null
          ? (t.lastEffect = e.next = e)
          : ((r = n.next), (n.next = e), (e.next = r), (t.lastEffect = e)),
        e
      );
    }
    function Ps() {
      return is().memoizedState;
    }
    function Fs(e, t, n, r) {
      var i = rs();
      ((W.flags |= e),
        (i.memoizedState = Ns(1 | t, { destroy: void 0 }, n, r === void 0 ? null : r)));
    }
    function Is(e, t, n, r) {
      var i = is();
      r = r === void 0 ? null : r;
      var a = i.memoizedState.inst;
      G !== null && r !== null && Yo(r, G.memoizedState.deps)
        ? (i.memoizedState = Ns(t, a, n, r))
        : ((W.flags |= e), (i.memoizedState = Ns(1 | t, a, n, r)));
    }
    function Ls(e, t) {
      Fs(8390656, 8, e, t);
    }
    function Rs(e, t) {
      Is(2048, 8, e, t);
    }
    function zs(e) {
      W.flags |= 4;
      var t = W.updateQueue;
      if (t === null) ((t = as()), (W.updateQueue = t), (t.events = [e]));
      else {
        var n = t.events;
        n === null ? (t.events = [e]) : n.push(e);
      }
    }
    function Bs(e) {
      var t = is().memoizedState;
      return (
        zs({ ref: t, nextImpl: e }),
        function () {
          if (q & 2) throw Error(i(440));
          return t.impl.apply(void 0, arguments);
        }
      );
    }
    function Vs(e, t) {
      return Is(4, 2, e, t);
    }
    function Hs(e, t) {
      return Is(4, 4, e, t);
    }
    function Us(e, t) {
      if (typeof t == `function`) {
        e = e();
        var n = t(e);
        return function () {
          typeof n == `function` ? n() : t(null);
        };
      }
      if (t != null)
        return (
          (e = e()),
          (t.current = e),
          function () {
            t.current = null;
          }
        );
    }
    function Ws(e, t, n) {
      ((n = n == null ? null : n.concat([e])), Is(4, 4, Us.bind(null, t, e), n));
    }
    function Gs() {}
    function Ks(e, t) {
      var n = is();
      t = t === void 0 ? null : t;
      var r = n.memoizedState;
      return t !== null && Yo(t, r[1]) ? r[0] : ((n.memoizedState = [e, t]), e);
    }
    function qs(e, t) {
      var n = is();
      t = t === void 0 ? null : t;
      var r = n.memoizedState;
      if (t !== null && Yo(t, r[1])) return r[0];
      if (((r = e()), Uo)) {
        Xe(!0);
        try {
          e();
        } finally {
          Xe(!1);
        }
      }
      return ((n.memoizedState = [r, t]), r);
    }
    function Js(e, t, n) {
      return n === void 0 || (zo & 1073741824 && !(Y & 261930))
        ? (e.memoizedState = t)
        : ((e.memoizedState = n), (e = Md()), (W.lanes |= e), (od |= e), n);
    }
    function Ys(e, t, n, r) {
      return Rr(n, t)
        ? n
        : Co.current === null
          ? !(zo & 106) || (zo & 1073741824 && !(Y & 261930))
            ? ((jc = !0), (e.memoizedState = n))
            : ((e = Md()), (W.lanes |= e), (od |= e), t)
          : ((e = Js(e, n, r)), Rr(e, t) || (jc = !0), e);
    }
    function Xs(e, t, n, r, i) {
      var a = R.p;
      R.p = a !== 0 && 8 > a ? a : 8;
      var o = L.T,
        s = {};
      ((s.types = o === null ? null : o.types), (L.T = s), cc(e, !1, t, n));
      try {
        var c = i(),
          l = L.S;
        (l !== null && l(s, c),
          typeof c == `object` && c && typeof c.then == `function`
            ? sc(e, t, Ba(c, r), jd(e))
            : sc(e, t, r, jd(e)));
      } catch (n) {
        sc(e, t, { then: function () {}, status: `rejected`, reason: n }, jd());
      } finally {
        ((R.p = a), o !== null && s.types !== null && (o.types = s.types), (L.T = o));
      }
    }
    function Zs() {}
    function Qs(e, t, n, r) {
      if (e.tag !== 5) throw Error(i(476));
      var a = $s(e).queue;
      Xs(
        e,
        a,
        t,
        me,
        n === null
          ? Zs
          : function () {
              return (ec(e), n(r));
            },
      );
    }
    function $s(e) {
      var t = e.memoizedState;
      if (t !== null) return t;
      t = {
        memoizedState: me,
        baseState: me,
        baseQueue: null,
        queue: {
          pending: null,
          lanes: 0,
          dispatch: null,
          lastRenderedReducer: ls,
          lastRenderedState: me,
        },
        next: null,
      };
      var n = {};
      return (
        (t.next = {
          memoizedState: n,
          baseState: n,
          baseQueue: null,
          queue: {
            pending: null,
            lanes: 0,
            dispatch: null,
            lastRenderedReducer: ls,
            lastRenderedState: n,
          },
          next: null,
        }),
        (e.memoizedState = t),
        (e = e.alternate),
        e !== null && (e.memoizedState = t),
        t
      );
    }
    function ec(e) {
      var t = $s(e);
      (t.next === null && (t = e.alternate.memoizedState), sc(e, t.next.queue, {}, jd()));
    }
    function tc() {
      return Sa(sh);
    }
    function nc() {
      return is().memoizedState;
    }
    function rc() {
      return is().memoizedState;
    }
    function ic(e) {
      for (var t = e.return; t !== null;) {
        switch (t.tag) {
          case 24:
          case 3:
            var n = jd();
            e = mo(n);
            var r = ho(t, e, n);
            (r !== null && (Pd(r, t, n), go(r, t, n)), (t = { cache: ka() }), (e.payload = t));
            return;
        }
        t = t.return;
      }
    }
    function ac(e, t, n) {
      var r = jd();
      ((n = {
        lane: r,
        revertLane: 0,
        gesture: null,
        action: n,
        hasEagerState: !1,
        eagerState: null,
        next: null,
      }),
        lc(e) ? uc(t, n) : ((n = Ci(e, t, n, r)), n !== null && (Pd(n, e, r), dc(n, t, r))));
    }
    function oc(e, t, n) {
      sc(e, t, n, jd());
    }
    function sc(e, t, n, r) {
      var i = {
        lane: r,
        revertLane: 0,
        gesture: null,
        action: n,
        hasEagerState: !1,
        eagerState: null,
        next: null,
      };
      if (lc(e)) uc(t, i);
      else {
        var a = e.alternate;
        if (
          e.lanes === 0 &&
          (a === null || a.lanes === 0) &&
          ((a = t.lastRenderedReducer), a !== null)
        )
          try {
            var o = t.lastRenderedState,
              s = a(o, n);
            if (((i.hasEagerState = !0), (i.eagerState = s), Rr(s, o)))
              return (Si(e, t, i, 0), $u === null && xi(), !1);
          } catch {}
        if (((n = Ci(e, t, i, r)), n !== null)) return (Pd(n, e, r), dc(n, t, r), !0);
      }
      return !1;
    }
    function cc(e, t, n, r) {
      if (
        ((r = {
          lane: 2,
          revertLane: Pf(),
          gesture: null,
          action: r,
          hasEagerState: !1,
          eagerState: null,
          next: null,
        }),
        lc(e))
      ) {
        if (t) throw Error(i(479));
      } else ((t = Ci(e, n, r, 2)), t !== null && Pd(t, e, 2));
    }
    function lc(e) {
      var t = e.alternate;
      return e === W || (t !== null && t === W);
    }
    function uc(e, t) {
      Ho = Vo = !0;
      var n = e.pending;
      (n === null ? (t.next = t) : ((t.next = n.next), (n.next = t)), (e.pending = t));
    }
    function dc(e, t, n) {
      if (n & 4194048) {
        var r = t.lanes;
        ((r &= e.pendingLanes), (n |= r), (t.lanes = n), pt(e, n));
      }
    }
    var fc = {
        readContext: Sa,
        use: ss,
        useCallback: Jo,
        useContext: Jo,
        useEffect: Jo,
        useImperativeHandle: Jo,
        useLayoutEffect: Jo,
        useInsertionEffect: Jo,
        useMemo: Jo,
        useReducer: Jo,
        useRef: Jo,
        useState: Jo,
        useDebugValue: Jo,
        useDeferredValue: Jo,
        useTransition: Jo,
        useSyncExternalStore: Jo,
        useId: Jo,
        useHostTransitionStatus: Jo,
        useFormState: Jo,
        useActionState: Jo,
        useOptimistic: Jo,
        useMemoCache: Jo,
        useCacheRefresh: Jo,
        useEffectEvent: Jo,
      },
      pc = {
        readContext: Sa,
        use: ss,
        useCallback: function (e, t) {
          return ((rs().memoizedState = [e, t === void 0 ? null : t]), e);
        },
        useContext: Sa,
        useEffect: Ls,
        useImperativeHandle: function (e, t, n) {
          ((n = n == null ? null : n.concat([e])), Fs(4194308, 4, Us.bind(null, t, e), n));
        },
        useLayoutEffect: function (e, t) {
          return Fs(4194308, 4, e, t);
        },
        useInsertionEffect: function (e, t) {
          Fs(4, 2, e, t);
        },
        useMemo: function (e, t) {
          var n = rs();
          t = t === void 0 ? null : t;
          var r = e();
          if (Uo) {
            Xe(!0);
            try {
              e();
            } finally {
              Xe(!1);
            }
          }
          return ((n.memoizedState = [r, t]), r);
        },
        useReducer: function (e, t, n) {
          var r = rs();
          if (n !== void 0) {
            var i = n(t);
            if (Uo) {
              Xe(!0);
              try {
                n(t);
              } finally {
                Xe(!1);
              }
            }
          } else i = t;
          return (
            (r.memoizedState = r.baseState = i),
            (e = {
              pending: null,
              lanes: 0,
              dispatch: null,
              lastRenderedReducer: e,
              lastRenderedState: i,
            }),
            (r.queue = e),
            (e = e.dispatch = ac.bind(null, W, e)),
            [r.memoizedState, e]
          );
        },
        useRef: function (e) {
          var t = rs();
          return ((e = { current: e }), (t.memoizedState = e));
        },
        useState: function (e) {
          e = ys(e);
          var t = e.queue,
            n = oc.bind(null, W, t);
          return ((t.dispatch = n), [e.memoizedState, n]);
        },
        useDebugValue: Gs,
        useDeferredValue: function (e, t) {
          return Js(rs(), e, t);
        },
        useTransition: function () {
          var e = ys(!1);
          return ((e = Xs.bind(null, W, e.queue, !0, !1)), (rs().memoizedState = e), [!1, e]);
        },
        useSyncExternalStore: function (e, t, n) {
          var r = W,
            a = rs();
          if (U) {
            if (n === void 0) throw Error(i(407));
            n = n();
          } else {
            if (((n = t()), $u === null)) throw Error(i(349));
            Y & 127 || ms(r, t, n);
          }
          a.memoizedState = n;
          var o = { value: n, getSnapshot: t };
          return (
            (a.queue = o),
            Ls(gs.bind(null, r, o, e), [e]),
            (r.flags |= 2048),
            Ns(9, { destroy: void 0 }, hs.bind(null, r, o, n, t), null),
            n
          );
        },
        useId: function () {
          var e = rs(),
            t = $u.identifierPrefix;
          if (U) {
            var n = Ji,
              r = qi;
            ((n = (r & ~(1 << (32 - Ze(r) - 1))).toString(32) + n),
              (t = `_` + t + `R_` + n),
              (n = Wo++),
              0 < n && (t += `H` + n.toString(32)),
              (t += `_`));
          } else ((n = qo++), (t = `_` + t + `r_` + n.toString(32) + `_`));
          return (e.memoizedState = t);
        },
        useHostTransitionStatus: tc,
        useFormState: Os,
        useActionState: Os,
        useOptimistic: function (e) {
          var t = rs();
          t.memoizedState = t.baseState = e;
          var n = {
            pending: null,
            lanes: 0,
            dispatch: null,
            lastRenderedReducer: null,
            lastRenderedState: null,
          };
          return ((t.queue = n), (t = cc.bind(null, W, !0, n)), (n.dispatch = t), [e, t]);
        },
        useMemoCache: cs,
        useCacheRefresh: function () {
          return (rs().memoizedState = ic.bind(null, W));
        },
        useEffectEvent: function (e) {
          var t = rs(),
            n = { impl: e };
          return (
            (t.memoizedState = n),
            function () {
              if (q & 2) throw Error(i(440));
              return n.impl.apply(void 0, arguments);
            }
          );
        },
      },
      mc = {
        readContext: Sa,
        use: ss,
        useCallback: Ks,
        useContext: Sa,
        useEffect: Rs,
        useImperativeHandle: Ws,
        useInsertionEffect: Vs,
        useLayoutEffect: Hs,
        useMemo: qs,
        useReducer: us,
        useRef: Ps,
        useState: function () {
          return us(ls);
        },
        useDebugValue: Gs,
        useDeferredValue: function (e, t) {
          return Ys(is(), G.memoizedState, e, t);
        },
        useTransition: function () {
          var e = us(ls)[0],
            t = is().memoizedState;
          return [typeof e == `boolean` ? e : os(e), t];
        },
        useSyncExternalStore: ps,
        useId: nc,
        useHostTransitionStatus: tc,
        useFormState: ks,
        useActionState: ks,
        useOptimistic: function (e, t) {
          return bs(is(), G, e, t);
        },
        useMemoCache: cs,
        useCacheRefresh: rc,
        useEffectEvent: Bs,
      },
      hc = {
        readContext: Sa,
        use: ss,
        useCallback: Ks,
        useContext: Sa,
        useEffect: Rs,
        useImperativeHandle: Ws,
        useInsertionEffect: Vs,
        useLayoutEffect: Hs,
        useMemo: qs,
        useReducer: fs,
        useRef: Ps,
        useState: function () {
          return fs(ls);
        },
        useDebugValue: Gs,
        useDeferredValue: function (e, t) {
          var n = is();
          return G === null ? Js(n, e, t) : Ys(n, G.memoizedState, e, t);
        },
        useTransition: function () {
          var e = fs(ls)[0],
            t = is().memoizedState;
          return [typeof e == `boolean` ? e : os(e), t];
        },
        useSyncExternalStore: ps,
        useId: nc,
        useHostTransitionStatus: tc,
        useFormState: Ms,
        useActionState: Ms,
        useOptimistic: function (e, t) {
          var n = is();
          return G === null ? ((n.baseState = e), [e, n.queue.dispatch]) : bs(n, G, e, t);
        },
        useMemoCache: cs,
        useCacheRefresh: rc,
        useEffectEvent: Bs,
      };
    function gc(e, t, n, r) {
      ((t = e.memoizedState),
        (n = n(r, t)),
        (n = n == null ? t : D({}, t, n)),
        (e.memoizedState = n),
        e.lanes === 0 && (e.updateQueue.baseState = n));
    }
    var _c = {
      enqueueSetState: function (e, t, n) {
        e = e._reactInternals;
        var r = jd(),
          i = mo(r);
        ((i.payload = t),
          n != null && (i.callback = n),
          (t = ho(e, i, r)),
          t !== null && (Pd(t, e, r), go(t, e, r)));
      },
      enqueueReplaceState: function (e, t, n) {
        e = e._reactInternals;
        var r = jd(),
          i = mo(r);
        ((i.tag = 1),
          (i.payload = t),
          n != null && (i.callback = n),
          (t = ho(e, i, r)),
          t !== null && (Pd(t, e, r), go(t, e, r)));
      },
      enqueueForceUpdate: function (e, t) {
        e = e._reactInternals;
        var n = jd(),
          r = mo(n);
        ((r.tag = 2),
          t != null && (r.callback = t),
          (t = ho(e, r, n)),
          t !== null && (Pd(t, e, n), go(t, e, n)));
      },
    };
    function vc(e, t, n, r, i, a, o) {
      return (
        (e = e.stateNode),
        typeof e.shouldComponentUpdate == `function`
          ? e.shouldComponentUpdate(r, a, o)
          : t.prototype && t.prototype.isPureReactComponent
            ? !zr(n, r) || !zr(i, a)
            : !0
      );
    }
    function yc(e, t, n, r) {
      ((e = t.state),
        typeof t.componentWillReceiveProps == `function` && t.componentWillReceiveProps(n, r),
        typeof t.UNSAFE_componentWillReceiveProps == `function` &&
          t.UNSAFE_componentWillReceiveProps(n, r),
        t.state !== e && _c.enqueueReplaceState(t, t.state, null));
    }
    function bc(e, t) {
      var n = t;
      if (`ref` in t) for (var r in ((n = {}), t)) r !== `ref` && (n[r] = t[r]);
      if ((e = e.defaultProps))
        for (var i in (n === t && (n = D({}, n)), e)) n[i] === void 0 && (n[i] = e[i]);
      return n;
    }
    function xc(e) {
      _i(e);
    }
    function Sc(e) {
      console.error(e);
    }
    function Cc(e) {
      _i(e);
    }
    function wc(e, t) {
      try {
        var n = e.onUncaughtError;
        n(t.value, { componentStack: t.stack });
      } catch (e) {
        setTimeout(function () {
          throw e;
        });
      }
    }
    function Tc(e, t, n) {
      try {
        var r = e.onCaughtError;
        r(n.value, { componentStack: n.stack, errorBoundary: t.tag === 1 ? t.stateNode : null });
      } catch (e) {
        setTimeout(function () {
          throw e;
        });
      }
    }
    function Ec(e, t, n) {
      return (
        (n = mo(n)),
        (n.tag = 3),
        (n.payload = { element: null }),
        (n.callback = function () {
          wc(e, t);
        }),
        n
      );
    }
    function Dc(e) {
      return ((e = mo(e)), (e.tag = 3), e);
    }
    function Oc(e, t, n, r) {
      var i = n.type.getDerivedStateFromError;
      if (typeof i == `function`) {
        var a = r.value;
        ((e.payload = function () {
          return i(a);
        }),
          (e.callback = function () {
            Tc(t, n, r);
          }));
      }
      var o = n.stateNode;
      o !== null &&
        typeof o.componentDidCatch == `function` &&
        (e.callback = function () {
          (Tc(t, n, r),
            typeof i != `function` && (vd === null ? (vd = new Set([this])) : vd.add(this)));
          var e = r.stack;
          this.componentDidCatch(r.value, { componentStack: e === null ? `` : e });
        });
    }
    function kc(e, t, n, r, a) {
      if (((n.flags |= 32768), typeof r == `object` && r && typeof r.then == `function`)) {
        if (((t = n.alternate), t !== null && ya(t, n, a, !0), (n = Oo.current), n !== null)) {
          switch (n.tag) {
            case 31:
            case 13:
            case 19:
              return (
                ko === null ? Kd() : n.alternate === null && ad === 0 && (ad = 3),
                (n.flags &= -257),
                (n.flags |= 65536),
                (n.lanes = a),
                r === Ya
                  ? (n.flags |= 16384)
                  : ((t = n.updateQueue),
                    t === null ? (n.updateQueue = new Set([r])) : t.add(r),
                    mf(e, r, a)),
                !1
              );
            case 22:
              return (
                (n.flags |= 65536),
                r === Ya
                  ? (n.flags |= 16384)
                  : ((t = n.updateQueue),
                    t === null
                      ? ((t = {
                          transitions: null,
                          markerInstances: null,
                          retryQueue: new Set([r]),
                        }),
                        (n.updateQueue = t))
                      : ((n = t.retryQueue), n === null ? (t.retryQueue = new Set([r])) : n.add(r)),
                    mf(e, r, a)),
                !1
              );
          }
          throw Error(i(435, n.tag));
        }
        return (mf(e, r, a), Kd(), !1);
      }
      if (U)
        return (
          (t = Oo.current),
          t === null
            ? (r !== ia && ((t = Error(i(423), { cause: r })), da(zi(t, n))),
              (e = e.current.alternate),
              (e.flags |= 65536),
              (a &= -a),
              (e.lanes |= a),
              (r = zi(r, n)),
              (a = Ec(e.stateNode, r, a)),
              _o(e, a),
              ad !== 4 && (ad = 2))
            : (!(t.flags & 65536) && (t.flags |= 256),
              (t.flags |= 65536),
              (t.lanes = a),
              r !== ia && ((e = Error(i(422), { cause: r })), da(zi(e, n)))),
          !1
        );
      var o = Error(i(520), { cause: r });
      if (((o = zi(o, n)), dd === null ? (dd = [o]) : dd.push(o), ad !== 4 && (ad = 2), t === null))
        return !0;
      ((r = zi(r, n)), (n = t));
      do {
        switch (n.tag) {
          case 3:
            return (
              (n.flags |= 65536),
              (e = a & -a),
              (n.lanes |= e),
              (e = Ec(n.stateNode, r, e)),
              _o(n, e),
              !1
            );
          case 1:
            if (
              ((t = n.type),
              (o = n.stateNode),
              !(n.flags & 128) &&
                (typeof t.getDerivedStateFromError == `function` ||
                  (o !== null &&
                    typeof o.componentDidCatch == `function` &&
                    (vd === null || !vd.has(o)))))
            )
              return (
                (n.flags |= 65536),
                (a &= -a),
                (n.lanes |= a),
                (a = Dc(a)),
                Oc(a, e, n, r),
                _o(n, a),
                !1
              );
            break;
          case 22:
            if (n.memoizedState !== null) return ((n.flags |= 65536), !1);
        }
        n = n.return;
      } while (n !== null);
      return !1;
    }
    var Ac = Error(i(461)),
      jc = !1;
    function Mc(e, t, n, r) {
      t.child = e === null ? lo(t, null, n, r) : co(t, e.child, n, r);
    }
    function Nc(e, t, n, r, i) {
      n = n.render;
      var a = t.ref;
      if (`ref` in r) {
        var o = {};
        for (var s in r) s !== `ref` && (o[s] = r[s]);
      } else o = r;
      return (
        xa(t),
        (r = Xo(e, t, n, o, a, i)),
        (s = es()),
        e !== null && !jc
          ? (ts(e, t, i), sl(e, t, i))
          : (U && s && Zi(t), (t.flags |= 1), Mc(e, t, r, i), t.child)
      );
    }
    function Pc(e, t, n, r, i) {
      if (e === null) {
        var a = n.type;
        return typeof a == `function` && !Ai(a) && a.defaultProps === void 0 && n.compare === null
          ? ((t.tag = 15), (t.type = a), Fc(e, t, a, r, i))
          : ((e = Ni(n.type, null, r, t, t.mode, i)),
            (e.ref = t.ref),
            (e.return = t),
            (t.child = e));
      }
      if (((a = e.child), !cl(e, i))) {
        var o = a.memoizedProps;
        if (((n = n.compare), (n = n === null ? zr : n), n(o, r) && e.ref === t.ref))
          return sl(e, t, i);
      }
      return ((t.flags |= 1), (e = ji(a, r)), (e.ref = t.ref), (e.return = t), (t.child = e));
    }
    function Fc(e, t, n, r, i) {
      if (e !== null) {
        var a = e.memoizedProps;
        if (zr(a, r) && e.ref === t.ref) {
          if (((jc = !1), (t.pendingProps = r = a), cl(e, i))) e.flags & 131072 && (jc = !0);
          else return ((t.lanes = e.lanes), sl(e, t, i));
        }
      }
      return Uc(e, t, n, r, i);
    }
    function Ic(e, t, n, r) {
      var i = r.children,
        a = e === null ? null : e.memoizedState;
      if (
        (e === null &&
          t.stateNode === null &&
          (t.stateNode = {
            _visibility: 1,
            _pendingMarkers: null,
            _retryCache: null,
            _transitions: null,
          }),
        r.mode === `hidden`)
      ) {
        if (t.flags & 128) {
          if (((a = a === null ? n : a.baseLanes | n), e !== null)) {
            for (r = t.child = e.child, i = 0; r !== null;)
              ((i = i | r.lanes | r.childLanes), (r = r.sibling));
            r = i & ~a;
          } else ((r = 0), (t.child = null));
          return Rc(e, t, a, n, r);
        }
        if (n & 536870912)
          ((t.memoizedState = { baseLanes: 0, cachePool: null }),
            e !== null && Wa(t, a === null ? null : a.cachePool),
            a === null ? Eo() : To(t, a),
            Mo(t));
        else return ((r = t.lanes = 536870912), Rc(e, t, a === null ? n : a.baseLanes | n, n, r));
      } else
        a === null
          ? (e !== null && Wa(t, null), Eo(), No())
          : (Wa(t, a.cachePool), To(t, a), No(), (t.memoizedState = null));
      return (Mc(e, t, i, n), t.child);
    }
    function Lc(e, t) {
      return (
        (e !== null && e.tag === 22) ||
          t.stateNode !== null ||
          (t.stateNode = {
            _visibility: 1,
            _pendingMarkers: null,
            _retryCache: null,
            _transitions: null,
          }),
        t.sibling
      );
    }
    function Rc(e, t, n, r, i) {
      var a = Ua();
      return (
        (a = a === null ? null : { parent: Oa._currentValue, pool: a }),
        (t.memoizedState = { baseLanes: n, cachePool: a }),
        e !== null && Wa(t, null),
        Eo(),
        Mo(t),
        e !== null && ya(e, t, r, !0),
        (t.childLanes = i),
        null
      );
    }
    function zc(e, t) {
      return (
        (t = Qc({ mode: t.mode, children: t.children }, e.mode)),
        (t.ref = e.ref),
        (e.child = t),
        (t.return = e),
        t
      );
    }
    function Bc(e, t, n) {
      return (
        co(t, e.child, null, n),
        (e = zc(t, t.pendingProps)),
        (e.flags |= 2),
        Po(t),
        (t.memoizedState = null),
        e
      );
    }
    function Vc(e, t, n) {
      var r = t.pendingProps,
        a = !!(t.flags & 128);
      if (((t.flags &= -129), e === null)) {
        if (U) {
          if (r.mode === `hidden`)
            return (
              (e = zc(t, r)),
              (t.lanes = 536870912),
              (e.memoizedState = { baseLanes: 0, cachePool: null }),
              Lc(null, e)
            );
          if (
            (jo(t),
            (e = ta)
              ? ((e = am(e, ra)),
                (e = e !== null && e.data === `&` ? e : null),
                e !== null &&
                  ((t.memoizedState = {
                    dehydrated: e,
                    treeContext: Ki === null ? null : { id: qi, overflow: Ji },
                    retryLane: 536870912,
                    hydrationErrors: null,
                  }),
                  (n = Ii(e)),
                  (n.return = t),
                  (t.child = n),
                  (ea = t),
                  (ta = null)))
              : (e = null),
            e === null)
          )
            throw aa(t);
          return ((t.lanes = 536870912), null);
        }
        return zc(t, r);
      }
      var o = e.memoizedState;
      if (o !== null) {
        var s = o.dehydrated;
        if ((jo(t), a)) {
          if (t.flags & 256) ((t.flags &= -257), (t = Bc(e, t, n)));
          else if (t.memoizedState !== null) ((t.child = e.child), (t.flags |= 128), (t = null));
          else throw Error(i(558));
        } else if ((jc || ya(e, t, n, !1), (a = (n & e.childLanes) !== 0), jc || a)) {
          if (Co.current === null) {
            if (((r = $u), r !== null && ((s = mt(r, n)), s !== 0 && s !== o.retryLane)))
              throw ((o.retryLane = s), wi(e, s), Pd(r, e, s), Ac);
            Kd();
          }
          t = Bc(e, t, n);
        } else
          ((e = o.treeContext),
            (ta = lm(s.nextSibling)),
            (ea = t),
            (U = !0),
            (na = null),
            (ra = !1),
            e !== null && $i(t, e),
            (t = zc(t, r)),
            (t.flags |= 134221824));
        return t;
      }
      return (
        (e = ji(e.child, { mode: r.mode, children: r.children })),
        (e.ref = t.ref),
        (t.child = e),
        (e.return = t),
        e
      );
    }
    function Hc(e, t) {
      var n = t.ref;
      if (n === null) e !== null && e.ref !== null && (t.flags |= 4194816);
      else {
        if (typeof n != `function` && typeof n != `object`) throw Error(i(284));
        (e === null || e.ref !== n) && (t.flags |= 4194816);
      }
    }
    function Uc(e, t, n, r, i) {
      return (
        xa(t),
        (n = Xo(e, t, n, r, void 0, i)),
        (r = es()),
        e !== null && !jc
          ? (ts(e, t, i), sl(e, t, i))
          : (U && r && Zi(t), (t.flags |= 1), Mc(e, t, n, i), t.child)
      );
    }
    function Wc(e, t, n, r, i, a) {
      return (
        xa(t),
        (t.updateQueue = null),
        (n = Qo(t, r, n, i)),
        Zo(e),
        (r = es()),
        e !== null && !jc
          ? (ts(e, t, a), sl(e, t, a))
          : (U && r && Zi(t), (t.flags |= 1), Mc(e, t, n, a), t.child)
      );
    }
    function Gc(e, t, n, r, i) {
      if ((xa(t), t.stateNode === null)) {
        var a = Di,
          o = n.contextType;
        (typeof o == `object` && o && (a = Sa(o)),
          (a = new n(r, a)),
          (t.memoizedState = a.state !== null && a.state !== void 0 ? a.state : null),
          (a.updater = _c),
          (t.stateNode = a),
          (a._reactInternals = t),
          (a = t.stateNode),
          (a.props = r),
          (a.state = t.memoizedState),
          (a.refs = {}),
          fo(t),
          (o = n.contextType),
          (a.context = typeof o == `object` && o ? Sa(o) : Di),
          (a.state = t.memoizedState),
          (o = n.getDerivedStateFromProps),
          typeof o == `function` && (gc(t, n, o, r), (a.state = t.memoizedState)),
          typeof n.getDerivedStateFromProps == `function` ||
            typeof a.getSnapshotBeforeUpdate == `function` ||
            (typeof a.UNSAFE_componentWillMount != `function` &&
              typeof a.componentWillMount != `function`) ||
            ((o = a.state),
            typeof a.componentWillMount == `function` && a.componentWillMount(),
            typeof a.UNSAFE_componentWillMount == `function` && a.UNSAFE_componentWillMount(),
            o !== a.state && _c.enqueueReplaceState(a, a.state, null),
            bo(t, r, a, i),
            yo(),
            (a.state = t.memoizedState)),
          typeof a.componentDidMount == `function` && (t.flags |= 4194308),
          (r = !0));
      } else if (e === null) {
        a = t.stateNode;
        var s = t.memoizedProps,
          c = bc(n, s);
        a.props = c;
        var l = a.context,
          u = n.contextType;
        ((o = Di), typeof u == `object` && u && (o = Sa(u)));
        var d = n.getDerivedStateFromProps;
        ((u = typeof d == `function` || typeof a.getSnapshotBeforeUpdate == `function`),
          (s = t.pendingProps !== s),
          u ||
            (typeof a.UNSAFE_componentWillReceiveProps != `function` &&
              typeof a.componentWillReceiveProps != `function`) ||
            ((s || l !== o) && yc(t, a, r, o)),
          (uo = !1));
        var f = t.memoizedState;
        ((a.state = f),
          bo(t, r, a, i),
          yo(),
          (l = t.memoizedState),
          s || f !== l || uo
            ? (typeof d == `function` && (gc(t, n, d, r), (l = t.memoizedState)),
              (c = uo || vc(t, n, c, r, f, l, o))
                ? (u ||
                    (typeof a.UNSAFE_componentWillMount != `function` &&
                      typeof a.componentWillMount != `function`) ||
                    (typeof a.componentWillMount == `function` && a.componentWillMount(),
                    typeof a.UNSAFE_componentWillMount == `function` &&
                      a.UNSAFE_componentWillMount()),
                  typeof a.componentDidMount == `function` && (t.flags |= 4194308))
                : (typeof a.componentDidMount == `function` && (t.flags |= 4194308),
                  (t.memoizedProps = r),
                  (t.memoizedState = l)),
              (a.props = r),
              (a.state = l),
              (a.context = o),
              (r = c))
            : (typeof a.componentDidMount == `function` && (t.flags |= 4194308), (r = !1)));
      } else {
        ((a = t.stateNode),
          po(e, t),
          (o = t.memoizedProps),
          (u = bc(n, o)),
          (a.props = u),
          (d = t.pendingProps),
          (f = a.context),
          (l = n.contextType),
          (c = Di),
          typeof l == `object` && l && (c = Sa(l)),
          (s = n.getDerivedStateFromProps),
          (l = typeof s == `function` || typeof a.getSnapshotBeforeUpdate == `function`) ||
            (typeof a.UNSAFE_componentWillReceiveProps != `function` &&
              typeof a.componentWillReceiveProps != `function`) ||
            ((o !== d || f !== c) && yc(t, a, r, c)),
          (uo = !1),
          (f = t.memoizedState),
          (a.state = f),
          bo(t, r, a, i),
          yo());
        var p = t.memoizedState;
        o !== d || f !== p || uo || (e !== null && e.dependencies !== null && ba(e.dependencies))
          ? (typeof s == `function` && (gc(t, n, s, r), (p = t.memoizedState)),
            (u =
              uo ||
              vc(t, n, u, r, f, p, c) ||
              (e !== null && e.dependencies !== null && ba(e.dependencies)))
              ? (l ||
                  (typeof a.UNSAFE_componentWillUpdate != `function` &&
                    typeof a.componentWillUpdate != `function`) ||
                  (typeof a.componentWillUpdate == `function` && a.componentWillUpdate(r, p, c),
                  typeof a.UNSAFE_componentWillUpdate == `function` &&
                    a.UNSAFE_componentWillUpdate(r, p, c)),
                typeof a.componentDidUpdate == `function` && (t.flags |= 4),
                typeof a.getSnapshotBeforeUpdate == `function` && (t.flags |= 1024))
              : (typeof a.componentDidUpdate != `function` ||
                  (o === e.memoizedProps && f === e.memoizedState) ||
                  (t.flags |= 4),
                typeof a.getSnapshotBeforeUpdate != `function` ||
                  (o === e.memoizedProps && f === e.memoizedState) ||
                  (t.flags |= 1024),
                (t.memoizedProps = r),
                (t.memoizedState = p)),
            (a.props = r),
            (a.state = p),
            (a.context = c),
            (r = u))
          : (typeof a.componentDidUpdate != `function` ||
              (o === e.memoizedProps && f === e.memoizedState) ||
              (t.flags |= 4),
            typeof a.getSnapshotBeforeUpdate != `function` ||
              (o === e.memoizedProps && f === e.memoizedState) ||
              (t.flags |= 1024),
            (r = !1));
      }
      return (
        (a = r),
        Hc(e, t),
        (r = !!(t.flags & 128)),
        a || r
          ? ((a = t.stateNode),
            (n = r && typeof n.getDerivedStateFromError != `function` ? null : a.render()),
            (t.flags |= 1),
            e !== null && r
              ? ((t.child = co(t, e.child, null, i)), (t.child = co(t, null, n, i)))
              : Mc(e, t, n, i),
            (t.memoizedState = a.state),
            (e = t.child))
          : (e = sl(e, t, i)),
        e
      );
    }
    function Kc(e, t, n, r) {
      return (la(), (t.flags |= 256), Mc(e, t, n, r), t.child);
    }
    var qc = { dehydrated: null, treeContext: null, retryLane: 0, hydrationErrors: null };
    function Jc(e) {
      return { baseLanes: e, cachePool: Ga() };
    }
    function Yc(e, t, n) {
      return ((e = e === null ? 0 : e.childLanes & ~n), t && (e |= ld), e);
    }
    function Xc(e, t, n) {
      var r = t.pendingProps,
        i = !1,
        a = !!(t.flags & 128),
        o;
      if (
        ((o = a) || (o = e !== null && e.memoizedState === null ? !1 : !!(Fo.current & 2)),
        o && ((i = !0), (t.flags &= -129)),
        (o = !!(t.flags & 32)),
        (t.flags &= -33),
        e === null)
      ) {
        if (U) {
          if (
            (i ? Ao(t) : No(),
            (e = ta)
              ? ((e = am(e, ra)),
                (e = e !== null && e.data !== `&` ? e : null),
                e !== null &&
                  ((t.memoizedState = {
                    dehydrated: e,
                    treeContext: Ki === null ? null : { id: qi, overflow: Ji },
                    retryLane: 536870912,
                    hydrationErrors: null,
                  }),
                  (n = Ii(e)),
                  (n.return = t),
                  (t.child = n),
                  (ea = t),
                  (ta = null)))
              : (e = null),
            e === null)
          )
            throw aa(t);
          return ((t.lanes = sm(e) ? 32 : 536870912), null);
        }
        return (
          (a = r.children),
          (r = r.fallback),
          i
            ? (No(),
              (i = t.mode),
              (a = Qc({ mode: `hidden`, children: a }, i)),
              (r = Pi(r, i, n, null)),
              (a.return = t),
              (r.return = t),
              (a.sibling = r),
              (t.child = a),
              (r = t.child),
              (r.memoizedState = Jc(n)),
              (r.childLanes = Yc(e, o, n)),
              (t.memoizedState = qc),
              Lc(null, r))
            : (Ao(t), Zc(t, a))
        );
      }
      var s = e.memoizedState;
      if (s !== null) {
        var c = s.dehydrated;
        if (c !== null) return el(e, t, a, o, r, c, s, n);
      }
      return i
        ? (No(),
          (i = r.fallback),
          (a = t.mode),
          (s = e.child),
          (c = s.sibling),
          (r = ji(s, { mode: `hidden`, children: r.children })),
          (r.subtreeFlags = s.subtreeFlags & 1206910976),
          c === null ? ((i = Pi(i, a, n, null)), (i.flags |= 2)) : (i = ji(c, i)),
          (i.return = t),
          (r.return = t),
          (r.sibling = i),
          (t.child = r),
          Lc(null, r),
          (r = t.child),
          (i = e.child.memoizedState),
          i === null
            ? (i = Jc(n))
            : ((a = i.cachePool),
              a === null
                ? (a = Ga())
                : ((s = Oa._currentValue), (a = a.parent === s ? a : { parent: s, pool: s })),
              (i = { baseLanes: i.baseLanes | n, cachePool: a })),
          (r.memoizedState = i),
          (r.childLanes = Yc(e, o, n)),
          (t.memoizedState = qc),
          Lc(e.child, r))
        : (Ao(t),
          (n = e.child),
          (e = n.sibling),
          (n = ji(n, { mode: `visible`, children: r.children })),
          (n.return = t),
          (n.sibling = null),
          e !== null &&
            ((o = t.deletions), o === null ? ((t.deletions = [e]), (t.flags |= 16)) : o.push(e)),
          (t.child = n),
          (t.memoizedState = null),
          n);
    }
    function Zc(e, t) {
      return ((t = Qc({ mode: `visible`, children: t }, e.mode)), (t.return = e), (e.child = t));
    }
    function Qc(e, t) {
      return ((e = ki(22, e, null, t)), (e.lanes = 0), e);
    }
    function $c(e, t, n) {
      return (
        co(t, e.child, null, n),
        (e = Zc(t, t.pendingProps.children)),
        (e.flags |= 2),
        (t.memoizedState = null),
        e
      );
    }
    function el(e, t, n, r, a, o, s, c) {
      if (n)
        return t.flags & 256
          ? (Ao(t), (t.flags &= -257), $c(e, t, c))
          : t.memoizedState === null
            ? (No(),
              (o = a.fallback),
              (s = t.mode),
              (a = Qc({ mode: `visible`, children: a.children }, s)),
              (o = Pi(o, s, c, null)),
              (o.flags |= 2),
              (a.return = t),
              (o.return = t),
              (a.sibling = o),
              (t.child = a),
              co(t, e.child, null, c),
              (a = t.child),
              (a.memoizedState = Jc(c)),
              (a.childLanes = Yc(e, r, c)),
              (t.memoizedState = qc),
              Lc(null, a))
            : (No(), (t.child = e.child), (t.flags |= 128), null);
      if ((Ao(t), sm(o))) {
        if (((r = o.nextSibling && o.nextSibling.dataset), r)) var l = r.dgst;
        return (
          (r = l),
          r !== `` &&
            ((a = Error(i(419))),
            (a.stack = ``),
            (a.digest = r),
            da({ value: a, source: null, stack: null })),
          $c(e, t, c)
        );
      }
      if ((jc || ya(e, t, c, !1), (r = (c & e.childLanes) !== 0), jc || r)) {
        if (Co.current !== null) return $c(e, t, c);
        if (((r = $u), r !== null && ((a = mt(r, c)), a !== 0 && a !== s.retryLane)))
          throw ((s.retryLane = a), wi(e, a), Pd(r, e, a), Ac);
        return (om(o) || Kd(), $c(e, t, c));
      }
      return om(o)
        ? ((t.flags |= 192), (t.child = e.child), null)
        : ((e = s.treeContext),
          (ta = lm(o.nextSibling)),
          (ea = t),
          (U = !0),
          (na = null),
          (ra = !1),
          e !== null && $i(t, e),
          (t = Zc(t, a.children)),
          (t.flags |= 134221824),
          t);
    }
    function tl(e, t, n) {
      e.lanes |= t;
      var r = e.alternate;
      (r !== null && (r.lanes |= t), _a(e.return, t, n));
    }
    function nl(e) {
      for (var t = null; e !== null;) {
        var n = e.alternate;
        (n !== null && Ro(n) === null && (t = e), (e = e.sibling));
      }
      return t;
    }
    function rl(e, t, n, r, i, a) {
      var o = e.memoizedState;
      o === null
        ? (e.memoizedState = {
            isBackwards: t,
            rendering: null,
            renderingStartTime: 0,
            last: r,
            tail: n,
            tailMode: i,
            treeForkCount: a,
          })
        : ((o.isBackwards = t),
          (o.rendering = null),
          (o.renderingStartTime = 0),
          (o.last = r),
          (o.tail = n),
          (o.tailMode = i),
          (o.treeForkCount = a));
    }
    function il(e) {
      var t = e.child;
      for (e.child = null; t !== null;) {
        var n = t.sibling;
        ((t.sibling = e.child), (e.child = t), (t = n));
      }
    }
    function al(e, t, n) {
      var r = t.pendingProps,
        i = r.revealOrder,
        a = r.tail;
      r = r.children;
      var o = Fo.current;
      if (t.flags & 128) return (Io(t, o), null);
      var s = !!(o & 2);
      if (
        (s ? ((o = (o & 1) | 2), (t.flags |= 128)) : (o &= 1),
        Io(t, o),
        i === `backwards` && e !== null ? (il(e), Mc(e, t, r, n), il(e)) : Mc(e, t, r, n),
        (r = U ? Ui : 0),
        !s && e !== null && e.flags & 128)
      )
        a: for (e = t.child; e !== null;) {
          if (e.tag === 13) e.memoizedState !== null && tl(e, n, t);
          else if (e.tag === 19) tl(e, n, t);
          else if (e.child !== null) {
            ((e.child.return = e), (e = e.child));
            continue;
          }
          if (e === t) break a;
          for (; e.sibling === null;) {
            if (e.return === null || e.return === t) break a;
            e = e.return;
          }
          ((e.sibling.return = e.return), (e = e.sibling));
        }
      switch (i) {
        case `backwards`:
          ((n = nl(t.child)),
            n === null
              ? ((i = t.child), (t.child = null))
              : ((i = n.sibling), (n.sibling = null), il(t)),
            rl(t, !0, i, null, a, r));
          break;
        case `unstable_legacy-backwards`:
          for (n = null, i = t.child, t.child = null; i !== null;) {
            if (((e = i.alternate), e !== null && Ro(e) === null)) {
              t.child = i;
              break;
            }
            ((e = i.sibling), (i.sibling = n), (n = i), (i = e));
          }
          rl(t, !0, n, null, a, r);
          break;
        case `together`:
          rl(t, !1, null, null, void 0, r);
          break;
        case `independent`:
          t.memoizedState = null;
          break;
        default:
          ((n = nl(t.child)),
            n === null ? ((i = t.child), (t.child = null)) : ((i = n.sibling), (n.sibling = null)),
            rl(t, !1, i, n, a, r));
      }
      return t.child;
    }
    function ol(e, t, n) {
      var r = t.pendingProps;
      return (ha(t, t.type, r.value), Mc(e, t, r.children, n), t.child);
    }
    function sl(e, t, n) {
      if (
        (e !== null && (t.dependencies = e.dependencies), (od |= t.lanes), (n & t.childLanes) === 0)
      ) {
        if (e !== null) {
          if ((ya(e, t, n, !1), (n & t.childLanes) === 0)) return null;
        } else return null;
      }
      if (e !== null && t.child !== e.child) throw Error(i(153));
      if (t.child !== null) {
        for (e = t.child, n = ji(e, e.pendingProps), t.child = n, n.return = t; e.sibling !== null;)
          ((e = e.sibling), (n = n.sibling = ji(e, e.pendingProps)), (n.return = t));
        n.sibling = null;
      }
      return t.child;
    }
    function cl(e, t) {
      return (e.lanes & t) !== 0 || ((e = e.dependencies), !!(e !== null && ba(e)));
    }
    function ll(e, t, n) {
      switch (t.tag) {
        case 3:
          (Ce(t, t.stateNode.containerInfo), ha(t, Oa, e.memoizedState.cache), la());
          break;
        case 27:
        case 5:
          Te(t);
          break;
        case 4:
          Ce(t, t.stateNode.containerInfo);
          break;
        case 10:
          ha(t, t.type, t.memoizedProps.value);
          break;
        case 31:
          if (t.memoizedState !== null) return ((t.flags |= 128), jo(t), null);
          break;
        case 13:
          var r = t.memoizedState;
          if (r !== null) {
            if (r.dehydrated !== null) return (Ao(t), (t.flags |= 128), null);
            r = ya(e, t, n, !1);
            var i = t.child.childLanes;
            return r || (n & i) !== 0
              ? Xc(e, t, n)
              : (Ao(t), (e = sl(e, t, n)), e === null ? null : e.sibling);
          }
          Ao(t);
          break;
        case 19:
          if (t.flags & 128) return al(e, t, n);
          if (
            ((i = !!(e.flags & 128)),
            (r = (n & t.childLanes) !== 0),
            (r ||= (ya(e, t, n, !1), (n & t.childLanes) !== 0)),
            i)
          ) {
            if (r) return al(e, t, n);
            t.flags |= 128;
          }
          if (
            ((i = t.memoizedState),
            i !== null && ((i.rendering = null), (i.tail = null), (i.lastEffect = null)),
            Io(t, Fo.current),
            r)
          )
            break;
          return null;
        case 22:
          return ((t.lanes = 0), Ic(e, t, n, t.pendingProps));
        case 24:
          ha(t, Oa, e.memoizedState.cache);
      }
      return sl(e, t, n);
    }
    function ul(e, t, n) {
      if (e !== null) {
        if (e.memoizedProps !== t.pendingProps) jc = !0;
        else {
          if (!cl(e, n) && !(t.flags & 128)) return ((jc = !1), ll(e, t, n));
          jc = !!(e.flags & 131072);
        }
      } else ((jc = !1), U && t.flags & 1048576 && Xi(t, Ui, t.index));
      switch (((t.lanes = 0), t.tag)) {
        case 16:
          a: {
            var r = t.pendingProps;
            if (((e = Qa(t.elementType)), (t.type = e), typeof e == `function`))
              Ai(e)
                ? ((r = bc(e, r)), (t.tag = 1), (t = Gc(null, t, e, r, n)))
                : ((t.tag = 0), (t = Uc(null, t, e, r, n)));
            else {
              if (e != null) {
                var a = e.$$typeof;
                if (a === N) {
                  ((t.tag = 11), (t = Nc(null, t, e, r, n)));
                  break a;
                }
                if (a === F) {
                  ((t.tag = 14), (t = Pc(null, t, e, r, n)));
                  break a;
                }
                if (a === M) {
                  ((t.tag = 10), (t.type = e), (t = ol(null, t, n)));
                  break a;
                }
              }
              throw ((t = fe(e) || e), Error(i(306, t, ``)));
            }
          }
          return t;
        case 0:
          return Uc(e, t, t.type, t.pendingProps, n);
        case 1:
          return ((r = t.type), (a = bc(r, t.pendingProps)), Gc(e, t, r, a, n));
        case 3:
          a: {
            if ((Ce(t, t.stateNode.containerInfo), e === null)) throw Error(i(387));
            r = t.pendingProps;
            var o = t.memoizedState;
            ((a = o.element), po(e, t), bo(t, r, null, n));
            var s = t.memoizedState;
            if (
              ((r = s.cache),
              ha(t, Oa, r),
              r !== o.cache && va(t, [Oa], n, !0),
              yo(),
              (r = s.element),
              o.isDehydrated)
            ) {
              if (
                ((o = { element: r, isDehydrated: !1, cache: s.cache }),
                (t.updateQueue.baseState = o),
                (t.memoizedState = o),
                t.flags & 256)
              ) {
                t = Kc(e, t, r, n);
                break a;
              }
              if (r !== a) {
                ((a = zi(Error(i(424)), t)), da(a), (t = Kc(e, t, r, n)));
                break a;
              }
              switch (((e = t.stateNode.containerInfo), e.nodeType)) {
                case 9:
                  e = e.body;
                  break;
                default:
                  e = e.nodeName === `HTML` ? e.ownerDocument.body : e;
              }
              for (
                ta = lm(e.firstChild),
                  ea = t,
                  U = !0,
                  na = null,
                  ra = !0,
                  n = lo(t, null, r, n),
                  t.child = n;
                n;
              )
                ((n.flags = (n.flags & -3) | 134221824), (n = n.sibling));
            } else {
              if ((la(), r === a)) {
                t = sl(e, t, n);
                break a;
              }
              Mc(e, t, r, n);
            }
            t = t.child;
          }
          return t;
        case 26:
          return (
            Hc(e, t),
            e === null
              ? (n = Nm(t.type, null, t.pendingProps, null))
                ? (t.memoizedState = n)
                : U || (t.stateNode = fp(t.type, t.pendingProps, xe.current, t))
              : (t.memoizedState = Nm(t.type, e.memoizedProps, t.pendingProps, e.memoizedState)),
            null
          );
        case 27:
          return (
            Te(t),
            e === null &&
              U &&
              ((r = t.stateNode = hm(t.type, t.pendingProps, xe.current)),
              (ea = t),
              (ra = !0),
              (a = ta),
              Sp(t.type) ? ((um = a), (ta = lm(r.firstChild))) : (ta = a)),
            Mc(e, t, t.pendingProps.children, n),
            Hc(e, t),
            e === null && (t.flags |= 4194304),
            t.child
          );
        case 5:
          return (
            e === null &&
              U &&
              ((a = r = ta) &&
                ((r = rm(r, t.type, t.pendingProps, ra)),
                r === null
                  ? (a = !1)
                  : ((t.stateNode = r), (ea = t), (ta = lm(r.firstChild)), (ra = !1), (a = !0))),
              a || aa(t)),
            Te(t),
            (a = t.type),
            (o = t.pendingProps),
            (s = e === null ? null : e.memoizedProps),
            (r = o.children),
            pp(a, o) ? (r = null) : s !== null && pp(a, s) && (t.flags |= 32),
            t.memoizedState !== null && ((a = Xo(e, t, $o, null, null, n)), (sh._currentValue = a)),
            Hc(e, t),
            Mc(e, t, r, n),
            t.child
          );
        case 6:
          return (
            e === null &&
              U &&
              ((e = n = ta) &&
                ((n = im(n, t.pendingProps, ra)),
                n === null ? (e = !1) : ((t.stateNode = n), (ea = t), (ta = null), (e = !0))),
              e || aa(t)),
            null
          );
        case 13:
          return Xc(e, t, n);
        case 4:
          return (
            Ce(t, t.stateNode.containerInfo),
            (r = t.pendingProps),
            e === null ? (t.child = co(t, null, r, n)) : Mc(e, t, r, n),
            t.child
          );
        case 11:
          return Nc(e, t, t.type, t.pendingProps, n);
        case 7:
          return ((r = t.pendingProps), Hc(e, t), Mc(e, t, r, n), t.child);
        case 8:
          return (Mc(e, t, t.pendingProps.children, n), t.child);
        case 12:
          return (Mc(e, t, t.pendingProps.children, n), t.child);
        case 10:
          return ol(e, t, n);
        case 9:
          return (
            (a = t.type._context),
            (r = t.pendingProps.children),
            xa(t),
            (a = Sa(a)),
            (r = r(a)),
            (t.flags |= 1),
            Mc(e, t, r, n),
            t.child
          );
        case 14:
          return Pc(e, t, t.type, t.pendingProps, n);
        case 15:
          return Fc(e, t, t.type, t.pendingProps, n);
        case 19:
          return al(e, t, n);
        case 31:
          return Vc(e, t, n);
        case 22:
          return Ic(e, t, n, t.pendingProps);
        case 24:
          return (
            xa(t),
            (r = Sa(Oa)),
            e === null
              ? ((a = Ua()),
                a === null &&
                  ((a = $u),
                  (o = ka()),
                  (a.pooledCache = o),
                  o.refCount++,
                  o !== null && (a.pooledCacheLanes |= n),
                  (a = o)),
                (t.memoizedState = { parent: r, cache: a }),
                fo(t),
                ha(t, Oa, a))
              : ((e.lanes & n) !== 0 && (po(e, t), bo(t, null, null, n), yo()),
                (a = e.memoizedState),
                (o = t.memoizedState),
                a.parent === r
                  ? ((r = o.cache), ha(t, Oa, r), r !== a.cache && va(t, [Oa], n, !0))
                  : ((a = { parent: r, cache: r }),
                    (t.memoizedState = a),
                    t.lanes === 0 && (t.memoizedState = t.updateQueue.baseState = a),
                    ha(t, Oa, r))),
            Mc(e, t, t.pendingProps.children, n),
            t.child
          );
        case 30:
          return (
            t.stateNode === null &&
              (t.stateNode = { autoName: null, paired: null, clones: null, ref: null }),
            (r = t.pendingProps),
            r.name != null && r.name !== `auto`
              ? (t.flags |= e === null ? 18882560 : 18874368)
              : U && Zi(t),
            e !== null && e.memoizedProps.name !== r.name ? (t.flags |= 4194816) : Hc(e, t),
            Mc(e, t, r.children, n),
            t.child
          );
        case 29:
          throw t.pendingProps;
      }
      throw Error(i(156, t.tag));
    }
    function dl(e) {
      e.flags |= 4;
    }
    function fl(e, t, n, r, i) {
      var a;
      if (
        ((a = !!(e.mode & 32)) &&
          (a = n === null ? Jm(t, r) : Jm(t, r) && (r.src !== n.src || r.srcSet !== n.srcSet)),
        a)
      ) {
        if (((e.flags |= 16777216), (i & 335544128) === i)) {
          if (e.stateNode.complete) e.flags |= 8192;
          else if (Ud()) e.flags |= 8192;
          else throw (($a = Ya), qa);
        }
      } else e.flags &= -16777217;
    }
    function pl(e, t) {
      if (t.type !== `stylesheet` || t.state.loading & 4) e.flags &= -16777217;
      else if (((e.flags |= 16777216), !Ym(t))) {
        if (Ud()) e.flags |= 8192;
        else throw (($a = Ya), qa);
      }
    }
    function ml(e, t) {
      (t !== null && (e.flags |= 4),
        e.flags & 16384 && ((t = e.tag === 22 ? 536870912 : ct()), (e.lanes |= t), (ud |= t)));
    }
    function hl(e, t) {
      if (!U)
        switch (e.tailMode) {
          case `visible`:
            break;
          case `collapsed`:
            for (var n = e.tail, r = null; n !== null;)
              (n.alternate !== null && (r = n), (n = n.sibling));
            r === null
              ? t || e.tail === null
                ? (e.tail = null)
                : (e.tail.sibling = null)
              : (r.sibling = null);
            break;
          default:
            for (t = e.tail, n = null; t !== null;)
              (t.alternate !== null && (n = t), (t = t.sibling));
            n === null ? (e.tail = null) : (n.sibling = null);
        }
    }
    function gl(e) {
      var t = e.alternate !== null && e.alternate.child === e.child,
        n = 0,
        r = 0;
      if (t)
        for (var i = e.child; i !== null;)
          ((n |= i.lanes | i.childLanes),
            (r |= i.subtreeFlags & 1206910976),
            (r |= i.flags & 1206910976),
            (i.return = e),
            (i = i.sibling));
      else
        for (i = e.child; i !== null;)
          ((n |= i.lanes | i.childLanes),
            (r |= i.subtreeFlags),
            (r |= i.flags),
            (i.return = e),
            (i = i.sibling));
      return ((e.subtreeFlags |= r), (e.childLanes = n), t);
    }
    function _l(e, t, n) {
      var r = t.pendingProps;
      switch ((Qi(t), t.tag)) {
        case 16:
        case 15:
        case 0:
        case 11:
        case 7:
        case 8:
        case 12:
        case 9:
        case 14:
          return (gl(t), null);
        case 1:
          return (gl(t), null);
        case 3:
          return (
            (n = t.stateNode),
            (r = null),
            e !== null && (r = e.memoizedState.cache),
            t.memoizedState.cache !== r && (t.flags |= 2048),
            ga(Oa),
            we(),
            n.pendingContext && ((n.context = n.pendingContext), (n.pendingContext = null)),
            (e === null || e.child === null) &&
              (ca(t)
                ? dl(t)
                : e === null ||
                  (e.memoizedState.isDehydrated && !(t.flags & 256)) ||
                  ((t.flags |= 1024), ua())),
            gl(t),
            null
          );
        case 26:
          var a = t.type,
            o = t.memoizedState;
          return (
            e === null
              ? (dl(t), o === null ? (gl(t), fl(t, a, null, r, n)) : (gl(t), pl(t, o)))
              : o
                ? o === e.memoizedState
                  ? (gl(t), (t.flags &= -16777217))
                  : (dl(t), gl(t), pl(t, o))
                : ((e = e.memoizedProps), e !== r && dl(t), gl(t), fl(t, a, e, r, n)),
            null
          );
        case 27:
          if ((Ee(t), (n = xe.current), (a = t.type), e !== null && t.stateNode != null))
            e.memoizedProps !== r && dl(t);
          else {
            if (!r) {
              if (t.stateNode === null) throw Error(i(166));
              return (gl(t), (t.subtreeFlags &= -33554433), null);
            }
            ((e = ye.current), ca(t) ? oa(t, e) : ((e = hm(a, r, n)), (t.stateNode = e), dl(t)));
          }
          return (gl(t), (t.subtreeFlags &= -33554433), null);
        case 5:
          if ((Ee(t), (a = t.type), e !== null && t.stateNode != null))
            e.memoizedProps !== r && dl(t);
          else {
            if (!r) {
              if (t.stateNode === null) throw Error(i(166));
              return (gl(t), (t.subtreeFlags &= -33554433), null);
            }
            if (((o = ye.current), ca(t))) oa(t, o);
            else {
              var s = lp(xe.current);
              switch (o) {
                case 1:
                  o = s.createElementNS(`http://www.w3.org/2000/svg`, a);
                  break;
                case 2:
                  o = s.createElementNS(`http://www.w3.org/1998/Math/MathML`, a);
                  break;
                default:
                  switch (a) {
                    case `svg`:
                      o = s.createElementNS(`http://www.w3.org/2000/svg`, a);
                      break;
                    case `math`:
                      o = s.createElementNS(`http://www.w3.org/1998/Math/MathML`, a);
                      break;
                    case `script`:
                      ((o = s.createElement(`div`)),
                        (o.innerHTML = `<script><\/script>`),
                        (o = o.removeChild(o.firstChild)));
                      break;
                    case `select`:
                      ((o =
                        typeof r.is == `string`
                          ? s.createElement(`select`, { is: r.is })
                          : s.createElement(`select`)),
                        r.multiple ? (o.multiple = !0) : r.size && (o.size = r.size));
                      break;
                    default:
                      o =
                        typeof r.is == `string`
                          ? s.createElement(a, { is: r.is })
                          : s.createElement(a);
                  }
              }
              ((o[bt] = t), (o[xt] = r));
              a: for (s = t.child; s !== null;) {
                if (s.tag === 5 || s.tag === 6) o.appendChild(s.stateNode);
                else if (s.tag !== 4 && s.tag !== 27 && s.child !== null) {
                  ((s.child.return = s), (s = s.child));
                  continue;
                }
                if (s === t) break a;
                for (; s.sibling === null;) {
                  if (s.return === null || s.return === t) break a;
                  s = s.return;
                }
                ((s.sibling.return = s.return), (s = s.sibling));
              }
              t.stateNode = o;
              a: switch ((np(o, a, r), a)) {
                case `button`:
                case `input`:
                case `select`:
                case `textarea`:
                  r = !!r.autoFocus;
                  break a;
                case `img`:
                  r = !0;
                  break a;
                default:
                  r = !1;
              }
              r && dl(t);
            }
          }
          return (
            gl(t),
            (t.subtreeFlags &= -33554433),
            fl(t, t.type, e === null ? null : e.memoizedProps, t.pendingProps, n),
            null
          );
        case 6:
          if (e && t.stateNode != null) e.memoizedProps !== r && dl(t);
          else {
            if (typeof r != `string` && t.stateNode === null) throw Error(i(166));
            if (((e = xe.current), ca(t))) {
              if (((e = t.stateNode), (n = t.memoizedProps), (r = null), (a = ea), a !== null))
                switch (a.tag) {
                  case 27:
                  case 5:
                    r = a.memoizedProps;
                }
              ((e[bt] = t),
                (e = !!(
                  e.nodeValue === n ||
                  (r !== null && !0 === r.suppressHydrationWarning) ||
                  ep(e.nodeValue, n)
                )),
                e || aa(t, !0));
            } else ((e = lp(e).createTextNode(r)), (e[bt] = t), (t.stateNode = e));
          }
          return (gl(t), null);
        case 31:
          if (((n = t.memoizedState), e === null || e.memoizedState !== null)) {
            if (((r = ca(t)), n !== null)) {
              if (e === null) {
                if (!r) throw Error(i(318));
                if (((e = t.memoizedState), (e = e === null ? null : e.dehydrated), !e))
                  throw Error(i(557));
                e[bt] = t;
              } else (la(), !(t.flags & 128) && (t.memoizedState = null), (t.flags |= 4));
              (gl(t), (e = !1));
            } else
              ((n = ua()),
                e !== null && e.memoizedState !== null && (e.memoizedState.hydrationErrors = n),
                (e = !0));
            if (!e) return t.flags & 256 ? (Po(t), t) : (Po(t), null);
            if (t.flags & 128) throw Error(i(558));
          }
          return (gl(t), null);
        case 13:
          if (
            ((r = t.memoizedState),
            e === null || (e.memoizedState !== null && e.memoizedState.dehydrated !== null))
          ) {
            if (((a = ca(t)), r !== null && r.dehydrated !== null)) {
              if (e === null) {
                if (!a) throw Error(i(318));
                if (((a = t.memoizedState), (a = a === null ? null : a.dehydrated), !a))
                  throw Error(i(317));
                a[bt] = t;
              } else (la(), !(t.flags & 128) && (t.memoizedState = null), (t.flags |= 4));
              (gl(t), (a = !1));
            } else
              ((a = ua()),
                e !== null && e.memoizedState !== null && (e.memoizedState.hydrationErrors = a),
                (a = !0));
            if (!a) return t.flags & 256 ? (Po(t), t) : (Po(t), null);
          }
          return (
            Po(t),
            t.flags & 128
              ? ((t.lanes = n), t)
              : ((n = r !== null),
                (e = e !== null && e.memoizedState !== null),
                n &&
                  ((r = t.child),
                  (a = null),
                  r.alternate !== null &&
                    r.alternate.memoizedState !== null &&
                    r.alternate.memoizedState.cachePool !== null &&
                    (a = r.alternate.memoizedState.cachePool.pool),
                  (o = null),
                  r.memoizedState !== null &&
                    r.memoizedState.cachePool !== null &&
                    (o = r.memoizedState.cachePool.pool),
                  o !== a && (r.flags |= 2048)),
                n !== e && n && (t.child.flags |= 8192),
                ml(t, t.updateQueue),
                gl(t),
                null)
          );
        case 4:
          return (
            we(), e === null && Wf(t.stateNode.containerInfo), (t.flags |= 67108864), gl(t), null
          );
        case 10:
          return (ga(t.type), gl(t), null);
        case 19:
          if ((Lo(t), (r = t.memoizedState), r === null)) return (gl(t), null);
          if (((a = !!(t.flags & 128)), (o = r.rendering), o === null)) {
            if (a) hl(r, !1);
            else {
              if (ad !== 0 || (e !== null && e.flags & 128))
                for (e = t.child; e !== null;) {
                  if (((o = Ro(e)), o !== null)) {
                    for (
                      t.flags |= 128,
                        hl(r, !1),
                        e = o.updateQueue,
                        t.updateQueue = e,
                        ml(t, e),
                        t.subtreeFlags = 0,
                        e = n,
                        n = t.child;
                      n !== null;
                    )
                      (Mi(n, e), (n = n.sibling));
                    return (Io(t, (Fo.current & 1) | 2), U && Yi(t, r.treeForkCount), t.child);
                  }
                  e = e.sibling;
                }
              r.tail !== null &&
                ze() > gd &&
                ((t.flags |= 128), (a = !0), hl(r, !1), (t.lanes = 4194304));
            }
          } else {
            if (!a) {
              if (((e = Ro(o)), e !== null)) {
                if (
                  ((t.flags |= 128),
                  (a = !0),
                  (e = e.updateQueue),
                  (t.updateQueue = e),
                  ml(t, e),
                  hl(r, !0),
                  r.tail === null &&
                    r.tailMode !== `collapsed` &&
                    r.tailMode !== `visible` &&
                    !o.alternate &&
                    !U)
                )
                  return (gl(t), null);
              } else
                2 * ze() - r.renderingStartTime > gd &&
                  n !== 536870912 &&
                  ((t.flags |= 128), (a = !0), hl(r, !1), (t.lanes = 4194304));
            }
            r.isBackwards
              ? ((o.sibling = t.child), (t.child = o))
              : ((e = r.last), e === null ? (t.child = o) : (e.sibling = o), (r.last = o));
          }
          if (r.tail !== null) {
            e = r.tail;
            a: {
              for (n = e; n !== null;) {
                if (n.alternate !== null) {
                  n = !1;
                  break a;
                }
                n = n.sibling;
              }
              n = !0;
            }
            return (
              (r.rendering = e),
              (r.tail = e.sibling),
              (r.renderingStartTime = ze()),
              (e.sibling = null),
              (o = Fo.current),
              (o = a ? (o & 1) | 2 : o & 1),
              r.tailMode === `visible` || r.tailMode === `collapsed` || !n || U
                ? Io(t, o)
                : ((n = o), z(Oo, t), z(Fo, n), ko === null && (ko = t)),
              U && Yi(t, r.treeForkCount),
              e
            );
          }
          return (gl(t), null);
        case 22:
        case 23:
          return (
            Po(t),
            Do(),
            (r = t.memoizedState !== null),
            e === null
              ? r && (t.flags |= 8192)
              : (e.memoizedState !== null) !== r && (t.flags |= 8192),
            r
              ? n & 536870912 &&
                !(t.flags & 128) &&
                (gl(t), t.subtreeFlags & 6 && (t.flags |= 8192))
              : gl(t),
            (n = t.updateQueue),
            n !== null && ml(t, n.retryQueue),
            (n = null),
            e !== null &&
              e.memoizedState !== null &&
              e.memoizedState.cachePool !== null &&
              (n = e.memoizedState.cachePool.pool),
            (r = null),
            t.memoizedState !== null &&
              t.memoizedState.cachePool !== null &&
              (r = t.memoizedState.cachePool.pool),
            r !== n && (t.flags |= 2048),
            e !== null && ve(Ha),
            null
          );
        case 24:
          return (
            (n = null),
            e !== null && (n = e.memoizedState.cache),
            t.memoizedState.cache !== n && (t.flags |= 2048),
            ga(Oa),
            gl(t),
            null
          );
        case 25:
          return null;
        case 30:
          return ((t.flags |= 33554432), gl(t), null);
      }
      throw Error(i(156, t.tag));
    }
    function vl(e, t) {
      switch ((Qi(t), t.tag)) {
        case 1:
          return ((e = t.flags), e & 65536 ? ((t.flags = (e & -65537) | 128), t) : null);
        case 3:
          return (
            ga(Oa),
            we(),
            (e = t.flags),
            e & 65536 && !(e & 128) ? ((t.flags = (e & -65537) | 128), t) : null
          );
        case 26:
        case 27:
        case 5:
          return (Ee(t), null);
        case 31:
          if (t.memoizedState !== null) {
            if ((Po(t), t.alternate === null)) throw Error(i(340));
            la();
          }
          return ((e = t.flags), e & 65536 ? ((t.flags = (e & -65537) | 128), t) : null);
        case 13:
          if ((Po(t), (e = t.memoizedState), e !== null && e.dehydrated !== null)) {
            if (t.alternate === null) throw Error(i(340));
            la();
          }
          return ((e = t.flags), e & 65536 ? ((t.flags = (e & -65537) | 128), t) : null);
        case 19:
          return (
            Lo(t),
            (e = t.flags),
            e & 65536
              ? ((t.flags = (e & -65537) | 128),
                (e = t.memoizedState),
                e !== null && ((e.rendering = null), (e.tail = null)),
                (t.flags |= 4),
                t)
              : null
          );
        case 4:
          return (we(), null);
        case 10:
          return (ga(t.type), null);
        case 22:
        case 23:
          return (
            Po(t),
            Do(),
            e !== null && ve(Ha),
            (e = t.flags),
            e & 65536 ? ((t.flags = (e & -65537) | 128), t) : null
          );
        case 24:
          return (ga(Oa), null);
        case 25:
          return null;
        default:
          return null;
      }
    }
    function yl(e, t) {
      switch ((Qi(t), t.tag)) {
        case 3:
          (ga(Oa), we());
          break;
        case 26:
        case 27:
        case 5:
          Ee(t);
          break;
        case 4:
          we();
          break;
        case 31:
          t.memoizedState !== null && Po(t);
          break;
        case 13:
          Po(t);
          break;
        case 19:
          Lo(t);
          break;
        case 10:
          ga(t.type);
          break;
        case 22:
        case 23:
          (Po(t), Do(), e !== null && ve(Ha));
          break;
        case 24:
          ga(Oa);
      }
    }
    function bl(e, t) {
      try {
        var n = t.updateQueue,
          r = n === null ? null : n.lastEffect;
        if (r !== null) {
          var i = r.next;
          n = i;
          do {
            if ((n.tag & e) === e) {
              r = void 0;
              var a = n.create,
                o = n.inst;
              ((r = a()), (o.destroy = r));
            }
            n = n.next;
          } while (n !== i);
        }
      } catch (e) {
        Z(t, t.return, e);
      }
    }
    function xl(e, t, n) {
      try {
        var r = t.updateQueue,
          i = r === null ? null : r.lastEffect;
        if (i !== null) {
          var a = i.next;
          r = a;
          do {
            if ((r.tag & e) === e) {
              var o = r.inst,
                s = o.destroy;
              if (s !== void 0) {
                ((o.destroy = void 0), (i = t));
                var c = n,
                  l = s;
                try {
                  l();
                } catch (e) {
                  Z(i, c, e);
                }
              }
            }
            r = r.next;
          } while (r !== a);
        }
      } catch (e) {
        Z(t, t.return, e);
      }
    }
    function Sl(e) {
      var t = e.updateQueue;
      if (t !== null) {
        var n = e.stateNode;
        try {
          So(t, n);
        } catch (t) {
          Z(e, e.return, t);
        }
      }
    }
    function Cl(e, t, n) {
      ((n.props = bc(e.type, e.memoizedProps)), (n.state = e.memoizedState));
      try {
        n.componentWillUnmount();
      } catch (n) {
        Z(e, t, n);
      }
    }
    function wl(e, t) {
      try {
        var n = e.ref;
        if (n !== null) {
          switch (e.tag) {
            case 26:
            case 27:
            case 5:
              var r = e.stateNode;
              break;
            case 30:
              var i = e.stateNode,
                a = mi(e.memoizedProps, i);
              ((i.ref === null || i.ref.name !== a) && (i.ref = Pp(a)), (r = i.ref));
              break;
            case 7:
              if (e.stateNode === null) {
                var o = new Fp(e);
                (h(e.child, !1, Qp, o, void 0, void 0), (e.stateNode = o));
              }
              r = e.stateNode;
              break;
            default:
              r = e.stateNode;
          }
          typeof n == `function` ? (e.refCleanup = n(r)) : (n.current = r);
        }
      } catch (n) {
        Z(e, t, n);
      }
    }
    function Tl(e, t) {
      var n = e.ref,
        r = e.refCleanup;
      if (n !== null) {
        if (typeof r == `function`)
          try {
            r();
          } catch (n) {
            Z(e, t, n);
          } finally {
            ((e.refCleanup = null), (e = e.alternate), e != null && (e.refCleanup = null));
          }
        else if (typeof n == `function`)
          try {
            n(null);
          } catch (n) {
            Z(e, t, n);
          }
        else n.current = null;
      }
    }
    function El(e, t) {
      if ((e.tag === 5 || e.tag === 27 || e.tag === 6) && e.alternate === null && t !== null)
        for (var n = 0; n < t.length; n++) em(e.stateNode, t[n]);
    }
    function Dl(e) {
      for (var t = e.return; t !== null && (Al(t) && em(e.stateNode, t.stateNode), !kl(t));)
        t = t.return;
    }
    function Ol(e) {
      for (var t = e.return; t !== null && (Al(t) && tm(e.stateNode, t.stateNode), !kl(t));)
        t = t.return;
    }
    function kl(e) {
      return e.tag === 5 || e.tag === 3 || e.tag === 27;
    }
    function Al(e) {
      return e && e.tag === 7 && e.stateNode !== null;
    }
    function jl(e) {
      var t = e.type,
        n = e.memoizedProps,
        r = e.stateNode;
      try {
        a: switch (t) {
          case `button`:
          case `input`:
          case `select`:
          case `textarea`:
            n.autoFocus && r.focus();
            break a;
          case `img`:
            n.src ? (r.src = n.src) : n.srcSet && (r.srcset = n.srcSet);
        }
      } catch (t) {
        Z(e, e.return, t);
      }
    }
    function Ml(e, t, n) {
      try {
        var r = e.stateNode;
        (ip(r, e.type, n, t), (r[xt] = t));
      } catch (t) {
        Z(e, e.return, t);
      }
    }
    function Nl(e) {
      return (
        e.tag === 5 || e.tag === 3 || e.tag === 26 || (e.tag === 27 && Sp(e.type)) || e.tag === 4
      );
    }
    function Pl(e) {
      a: for (;;) {
        for (; e.sibling === null;) {
          if (e.return === null || Nl(e.return)) return null;
          e = e.return;
        }
        for (
          e.sibling.return = e.return, e = e.sibling;
          e.tag !== 5 && e.tag !== 6 && e.tag !== 18;
        ) {
          if ((e.tag === 27 && Sp(e.type)) || e.flags & 2 || e.child === null || e.tag === 4)
            continue a;
          ((e.child.return = e), (e = e.child));
        }
        if (!(e.flags & 2)) return e.stateNode;
      }
    }
    function Fl(e, t, n, r) {
      var i = e.tag;
      if (i === 5 || i === 6)
        ((i = e.stateNode),
          t
            ? (n.nodeType === 9
                ? n.body
                : n.nodeName === `HTML`
                  ? n.ownerDocument.body
                  : n
              ).insertBefore(i, t)
            : ((t = n.nodeType === 9 ? n.body : n.nodeName === `HTML` ? n.ownerDocument.body : n),
              t.appendChild(i),
              (n = n._reactRootContainer),
              n != null || t.onclick !== null || (t.onclick = gn)),
          El(e, r),
          (V = !0));
      else if (
        i !== 4 &&
        (i === 27 && (El(e, r), (r = null), Sp(e.type) && ((n = e.stateNode), (t = null))),
        (e = e.child),
        e !== null)
      )
        for (Fl(e, t, n, r), e = e.sibling; e !== null;) (Fl(e, t, n, r), (e = e.sibling));
    }
    function Il(e, t, n, r) {
      var i = e.tag;
      if (i === 5 || i === 6)
        ((i = e.stateNode), t ? n.insertBefore(i, t) : n.appendChild(i), El(e, r), (V = !0));
      else if (
        i !== 4 &&
        (i === 27 && (El(e, r), (r = null), Sp(e.type) && (n = e.stateNode)),
        (e = e.child),
        e !== null)
      )
        for (Il(e, t, n, r), e = e.sibling; e !== null;) (Il(e, t, n, r), (e = e.sibling));
    }
    function Ll(e) {
      var t = e.stateNode,
        n = e.memoizedProps;
      try {
        for (var r = e.type, i = t.attributes; i.length;) t.removeAttributeNode(i[0]);
        (np(t, r, n), (t[bt] = e), (t[xt] = n));
      } catch (t) {
        Z(e, e.return, t);
      }
    }
    var Rl = !1,
      zl = null;
    function Bl(e) {
      (e.tag === 30 || e.subtreeFlags & 33554432) && (Rl = !0);
    }
    var Vl = null;
    function Hl() {
      var e = Vl;
      return ((Vl = null), e);
    }
    var Ul = 0;
    function Wl(e, t, n, r, i) {
      return ((Ul = 0), Gl(e.child, t, n, r, i));
    }
    function Gl(e, t, n, r, i) {
      for (var a = !1; e !== null;) {
        if (e.tag === 5) {
          var o = e.stateNode;
          if (r !== null) {
            var s = Op(o);
            (r.push(s), s.view && (a = !0));
          } else a || (Op(o).view && (a = !0));
          ((Rl = !0), Tp(o, Ul === 0 ? t : t + `_` + Ul, n), Ul++);
        } else
          (e.tag !== 22 || e.memoizedState === null) &&
            ((e.tag === 30 && i) || (Gl(e.child, t, n, r, i) && (a = !0)));
        e = e.sibling;
      }
      return a;
    }
    function Kl(e, t) {
      for (; e !== null;)
        (e.tag === 5
          ? Ep(e.stateNode, e.memoizedProps)
          : (e.tag !== 22 || e.memoizedState === null) && ((e.tag === 30 && t) || Kl(e.child, t)),
          (e = e.sibling));
    }
    function ql(e) {
      if (e.subtreeFlags & 18874368)
        for (e = e.child; e !== null;) {
          if (
            (e.tag !== 22 || e.memoizedState === null) &&
            (ql(e), e.tag === 30 && e.flags & 18874368 && e.stateNode.paired)
          ) {
            var t = e.memoizedProps;
            if (t.name == null || t.name === `auto`) throw Error(i(544));
            var n = t.name;
            ((t = gi(t.default, t.share)),
              t !== `none` && (Wl(e, n, t, null, !1) || Kl(e.child, !1)));
          }
          e = e.sibling;
        }
    }
    function Jl(e, t) {
      if (e.tag === 30) {
        var n = e.stateNode,
          r = e.memoizedProps,
          i = mi(r, n),
          a = gi(r.default, n.paired ? r.share : r.enter);
        a === `none`
          ? ql(e)
          : Wl(e, i, a, null, !1)
            ? (ql(e), n.paired || t || Nd(e, r.onEnter))
            : Kl(e.child, !1);
      } else if (e.subtreeFlags & 33554432)
        for (e = e.child; e !== null;) (Jl(e, t), (e = e.sibling));
      else ql(e);
    }
    function Yl(e) {
      if (zl !== null && zl.size !== 0) {
        var t = zl;
        if (e.subtreeFlags & 18874368)
          for (e = e.child; e !== null;) {
            if (e.tag !== 22 || e.memoizedState === null) {
              if (e.tag === 30 && e.flags & 18874368) {
                var n = e.memoizedProps,
                  r = n.name;
                if (r != null && r !== `auto`) {
                  var i = t.get(r);
                  if (i !== void 0) {
                    var a = gi(n.default, n.share);
                    if (
                      (a !== `none` &&
                        (Wl(e, r, a, null, !1)
                          ? ((a = e.stateNode), (i.paired = a), (a.paired = i), Nd(e, n.onShare))
                          : Kl(e.child, !1)),
                      t.delete(r),
                      t.size === 0)
                    )
                      break;
                  }
                }
              }
              Yl(e);
            }
            e = e.sibling;
          }
      }
    }
    function Xl(e) {
      if (e.tag === 30) {
        var t = e.memoizedProps,
          n = mi(t, e.stateNode),
          r = zl === null ? void 0 : zl.get(n),
          i = gi(t.default, r === void 0 ? t.exit : t.share);
        (i !== `none` &&
          (Wl(e, n, i, null, !1)
            ? r === void 0
              ? Nd(e, t.onExit)
              : ((i = e.stateNode), (r.paired = i), (i.paired = r), zl.delete(n), Nd(e, t.onShare))
            : Kl(e.child, !1)),
          zl !== null && Yl(e));
      } else if (e.subtreeFlags & 33554432) for (e = e.child; e !== null;) (Xl(e), (e = e.sibling));
      else zl !== null && Yl(e);
    }
    function Zl(e) {
      for (e = e.child; e !== null;) {
        if (e.tag === 30) {
          var t = e.memoizedProps,
            n = mi(t, e.stateNode);
          ((t = gi(t.default, t.update)),
            (e.flags &= -5),
            t !== `none` && Wl(e, n, t, (e.memoizedState = []), !1));
        } else e.subtreeFlags & 33554432 && Zl(e);
        e = e.sibling;
      }
    }
    function Ql(e) {
      if (e.subtreeFlags & 18874368)
        for (e = e.child; e !== null;) {
          if (e.tag !== 22 || e.memoizedState === null) {
            if (e.tag === 30 && e.flags & 18874368) {
              var t = e.stateNode;
              t.paired !== null && ((t.paired = null), Kl(e.child, !1));
            }
            Ql(e);
          }
          e = e.sibling;
        }
    }
    function $l(e) {
      if (e.tag === 30) ((e.stateNode.paired = null), Kl(e.child, !1), Ql(e));
      else if (e.subtreeFlags & 33554432) for (e = e.child; e !== null;) ($l(e), (e = e.sibling));
      else Ql(e);
    }
    function eu(e) {
      for (e = e.child; e !== null;)
        (e.tag === 30 ? Kl(e.child, !1) : e.subtreeFlags & 33554432 && eu(e), (e = e.sibling));
    }
    function tu(e, t, n, r, i, a, o) {
      for (var s = !1; t !== null;) {
        if (t.tag === 5) {
          var c = t.stateNode;
          if (a !== null && Ul < a.length) {
            var l = a[Ul],
              u = Op(c);
            (l.view || u.view) && (s = !0);
            var d;
            if ((d = !(e.flags & 4))) {
              if (u.clip) d = !0;
              else {
                d = l.rect;
                var f = u.rect;
                d = d.y !== f.y || d.x !== f.x || d.height !== f.height || d.width !== f.width;
              }
            }
            (d && (e.flags |= 4),
              u.abs
                ? (u = !l.abs)
                : ((l = l.rect), (u = u.rect), (u = l.height !== u.height || l.width !== u.width)),
              u && (e.flags |= 32));
          } else e.flags |= 32;
          (e.flags & 4 && Tp(c, Ul === 0 ? n : n + `_` + Ul, i),
            (s && e.flags & 4) ||
              (Vl === null && (Vl = []), Vl.push(c, Ul === 0 ? r : r + `_` + Ul, t.memoizedProps)),
            Ul++);
        } else
          (t.tag !== 22 || t.memoizedState === null) &&
            (t.tag === 30 && o
              ? (e.flags |= t.flags & 32)
              : tu(e, t.child, n, r, i, a, o) && (s = !0));
        t = t.sibling;
      }
      return s;
    }
    function nu(e, t) {
      for (e = e.child; e !== null;) {
        if (e.tag === 30) {
          var n = e.memoizedProps,
            r = e.stateNode,
            i = mi(n, r),
            a = gi(n.default, n.update);
          if (t) {
            r = r.clones;
            var o = r === null ? null : r.map(kp);
          } else ((o = e.memoizedState), (e.memoizedState = null));
          r = e;
          var s = e.child;
          ((Ul = 0), (i = tu(r, s, i, i, a, o, !1)), e.flags & 4 && i && (t || Nd(e, n.onUpdate)));
        } else e.subtreeFlags & 33554432 && nu(e, t);
        e = e.sibling;
      }
    }
    var ru = !1,
      K = !1,
      iu = !1,
      au = !1,
      ou = typeof WeakSet == `function` ? WeakSet : Set,
      su = null,
      cu = !1,
      lu = !1,
      uu = !1,
      du = !1;
    function fu(e, t, n) {
      if (((e = e.containerInfo), (sp = gh), (e = Wr(e)), Gr(e))) {
        if (`selectionStart` in e) var r = { start: e.selectionStart, end: e.selectionEnd };
        else
          a: {
            r = ((r = e.ownerDocument) && r.defaultView) || window;
            var i = r.getSelection && r.getSelection();
            if (i && i.rangeCount !== 0) {
              r = i.anchorNode;
              var a = i.anchorOffset,
                o = i.focusNode;
              i = i.focusOffset;
              try {
                (r.nodeType, o.nodeType);
              } catch {
                r = null;
                break a;
              }
              var s = 0,
                c = -1,
                l = -1,
                u = 0,
                d = 0,
                f = e,
                p = null;
              b: for (;;) {
                for (
                  var m;
                  f !== r || (a !== 0 && f.nodeType !== 3) || (c = s + a),
                    f !== o || (i !== 0 && f.nodeType !== 3) || (l = s + i),
                    f.nodeType === 3 && (s += f.nodeValue.length),
                    (m = f.firstChild) !== null;
                )
                  ((p = f), (f = m));
                for (;;) {
                  if (f === e) break b;
                  if (
                    (p === r && ++u === a && (c = s),
                    p === o && ++d === i && (l = s),
                    (m = f.nextSibling) !== null)
                  )
                    break;
                  ((f = p), (p = f.parentNode));
                }
                f = m;
              }
              r = c === -1 || l === -1 ? null : { start: c, end: l };
            } else r = null;
          }
        r ||= { start: 0, end: 0 };
      } else r = null;
      for (
        cp = { focusedElem: e, selectionRange: r },
          gh = !1,
          n = (n & 335544064) === n,
          su = t,
          t = n ? 9270 : 1024;
        su !== null;
      ) {
        if (((e = su), n && ((r = e.deletions), r !== null)))
          for (a = 0; a < r.length; a++) n && Xl(r[a]);
        if (e.alternate === null && e.flags & 2) (n && Bl(e), pu(n));
        else {
          if (e.tag === 22) {
            if (((r = e.alternate), e.memoizedState !== null)) {
              (r !== null && r.memoizedState === null && n && Xl(r), pu(n));
              continue;
            }
            if (r !== null && r.memoizedState !== null) {
              (n && Bl(e), pu(n));
              continue;
            }
          }
          ((r = e.child),
            (e.subtreeFlags & t) !== 0 && r !== null
              ? ((r.return = e), (su = r))
              : (n && Zl(e), pu(n)));
        }
      }
      zl = null;
    }
    function pu(e) {
      for (; su !== null;) {
        var t = su,
          n = e,
          r = t.alternate,
          a = t.flags;
        switch (t.tag) {
          case 0:
          case 11:
          case 15:
            break;
          case 1:
            if (a & 1024 && r !== null) {
              ((n = void 0), (a = r.memoizedProps), (r = r.memoizedState));
              var o = t.stateNode;
              try {
                var s = bc(t.type, a);
                ((n = o.getSnapshotBeforeUpdate(s, r)),
                  (o.__reactInternalSnapshotBeforeUpdate = n));
              } catch (e) {
                Z(t, t.return, e);
              }
            }
            break;
          case 3:
            if (a & 1024) {
              if (((r = t.stateNode.containerInfo), (n = r.nodeType), n === 9)) nm(r);
              else if (n === 1)
                switch (r.nodeName) {
                  case `HEAD`:
                  case `HTML`:
                  case `BODY`:
                    nm(r);
                    break;
                  default:
                    r.textContent = ``;
                }
            }
            break;
          case 5:
          case 26:
          case 27:
          case 6:
          case 4:
          case 17:
            break;
          case 30:
            n &&
              r !== null &&
              ((n = mi(r.memoizedProps, r.stateNode)),
              (a = t.memoizedProps),
              (a = gi(a.default, a.update)),
              a !== `none` && Wl(r, n, a, (r.memoizedState = []), !0));
            break;
          default:
            if (a & 1024) throw Error(i(163));
        }
        if (((r = t.sibling), r !== null)) {
          ((r.return = t.return), (su = r));
          break;
        }
        su = t.return;
      }
    }
    function mu(e, t, n) {
      var r = n.flags;
      switch (n.tag) {
        case 0:
        case 11:
        case 15:
          (Pu(e, n), r & 4 && bl(5, n));
          break;
        case 1:
          if ((Pu(e, n), r & 4)) {
            if (((e = n.stateNode), t === null))
              try {
                e.componentDidMount();
              } catch (e) {
                Z(n, n.return, e);
              }
            else {
              var i = bc(n.type, t.memoizedProps);
              t = t.memoizedState;
              try {
                e.componentDidUpdate(i, t, e.__reactInternalSnapshotBeforeUpdate);
              } catch (e) {
                Z(n, n.return, e);
              }
            }
          }
          (r & 64 && Sl(n), r & 512 && wl(n, n.return));
          break;
        case 3:
          if ((Pu(e, n), r & 64 && ((e = n.updateQueue), e !== null))) {
            if (((t = null), n.child !== null))
              switch (n.child.tag) {
                case 27:
                case 5:
                  t = n.child.stateNode;
                  break;
                case 1:
                  t = n.child.stateNode;
              }
            try {
              So(e, t);
            } catch (e) {
              Z(n, n.return, e);
            }
          }
          break;
        case 27:
          t === null && r & 4 && Ll(n);
        case 26:
        case 5:
          (Pu(e, n), t === null && r & 4 && jl(n), r & 512 && wl(n, n.return));
          break;
        case 12:
          Pu(e, n);
          break;
        case 31:
          (Pu(e, n), r & 4 && Cu(e, n));
          break;
        case 13:
          (Pu(e, n),
            r & 4 && wu(e, n),
            r & 64 &&
              ((e = n.memoizedState),
              e !== null &&
                ((e = e.dehydrated), e !== null && ((n = _f.bind(null, n)), cm(e, n)))));
          break;
        case 22:
          if (((r = n.memoizedState !== null || ru), !r)) {
            var a = (t !== null && t.memoizedState !== null) || K;
            ((t = ru),
              (i = K),
              (ru = r),
              (K = a) && !i ? ((r = 2), n.subtreeFlags & 8772 && (r |= 1), Iu(e, n, r)) : Pu(e, n),
              (ru = t),
              (K = i));
          }
          break;
        case 30:
          (Pu(e, n), r & 512 && wl(n, n.return));
          break;
        case 7:
          r & 512 && wl(n, n.return);
        default:
          Pu(e, n);
      }
    }
    function hu(e, t) {
      for (e = e.child; e !== null;) (gu(e, t), (e = e.sibling));
    }
    function gu(e, t) {
      switch (e.tag) {
        case 5:
        case 26:
          try {
            var n = e.stateNode;
            if (t) {
              var r = n.style;
              typeof r.setProperty == `function`
                ? r.setProperty(`display`, `none`, `important`)
                : (r.display = `none`);
            } else {
              var i = e.stateNode,
                a = e.memoizedProps.style,
                o = a != null && a.hasOwnProperty(`display`) ? a.display : null;
              i.style.display = o == null || typeof o == `boolean` ? `` : (`` + o).trim();
            }
          } catch (t) {
            Z(e, e.return, t);
          }
          _u(e, t);
          break;
        case 6:
          try {
            ((e.stateNode.nodeValue = t ? `` : e.memoizedProps), (V = !0));
          } catch (t) {
            Z(e, e.return, t);
          }
          break;
        case 18:
          try {
            var s = e.stateNode;
            t ? wp(s, !0) : wp(e.stateNode, !1);
          } catch (t) {
            Z(e, e.return, t);
          }
          break;
        case 22:
        case 23:
          e.memoizedState === null && hu(e, t);
          break;
        default:
          hu(e, t);
      }
    }
    function _u(e, t) {
      if (e.subtreeFlags & 67108864)
        for (e = e.child; e !== null;) {
          a: {
            var n = e,
              r = t;
            switch (n.tag) {
              case 4:
                gu(n, r);
                break a;
              case 22:
                n.memoizedState === null && _u(n, r);
                break a;
              default:
                _u(n, r);
            }
          }
          e = e.sibling;
        }
    }
    function vu(e) {
      var t = e.alternate;
      (t !== null && ((e.alternate = null), vu(t)),
        (e.child = null),
        (e.deletions = null),
        (e.sibling = null),
        e.tag === 5 && ((t = e.stateNode), t !== null && kt(t)),
        (e.stateNode = null),
        (e.return = null),
        (e.dependencies = null),
        (e.memoizedProps = null),
        (e.memoizedState = null),
        (e.pendingProps = null),
        (e.stateNode = null),
        (e.updateQueue = null));
    }
    var yu = null,
      bu = !1;
    function xu(e, t, n) {
      for (n = n.child; n !== null;) (Su(e, t, n), (n = n.sibling));
    }
    function Su(e, t, n) {
      if (Ye && typeof Ye.onCommitFiberUnmount == `function`)
        try {
          Ye.onCommitFiberUnmount(Je, n);
        } catch {}
      switch (n.tag) {
        case 26:
          (K || Tl(n, t),
            xu(e, t, n),
            n.memoizedState
              ? n.memoizedState.count--
              : n.stateNode && !K && ((n = n.stateNode), n.parentNode.removeChild(n)));
          break;
        case 27:
          (K || Tl(n, t), Ol(n));
          var r = yu,
            i = bu;
          (Sp(n.type) && ((yu = n.stateNode), (bu = !1)),
            xu(e, t, n),
            gm(n.stateNode, n.type, n.memoizedProps),
            (yu = r),
            (bu = i));
          break;
        case 5:
          (K || Tl(n, t), Ol(n));
        case 6:
          if (
            (n.tag === 6 && Ol(n),
            (r = yu),
            (i = bu),
            (yu = null),
            xu(e, t, n),
            (yu = r),
            (bu = i),
            yu !== null)
          ) {
            if (bu)
              try {
                ((yu.nodeType === 9
                  ? yu.body
                  : yu.nodeName === `HTML`
                    ? yu.ownerDocument.body
                    : yu
                ).removeChild(n.stateNode),
                  (V = !0));
              } catch (e) {
                Z(n, t, e);
              }
            else
              try {
                (yu.removeChild(n.stateNode), (V = !0));
              } catch (e) {
                Z(n, t, e);
              }
          }
          break;
        case 18:
          yu !== null &&
            (bu
              ? ((e = yu),
                Cp(
                  e.nodeType === 9 ? e.body : e.nodeName === `HTML` ? e.ownerDocument.body : e,
                  n.stateNode,
                ),
                Hh(e))
              : Cp(yu, n.stateNode));
          break;
        case 4:
          ((r = yu),
            (i = bu),
            (yu = n.stateNode.containerInfo),
            (bu = !0),
            xu(e, t, n),
            (yu = r),
            (bu = i));
          break;
        case 0:
        case 11:
        case 14:
        case 15:
          (xl(2, n, t), K || xl(4, n, t), xu(e, t, n));
          break;
        case 1:
          (K ||
            (Tl(n, t),
            (r = n.stateNode),
            typeof r.componentWillUnmount == `function` && Cl(n, t, r)),
            xu(e, t, n));
          break;
        case 21:
          xu(e, t, n);
          break;
        case 22:
          ((K = (r = K) || n.memoizedState !== null), xu(e, t, n), (K = r));
          break;
        case 30:
          (Tl(n, t), xu(e, t, n));
          break;
        case 7:
          (K || Tl(n, t), xu(e, t, n));
          break;
        default:
          xu(e, t, n);
      }
    }
    function Cu(e, t) {
      if (
        t.memoizedState === null &&
        ((e = t.alternate), e !== null && ((e = e.memoizedState), e !== null))
      ) {
        e = e.dehydrated;
        try {
          Hh(e);
        } catch (e) {
          Z(t, t.return, e);
        }
      }
    }
    function wu(e, t) {
      if (
        t.memoizedState === null &&
        ((e = t.alternate),
        e !== null && ((e = e.memoizedState), e !== null && ((e = e.dehydrated), e !== null)))
      )
        try {
          Hh(e);
        } catch (e) {
          Z(t, t.return, e);
        }
    }
    function Tu(e) {
      switch (e.tag) {
        case 31:
        case 13:
        case 19:
          var t = e.stateNode;
          return (t === null && (t = e.stateNode = new ou()), t);
        case 22:
          return (
            (e = e.stateNode), (t = e._retryCache), t === null && (t = e._retryCache = new ou()), t
          );
        default:
          throw Error(i(435, e.tag));
      }
    }
    function Eu(e, t) {
      var n = Tu(e);
      t.forEach(function (t) {
        if (!n.has(t)) {
          n.add(t);
          var r = vf.bind(null, e, t);
          t.then(r, r);
        }
      });
    }
    function Du(e, t, n) {
      var r = t.deletions;
      if (r !== null)
        for (var a = 0; a < r.length; a++) {
          var o = r[a],
            s = e,
            c = t,
            l = c;
          a: for (; l !== null;) {
            switch (l.tag) {
              case 27:
                if (Sp(l.type)) {
                  ((yu = l.stateNode), (bu = !1));
                  break a;
                }
                break;
              case 5:
                ((yu = l.stateNode), (bu = !1));
                break a;
              case 3:
              case 4:
                ((yu = l.stateNode.containerInfo), (bu = !0));
                break a;
            }
            l = l.return;
          }
          if (yu === null) throw Error(i(160));
          (Su(s, c, o),
            (yu = null),
            (bu = !1),
            (s = o.alternate),
            s !== null && (s.return = null),
            (o.return = null));
        }
      if (t.subtreeFlags & 13886) for (t = t.child; t !== null;) (ku(t, e, n), (t = t.sibling));
    }
    var Ou = null;
    function ku(e, t, n) {
      var r = e.alternate,
        a = e.flags;
      switch (e.tag) {
        case 0:
        case 11:
        case 14:
        case 15:
          if (a & 4 && ((r = e.updateQueue), (r = r === null ? null : r.events), r !== null))
            for (var o = 0; o < r.length; o++) {
              var s = r[o];
              s.ref.impl = s.nextImpl;
            }
          (Du(t, e, n), Au(e), a & 4 && (xl(3, e, e.return), bl(3, e), xl(5, e, e.return)));
          break;
        case 1:
          (Du(t, e, n),
            Au(e),
            a & 512 && (K || r === null || Tl(r, r.return)),
            a & 64 &&
              ru &&
              ((e = e.updateQueue),
              e !== null &&
                ((t = e.callbacks),
                t !== null &&
                  ((n = e.shared.hiddenCallbacks),
                  (e.shared.hiddenCallbacks = n === null ? t : n.concat(t))))));
          break;
        case 26:
          if (
            ((o = Ou), Du(t, e, n), Au(e), a & 512 && (K || r === null || Tl(r, r.return)), a & 4)
          ) {
            if (((a = r === null ? null : r.memoizedState), (n = e.memoizedState), r === null)) {
              if (n === null) {
                if (e.stateNode === null) {
                  if (ru) e.stateNode = fp(e.type, e.memoizedProps, t.containerInfo, e);
                  else {
                    a: {
                      ((t = e.type), (n = e.memoizedProps), (a = o.ownerDocument || o));
                      b: switch (t) {
                        case `title`:
                          ((r = a.getElementsByTagName(`title`)[0]),
                            (!r ||
                              r[Dt] ||
                              r[bt] ||
                              r.namespaceURI === `http://www.w3.org/2000/svg` ||
                              r.hasAttribute(`itemprop`)) &&
                              ((r = a.createElement(t)),
                              a.head.insertBefore(r, a.querySelector(`head > title`))),
                            np(r, t, n),
                            (r[bt] = e),
                            Pt(r),
                            (t = r));
                          break a;
                        case `link`:
                          if ((o = Gm(`link`, `href`, a).get(t + (n.href || ``)))) {
                            for (s = 0; s < o.length; s++)
                              if (
                                ((r = o[s]),
                                r.getAttribute(`href`) ===
                                  (n.href == null || n.href === `` ? null : n.href) &&
                                  r.getAttribute(`rel`) === (n.rel == null ? null : n.rel) &&
                                  r.getAttribute(`title`) === (n.title == null ? null : n.title) &&
                                  r.getAttribute(`crossorigin`) ===
                                    (n.crossOrigin == null ? null : n.crossOrigin))
                              ) {
                                o.splice(s, 1);
                                break b;
                              }
                          }
                          ((r = a.createElement(t)), np(r, t, n), a.head.appendChild(r));
                          break;
                        case `meta`:
                          if ((o = Gm(`meta`, `content`, a).get(t + (n.content || ``)))) {
                            for (s = 0; s < o.length; s++)
                              if (
                                ((r = o[s]),
                                r.getAttribute(`content`) ===
                                  (n.content == null ? null : `` + n.content) &&
                                  r.getAttribute(`name`) === (n.name == null ? null : n.name) &&
                                  r.getAttribute(`property`) ===
                                    (n.property == null ? null : n.property) &&
                                  r.getAttribute(`http-equiv`) ===
                                    (n.httpEquiv == null ? null : n.httpEquiv) &&
                                  r.getAttribute(`charset`) ===
                                    (n.charSet == null ? null : n.charSet))
                              ) {
                                o.splice(s, 1);
                                break b;
                              }
                          }
                          ((r = a.createElement(t)), np(r, t, n), a.head.appendChild(r));
                          break;
                        default:
                          throw Error(i(468, t));
                      }
                      ((r[bt] = e), Pt(r), (t = r));
                    }
                    e.stateNode = t;
                  }
                } else ru || Km(o, e.type, e.stateNode);
              } else e.stateNode = Bm(o, n, e.memoizedProps);
            } else
              a === n
                ? n === null && e.stateNode !== null && Ml(e, e.memoizedProps, r.memoizedProps)
                : (a === null
                    ? ((t = r.stateNode), t === null || K || t.parentNode.removeChild(t))
                    : a.count--,
                  n === null ? ru || Km(o, e.type, e.stateNode) : Bm(o, n, e.memoizedProps));
          }
          break;
        case 27:
          (Du(t, e, n),
            Au(e),
            a & 512 && (K || r === null || Tl(r, r.return)),
            r !== null && a & 4 && Ml(e, e.memoizedProps, r.memoizedProps));
          break;
        case 5:
          if (
            ((o = iu),
            (iu = !1),
            Du(t, e, n),
            (iu = o),
            Au(e),
            a & 512 && (K || r === null || Tl(r, r.return)),
            e.flags & 32)
          ) {
            t = e.stateNode;
            try {
              (cn(t, ``), (V = !0));
            } catch (t) {
              Z(e, e.return, t);
            }
          }
          (a & 4 &&
            e.stateNode != null &&
            ((t = e.memoizedProps), Ml(e, t, r === null ? t : r.memoizedProps)),
            a & 1024 && (au = !0));
          break;
        case 6:
          if ((Du(t, e, n), Au(e), a & 4)) {
            if (e.stateNode === null) throw Error(i(162));
            ((t = e.memoizedProps), (n = e.stateNode));
            try {
              ((n.nodeValue = t), (V = !0));
            } catch (t) {
              Z(e, e.return, t);
            }
          }
          break;
        case 3:
          if (
            ((V = !1),
            (Wm = null),
            (o = Ou),
            (Ou = bm(t.containerInfo)),
            Du(t, e, n),
            (Ou = o),
            Au(e),
            a & 4 && r !== null && r.memoizedState.isDehydrated)
          )
            try {
              Hh(t.containerInfo);
            } catch (t) {
              Z(e, e.return, t);
            }
          (au && ((au = !1), ju(e)), (V = !1));
          break;
        case 4:
          ((a = iu),
            (iu = ru),
            (r = Wt()),
            (o = Ou),
            (Ou = bm(e.stateNode.containerInfo)),
            Du(t, e, n),
            Au(e),
            (Ou = o),
            V && lu && (uu = !0),
            (V = r),
            (iu = a));
          break;
        case 12:
          (Du(t, e, n), Au(e));
          break;
        case 31:
          (Du(t, e, n),
            Au(e),
            a & 4 && ((t = e.updateQueue), t !== null && ((e.updateQueue = null), Eu(e, t))));
          break;
        case 13:
          (Du(t, e, n),
            Au(e),
            e.child.flags & 8192 &&
              (e.memoizedState !== null) != (r !== null && r.memoizedState !== null) &&
              (md = ze()),
            a & 4 && ((t = e.updateQueue), t !== null && ((e.updateQueue = null), Eu(e, t))));
          break;
        case 22:
          ((o = e.memoizedState !== null), (s = r !== null && r.memoizedState !== null));
          var c = ru,
            l = K,
            u = iu;
          ((ru = c || o),
            (iu = u || o),
            (K = l || s),
            Du(t, e, n),
            (K = l),
            (iu = u),
            (ru = c),
            Au(e),
            a & 8192 &&
              ((t = e.stateNode),
              (t._visibility = o ? t._visibility & -2 : t._visibility | 1),
              !o ||
                r === null ||
                s ||
                ru ||
                K ||
                ((t = s || K),
                (n = ru),
                (r = K),
                (ru = o || ru),
                (K = t),
                Fu(e, 2),
                (ru = n),
                (K = r)),
              (!o && iu) || hu(e, o)),
            a & 4 &&
              ((t = e.updateQueue),
              t !== null && ((n = t.retryQueue), n !== null && ((t.retryQueue = null), Eu(e, n)))));
          break;
        case 19:
          (Du(t, e, n),
            Au(e),
            a & 4 && ((t = e.updateQueue), t !== null && ((e.updateQueue = null), Eu(e, t))));
          break;
        case 30:
          (a & 512 && (K || r === null || Tl(r, r.return)),
            (a = Wt()),
            (o = lu),
            (s = (n & 335544064) === n),
            (c = e.memoizedProps),
            (lu = s && gi(c.default, c.update) !== `none`),
            Du(t, e, n),
            Au(e),
            s && r !== null && V && (e.flags |= 4),
            (lu = o),
            (V = a));
          break;
        case 21:
          break;
        case 7:
          (a & 512 && (K || r === null || Tl(r, r.return)),
            r && r.stateNode !== null && (r.stateNode._fragmentFiber = e));
        default:
          (Du(t, e, n), Au(e));
      }
    }
    function Au(e) {
      var t = e.flags;
      if (t & 2) {
        try {
          for (var n, r = e.return; r !== null;) {
            if (Nl(r)) {
              n = r;
              break;
            }
            r = r.return;
          }
          r = null;
          for (var a = e.return; a !== null;) {
            if (Al(a)) {
              var o = a.stateNode;
              r === null ? (r = [o]) : r.push(o);
            }
            if (kl(a)) break;
            a = a.return;
          }
          var s = r;
          if (n == null) throw Error(i(160));
          switch (n.tag) {
            case 27:
              var c = n.stateNode;
              Il(e, Pl(e), c, s);
              break;
            case 5:
              var l = n.stateNode;
              (n.flags & 32 && (cn(l, ``), (n.flags &= -33)), Il(e, Pl(e), l, s));
              break;
            case 3:
            case 4:
              var u = n.stateNode.containerInfo;
              Fl(e, Pl(e), u, s);
              break;
            default:
              throw Error(i(161));
          }
        } catch (t) {
          Z(e, e.return, t);
        }
        e.flags &= -3;
      }
      t & 4096 && (e.flags &= -4097);
    }
    function ju(e) {
      if (e.subtreeFlags & 1024)
        for (e = e.child; e !== null;) {
          var t = e;
          (ju(t),
            t.tag === 5 && t.flags & 1024 && ((t = t.stateNode), (gh = !0), t.reset(), (gh = !1)),
            (e = e.sibling));
        }
    }
    function Mu(e, t) {
      if (t.subtreeFlags & 9270) for (t = t.child; t !== null;) (Nu(t, e), (t = t.sibling));
      else nu(t, !1);
    }
    function Nu(e, t) {
      var n = e.alternate;
      if (n === null) Jl(e, !1);
      else
        switch (e.tag) {
          case 3:
            if (((du = cu = !1), Hl(), Mu(t, e), !cu && !uu)) {
              if (((e = Vl), e !== null))
                for (var r = 0; r < e.length; r += 3) {
                  n = e[r];
                  var i = e[r + 1];
                  (Ep(n, e[r + 2]),
                    (n = n.ownerDocument.documentElement),
                    n !== null &&
                      n.animate(
                        { opacity: [0, 0], pointerEvents: [`none`, `none`] },
                        {
                          duration: 0,
                          fill: `forwards`,
                          pseudoElement: `::view-transition-group(` + i + `)`,
                        },
                      ));
                }
              ((e = t.containerInfo),
                (e = e.nodeType === 9 ? e.documentElement : e.ownerDocument.documentElement),
                e !== null &&
                  e.style.viewTransitionName === `` &&
                  ((e.style.viewTransitionName = `none`),
                  e.animate(
                    { opacity: [0, 0], pointerEvents: [`none`, `none`] },
                    {
                      duration: 0,
                      fill: `forwards`,
                      pseudoElement: `::view-transition-group(root)`,
                    },
                  ),
                  e.animate(
                    { width: [0, 0], height: [0, 0] },
                    { duration: 0, fill: `forwards`, pseudoElement: `::view-transition` },
                  )),
                (du = !0));
            }
            Vl = null;
            break;
          case 5:
            Mu(t, e);
            break;
          case 4:
            ((r = cu), (cu = !1), Mu(t, e), cu && (uu = !0), (cu = r));
            break;
          case 22:
            e.memoizedState === null && (n.memoizedState === null ? Mu(t, e) : Jl(e, !1));
            break;
          case 30:
            ((r = cu), (i = Hl()), (cu = !1), Mu(t, e), cu && (e.flags |= 4));
            var a = e.memoizedProps,
              o = e.stateNode;
            ((t = mi(a, o)), (o = mi(n.memoizedProps, o)));
            var s = gi(a.default, a.update);
            (s === `none`
              ? (t = !1)
              : ((a = n.memoizedState),
                (n.memoizedState = null),
                (n = e.child),
                (Ul = 0),
                (t = tu(e, n, t, o, s, a, !0)),
                Ul !== (a === null ? 0 : a.length) && (e.flags |= 32)),
              e.flags & 4 && t
                ? (Nd(e, e.memoizedProps.onUpdate), (Vl = i))
                : i !== null && (i.push.apply(i, Vl), (Vl = i)),
              (cu = e.flags & 32 ? !0 : r));
            break;
          default:
            Mu(t, e);
        }
    }
    function Pu(e, t) {
      if (t.subtreeFlags & 8772)
        for (t = t.child; t !== null;) (mu(e, t.alternate, t), (t = t.sibling));
    }
    function Fu(e, t) {
      for (e = e.child; e !== null;) {
        var n = e,
          r = t;
        switch (n.tag) {
          case 0:
          case 11:
          case 14:
          case 15:
            (xl(4, n, n.return), Fu(n, r));
            break;
          case 1:
            Tl(n, n.return);
            var i = n.stateNode;
            (typeof i.componentWillUnmount == `function` && Cl(n, n.return, i), Fu(n, r));
            break;
          case 27:
            r & 2 && gm(n.stateNode, n.type, n.memoizedProps);
          case 5:
            (Tl(n, n.return), (n.tag !== 5 && n.tag !== 27) || Ol(n), Fu(n, r));
            break;
          case 6:
            Ol(n);
            break;
          case 26:
            (Tl(n, n.return),
              (i = n.stateNode),
              n.memoizedState !== null || i === null || K || i.parentNode.removeChild(i),
              Fu(n, r));
            break;
          case 22:
            n.memoizedState === null && Fu(n, r);
            break;
          case 30:
            (Tl(n, n.return), Fu(n, r));
            break;
          case 7:
            Tl(n, n.return);
          default:
            Fu(n, r);
        }
        e = e.sibling;
      }
    }
    function Iu(e, t, n) {
      for (n = t.subtreeFlags & 8772 ? n : n & -2, t = t.child; t !== null;) {
        var r = t.alternate,
          i = e,
          a = t,
          o = a.flags,
          s = !!(n & 1);
        switch (a.tag) {
          case 0:
          case 11:
          case 15:
            (Iu(i, a, n), bl(4, a));
            break;
          case 1:
            if ((Iu(i, a, n), (r = a), (i = r.stateNode), typeof i.componentDidMount == `function`))
              try {
                i.componentDidMount();
              } catch (e) {
                Z(r, r.return, e);
              }
            if (((r = a), (i = r.updateQueue), i !== null)) {
              var c = r.stateNode;
              try {
                var l = i.shared.hiddenCallbacks;
                if (l !== null)
                  for (i.shared.hiddenCallbacks = null, i = 0; i < l.length; i++) xo(l[i], c);
              } catch (e) {
                Z(r, r.return, e);
              }
            }
            (s && o & 64 && Sl(a), wl(a, a.return));
            break;
          case 27:
            n & 2 && Ll(a);
          case 5:
            ((a.tag !== 5 && a.tag !== 27) || Dl(a),
              Iu(i, a, n),
              s && r === null && o & 4 && jl(a),
              wl(a, a.return));
            break;
          case 6:
            Dl(a);
            break;
          case 26:
            ((c = a.stateNode),
              a.memoizedState !== null || c === null || ru || Km(bm(c.ownerDocument), a.type, c),
              Iu(i, a, n),
              s && r === null && o & 4 && jl(a),
              wl(a, a.return));
            break;
          case 12:
            Iu(i, a, n);
            break;
          case 31:
            (Iu(i, a, n), s && o & 4 && Cu(i, a));
            break;
          case 13:
            (Iu(i, a, n), s && o & 4 && wu(i, a));
            break;
          case 22:
            (a.memoizedState === null && Iu(i, a, n), wl(a, a.return));
            break;
          case 30:
            (Iu(i, a, n), wl(a, a.return));
            break;
          case 7:
            wl(a, a.return);
          default:
            Iu(i, a, n);
        }
        t = t.sibling;
      }
    }
    function Lu(e, t) {
      var n = null;
      (e !== null &&
        e.memoizedState !== null &&
        e.memoizedState.cachePool !== null &&
        (n = e.memoizedState.cachePool.pool),
        (e = null),
        t.memoizedState !== null &&
          t.memoizedState.cachePool !== null &&
          (e = t.memoizedState.cachePool.pool),
        e !== n && (e != null && e.refCount++, n != null && Aa(n)));
    }
    function Ru(e, t) {
      ((e = null),
        t.alternate !== null && (e = t.alternate.memoizedState.cache),
        (t = t.memoizedState.cache),
        t !== e && (t.refCount++, e != null && Aa(e)));
    }
    function zu(e, t, n, r) {
      var i = (n & 335544064) === n;
      if (t.subtreeFlags & (i ? 10262 : 10256))
        for (t = t.child; t !== null;) (Bu(e, t, n, r), (t = t.sibling));
      else i && eu(t);
    }
    function Bu(e, t, n, r) {
      var i = (n & 335544064) === n;
      i && t.alternate === null && t.return !== null && t.return.alternate !== null && $l(t);
      var a = t.flags;
      switch (t.tag) {
        case 0:
        case 11:
        case 15:
          (zu(e, t, n, r), a & 2048 && bl(9, t));
          break;
        case 1:
          zu(e, t, n, r);
          break;
        case 3:
          (zu(e, t, n, r),
            i &&
              du &&
              ((e = e.containerInfo),
              (e = e.nodeType === 9 ? e.body : e.nodeName === `HTML` ? e.ownerDocument.body : e),
              e.style.viewTransitionName === `root` && (e.style.viewTransitionName = ``),
              (e = e.ownerDocument.documentElement),
              e !== null &&
                e.style.viewTransitionName === `none` &&
                (e.style.viewTransitionName = ``)),
            a & 2048 &&
              ((a = null),
              t.alternate !== null && (a = t.alternate.memoizedState.cache),
              (t = t.memoizedState.cache),
              t !== a && (t.refCount++, a != null && Aa(a))));
          break;
        case 12:
          if (a & 2048) {
            (zu(e, t, n, r), (a = t.stateNode));
            try {
              var o = t.memoizedProps,
                s = o.id,
                c = o.onPostCommit;
              typeof c == `function` &&
                c(s, t.alternate === null ? `mount` : `update`, a.passiveEffectDuration, -0);
            } catch (e) {
              Z(t, t.return, e);
            }
          } else zu(e, t, n, r);
          break;
        case 31:
          zu(e, t, n, r);
          break;
        case 13:
          zu(e, t, n, r);
          break;
        case 23:
          break;
        case 22:
          ((o = t.stateNode),
            (s = t.alternate),
            t.memoizedState === null
              ? (i && s !== null && s.memoizedState !== null && $l(t),
                o._visibility & 2
                  ? zu(e, t, n, r)
                  : ((o._visibility |= 2), Vu(e, t, n, r, !!(t.subtreeFlags & 10256) || !1)))
              : (i && s !== null && s.memoizedState === null && $l(s),
                o._visibility & 2 ? zu(e, t, n, r) : Hu(e, t)),
            a & 2048 && Lu(s, t));
          break;
        case 24:
          (zu(e, t, n, r), a & 2048 && Ru(t.alternate, t));
          break;
        case 30:
          (i && ((a = t.alternate), a !== null && (Kl(a.child, !0), Kl(t.child, !0))),
            zu(e, t, n, r));
          break;
        default:
          zu(e, t, n, r);
      }
    }
    function Vu(e, t, n, r, i) {
      for (i &&= !!(t.subtreeFlags & 10256) || !1, t = t.child; t !== null;) {
        var a = e,
          o = t,
          s = n,
          c = r,
          l = o.flags;
        switch (o.tag) {
          case 0:
          case 11:
          case 15:
            (Vu(a, o, s, c, i), bl(8, o));
            break;
          case 23:
            break;
          case 22:
            var u = o.stateNode;
            (o.memoizedState === null
              ? ((u._visibility |= 2), Vu(a, o, s, c, i))
              : u._visibility & 2
                ? Vu(a, o, s, c, i)
                : Hu(a, o),
              i && l & 2048 && Lu(o.alternate, o));
            break;
          case 24:
            (Vu(a, o, s, c, i), i && l & 2048 && Ru(o.alternate, o));
            break;
          default:
            Vu(a, o, s, c, i);
        }
        t = t.sibling;
      }
    }
    function Hu(e, t) {
      if (t.subtreeFlags & 10256)
        for (t = t.child; t !== null;) {
          var n = e,
            r = t,
            i = r.flags;
          switch (r.tag) {
            case 22:
              (Hu(n, r), i & 2048 && Lu(r.alternate, r));
              break;
            case 24:
              (Hu(n, r), i & 2048 && Ru(r.alternate, r));
              break;
            default:
              Hu(n, r);
          }
          t = t.sibling;
        }
    }
    var Uu = 8192;
    function Wu(e, t, n) {
      if (e.subtreeFlags & Uu) for (e = e.child; e !== null;) (Gu(e, t, n), (e = e.sibling));
    }
    function Gu(e, t, n) {
      switch (e.tag) {
        case 26:
          (Wu(e, t, n),
            e.flags & Uu &&
              (e.memoizedState === null
                ? ((e = e.stateNode), (t & 335544128) === t && Zm(n, e))
                : Qm(n, Ou, e.memoizedState, e.memoizedProps)));
          break;
        case 5:
          (Wu(e, t, n), e.flags & Uu && ((e = e.stateNode), (t & 335544128) === t && Zm(n, e)));
          break;
        case 3:
        case 4:
          var r = Ou;
          ((Ou = bm(e.stateNode.containerInfo)), Wu(e, t, n), (Ou = r));
          break;
        case 22:
          e.memoizedState === null &&
            ((r = e.alternate),
            r !== null && r.memoizedState !== null
              ? ((r = Uu), (Uu = 16777216), Wu(e, t, n), (Uu = r))
              : Wu(e, t, n));
          break;
        case 30:
          if ((e.flags & Uu) !== 0 && ((r = e.memoizedProps.name), r != null && r !== `auto`)) {
            var i = e.stateNode;
            ((i.paired = null), zl === null && (zl = new Map()), zl.set(r, i));
          }
          Wu(e, t, n);
          break;
        default:
          Wu(e, t, n);
      }
    }
    function Ku(e) {
      var t = e.alternate;
      if (t !== null && ((e = t.child), e !== null)) {
        t.child = null;
        do ((t = e.sibling), (e.sibling = null), (e = t));
        while (e !== null);
      }
    }
    function qu(e) {
      var t = e.deletions;
      if (e.flags & 16) {
        if (t !== null)
          for (var n = 0; n < t.length; n++) {
            var r = t[n];
            ((su = r), Xu(r, e));
          }
        Ku(e);
      }
      if (e.subtreeFlags & 10256) for (e = e.child; e !== null;) (Ju(e), (e = e.sibling));
    }
    function Ju(e) {
      switch (e.tag) {
        case 0:
        case 11:
        case 15:
          (qu(e), e.flags & 2048 && xl(9, e, e.return));
          break;
        case 3:
          qu(e);
          break;
        case 12:
          qu(e);
          break;
        case 22:
          var t = e.stateNode;
          e.memoizedState !== null &&
          t._visibility & 2 &&
          (e.return === null || e.return.tag !== 13)
            ? ((t._visibility &= -3), Yu(e))
            : qu(e);
          break;
        default:
          qu(e);
      }
    }
    function Yu(e) {
      var t = e.deletions;
      if (e.flags & 16) {
        if (t !== null)
          for (var n = 0; n < t.length; n++) {
            var r = t[n];
            ((su = r), Xu(r, e));
          }
        Ku(e);
      }
      for (e = e.child; e !== null;) {
        switch (((t = e), t.tag)) {
          case 0:
          case 11:
          case 15:
            (xl(8, t, t.return), Yu(t));
            break;
          case 22:
            ((n = t.stateNode), n._visibility & 2 && ((n._visibility &= -3), Yu(t)));
            break;
          default:
            Yu(t);
        }
        e = e.sibling;
      }
    }
    function Xu(e, t) {
      for (; su !== null;) {
        var n = su;
        switch (n.tag) {
          case 0:
          case 11:
          case 15:
            xl(8, n, t);
            break;
          case 23:
          case 22:
            if (n.memoizedState !== null && n.memoizedState.cachePool !== null) {
              var r = n.memoizedState.cachePool.pool;
              r != null && r.refCount++;
            }
            break;
          case 24:
            Aa(n.memoizedState.cache);
        }
        if (((r = n.child), r !== null)) ((r.return = n), (su = r));
        else
          a: for (n = e; su !== null;) {
            r = su;
            var i = r.sibling,
              a = r.return;
            if ((vu(r), r === n)) {
              su = null;
              break a;
            }
            if (i !== null) {
              ((i.return = a), (su = i));
              break a;
            }
            su = a;
          }
      }
    }
    var Zu = {
        getCacheForType: function (e) {
          var t = Sa(Oa),
            n = t.data.get(e);
          return (n === void 0 && ((n = e()), t.data.set(e, n)), n);
        },
        cacheSignal: function () {
          return Sa(Oa).controller.signal;
        },
      },
      Qu = typeof WeakMap == `function` ? WeakMap : Map,
      q = 0,
      $u = null,
      J = null,
      Y = 0,
      X = 0,
      ed = null,
      td = !1,
      nd = !1,
      rd = !1,
      id = 0,
      ad = 0,
      od = 0,
      sd = 0,
      cd = 0,
      ld = 0,
      ud = 0,
      dd = null,
      fd = null,
      pd = !1,
      md = 0,
      hd = 0,
      gd = 1 / 0,
      _d = null,
      vd = null,
      yd = 0,
      bd = null,
      xd = null,
      Sd = 0,
      Cd = 0,
      wd = null,
      Td = null,
      Ed = null,
      Dd = null,
      Od = null,
      kd = 0,
      Ad = null;
    function jd() {
      return q & 2 && Y !== 0 ? Y & -Y : L.T === null ? _t() : Pf();
    }
    function Md() {
      if (ld === 0) {
        if (!(Y & 536870912) || U) {
          var e = nt;
          ((nt <<= 1), !(nt & 3932160) && (nt = 262144), (ld = e));
        } else ld = 536870912;
      }
      return ((e = Oo.current), e !== null && (e.flags |= 32), ld);
    }
    function Nd(e, t) {
      if (t != null) {
        var n = e.stateNode,
          r = n.ref;
        (r === null && (r = n.ref = Pp(mi(e.memoizedProps, n))),
          Dd === null && (Dd = []),
          Dd.push(t.bind(null, r)));
      }
    }
    function Pd(e, t, n) {
      (((e === $u && (X === 2 || X === 9)) || e.cancelPendingCommit !== null) &&
        (Vd(e, 0), Rd(e, Y, ld, !1)),
        ut(e, n),
        (!(q & 2) || e !== $u) &&
          (e === $u && (!(q & 2) && (sd |= n), ad === 4 && Rd(e, Y, ld, !1)), Ef(e)));
    }
    function Fd(e, t, n) {
      if (q & 6) throw Error(i(327));
      var r = (!n && !(t & 127) && (t & e.expiredLanes) === 0) || at(e, t),
        a = r ? Yd(e, t) : qd(e, t, !0),
        o = r;
      do {
        if (a === 0) {
          nd && !r && Rd(e, t, 0, !1);
          break;
        }
        if (((n = e.current.alternate), o && !Ld(n))) ((a = qd(e, t, !1)), (o = !1));
        else {
          if (a === 2) {
            if (((o = t), e.errorRecoveryDisabledLanes & o)) var s = 0;
            else
              ((s = e.pendingLanes & -536870913),
                (s = s === 0 ? (s & 536870912 ? 536870912 : 0) : s));
            if (s !== 0) {
              t = s;
              a: {
                var c = e;
                a = dd;
                var l = c.current.memoizedState.isDehydrated;
                if ((l && (Vd(c, s).flags |= 256), (s = qd(c, s, !1)), s !== 2 && s !== 6)) {
                  if (rd && !l) {
                    ((c.errorRecoveryDisabledLanes |= o), (sd |= o), (a = 4));
                    break a;
                  }
                  ((o = fd),
                    (fd = a),
                    o !== null && (fd === null ? (fd = o) : fd.push.apply(fd, o)));
                }
                a = s;
              }
              if (((o = !1), a !== 2)) continue;
            }
          }
          if (a === 1) {
            (Vd(e, 0), Rd(e, t, 0, !0));
            break;
          }
          a: {
            switch (((r = e), (o = a), o)) {
              case 0:
              case 1:
                throw Error(i(345));
              case 4:
                if ((t & 4194048) !== t && (t & 62914560) !== t) break;
              case 6:
                Rd(r, t, ld, !td);
                break a;
              case 2:
                fd = null;
                break;
              case 3:
              case 5:
                break;
              default:
                throw Error(i(329));
            }
            if ((t & 62914560) === t && ((a = md + 300 - ze()), 10 < a)) {
              if ((Rd(r, t, ld, !td), it(r, 0, !0) !== 0)) break a;
              ((Sd = t),
                (r.timeoutHandle = gp(
                  Id.bind(null, r, n, fd, _d, pd, t, ld, sd, ud, td, o, `Throttled`, -0, 0),
                  a,
                )));
              break a;
            }
            Id(r, n, fd, _d, pd, t, ld, sd, ud, td, o, null, -0, 0);
          }
          break;
        }
      } while (1);
      Ef(e);
    }
    function Id(e, t, n, r, i, a, o, s, c, l, u, d, f, p) {
      e.timeoutHandle = -1;
      var m = t.subtreeFlags,
        h = (a & 335544064) === a;
      ((d = null),
        (h || m & 8192 || (m & 16785408) == 16785408) &&
        ((d = {
          stylesheets: null,
          count: 0,
          imgCount: 0,
          imgBytes: 0,
          suspenseyImages: [],
          waitingForImages: !0,
          waitingForViewTransition: !1,
          unsuspend: gn,
        }),
        (zl = null),
        Gu(t, a, d),
        h &&
          ((m = d),
          (h = e.containerInfo),
          (h = (h.nodeType === 9 ? h : h.ownerDocument).__reactViewTransition),
          h != null &&
            (m.count++,
            (m.waitingForViewTransition = !0),
            (m = nh.bind(m)),
            h.finished.then(m, m))),
        (m = (a & 62914560) === a ? md - ze() : (a & 4194048) === a ? hd - ze() : 0),
        (m = eh(d, m)),
        m !== null)
          ? ((Sd = a),
            (e.cancelPendingCommit = m(
              nf.bind(null, e, t, a, n, r, i, o, s, c, l, u, d, null, f, p),
            )),
            Rd(e, a, o, !l))
          : nf(e, t, a, n, r, i, o, s, c, l, u, d));
    }
    function Ld(e) {
      for (var t = e; ;) {
        var n = t.tag;
        if (
          (n === 0 || n === 11 || n === 15) &&
          t.flags & 16384 &&
          ((n = t.updateQueue), n !== null && ((n = n.stores), n !== null))
        )
          for (var r = 0; r < n.length; r++) {
            var i = n[r],
              a = i.getSnapshot;
            i = i.value;
            try {
              if (!Rr(a(), i)) return !1;
            } catch {
              return !1;
            }
          }
        if (((n = t.child), t.subtreeFlags & 16384 && n !== null)) ((n.return = t), (t = n));
        else {
          if (t === e) break;
          for (; t.sibling === null;) {
            if (t.return === null || t.return === e) return !0;
            t = t.return;
          }
          ((t.sibling.return = t.return), (t = t.sibling));
        }
      }
      return !0;
    }
    function Rd(e, t, n, r) {
      ((t = ot(e, t)),
        (t &= ~cd),
        (t &= ~sd),
        (e.suspendedLanes |= t),
        (e.pingedLanes &= ~t),
        r && (e.warmLanes |= t),
        (r = e.expirationTimes));
      for (var i = t; 0 < i;) {
        var a = 31 - Ze(i),
          o = 1 << a;
        ((r[a] = -1), (i &= ~o));
      }
      n !== 0 && ft(e, n, t);
    }
    function zd() {
      return q & 6 ? !0 : (Df(0, !1), !1);
    }
    function Bd() {
      if (J !== null) {
        if (X === 0) var e = J.return;
        else ((e = J), (ma = pa = null), ns(e), (no = null), (ro = 0), (e = J));
        for (; e !== null;) (yl(e.alternate, e), (e = e.return));
        J = null;
      }
    }
    function Vd(e, t) {
      var n = e.timeoutHandle;
      return (
        n !== -1 && ((e.timeoutHandle = -1), _p(n)),
        (n = e.cancelPendingCommit),
        n !== null && ((e.cancelPendingCommit = null), n()),
        (Sd = 0),
        Bd(),
        ($u = e),
        (J = n = ji(e.current, null)),
        (Y = t),
        (X = 0),
        (ed = null),
        (td = !1),
        (nd = at(e, t)),
        (rd = !1),
        (ud = ld = cd = sd = od = ad = 0),
        (fd = dd = null),
        (pd = !1),
        (id = ot(e, t)),
        xi(),
        n
      );
    }
    function Hd(e, t) {
      ((W = null),
        (L.H = fc),
        t === Ka || t === Ja
          ? ((t = eo()), (X = 3))
          : t === qa
            ? ((t = eo()), (X = 4))
            : (X = t === Ac ? 8 : typeof t == `object` && t && typeof t.then == `function` ? 6 : 1),
        (ed = t),
        J === null && ((ad = 1), wc(e, zi(t, e.current))));
    }
    function Ud() {
      var e = Oo.current;
      return e === null
        ? !0
        : (Y & 4194048) === Y
          ? ko === null
          : (Y & 62914560) === Y || Y & 536870912
            ? e === ko
            : !1;
    }
    function Wd() {
      var e = L.H;
      return ((L.H = fc), e === null ? fc : e);
    }
    function Gd() {
      var e = L.A;
      return ((L.A = Zu), e);
    }
    function Kd() {
      ((ad = 4),
        td || ((Y & 4194048) !== Y && Oo.current !== null) || (nd = !0),
        (!(od & 134217727) && !(sd & 134217727)) || $u === null || Rd($u, Y, ld, !1));
    }
    function qd(e, t, n) {
      var r = q;
      q |= 2;
      var i = Wd(),
        a = Gd();
      (($u !== e || Y !== t) && ((_d = null), Vd(e, t)), (t = !1));
      var o = ad;
      a: do
        try {
          if (X !== 0 && J !== null) {
            var s = J,
              c = ed;
            switch (X) {
              case 8:
                (Bd(), (o = 6));
                break a;
              case 3:
              case 2:
              case 9:
              case 6:
                Oo.current === null && (t = !0);
                var l = X;
                if (((X = 0), (ed = null), $d(e, s, c, l), n && nd)) {
                  o = 0;
                  break a;
                }
                break;
              default:
                ((l = X), (X = 0), (ed = null), $d(e, s, c, l));
            }
          }
          (Jd(), (o = ad));
          break;
        } catch (t) {
          Hd(e, t);
        }
      while (1);
      return (
        t && e.shellSuspendCounter++,
        (ma = pa = null),
        (q = r),
        (L.H = i),
        (L.A = a),
        J === null && (($u = null), (Y = 0), xi()),
        o
      );
    }
    function Jd() {
      for (; J !== null;) Zd(J);
    }
    function Yd(e, t) {
      var n = q;
      q |= 2;
      var r = Wd(),
        a = Gd();
      $u !== e || Y !== t ? ((_d = null), (gd = ze() + 500), Vd(e, t)) : (nd = at(e, t));
      a: do
        try {
          if (X !== 0 && J !== null) {
            t = J;
            var o = ed;
            b: switch (X) {
              case 1:
                ((X = 0), (ed = null), $d(e, t, o, 1));
                break;
              case 2:
              case 9:
                if (Xa(o)) {
                  ((X = 0), (ed = null), Qd(t));
                  break;
                }
                ((t = function () {
                  ((X !== 2 && X !== 9) || $u !== e || (X = 7), Ef(e));
                }),
                  o.then(t, t));
                break a;
              case 3:
                X = 7;
                break a;
              case 4:
                X = 5;
                break a;
              case 7:
                Xa(o) ? ((X = 0), (ed = null), Qd(t)) : ((X = 0), (ed = null), $d(e, t, o, 7));
                break;
              case 5:
                var s = null;
                switch (J.tag) {
                  case 26:
                    s = J.memoizedState;
                  case 5:
                  case 27:
                    var c = J;
                    if (s ? Ym(s) : c.stateNode.complete) {
                      ((X = 0), (ed = null));
                      var l = c.sibling;
                      if (l !== null) J = l;
                      else {
                        var u = c.return;
                        u === null ? (J = null) : ((J = u), ef(u));
                      }
                      break b;
                    }
                }
                ((X = 0), (ed = null), $d(e, t, o, 5));
                break;
              case 6:
                ((X = 0), (ed = null), $d(e, t, o, 6));
                break;
              case 8:
                (Bd(), (ad = 6));
                break a;
              default:
                throw Error(i(462));
            }
          }
          Xd();
          break;
        } catch (t) {
          Hd(e, t);
        }
      while (1);
      return (
        (ma = pa = null),
        (L.H = r),
        (L.A = a),
        (q = n),
        J === null ? (($u = null), (Y = 0), xi(), ad) : 0
      );
    }
    function Xd() {
      for (; J !== null && !Le();) Zd(J);
    }
    function Zd(e) {
      var t = ul(e.alternate, e, id);
      ((e.memoizedProps = e.pendingProps), t === null ? ef(e) : (J = t));
    }
    function Qd(e) {
      var t = e,
        n = t.alternate;
      switch (t.tag) {
        case 15:
        case 0:
          t = Wc(n, t, t.pendingProps, t.type, void 0, Y);
          break;
        case 11:
          t = Wc(n, t, t.pendingProps, t.type.render, t.ref, Y);
          break;
        case 5:
          ns(t);
          var r = t;
          r === ea &&
            (U
              ? (sa(r), r.tag === 5 && r.stateNode != null && (ta = r.stateNode))
              : (sa(r), (U = !0)));
        default:
          (yl(n, t), (t = J = Mi(t, id)), (t = ul(n, t, id)));
      }
      ((e.memoizedProps = e.pendingProps), t === null ? ef(e) : (J = t));
    }
    function $d(e, t, n, r) {
      ((ma = pa = null), ns(t), (no = null), (ro = 0));
      var i = t.return;
      try {
        if (kc(e, i, t, n, Y)) {
          ((ad = 1), wc(e, zi(n, e.current)), (J = null));
          return;
        }
      } catch (t) {
        if (i !== null) throw ((J = i), t);
        ((ad = 1), wc(e, zi(n, e.current)), (J = null));
        return;
      }
      t.flags & 32768
        ? (U || r === 1
            ? (e = !0)
            : nd || Y & 536870912
              ? (e = !1)
              : ((td = e = !0),
                (r === 2 || r === 9 || r === 3 || r === 6) &&
                  ((r = Oo.current), r !== null && r.tag === 13 && (r.flags |= 16384))),
          tf(t, e))
        : ef(t);
    }
    function ef(e) {
      var t = e;
      do {
        if (t.flags & 32768) {
          tf(t, td);
          return;
        }
        e = t.return;
        var n = _l(t.alternate, t, id);
        if (n !== null) {
          J = n;
          return;
        }
        if (((t = t.sibling), t !== null)) {
          J = t;
          return;
        }
        J = t = e;
      } while (t !== null);
      ad === 0 && (ad = 5);
    }
    function tf(e, t) {
      do {
        var n = vl(e.alternate, e);
        if (n !== null) {
          ((n.flags &= 32767), (J = n));
          return;
        }
        if (
          ((n = e.return),
          n !== null && ((n.flags |= 32768), (n.subtreeFlags = 0), (n.deletions = null)),
          !t && ((e = e.sibling), e !== null))
        ) {
          J = e;
          return;
        }
        J = e = n;
      } while (e !== null);
      ((ad = 6), (J = null));
    }
    function nf(e, t, n, r, a, o, s, c, l, u, d, f) {
      e.cancelPendingCommit = null;
      do df();
      while (yd !== 0);
      if (q & 6) throw Error(i(327));
      if (t !== null) {
        if (t === e.current) throw Error(i(177));
        (e === $u && ((J = $u = null), (Y = 0)),
          (xd = t),
          (bd = e),
          (Sd = n),
          (wd = a),
          (Td = r),
          rf(e, t, n, s, c, l, f));
      }
    }
    function rf(e, t, n, r, i, a, o) {
      var s = t.lanes | t.childLanes;
      if (
        ((Cd = s),
        (s |= bi),
        dt(e, n, s, r, i, a),
        (Dd = null),
        (n & 335544064) === n ? ((Od = Na(e)), (r = 10262)) : ((Od = null), (r = 10256)),
        (t.subtreeFlags & r) !== 0 || (t.flags & r) !== 0
          ? ((e.callbackNode = null),
            (e.callbackPriority = 0),
            yf(Ue, function () {
              return (ff(), null);
            }))
          : ((e.callbackNode = null), (e.callbackPriority = 0)),
        (Rl = !1),
        (r = !!(t.flags & 13878)),
        t.subtreeFlags & 13878 || r)
      ) {
        ((r = L.T), (L.T = null), (i = R.p), (R.p = 2), (a = q), (q |= 4));
        try {
          fu(e, t, n);
        } finally {
          ((q = a), (R.p = i), (L.T = r));
        }
      }
      ((yd = 1),
        Rl
          ? (Ed = Mp(o, e.containerInfo, Od, sf, cf, of, lf, ff, af, null, null))
          : (sf(), cf(), lf()));
    }
    function af(e) {
      if (yd !== 0) {
        var t = bd.onRecoverableError;
        t(e, { componentStack: null });
      }
    }
    function of() {
      yd === 3 && ((yd = 0), Nu(xd, bd), (yd = 4));
    }
    function sf() {
      if (yd === 1) {
        yd = 0;
        var e = bd,
          t = xd,
          n = Sd,
          r = !!(t.flags & 13878);
        if (t.subtreeFlags & 13878 || r) {
          ((r = L.T), (L.T = null));
          var i = R.p;
          R.p = 2;
          var a = q;
          q |= 4;
          try {
            ((lu = uu = !1), ku(t, e, n), (n = cp));
            var o = Wr(e.containerInfo),
              s = n.focusedElem,
              c = n.selectionRange;
            if (o !== s && s && s.ownerDocument && Ur(s.ownerDocument.documentElement, s)) {
              if (c !== null && Gr(s)) {
                var l = c.start,
                  u = c.end;
                if ((u === void 0 && (u = l), `selectionStart` in s))
                  ((s.selectionStart = l), (s.selectionEnd = Math.min(u, s.value.length)));
                else {
                  var d = s.ownerDocument || document,
                    f = (d && d.defaultView) || window;
                  if (f.getSelection) {
                    var p = f.getSelection(),
                      m = s.textContent.length,
                      h = Math.min(c.start, m),
                      g = c.end === void 0 ? h : Math.min(c.end, m);
                    !p.extend && h > g && ((o = g), (g = h), (h = o));
                    var _ = Hr(s, h),
                      v = Hr(s, g);
                    if (
                      _ &&
                      v &&
                      (p.rangeCount !== 1 ||
                        p.anchorNode !== _.node ||
                        p.anchorOffset !== _.offset ||
                        p.focusNode !== v.node ||
                        p.focusOffset !== v.offset)
                    ) {
                      var y = d.createRange();
                      (y.setStart(_.node, _.offset),
                        p.removeAllRanges(),
                        h > g
                          ? (p.addRange(y), p.extend(v.node, v.offset))
                          : (y.setEnd(v.node, v.offset), p.addRange(y)));
                    }
                  }
                }
              }
              for (d = [], p = s; (p = p.parentNode);)
                p.nodeType === 1 && d.push({ element: p, left: p.scrollLeft, top: p.scrollTop });
              for (typeof s.focus == `function` && s.focus(), s = 0; s < d.length; s++) {
                var b = d[s];
                ((b.element.scrollLeft = b.left), (b.element.scrollTop = b.top));
              }
            }
            ((gh = !!sp), (cp = sp = null));
          } finally {
            ((q = a), (R.p = i), (L.T = r));
          }
        }
        ((e.current = t), (yd = 2));
      }
    }
    function cf() {
      if (yd === 2) {
        yd = 0;
        var e = bd,
          t = xd,
          n = !!(t.flags & 8772);
        if (t.subtreeFlags & 8772 || n) {
          ((n = L.T), (L.T = null));
          var r = R.p;
          R.p = 2;
          var i = q;
          q |= 4;
          try {
            mu(e, t.alternate, t);
          } finally {
            ((q = i), (R.p = r), (L.T = n));
          }
        }
        yd = 3;
      }
    }
    function lf() {
      if (yd === 4 || yd === 3) {
        yd = 0;
        var e = Ed;
        ((Ed = null), Re());
        var t = bd,
          n = xd,
          r = Sd,
          i = Td,
          a = (r & 335544064) === r ? 10262 : 10256;
        if (
          ((n.subtreeFlags & a) !== 0 || (n.flags & a) !== 0
            ? (yd = 5)
            : ((yd = 0), (xd = bd = null), uf(t, t.pendingLanes)),
          (a = t.pendingLanes),
          a === 0 && (vd = null),
          gt(r),
          (n = n.stateNode),
          Ye && typeof Ye.onCommitFiberRoot == `function`)
        )
          try {
            Ye.onCommitFiberRoot(Je, n, void 0, (n.current.flags & 128) == 128);
          } catch {}
        if (i !== null) {
          ((n = L.T), (a = R.p), (R.p = 2), (L.T = null));
          try {
            for (var o = t.onRecoverableError, s = 0; s < i.length; s++) {
              var c = i[s];
              o(c.value, { componentStack: c.stack });
            }
          } finally {
            ((L.T = n), (R.p = a));
          }
        }
        if (
          ((i = Dd),
          (o = Od),
          (Od = null),
          i !== null && ((Dd = null), o === null && (o = []), e !== null))
        )
          for (c = 0; c < i.length; c++)
            ((n = (0, i[c])(o)), n !== void 0 && e.finished.finally(n));
        (Sd & 3 && df(),
          Ef(t),
          (a = t.pendingLanes),
          r & 261930 && a & 42 ? (t === Ad ? kd++ : ((kd = 0), (Ad = t))) : ((kd = 0), (Ad = null)),
          Df(0, !1));
      }
    }
    function uf(e, t) {
      (e.pooledCacheLanes &= t) === 0 &&
        ((t = e.pooledCache), t != null && ((e.pooledCache = null), Aa(t)));
    }
    function df() {
      return (Ed !== null && (Ed.skipTransition(), (Ed = null)), sf(), cf(), lf(), ff());
    }
    function ff() {
      if (yd !== 5) return !1;
      var e = bd,
        t = Cd;
      Cd = 0;
      var n = gt(Sd),
        r = L.T,
        a = R.p;
      try {
        ((R.p = 32 > n ? 32 : n), (L.T = null), (n = wd), (wd = null));
        var o = bd,
          s = Sd;
        if (((yd = 0), (xd = bd = null), (Sd = 0), q & 6)) throw Error(i(331));
        var c = q;
        if (
          ((q |= 4),
          Ju(o.current),
          Bu(o, o.current, s, n),
          (q = c),
          Df(0, !1),
          Ye && typeof Ye.onPostCommitFiberRoot == `function`)
        )
          try {
            Ye.onPostCommitFiberRoot(Je, o);
          } catch {}
        return !0;
      } finally {
        ((R.p = a), (L.T = r), uf(e, t));
      }
    }
    function pf(e, t, n) {
      ((t = zi(n, t)),
        (t = Ec(e.stateNode, t, 2)),
        (e = ho(e, t, 2)),
        e !== null && (ut(e, 2), Ef(e)));
    }
    function Z(e, t, n) {
      if (e.tag === 3) pf(e, e, n);
      else
        for (; t !== null;) {
          if (t.tag === 3) {
            pf(t, e, n);
            break;
          }
          if (t.tag === 1) {
            var r = t.stateNode;
            if (
              typeof t.type.getDerivedStateFromError == `function` ||
              (typeof r.componentDidCatch == `function` && (vd === null || !vd.has(r)))
            ) {
              ((e = zi(n, e)),
                (n = Dc(2)),
                (r = ho(t, n, 2)),
                r !== null && (Oc(n, r, t, e), ut(r, 2), Ef(r)));
              break;
            }
          }
          t = t.return;
        }
    }
    function mf(e, t, n) {
      var r = e.pingCache;
      if (r === null) {
        r = e.pingCache = new Qu();
        var i = new Set();
        r.set(t, i);
      } else ((i = r.get(t)), i === void 0 && ((i = new Set()), r.set(t, i)));
      i.has(n) || ((rd = !0), i.add(n), (e = hf.bind(null, e, t, n)), t.then(e, e));
    }
    function hf(e, t, n) {
      var r = e.pingCache;
      (r !== null && r.delete(t),
        (e.pingedLanes |= e.suspendedLanes & n),
        (e.warmLanes &= ~n),
        $u === e &&
          (Y & n) === n &&
          (ad === 4 || (ad === 3 && (Y & 62914560) === Y && 300 > ze() - md)
            ? q & 2
              ? (cd |= n)
              : Vd(e, 0)
            : (cd |= n),
          ud === Y && (ud = 0)),
        Ef(e));
    }
    function gf(e, t) {
      (t === 0 && (t = ct()), (e = wi(e, t)), e !== null && (ut(e, t), Ef(e)));
    }
    function _f(e) {
      var t = e.memoizedState,
        n = 0;
      (t !== null && (n = t.retryLane), gf(e, n));
    }
    function vf(e, t) {
      var n = 0;
      switch (e.tag) {
        case 31:
        case 13:
          var r = e.stateNode,
            a = e.memoizedState;
          a !== null && (n = a.retryLane);
          break;
        case 19:
          r = e.stateNode;
          break;
        case 22:
          r = e.stateNode._retryCache;
          break;
        default:
          throw Error(i(314));
      }
      (r !== null && r.delete(t), gf(e, n));
    }
    function yf(e, t) {
      return Fe(e, t);
    }
    var bf = null,
      xf = null,
      Sf = !1,
      Cf = !1,
      wf = !1,
      Tf = 0;
    function Ef(e) {
      (e !== xf && e.next === null && (xf === null ? (bf = xf = e) : (xf = xf.next = e)),
        (Cf = !0),
        Sf || ((Sf = !0), Nf()));
    }
    function Df(e, t) {
      if (!wf && Cf) {
        wf = !0;
        do
          for (var n = !1, r = bf; r !== null;) {
            if (!t) {
              if (e !== 0) {
                var i = r.pendingLanes;
                if (i === 0) var a = 0;
                else {
                  var o = r.suspendedLanes,
                    s = r.pingedLanes;
                  ((a = (1 << (31 - Ze(42 | e) + 1)) - 1),
                    (a &= i & ~(o & ~s)),
                    (a = a & 201326741 ? (a & 201326741) | 1 : a ? a | 2 : 0));
                }
                a !== 0 && ((n = !0), Mf(r, a));
              } else
                ((a = Y),
                  (a = it(
                    r,
                    r === $u ? a : 0,
                    r.cancelPendingCommit !== null || r.timeoutHandle !== -1,
                  )),
                  !(a & 3) || at(r, a) || ((n = !0), Mf(r, a)));
            }
            r = r.next;
          }
        while (n);
        wf = !1;
      }
    }
    function Of() {
      kf();
    }
    function kf() {
      Cf = Sf = !1;
      var e = 0;
      Tf !== 0 && hp() && (e = Tf);
      for (var t = ze(), n = null, r = bf; r !== null;) {
        var i = r.next,
          a = Af(r, t);
        (a === 0
          ? ((r.next = null), n === null ? (bf = i) : (n.next = i), i === null && (xf = n))
          : ((n = r), (e !== 0 || a & 3) && (Cf = !0)),
          (r = i));
      }
      ((yd !== 0 && yd !== 5) || Df(e, !1), Tf !== 0 && (Tf = 0));
    }
    function Af(e, t) {
      for (
        var n = e.suspendedLanes,
          r = e.pingedLanes,
          i = e.expirationTimes,
          a = e.pendingLanes & -62914561;
        0 < a;
      ) {
        var o = 31 - Ze(a),
          s = 1 << o,
          c = i[o];
        (c === -1
          ? ((s & n) === 0 || (s & r) !== 0) && (i[o] = st(s, t))
          : c <= t && (e.expiredLanes |= s),
          (a &= ~s));
      }
      if (
        ((t = $u),
        (n = Y),
        (n = it(e, e === t ? n : 0, e.cancelPendingCommit !== null || e.timeoutHandle !== -1)),
        (r = e.callbackNode),
        n === 0 || (e === t && (X === 2 || X === 9)) || e.cancelPendingCommit !== null)
      )
        return (
          r !== null && r !== null && Ie(r), (e.callbackNode = null), (e.callbackPriority = 0)
        );
      if (!(n & 3) || at(e, n)) {
        if (((t = n & -n), t === e.callbackPriority)) return t;
        switch ((r !== null && Ie(r), gt(n))) {
          case 2:
          case 8:
            n = He;
            break;
          case 32:
            n = Ue;
            break;
          case 268435456:
            n = Ge;
            break;
          default:
            n = Ue;
        }
        return (
          (r = jf.bind(null, e)), (n = Fe(n, r)), (e.callbackPriority = t), (e.callbackNode = n), t
        );
      }
      return (
        r !== null && r !== null && Ie(r), (e.callbackPriority = 2), (e.callbackNode = null), 2
      );
    }
    function jf(e, t) {
      if (yd !== 0 && yd !== 5) return ((e.callbackNode = null), (e.callbackPriority = 0), null);
      var n = e.callbackNode;
      if (df() && e.callbackNode !== n) return null;
      var r = Y;
      return (
        (r = it(e, e === $u ? r : 0, e.cancelPendingCommit !== null || e.timeoutHandle !== -1)),
        r === 0
          ? null
          : (Fd(e, r, t),
            Af(e, ze()),
            e.callbackNode != null && e.callbackNode === n ? jf.bind(null, e) : null)
      );
    }
    function Mf(e, t) {
      if (df()) return null;
      Fd(e, t, !0);
    }
    function Nf() {
      bp(function () {
        q & 6 ? Fe(Ve, Of) : kf();
      });
    }
    function Pf() {
      if (Tf === 0) {
        var e = Ia;
        (e === 0 && ((e = tt), (tt <<= 1), !(tt & 261888) && (tt = 256)), (Tf = e));
      }
      return Tf;
    }
    function Ff(e) {
      return e == null || typeof e == `symbol` || typeof e == `boolean`
        ? null
        : typeof e == `function`
          ? e
          : hn(e);
    }
    function If(e, t, n, r, i) {
      if (t === `submit` && n && n.stateNode === i) {
        var a = Ff((i[xt] || null).action),
          o = r.submitter;
        o &&
          ((t = (t = o[xt] || null) ? Ff(t.formAction) : o.getAttribute(`formAction`)),
          t !== null && ((a = t), (o = null)));
        var s = new Ln(`action`, `action`, null, r, i);
        e.push({
          event: s,
          listeners: [
            {
              instance: null,
              listener: function () {
                if (r.defaultPrevented) {
                  if (Tf !== 0) {
                    var e = new FormData(i, o);
                    Qs(n, { pending: !0, data: e, method: i.method, action: a }, null, e);
                  }
                } else
                  typeof a == `function` &&
                    (s.preventDefault(),
                    (e = new FormData(i, o)),
                    Qs(n, { pending: !0, data: e, method: i.method, action: a }, a, e));
              },
              currentTarget: i,
            },
          ],
        });
      }
    }
    for (var Lf = 0; Lf < di.length; Lf++) {
      var Rf = di[Lf];
      fi(Rf.toLowerCase(), `on` + (Rf[0].toUpperCase() + Rf.slice(1)));
    }
    (fi(ri, `onAnimationEnd`),
      fi(ii, `onAnimationIteration`),
      fi(ai, `onAnimationStart`),
      fi(`dblclick`, `onDoubleClick`),
      fi(`focusin`, `onFocus`),
      fi(`focusout`, `onBlur`),
      fi(oi, `onTransitionRun`),
      fi(si, `onTransitionStart`),
      fi(ci, `onTransitionCancel`),
      fi(li, `onTransitionEnd`),
      zt(`onMouseEnter`, [`mouseout`, `mouseover`]),
      zt(`onMouseLeave`, [`mouseout`, `mouseover`]),
      zt(`onPointerEnter`, [`pointerout`, `pointerover`]),
      zt(`onPointerLeave`, [`pointerout`, `pointerover`]),
      Rt(
        `onChange`,
        `change click focusin focusout input keydown keyup selectionchange`.split(` `),
      ),
      Rt(
        `onSelect`,
        `focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange`.split(
          ` `,
        ),
      ),
      Rt(`onBeforeInput`, [`compositionend`, `keypress`, `textInput`, `paste`]),
      Rt(`onCompositionEnd`, `compositionend focusout keydown keypress keyup mousedown`.split(` `)),
      Rt(
        `onCompositionStart`,
        `compositionstart focusout keydown keypress keyup mousedown`.split(` `),
      ),
      Rt(
        `onCompositionUpdate`,
        `compositionupdate focusout keydown keypress keyup mousedown`.split(` `),
      ));
    var zf =
        `abort canplay canplaythrough durationchange emptied encrypted ended error loadeddata loadedmetadata loadstart pause play playing progress ratechange resize seeked seeking stalled suspend timeupdate volumechange waiting`.split(
          ` `,
        ),
      Bf = new Set(
        `beforetoggle cancel close invalid load scroll scrollend toggle`.split(` `).concat(zf),
      );
    function Vf(e, t) {
      t = !!(t & 4);
      for (var n = 0; n < e.length; n++) {
        var r = e[n],
          i = r.event;
        r = r.listeners;
        a: {
          var a = void 0;
          if (t)
            for (var o = r.length - 1; 0 <= o; o--) {
              var s = r[o],
                c = s.instance,
                l = s.currentTarget;
              if (((s = s.listener), c !== a && i.isPropagationStopped())) break a;
              ((a = s), (i.currentTarget = l));
              try {
                a(i);
              } catch (e) {
                _i(e);
              }
              ((i.currentTarget = null), (a = c));
            }
          else
            for (o = 0; o < r.length; o++) {
              if (
                ((s = r[o]),
                (c = s.instance),
                (l = s.currentTarget),
                (s = s.listener),
                c !== a && i.isPropagationStopped())
              )
                break a;
              ((a = s), (i.currentTarget = l));
              try {
                a(i);
              } catch (e) {
                _i(e);
              }
              ((i.currentTarget = null), (a = c));
            }
        }
      }
    }
    function Q(e, t) {
      var n = t[Ct];
      n === void 0 && (n = t[Ct] = new Set());
      var r = e + `__bubble`;
      n.has(r) || (Gf(t, e, 2, !1), n.add(r));
    }
    function Hf(e, t, n) {
      var r = 0;
      (t && (r |= 4), Gf(n, e, r, t));
    }
    var Uf = `_reactListening` + Math.random().toString(36).slice(2);
    function Wf(e) {
      if (!e[Uf]) {
        ((e[Uf] = !0),
          It.forEach(function (t) {
            t !== `selectionchange` && (Bf.has(t) || Hf(t, !1, e), Hf(t, !0, e));
          }));
        var t = e.nodeType === 9 ? e : e.ownerDocument;
        t === null || t[Uf] || ((t[Uf] = !0), Hf(`selectionchange`, !1, t));
      }
    }
    function Gf(e, t, n, r) {
      switch (Ch(t)) {
        case 2:
          var i = _h;
          break;
        case 8:
          i = vh;
          break;
        default:
          i = yh;
      }
      ((n = i.bind(null, t, n, e)),
        (i = void 0),
        !En || (t !== `touchstart` && t !== `touchmove` && t !== `wheel`) || (i = !0),
        r
          ? i === void 0
            ? e.addEventListener(t, n, !0)
            : e.addEventListener(t, n, { capture: !0, passive: i })
          : i === void 0
            ? e.addEventListener(t, n, !1)
            : e.addEventListener(t, n, { passive: i }));
    }
    function Kf(e, t, n, r, i) {
      var a = r;
      if (!(t & 1) && !(t & 2) && r !== null)
        a: for (;;) {
          if (r === null) return;
          var s = r.tag;
          if (s === 3 || s === 4) {
            var c = r.stateNode.containerInfo;
            if (c === i) break;
            if (s === 4)
              for (s = r.return; s !== null;) {
                var l = s.tag;
                if ((l === 3 || l === 4) && s.stateNode.containerInfo === i) return;
                s = s.return;
              }
            for (; c !== null;) {
              if (((s = At(c)), s === null)) return;
              if (((l = s.tag), l === 5 || l === 6 || l === 26 || l === 27)) {
                r = a = s;
                continue a;
              }
              c = c.parentNode;
            }
          }
          r = r.return;
        }
      Cn(function () {
        var r = a,
          i = vn(n),
          s = [];
        a: {
          var c = ui.get(e);
          if (c !== void 0) {
            var l = Ln,
              u = e;
            switch (e) {
              case `keypress`:
                if (Mn(n) === 0) break a;
              case `keydown`:
              case `keyup`:
                l = tr;
                break;
              case `focusin`:
                ((u = `focus`), (l = Kn));
                break;
              case `focusout`:
                ((u = `blur`), (l = Kn));
                break;
              case `beforeblur`:
              case `afterblur`:
                l = Kn;
                break;
              case `click`:
                if (n.button === 2) break a;
              case `auxclick`:
              case `dblclick`:
              case `mousedown`:
              case `mousemove`:
              case `mouseup`:
              case `mouseout`:
              case `mouseover`:
              case `contextmenu`:
                l = Wn;
                break;
              case `drag`:
              case `dragend`:
              case `dragenter`:
              case `dragexit`:
              case `dragleave`:
              case `dragover`:
              case `dragstart`:
              case `drop`:
                l = Gn;
                break;
              case `touchcancel`:
              case `touchend`:
              case `touchmove`:
              case `touchstart`:
                l = ir;
                break;
              case ri:
              case ii:
              case ai:
                l = qn;
                break;
              case li:
                l = ar;
                break;
              case `scroll`:
              case `scrollend`:
                l = zn;
                break;
              case `wheel`:
                l = or;
                break;
              case `copy`:
              case `cut`:
              case `paste`:
                l = Jn;
                break;
              case `gotpointercapture`:
              case `lostpointercapture`:
              case `pointercancel`:
              case `pointerdown`:
              case `pointermove`:
              case `pointerout`:
              case `pointerover`:
              case `pointerup`:
                l = nr;
                break;
              case `submit`:
                l = rr;
                break;
              case `toggle`:
              case `beforetoggle`:
                l = sr;
            }
            var d = !!(t & 4),
              f = !d && (e === `scroll` || e === `scrollend`),
              p = d ? (c === null ? null : c + `Capture`) : c;
            d = [];
            for (var m = r, h; m !== null;) {
              var g = m;
              if (
                ((h = g.stateNode),
                (g = g.tag),
                (g !== 5 && g !== 26 && g !== 27) ||
                  h === null ||
                  p === null ||
                  ((g = wn(m, p)), g != null && d.push(qf(m, g, h))),
                f)
              )
                break;
              m = m.return;
            }
            0 < d.length && ((c = new l(c, u, null, n, i)), s.push({ event: c, listeners: d }));
          }
        }
        if (!(t & 7)) {
          a: {
            if (
              ((l = e === `mouseover` || e === `pointerover`),
              (c = e === `mouseout` || e === `pointerout`),
              l && n !== _n && (u = n.relatedTarget || n.fromElement) && (At(u) || u[St]))
            )
              break a;
            (c || l) &&
              ((u =
                i.window === i
                  ? i
                  : (l = i.ownerDocument)
                    ? l.defaultView || l.parentWindow
                    : window),
              c
                ? ((l = n.relatedTarget || n.toElement),
                  (c = r),
                  (l = l ? At(l) : null),
                  l !== null &&
                    ((f = o(l)), (d = l.tag), l !== f || (d !== 5 && d !== 27 && d !== 6)) &&
                    (l = null))
                : ((c = null), (l = r)),
              c !== l &&
                ((d = Wn),
                (g = `onMouseLeave`),
                (p = `onMouseEnter`),
                (m = `mouse`),
                (e === `pointerout` || e === `pointerover`) &&
                  ((d = nr), (g = `onPointerLeave`), (p = `onPointerEnter`), (m = `pointer`)),
                (f = c == null ? u : Mt(c)),
                (h = l == null ? u : Mt(l)),
                (u = new d(g, m + `leave`, c, n, i)),
                (u.target = f),
                (u.relatedTarget = h),
                (g = null),
                At(i) === r &&
                  ((d = new d(p, m + `enter`, l, n, i)),
                  (d.target = h),
                  (d.relatedTarget = f),
                  (g = d)),
                (f = g),
                (d = c && l ? E(c, l, Yf) : null),
                c !== null && Xf(s, u, c, d, !1),
                l !== null && f !== null && Xf(s, f, l, d, !0)));
          }
          a: {
            if (
              ((c = r ? Mt(r) : window),
              (l = c.nodeName && c.nodeName.toLowerCase()),
              l === `select` || (l === `input` && c.type === `file`))
            )
              var _ = Er;
            else if (xr(c)) {
              if (Dr) _ = Ir;
              else {
                _ = Pr;
                var v = Nr;
              }
            } else
              ((l = c.nodeName),
                !l || l.toLowerCase() !== `input` || (c.type !== `checkbox` && c.type !== `radio`)
                  ? r && fn(r.elementType) && (_ = Er)
                  : (_ = Fr));
            if ((_ &&= _(e, r))) {
              Sr(s, _, n, i);
              break a;
            }
            v && v(e, c, r);
          }
          switch (((v = r ? Mt(r) : window), e)) {
            case `focusin`:
              (xr(v) || v.contentEditable === `true`) && ((qr = v), (Jr = r), (Yr = null));
              break;
            case `focusout`:
              Yr = Jr = qr = null;
              break;
            case `mousedown`:
              Xr = !0;
              break;
            case `contextmenu`:
            case `mouseup`:
            case `dragend`:
              ((Xr = !1), Zr(s, n, i));
              break;
            case `selectionchange`:
              if (Kr) break;
            case `keydown`:
            case `keyup`:
              Zr(s, n, i);
          }
          var y;
          if (lr)
            b: {
              switch (e) {
                case `compositionstart`:
                  var b = `onCompositionStart`;
                  break b;
                case `compositionend`:
                  b = `onCompositionEnd`;
                  break b;
                case `compositionupdate`:
                  b = `onCompositionUpdate`;
                  break b;
              }
              b = void 0;
            }
          else
            _r
              ? hr(e, n) && (b = `onCompositionEnd`)
              : e === `keydown` && n.keyCode === 229 && (b = `onCompositionStart`);
          (b &&
            (fr &&
              n.locale !== `ko` &&
              (_r || b !== `onCompositionStart`
                ? b === `onCompositionEnd` && _r && (y = jn())
                : ((On = i), (kn = `value` in On ? On.value : On.textContent), (_r = !0))),
            (v = Jf(r, b)),
            0 < v.length &&
              ((b = new Yn(b, e, null, n, i)),
              s.push({ event: b, listeners: v }),
              y ? (b.data = y) : ((y = gr(n)), y !== null && (b.data = y)))),
            (y = dr ? vr(e, n) : yr(e, n)) &&
              ((b = Jf(r, `onBeforeInput`)),
              0 < b.length &&
                ((v = new Yn(`onBeforeInput`, `beforeinput`, null, n, i)),
                s.push({ event: v, listeners: b }),
                (v.data = y))),
            If(s, e, r, n, i));
        }
        Vf(s, t);
      });
    }
    function qf(e, t, n) {
      return { instance: e, listener: t, currentTarget: n };
    }
    function Jf(e, t) {
      for (var n = t + `Capture`, r = []; e !== null;) {
        var i = e,
          a = i.stateNode;
        if (
          ((i = i.tag),
          (i !== 5 && i !== 26 && i !== 27) ||
            a === null ||
            ((i = wn(e, n)),
            i != null && r.unshift(qf(e, i, a)),
            (i = wn(e, t)),
            i != null && r.push(qf(e, i, a))),
          e.tag === 3)
        )
          return r;
        e = e.return;
      }
      return [];
    }
    function Yf(e) {
      if (e === null) return null;
      do e = e.return;
      while (e && e.tag !== 5 && e.tag !== 27);
      return e || null;
    }
    function Xf(e, t, n, r, i) {
      for (var a = t._reactName, o = []; n !== null && n !== r;) {
        var s = n,
          c = s.alternate,
          l = s.stateNode;
        if (((s = s.tag), c !== null && c === r)) break;
        ((s !== 5 && s !== 26 && s !== 27) ||
          l === null ||
          ((c = l),
          i
            ? ((l = wn(n, a)), l != null && o.unshift(qf(n, l, c)))
            : i || ((l = wn(n, a)), l != null && o.push(qf(n, l, c)))),
          (n = n.return));
      }
      o.length !== 0 && e.push({ event: t, listeners: o });
    }
    var Zf = /\r\n?/g,
      Qf = /\u0000|\uFFFD/g;
    function $f(e) {
      return (typeof e == `string` ? e : `` + e)
        .replace(
          Zf,
          `
`,
        )
        .replace(Qf, ``);
    }
    function ep(e, t) {
      return ((t = $f(t)), $f(e) === t);
    }
    function $(e, t, n, r, a, o) {
      switch (n) {
        case `children`:
          if (typeof r == `string`) t === `body` || (t === `textarea` && r === ``) || cn(e, r);
          else if (typeof r == `number` || typeof r == `bigint`) t !== `body` && cn(e, `` + r);
          else return;
          break;
        case `className`:
          Kt(e, `class`, r);
          break;
        case `tabIndex`:
          Kt(e, `tabindex`, r);
          break;
        case `dir`:
        case `role`:
        case `viewBox`:
        case `width`:
        case `height`:
          Kt(e, n, r);
          break;
        case `style`:
          dn(e, r, o);
          return;
        case `data`:
          if (t !== `object`) {
            Kt(e, `data`, r);
            break;
          }
        case `src`:
        case `href`:
          if (
            (r === `` && (t !== `a` || n !== `href`)) ||
            r == null ||
            typeof r == `function` ||
            typeof r == `symbol` ||
            typeof r == `boolean`
          ) {
            e.removeAttribute(n);
            break;
          }
          ((r = hn(r)), e.setAttribute(n, r));
          break;
        case `action`:
        case `formAction`:
          if (typeof r == `function`) {
            e.setAttribute(
              n,
              `javascript:throw new Error('A React form was unexpectedly submitted. If you called form.submit() manually, consider using form.requestSubmit() instead. If you\\'re trying to use event.stopPropagation() in a submit event handler, consider also calling event.preventDefault().')`,
            );
            break;
          }
          if (
            (typeof o == `function` &&
              (n === `formAction`
                ? (t !== `input` && $(e, t, `name`, a.name, a, null),
                  $(e, t, `formEncType`, a.formEncType, a, null),
                  $(e, t, `formMethod`, a.formMethod, a, null),
                  $(e, t, `formTarget`, a.formTarget, a, null))
                : ($(e, t, `encType`, a.encType, a, null),
                  $(e, t, `method`, a.method, a, null),
                  $(e, t, `target`, a.target, a, null))),
            r == null || typeof r == `symbol` || typeof r == `boolean`)
          ) {
            e.removeAttribute(n);
            break;
          }
          ((r = hn(r)), e.setAttribute(n, r));
          break;
        case `onClick`:
          r != null && (e.onclick = gn);
          return;
        case `onScroll`:
          r != null && Q(`scroll`, e);
          return;
        case `onScrollEnd`:
          r != null && Q(`scrollend`, e);
          return;
        case `dangerouslySetInnerHTML`:
          if (r != null) {
            if (typeof r != `object` || !(`__html` in r)) throw Error(i(61));
            if (((n = r.__html), n != null)) {
              if (a.children != null) throw Error(i(60));
              o?.__html !== n && (e.innerHTML = n);
            }
          }
          break;
        case `multiple`:
          e.multiple = r && typeof r != `function` && typeof r != `symbol`;
          break;
        case `muted`:
          e.muted = r && typeof r != `function` && typeof r != `symbol`;
          break;
        case `suppressContentEditableWarning`:
        case `suppressHydrationWarning`:
        case `defaultValue`:
        case `defaultChecked`:
        case `innerHTML`:
        case `ref`:
          break;
        case `autoFocus`:
          break;
        case `xlinkHref`:
          if (
            r == null ||
            typeof r == `function` ||
            typeof r == `boolean` ||
            typeof r == `symbol`
          ) {
            e.removeAttribute(`xlink:href`);
            break;
          }
          ((n = hn(r)), e.setAttributeNS(`http://www.w3.org/1999/xlink`, `xlink:href`, n));
          break;
        case `contentEditable`:
        case `spellCheck`:
        case `draggable`:
        case `value`:
        case `autoReverse`:
        case `externalResourcesRequired`:
        case `focusable`:
        case `preserveAlpha`:
          r != null && typeof r != `function` && typeof r != `symbol`
            ? e.setAttribute(n, r)
            : e.removeAttribute(n);
          break;
        case `inert`:
        case `allowFullScreen`:
        case `async`:
        case `autoPlay`:
        case `controls`:
        case `credentialless`:
        case `default`:
        case `defer`:
        case `disabled`:
        case `disablePictureInPicture`:
        case `disableRemotePlayback`:
        case `formNoValidate`:
        case `hidden`:
        case `loop`:
        case `noModule`:
        case `noValidate`:
        case `open`:
        case `playsInline`:
        case `readOnly`:
        case `required`:
        case `reversed`:
        case `scoped`:
        case `seamless`:
        case `itemScope`:
          r && typeof r != `function` && typeof r != `symbol`
            ? e.setAttribute(n, ``)
            : e.removeAttribute(n);
          break;
        case `capture`:
        case `download`:
          !0 === r
            ? e.setAttribute(n, ``)
            : !1 !== r && r != null && typeof r != `function` && typeof r != `symbol`
              ? e.setAttribute(n, r)
              : e.removeAttribute(n);
          break;
        case `cols`:
        case `rows`:
        case `size`:
        case `span`:
          r != null && typeof r != `function` && typeof r != `symbol` && !isNaN(r) && 1 <= r
            ? e.setAttribute(n, r)
            : e.removeAttribute(n);
          break;
        case `rowSpan`:
        case `start`:
          r == null || typeof r == `function` || typeof r == `symbol` || isNaN(r)
            ? e.removeAttribute(n)
            : e.setAttribute(n, r);
          break;
        case `popover`:
          (Q(`beforetoggle`, e), Q(`toggle`, e), Gt(e, `popover`, r));
          break;
        case `xlinkActuate`:
          qt(e, `http://www.w3.org/1999/xlink`, `xlink:actuate`, r);
          break;
        case `xlinkArcrole`:
          qt(e, `http://www.w3.org/1999/xlink`, `xlink:arcrole`, r);
          break;
        case `xlinkRole`:
          qt(e, `http://www.w3.org/1999/xlink`, `xlink:role`, r);
          break;
        case `xlinkShow`:
          qt(e, `http://www.w3.org/1999/xlink`, `xlink:show`, r);
          break;
        case `xlinkTitle`:
          qt(e, `http://www.w3.org/1999/xlink`, `xlink:title`, r);
          break;
        case `xlinkType`:
          qt(e, `http://www.w3.org/1999/xlink`, `xlink:type`, r);
          break;
        case `xmlBase`:
          qt(e, `http://www.w3.org/XML/1998/namespace`, `xml:base`, r);
          break;
        case `xmlLang`:
          qt(e, `http://www.w3.org/XML/1998/namespace`, `xml:lang`, r);
          break;
        case `xmlSpace`:
          qt(e, `http://www.w3.org/XML/1998/namespace`, `xml:space`, r);
          break;
        case `is`:
          Gt(e, `is`, r);
          break;
        case `innerText`:
        case `textContent`:
          return;
        default:
          if (!(2 < n.length) || (n[0] !== `o` && n[0] !== `O`) || (n[1] !== `n` && n[1] !== `N`))
            ((n = pn.get(n) || n), Gt(e, n, r));
          else return;
      }
      V = !0;
    }
    function tp(e, t, n, r, a, o) {
      switch (n) {
        case `style`:
          dn(e, r, o);
          return;
        case `dangerouslySetInnerHTML`:
          if (r != null) {
            if (typeof r != `object` || !(`__html` in r)) throw Error(i(61));
            if (((n = r.__html), n != null)) {
              if (a.children != null) throw Error(i(60));
              o?.__html !== n && (e.innerHTML = n);
            }
          }
          break;
        case `children`:
          if (typeof r == `string`) cn(e, r);
          else if (typeof r == `number` || typeof r == `bigint`) cn(e, `` + r);
          else return;
          break;
        case `onScroll`:
          r != null && Q(`scroll`, e);
          return;
        case `onScrollEnd`:
          r != null && Q(`scrollend`, e);
          return;
        case `onClick`:
          r != null && (e.onclick = gn);
          return;
        case `suppressContentEditableWarning`:
        case `suppressHydrationWarning`:
        case `innerHTML`:
        case `ref`:
          return;
        case `innerText`:
        case `textContent`:
          return;
        default:
          if (!Lt.hasOwnProperty(n))
            a: {
              if (
                n[0] === `o` &&
                n[1] === `n` &&
                ((a = n.endsWith(`Capture`)),
                (o = n.slice(2, a ? n.length - 7 : void 0)),
                (t = e[xt] || null),
                (t = t == null ? null : t[n]),
                typeof t == `function` && e.removeEventListener(o, t, a),
                typeof r == `function`)
              ) {
                (typeof t != `function` &&
                  t !== null &&
                  (n in e ? (e[n] = null) : e.hasAttribute(n) && e.removeAttribute(n)),
                  e.addEventListener(o, r, a));
                break a;
              }
              ((V = !0), n in e ? (e[n] = r) : !0 === r ? e.setAttribute(n, ``) : Gt(e, n, r));
            }
          return;
      }
      V = !0;
    }
    function np(e, t, n) {
      switch (t) {
        case `div`:
        case `span`:
        case `svg`:
        case `path`:
        case `a`:
        case `g`:
        case `p`:
        case `li`:
          break;
        case `img`:
          (Q(`error`, e), Q(`load`, e));
          var r = !1,
            a = !1,
            o;
          for (o in n)
            if (n.hasOwnProperty(o)) {
              var s = n[o];
              if (s != null)
                switch (o) {
                  case `src`:
                    r = !0;
                    break;
                  case `srcSet`:
                    a = !0;
                    break;
                  case `children`:
                  case `dangerouslySetInnerHTML`:
                    throw Error(i(137, t));
                  default:
                    $(e, t, o, s, n, null);
                }
            }
          (a && $(e, t, `srcSet`, n.srcSet, n, null), r && $(e, t, `src`, n.src, n, null));
          return;
        case `input`:
          Q(`invalid`, e);
          var c = (o = s = a = null),
            l = null,
            u = null;
          for (r in n)
            if (n.hasOwnProperty(r)) {
              var d = n[r];
              if (d != null)
                switch (r) {
                  case `name`:
                    a = d;
                    break;
                  case `type`:
                    s = d;
                    break;
                  case `checked`:
                    l = d;
                    break;
                  case `defaultChecked`:
                    u = d;
                    break;
                  case `value`:
                    o = d;
                    break;
                  case `defaultValue`:
                    c = d;
                    break;
                  case `children`:
                  case `dangerouslySetInnerHTML`:
                    if (d != null) throw Error(i(137, t));
                    break;
                  default:
                    $(e, t, r, d, n, null);
                }
            }
          nn(e, o, c, l, u, s, a, !1);
          return;
        case `select`:
          for (a in (Q(`invalid`, e), (r = s = o = null), n))
            if (n.hasOwnProperty(a) && ((c = n[a]), c != null))
              switch (a) {
                case `value`:
                  o = c;
                  break;
                case `defaultValue`:
                  s = c;
                  break;
                case `multiple`:
                  r = c;
                default:
                  $(e, t, a, c, n, null);
              }
          ((t = o),
            (n = s),
            (e.multiple = !!r),
            t == null ? n != null && an(e, !!r, n, !0) : an(e, !!r, t, !1));
          return;
        case `textarea`:
          for (s in (Q(`invalid`, e), (o = a = r = null), n))
            if (n.hasOwnProperty(s) && ((c = n[s]), c != null))
              switch (s) {
                case `value`:
                  r = c;
                  break;
                case `defaultValue`:
                  a = c;
                  break;
                case `children`:
                  o = c;
                  break;
                case `dangerouslySetInnerHTML`:
                  if (c != null) throw Error(i(91));
                  break;
                default:
                  $(e, t, s, c, n, null);
              }
          sn(e, r, a, o);
          return;
        case `option`:
          for (l in n)
            if (n.hasOwnProperty(l) && ((r = n[l]), r != null))
              switch (l) {
                case `selected`:
                  e.selected = r && typeof r != `function` && typeof r != `symbol`;
                  break;
                default:
                  $(e, t, l, r, n, null);
              }
          return;
        case `dialog`:
          (Q(`beforetoggle`, e), Q(`toggle`, e), Q(`cancel`, e), Q(`close`, e));
          break;
        case `iframe`:
        case `object`:
          Q(`load`, e);
          break;
        case `video`:
        case `audio`:
          for (r = 0; r < zf.length; r++) Q(zf[r], e);
          break;
        case `image`:
          (Q(`error`, e), Q(`load`, e));
          break;
        case `details`:
          Q(`toggle`, e);
          break;
        case `embed`:
        case `source`:
        case `link`:
          (Q(`error`, e), Q(`load`, e));
        case `area`:
        case `base`:
        case `br`:
        case `col`:
        case `hr`:
        case `keygen`:
        case `meta`:
        case `param`:
        case `track`:
        case `wbr`:
        case `menuitem`:
          for (u in n)
            if (n.hasOwnProperty(u) && ((r = n[u]), r != null))
              switch (u) {
                case `children`:
                case `dangerouslySetInnerHTML`:
                  throw Error(i(137, t));
                default:
                  $(e, t, u, r, n, null);
              }
          return;
        default:
          if (fn(t)) {
            for (d in n)
              n.hasOwnProperty(d) && ((r = n[d]), r !== void 0 && tp(e, t, d, r, n, void 0));
            return;
          }
      }
      for (c in n) n.hasOwnProperty(c) && ((r = n[c]), r != null && $(e, t, c, r, n, null));
    }
    var rp = {};
    function ip(e, t, n, r) {
      switch (t) {
        case `div`:
        case `span`:
        case `svg`:
        case `path`:
        case `a`:
        case `g`:
        case `p`:
        case `li`:
          break;
        case `input`:
          var a = null,
            o = null,
            s = null,
            c = null,
            l = null,
            u = null,
            d = null;
          for (m in n) {
            var f = n[m];
            if (n.hasOwnProperty(m) && f != null)
              switch (m) {
                case `checked`:
                  break;
                case `value`:
                  break;
                case `defaultValue`:
                  l = f;
                default:
                  r.hasOwnProperty(m) || $(e, t, m, null, r, f);
              }
          }
          for (var p in r) {
            var m = r[p];
            if (((f = n[p]), r.hasOwnProperty(p) && (m != null || f != null)))
              switch (p) {
                case `type`:
                  (m !== f && (V = !0), (o = m));
                  break;
                case `name`:
                  (m !== f && (V = !0), (a = m));
                  break;
                case `checked`:
                  (m !== f && (V = !0), (u = m));
                  break;
                case `defaultChecked`:
                  (m !== f && (V = !0), (d = m));
                  break;
                case `value`:
                  (m !== f && (V = !0), (s = m));
                  break;
                case `defaultValue`:
                  (m !== f && (V = !0), (c = m));
                  break;
                case `children`:
                case `dangerouslySetInnerHTML`:
                  if (m != null) throw Error(i(137, t));
                  break;
                default:
                  m !== f && $(e, t, p, m, r, f);
              }
          }
          tn(e, s, c, l, u, d, o, a);
          return;
        case `select`:
          for (o in ((m = s = c = p = null), n))
            if (((l = n[o]), n.hasOwnProperty(o) && l != null))
              switch (o) {
                case `value`:
                  break;
                case `multiple`:
                  m = l;
                default:
                  r.hasOwnProperty(o) || $(e, t, o, null, r, l);
              }
          for (a in r)
            if (((o = r[a]), (l = n[a]), r.hasOwnProperty(a) && (o != null || l != null)))
              switch (a) {
                case `value`:
                  (o !== l && (V = !0), (p = o));
                  break;
                case `defaultValue`:
                  (o !== l && (V = !0), (c = o));
                  break;
                case `multiple`:
                  (o !== l && (V = !0), (s = o));
                default:
                  o !== l && $(e, t, a, o, r, l);
              }
          ((t = c),
            (n = s),
            (r = m),
            p == null
              ? !!r != !!n && (t == null ? an(e, !!n, n ? [] : ``, !1) : an(e, !!n, t, !0))
              : an(e, !!n, p, !1));
          return;
        case `textarea`:
          for (c in ((m = p = null), n))
            if (((a = n[c]), n.hasOwnProperty(c) && a != null && !r.hasOwnProperty(c)))
              switch (c) {
                case `value`:
                  break;
                case `children`:
                  break;
                default:
                  $(e, t, c, null, r, a);
              }
          for (s in r)
            if (((a = r[s]), (o = n[s]), r.hasOwnProperty(s) && (a != null || o != null)))
              switch (s) {
                case `value`:
                  (a !== o && (V = !0), (p = a));
                  break;
                case `defaultValue`:
                  (a !== o && (V = !0), (m = a));
                  break;
                case `children`:
                  break;
                case `dangerouslySetInnerHTML`:
                  if (a != null) throw Error(i(91));
                  break;
                default:
                  a !== o && $(e, t, s, a, r, o);
              }
          on(e, p, m);
          return;
        case `option`:
          for (var h in n)
            if (((p = n[h]), n.hasOwnProperty(h) && p != null && !r.hasOwnProperty(h)))
              switch (h) {
                case `selected`:
                  e.selected = !1;
                  break;
                default:
                  $(e, t, h, null, r, p);
              }
          for (l in r)
            if (
              ((p = r[l]), (m = n[l]), r.hasOwnProperty(l) && p !== m && (p != null || m != null))
            )
              switch (l) {
                case `selected`:
                  (p !== m && (V = !0),
                    (e.selected = p && typeof p != `function` && typeof p != `symbol`));
                  break;
                default:
                  $(e, t, l, p, r, m);
              }
          return;
        case `img`:
        case `link`:
        case `area`:
        case `base`:
        case `br`:
        case `col`:
        case `embed`:
        case `hr`:
        case `keygen`:
        case `meta`:
        case `param`:
        case `source`:
        case `track`:
        case `wbr`:
        case `menuitem`:
          for (var g in n)
            ((p = n[g]),
              n.hasOwnProperty(g) && p != null && !r.hasOwnProperty(g) && $(e, t, g, null, r, p));
          for (u in r)
            if (
              ((p = r[u]), (m = n[u]), r.hasOwnProperty(u) && p !== m && (p != null || m != null))
            )
              switch (u) {
                case `children`:
                case `dangerouslySetInnerHTML`:
                  if (p != null) throw Error(i(137, t));
                  break;
                default:
                  $(e, t, u, p, r, m);
              }
          return;
        default:
          if (fn(t)) {
            for (var _ in n)
              ((p = n[_]),
                n.hasOwnProperty(_) &&
                  p !== void 0 &&
                  !r.hasOwnProperty(_) &&
                  tp(e, t, _, void 0, r, p));
            for (d in r)
              ((p = r[d]),
                (m = n[d]),
                !r.hasOwnProperty(d) ||
                  p === m ||
                  (p === void 0 && m === void 0) ||
                  tp(e, t, d, p, r, m));
            return;
          }
      }
      for (var v in n)
        ((p = n[v]),
          n.hasOwnProperty(v) && p != null && !r.hasOwnProperty(v) && $(e, t, v, null, r, p));
      for (f in r)
        ((p = r[f]),
          (m = n[f]),
          !r.hasOwnProperty(f) || p === m || (p == null && m == null) || $(e, t, f, p, r, m));
    }
    function ap(e) {
      switch (e) {
        case `css`:
        case `script`:
        case `font`:
        case `img`:
        case `image`:
        case `input`:
        case `link`:
          return !0;
        default:
          return !1;
      }
    }
    function op() {
      if (typeof performance.getEntriesByType == `function`) {
        for (
          var e = 0, t = 0, n = performance.getEntriesByType(`resource`), r = 0;
          r < n.length;
          r++
        ) {
          var i = n[r],
            a = i.transferSize,
            o = i.initiatorType,
            s = i.duration;
          if (a && s && ap(o)) {
            for (o = 0, s = i.responseEnd, r += 1; r < n.length; r++) {
              var c = n[r],
                l = c.startTime;
              if (l > s) break;
              var u = c.transferSize,
                d = c.initiatorType;
              u && ap(d) && ((c = c.responseEnd), (o += u * (c < s ? 1 : (s - l) / (c - l))));
            }
            if ((--r, (t += (8 * (a + o)) / (i.duration / 1e3)), e++, 10 < e)) break;
          }
        }
        if (0 < e) return t / e / 1e6;
      }
      return navigator.connection && ((e = navigator.connection.downlink), typeof e == `number`)
        ? e
        : 5;
    }
    var sp = null,
      cp = null;
    function lp(e) {
      return e.nodeType === 9 ? e : e.ownerDocument;
    }
    function up(e) {
      switch (e) {
        case `http://www.w3.org/2000/svg`:
          return 1;
        case `http://www.w3.org/1998/Math/MathML`:
          return 2;
        default:
          return 0;
      }
    }
    function dp(e, t) {
      if (e === 0)
        switch (t) {
          case `svg`:
            return 1;
          case `math`:
            return 2;
          default:
            return 0;
        }
      return e === 1 && t === `foreignObject` ? 0 : e;
    }
    function fp(e, t, n, r) {
      return ((n = lp(n).createElement(e)), (n[bt] = r), (n[xt] = t), np(n, e, t), Pt(n), n);
    }
    function pp(e, t) {
      return (
        e === `textarea` ||
        e === `noscript` ||
        typeof t.children == `string` ||
        typeof t.children == `number` ||
        typeof t.children == `bigint` ||
        (typeof t.dangerouslySetInnerHTML == `object` &&
          t.dangerouslySetInnerHTML !== null &&
          t.dangerouslySetInnerHTML.__html != null)
      );
    }
    var mp = null;
    function hp() {
      var e = window.event;
      return e && e.type === `popstate` ? e !== mp && ((mp = e), !0) : ((mp = null), !1);
    }
    var gp = typeof setTimeout == `function` ? setTimeout : void 0,
      _p = typeof clearTimeout == `function` ? clearTimeout : void 0,
      vp = typeof Promise == `function` ? Promise : void 0,
      yp = typeof requestAnimationFrame == `function` ? requestAnimationFrame : gp,
      bp =
        typeof queueMicrotask == `function`
          ? queueMicrotask
          : vp === void 0
            ? gp
            : function (e) {
                return vp.resolve(null).then(e).catch(xp);
              };
    function xp(e) {
      setTimeout(function () {
        throw e;
      });
    }
    function Sp(e) {
      return e === `head`;
    }
    function Cp(e, t) {
      var n = t,
        r = 0;
      do {
        var i = n.nextSibling;
        if ((e.removeChild(n), i && i.nodeType === 8)) {
          if (((n = i.data), n === `/$` || n === `/&`)) {
            if (r === 0) {
              (e.removeChild(i), Hh(t));
              return;
            }
            r--;
          } else if (n === `$` || n === `$?` || n === `$~` || n === `$!` || n === `&`) r++;
          else if (n === `html`) _m(e.ownerDocument.documentElement);
          else if (n === `head`) {
            ((n = e.ownerDocument.head), _m(n));
            for (var a = n.firstChild; a;) {
              var o = a.nextSibling,
                s = a.nodeName;
              (a[Dt] ||
                s === `SCRIPT` ||
                s === `STYLE` ||
                (s === `LINK` && a.rel.toLowerCase() === `stylesheet`) ||
                n.removeChild(a),
                (a = o));
            }
          } else n === `body` && _m(e.ownerDocument.body);
        }
        n = i;
      } while (n);
      Hh(t);
    }
    function wp(e, t) {
      var n = e;
      e = 0;
      do {
        var r = n.nextSibling;
        if (
          (n.nodeType === 1
            ? t
              ? ((n._stashedDisplay = n.style.display), (n.style.display = `none`))
              : ((n.style.display = n._stashedDisplay || ``),
                n.getAttribute(`style`) === `` && n.removeAttribute(`style`))
            : n.nodeType === 3 &&
              (t
                ? ((n._stashedText = n.nodeValue), (n.nodeValue = ``))
                : (n.nodeValue = n._stashedText || ``)),
          r && r.nodeType === 8)
        ) {
          if (((n = r.data), n === `/$`)) {
            if (e === 0) break;
            e--;
          } else (n !== `$` && n !== `$?` && n !== `$~` && n !== `$!`) || e++;
        }
        n = r;
      } while (n);
    }
    function Tp(e, t, n) {
      if (
        ((t = CSS.escape(t) === t ? t : `r-` + btoa(t).replace(/=/g, ``)),
        (e.style.viewTransitionName = t),
        n != null && (e.style.viewTransitionClass = n),
        (n = getComputedStyle(e)),
        n.display === `inline`)
      ) {
        if (((t = e.getClientRects()), t.length === 1)) var r = 1;
        else
          for (var i = (r = 0); i < t.length; i++) {
            var a = t[i];
            0 < a.width && 0 < a.height && r++;
          }
        r === 1 &&
          ((e = e.style),
          (e.display = t.length === 1 ? `inline-block` : `block`),
          (e.marginTop = `-` + n.paddingTop),
          (e.marginBottom = `-` + n.paddingBottom));
      }
    }
    function Ep(e, t) {
      ((e = e.style), (t = t.style));
      var n =
        t == null
          ? null
          : t.hasOwnProperty(`viewTransitionName`)
            ? t.viewTransitionName
            : t.hasOwnProperty(`view-transition-name`)
              ? t[`view-transition-name`]
              : null;
      ((e.viewTransitionName = n == null || typeof n == `boolean` ? `` : (`` + n).trim()),
        (n =
          t == null
            ? null
            : t.hasOwnProperty(`viewTransitionClass`)
              ? t.viewTransitionClass
              : t.hasOwnProperty(`view-transition-class`)
                ? t[`view-transition-class`]
                : null),
        (e.viewTransitionClass = n == null || typeof n == `boolean` ? `` : (`` + n).trim()),
        e.display === `inline-block` &&
          (t == null
            ? (e.display = e.margin = ``)
            : ((n = t.display),
              (e.display = n == null || typeof n == `boolean` ? `` : n),
              (n = t.margin),
              n == null
                ? ((n = t.hasOwnProperty(`marginTop`) ? t.marginTop : t[`margin-top`]),
                  (e.marginTop = n == null || typeof n == `boolean` ? `` : n),
                  (t = t.hasOwnProperty(`marginBottom`) ? t.marginBottom : t[`margin-bottom`]),
                  (e.marginBottom = t == null || typeof t == `boolean` ? `` : t))
                : (e.margin = n))));
    }
    function Dp(e, t, n) {
      return (
        (n = n.ownerDocument.defaultView),
        {
          rect: e,
          abs: t.position === `absolute` || t.position === `fixed`,
          clip:
            t.clipPath !== `none` ||
            t.overflow !== `visible` ||
            t.filter !== `none` ||
            t.mask !== `none` ||
            t.mask !== `none` ||
            t.borderRadius !== `0px`,
          view: 0 <= e.bottom && 0 <= e.right && e.top <= n.innerHeight && e.left <= n.innerWidth,
        }
      );
    }
    function Op(e) {
      return Dp(e.getBoundingClientRect(), getComputedStyle(e), e);
    }
    function kp(e) {
      var t = e.getBoundingClientRect();
      t = new DOMRect(t.x + 2e4, t.y + 2e4, t.width, t.height);
      var n = getComputedStyle(e);
      return Dp(t, n, e);
    }
    function Ap(e) {
      return e.documentElement.clientHeight;
    }
    function jp(e) {
      (this.addEventListener(`load`, e), this.addEventListener(`error`, e));
    }
    function Mp(e, t, n, r, i, a, o, s, c) {
      var l = t.nodeType === 9 ? t : t.ownerDocument;
      try {
        var u = l.startViewTransition({
          update: function () {
            var t = l.defaultView,
              n = t.navigation && t.navigation.transition,
              o = l.fonts.status;
            r();
            var s = [];
            if (
              (o === `loaded` && (Ap(l), l.fonts.status === `loading` && s.push(l.fonts.ready)),
              (o = s.length),
              e !== null)
            )
              for (var c = e.suspenseyImages, u = 0, d = 0; d < c.length; d++) {
                var f = c[d];
                if (!f.complete) {
                  var p = f.getBoundingClientRect();
                  if (
                    0 < p.bottom &&
                    0 < p.right &&
                    p.top < t.innerHeight &&
                    p.left < t.innerWidth
                  ) {
                    if (((u += Xm(f)), u > $m)) {
                      s.length = o;
                      break;
                    }
                    ((f = new Promise(jp.bind(f))), s.push(f));
                  }
                }
              }
            if (0 < s.length)
              return (
                (t = Promise.race([
                  Promise.all(s),
                  new Promise(function (e) {
                    return setTimeout(e, 500);
                  }),
                ]).then(i, i)),
                (n ? Promise.allSettled([n.finished, t]) : t).then(a, a)
              );
            if ((i(), n)) return n.finished.then(a, a);
            a();
          },
          types: n,
        });
        l.__reactViewTransition = u;
        var d = [];
        return (
          u.ready.then(
            function () {
              for (
                var e = l.documentElement.getAnimations({ subtree: !0 }), t = 0;
                t < e.length;
                t++
              ) {
                var n = e[t],
                  r = n.effect,
                  i = r.pseudoElement;
                if (i != null && i.startsWith(`::view-transition`)) {
                  (d.push(n), (n = r.getKeyframes()));
                  for (var a = (i = void 0), s = !0, c = 0; c < n.length; c++) {
                    var u = n[c],
                      f = u.width;
                    if (i === void 0) i = f;
                    else if (i !== f) {
                      s = !1;
                      break;
                    }
                    if (((f = u.height), a === void 0)) a = f;
                    else if (a !== f) {
                      s = !1;
                      break;
                    }
                    (delete u.width, delete u.height, u.transform === `none` && delete u.transform);
                  }
                  s &&
                    i !== void 0 &&
                    a !== void 0 &&
                    (r.setKeyframes(n),
                    (s = getComputedStyle(r.target, r.pseudoElement)),
                    s.width !== i || s.height !== a) &&
                    ((s = n[0]),
                    (s.width = i),
                    (s.height = a),
                    (s = n[n.length - 1]),
                    (s.width = i),
                    (s.height = a),
                    r.setKeyframes(n));
                }
              }
              o();
            },
            function (e) {
              l.__reactViewTransition === u && (l.__reactViewTransition = null);
              try {
                if (typeof e == `object` && e)
                  switch (e.name) {
                    case `InvalidStateError`:
                      (e.message ===
                        `View transition was skipped because document visibility state is hidden.` ||
                        e.message ===
                          `Skipping view transition because document visibility state has become hidden.` ||
                        e.message === `Skipping view transition because viewport size changed.` ||
                        e.message === `Transition was aborted because of invalid state`) &&
                        (e = null);
                  }
                e !== null && c(e);
              } finally {
                (r(), i(), o());
              }
            },
          ),
          u.finished.finally(function () {
            for (var e = 0; e < d.length; e++) d[e].cancel();
            (l.__reactViewTransition === u && (l.__reactViewTransition = null), s());
          }),
          u
        );
      } catch {
        return (r(), i(), o(), null);
      }
    }
    function Np(e, t) {
      ((this._scope = document.documentElement),
        (this._selector = `::view-transition-` + e + `(` + t + `)`));
    }
    ((Np.prototype.animate = function (e, t) {
      return (
        (t = typeof t == `number` ? { duration: t } : D({}, t)),
        (t.pseudoElement = this._selector),
        this._scope.animate(e, t)
      );
    }),
      (Np.prototype.getAnimations = function () {
        for (
          var e = this._scope,
            t = this._selector,
            n = e.getAnimations({ subtree: !0 }),
            r = [],
            i = 0;
          i < n.length;
          i++
        ) {
          var a = n[i].effect;
          a !== null && a.target === e && a.pseudoElement === t && r.push(n[i]);
        }
        return r;
      }),
      (Np.prototype.getComputedStyle = function () {
        return getComputedStyle(this._scope, this._selector);
      }));
    function Pp(e) {
      return {
        name: e,
        group: new Np(`group`, e),
        imagePair: new Np(`image-pair`, e),
        old: new Np(`old`, e),
        new: new Np(`new`, e),
      };
    }
    function Fp(e) {
      ((this._fragmentFiber = e), (this._observers = this._eventListeners = null));
    }
    Fp.prototype.addEventListener = function (e, t, n) {
      var r = null,
        i = null;
      if (
        !(n != null && typeof n != `boolean` && ((r = n.signal || null), r !== null && r.aborted))
      ) {
        this._eventListeners === null && (this._eventListeners = []);
        var a = this._eventListeners;
        if (Bp(a, e, t, n) === -1) {
          var o = this,
            s = t;
          (n != null &&
            typeof n != `boolean` &&
            !0 === n.once &&
            (s = function (r) {
              (o.removeEventListener(e, t, n),
                typeof t == `function` ? t.call(this, r) : t.handleEvent(r));
            }),
            r !== null &&
              ((i = o.removeEventListener.bind(o, e, t, n)),
              r.addEventListener(`abort`, i, { once: !0 }),
              (i = r.removeEventListener.bind(r, `abort`, i))),
            (r = Rp(n)),
            a.push({
              type: e,
              listener: t,
              optionsOrUseCapture: n,
              attachedListener: s,
              cleanup: i,
            }),
            h(this._fragmentFiber.child, !1, Ip, e, s, r));
        }
        this._eventListeners = a;
      }
    };
    function Ip(e, t, n, r) {
      return (b(e).addEventListener(t, n, r), !1);
    }
    Fp.prototype.removeEventListener = function (e, t, n) {
      var r = this._eventListeners;
      if (r !== null && ((t = Bp(r, e, t, n)), t !== -1)) {
        var i = r[t];
        n = i.attachedListener;
        var a = i.cleanup;
        ((i = Rp(i.optionsOrUseCapture)),
          h(this._fragmentFiber.child, !1, Lp, e, n, i),
          r.splice(t, 1),
          a !== null && a());
      }
    };
    function Lp(e, t, n, r) {
      return (b(e).removeEventListener(t, n, r), !1);
    }
    function Rp(e) {
      return e != null &&
        typeof e != `boolean` &&
        (!0 === e.once || e.signal instanceof AbortSignal)
        ? { capture: e.capture, passive: e.passive }
        : e;
    }
    function zp(e) {
      return e == null
        ? `c=0`
        : typeof e == `boolean`
          ? `c=` + (e ? `1` : `0`)
          : `c=` + (e.capture ? `1` : `0`);
    }
    function Bp(e, t, n, r) {
      if (e.length === 0) return -1;
      r = zp(r);
      for (var i = 0; i < e.length; i++) {
        var a = e[i];
        if (a.type === t && a.listener === n && zp(a.optionsOrUseCapture) === r) return i;
      }
      return -1;
    }
    ((Fp.prototype.dispatchEvent = function (e) {
      var t = g(this._fragmentFiber);
      if (t === null) return !0;
      t = b(t);
      var n = this._eventListeners;
      if ((n !== null && 0 < n.length) || !e.bubbles) {
        var r = t.nodeType === 9 ? t.createComment(``) : document.createTextNode(``);
        if (n)
          for (var i = 0; i < n.length; i++) {
            var a = n[i];
            r.addEventListener(a.type, a.attachedListener, Rp(a.optionsOrUseCapture));
          }
        if ((t.appendChild(r), (e = r.dispatchEvent(e)), n))
          for (i = 0; i < n.length; i++)
            ((a = n[i]),
              r.removeEventListener(a.type, a.attachedListener, Rp(a.optionsOrUseCapture)));
        return (t.removeChild(r), e);
      }
      return t.dispatchEvent(e);
    }),
      (Fp.prototype.focus = function (e) {
        h(this._fragmentFiber.child, !0, Vp, e, void 0, void 0);
      }));
    function Vp(e, t) {
      return e.tag !== 6 && ((e = b(e)), pm(e, t));
    }
    Fp.prototype.focusLast = function (e) {
      var t = [];
      h(this._fragmentFiber.child, !0, Hp, t, void 0, void 0);
      for (var n = t.length - 1; 0 <= n && !Vp(t[n], e); n--);
    };
    function Hp(e, t) {
      return (t.push(e), !1);
    }
    Fp.prototype.blur = function () {
      var e = g(this._fragmentFiber);
      e !== null &&
        ((e = b(e)),
        (e = lp(e).activeElement),
        e !== null && h(this._fragmentFiber.child, !1, Up, e, void 0, void 0));
    };
    function Up(e, t) {
      return e.tag !== 6 && ((e = b(e)), e === t || e.contains(t) ? (t.blur(), !0) : !1);
    }
    Fp.prototype.observeUsing = function (e) {
      (this._observers === null && (this._observers = new Set()),
        this._observers.add(e),
        h(this._fragmentFiber.child, !1, Wp, e, void 0, void 0));
    };
    function Wp(e, t) {
      return (e.tag === 6 || ((e = b(e)), t.observe(e)), !1);
    }
    Fp.prototype.unobserveUsing = function (e) {
      var t = this._observers;
      if (t !== null && t.has(e)) {
        (t.delete(e), h(this._fragmentFiber.child, !1, Gp, e, void 0, void 0));
        for (var n = (t = 0); n < Kp.length; n++) {
          var r = Kp[n];
          r.fragmentInstance === this && r.observer === e ? e.unobserve(r.instance) : (Kp[t++] = r);
        }
        Kp.length = t;
      }
    };
    function Gp(e, t) {
      return (e.tag === 6 || ((e = b(e)), t.unobserve(e)), !1);
    }
    var Kp = [],
      qp = !1;
    function Jp(e, t, n) {
      (Kp.push({ fragmentInstance: e, observer: t, instance: n }),
        qp ||
          ((qp = !0),
          mm(function () {
            qp = !1;
            var e = Kp;
            Kp = [];
            for (var t = 0; t < e.length; t++) {
              var n = e[t];
              n.observer.unobserve(n.instance);
            }
          })));
    }
    Fp.prototype.getClientRects = function () {
      var e = [];
      return (h(this._fragmentFiber.child, !1, Yp, e, void 0, void 0), e);
    };
    function Yp(e, t) {
      if (e.tag === 6) {
        e = e.stateNode;
        var n = e.ownerDocument.createRange();
        (n.selectNodeContents(e), t.push.apply(t, n.getClientRects()));
      } else ((e = b(e)), t.push.apply(t, e.getClientRects()));
      return !1;
    }
    ((Fp.prototype.getRootNode = function (e) {
      var t = g(this._fragmentFiber);
      return t === null ? this : b(t).getRootNode(e);
    }),
      (Fp.prototype.compareDocumentPosition = function (e) {
        var t = g(this._fragmentFiber);
        if (t === null) return Node.DOCUMENT_POSITION_DISCONNECTED;
        var n = [];
        h(this._fragmentFiber.child, !1, Hp, n, void 0, void 0);
        var r = b(t);
        if (n.length === 0) {
          if (((n = r), _(this._fragmentFiber))) {
            a: {
              for (t = this._fragmentFiber.return; t !== null;) {
                if (t.tag === 4) {
                  t = t.stateNode.containerInfo;
                  break a;
                }
                if (t.tag === 3 || t.tag === 5 || t.tag === 27) break;
                t = t.return;
              }
              t = null;
            }
            t != null && (n = t);
          }
          t = this._fragmentFiber;
          var i = (r = n.compareDocumentPosition(e));
          return (
            n === e
              ? (i = Node.DOCUMENT_POSITION_CONTAINS)
              : r & Node.DOCUMENT_POSITION_CONTAINED_BY &&
                ((n = v(t)[1]),
                n === null
                  ? (i = Node.DOCUMENT_POSITION_PRECEDING)
                  : ((e = b(n).compareDocumentPosition(e)),
                    (i =
                      e === 0 || e & Node.DOCUMENT_POSITION_FOLLOWING
                        ? Node.DOCUMENT_POSITION_FOLLOWING
                        : Node.DOCUMENT_POSITION_PRECEDING))),
            (i |= Node.DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC)
          );
        }
        ((t = b(n[0])), (i = b(n[n.length - 1])));
        var a = _(this._fragmentFiber) ? t.parentElement : r;
        if (a == null) return Node.DOCUMENT_POSITION_DISCONNECTED;
        ((r = a.compareDocumentPosition(t) & Node.DOCUMENT_POSITION_CONTAINED_BY),
          (a = a.compareDocumentPosition(i) & Node.DOCUMENT_POSITION_CONTAINED_BY));
        var o = t.compareDocumentPosition(e),
          s = i.compareDocumentPosition(e),
          c = o & Node.DOCUMENT_POSITION_CONTAINED_BY || s & Node.DOCUMENT_POSITION_CONTAINED_BY;
        return (
          (s =
            r && a && o & Node.DOCUMENT_POSITION_FOLLOWING && s & Node.DOCUMENT_POSITION_PRECEDING),
          (t =
            (r && t === e) || (a && i === e) || c || s
              ? Node.DOCUMENT_POSITION_CONTAINED_BY
              : (!r && t === e) || (!a && i === e)
                ? Node.DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC
                : o),
          t & Node.DOCUMENT_POSITION_DISCONNECTED ||
          t & Node.DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC ||
          Xp(t, this._fragmentFiber, n[0], n[n.length - 1], e)
            ? t
            : Node.DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC
        );
      }));
    function Xp(e, t, n, r, i) {
      var a = At(i);
      if (e & Node.DOCUMENT_POSITION_CONTAINED_BY) {
        if ((n = !!a))
          a: {
            for (; a !== null;) {
              if (a.tag === 7 && (a === t || a.alternate === t)) {
                n = !0;
                break a;
              }
              a = a.return;
            }
            n = !1;
          }
        return n;
      }
      if (e & Node.DOCUMENT_POSITION_CONTAINS) {
        if (a === null)
          return ((a = i.ownerDocument), i === a || i === a.documentElement || i === a.body);
        a: {
          for (a = t, t = g(t); a !== null;) {
            if (!((a.tag !== 5 && a.tag !== 3 && a.tag !== 27) || (a !== t && a.alternate !== t))) {
              a = !0;
              break a;
            }
            a = a.return;
          }
          a = !1;
        }
        return a;
      }
      return e & Node.DOCUMENT_POSITION_PRECEDING
        ? ((t = !!a) &&
            !(t = a === n) &&
            ((t = E(n, a, T)),
            t === null ? (t = !1) : (h(t, !0, C, a, n), (a = x), (x = null), (t = a !== null))),
          t)
        : e & Node.DOCUMENT_POSITION_FOLLOWING
          ? ((t = !!a) &&
              !(t = a === r) &&
              ((t = E(r, a, T)),
              t === null
                ? (t = !1)
                : (h(t, !0, w, a, r), (a = x), (S = x = null), (t = a !== null))),
            t)
          : !1;
    }
    function Zp(e, t) {
      var n = e.ownerDocument.createRange();
      (n.selectNodeContents(e),
        (e = n.getBoundingClientRect()),
        window.scrollTo(
          window.scrollX + e.left,
          t ? window.scrollY + e.top : window.scrollY + e.bottom - window.innerHeight,
        ));
    }
    Fp.prototype.scrollIntoView = function (e) {
      if (typeof e == `object`) throw Error(i(566));
      var t = [];
      h(this._fragmentFiber.child, !1, Hp, t, void 0, void 0);
      var n = !1 !== e;
      if (t.length === 0) {
        var r = v(this._fragmentFiber);
        if (((r = n ? r[1] || r[0] || g(this._fragmentFiber) : r[0] || r[1]), r === null)) return;
        if (r.tag === 6) {
          ((e = b(r)), Zp(e, n));
          return;
        }
        if (((r = b(r)), r.nodeType !== 9)) {
          if (r.nodeType === 11) {
            ((n = `host` in r ? r.host : null), n !== null && n.scrollIntoView(e));
            return;
          }
          r.scrollIntoView(e);
        }
      }
      for (r = n ? t.length - 1 : 0; r !== (n ? -1 : t.length);) {
        var a = t[r];
        (a.tag === 6 ? ((a = b(a)), Zp(a, n)) : b(a).scrollIntoView(e), (r += n ? -1 : 1));
      }
    };
    function Qp(e, t) {
      return ((e = b(e)), $p(e, t), !1);
    }
    function $p(e, t) {
      ((e.reactFragments ??= new Set()), e.reactFragments.add(t));
    }
    function em(e, t) {
      var n = t._eventListeners;
      if (n !== null)
        for (var r = 0; r < n.length; r++) {
          var i = n[r];
          e.addEventListener(i.type, i.attachedListener, Rp(i.optionsOrUseCapture));
        }
      e.nodeType !== 3 &&
        ((n = t._observers),
        n !== null &&
          n.forEach(function (n) {
            for (var r = 0, i = 0; i < Kp.length; i++) {
              var a = Kp[i];
              (a.fragmentInstance !== t || a.observer !== n || a.instance !== e) && (Kp[r++] = a);
            }
            ((Kp.length = r), n.observe(e));
          }),
        $p(e, t));
    }
    function tm(e, t) {
      var n = t._eventListeners;
      if (n !== null)
        for (var r = 0; r < n.length; r++) {
          var i = n[r];
          e.removeEventListener(i.type, i.attachedListener, Rp(i.optionsOrUseCapture));
        }
      e.nodeType !== 3 &&
        ((n = t._observers),
        n !== null &&
          n.forEach(function (n) {
            typeof n.rootMargin == `string` ? Jp(t, n, e) : n.unobserve(e);
          }),
        e.reactFragments != null && e.reactFragments.delete(t));
    }
    function nm(e) {
      var t = e.firstChild;
      for (t && t.nodeType === 10 && (t = t.nextSibling); t;) {
        var n = t;
        switch (((t = t.nextSibling), n.nodeName)) {
          case `HTML`:
          case `HEAD`:
          case `BODY`:
            (nm(n), kt(n));
            continue;
          case `SCRIPT`:
          case `STYLE`:
            continue;
          case `LINK`:
            if (n.rel.toLowerCase() === `stylesheet`) continue;
        }
        e.removeChild(n);
      }
    }
    function rm(e, t, n, r) {
      for (; e.nodeType === 1;) {
        var i = n;
        if (e.nodeName.toLowerCase() !== t.toLowerCase()) {
          if (!r && (e.nodeName !== `INPUT` || e.type !== `hidden`)) break;
        } else if (!r) {
          if (t === `input` && e.type === `hidden`) {
            var a = i.name == null ? null : `` + i.name;
            if (i.type === `hidden` && e.getAttribute(`name`) === a) return e;
          } else return e;
        } else if (!e[Dt])
          switch (t) {
            case `meta`:
              if (!e.hasAttribute(`itemprop`)) break;
              return e;
            case `link`:
              if (
                ((a = e.getAttribute(`rel`)),
                (a === `stylesheet` && e.hasAttribute(`data-precedence`)) ||
                  a !== i.rel ||
                  e.getAttribute(`href`) !== (i.href == null || i.href === `` ? null : i.href) ||
                  e.getAttribute(`crossorigin`) !==
                    (i.crossOrigin == null ? null : i.crossOrigin) ||
                  e.getAttribute(`title`) !== (i.title == null ? null : i.title))
              )
                break;
              return e;
            case `style`:
              if (e.hasAttribute(`data-precedence`)) break;
              return e;
            case `script`:
              if (
                ((a = e.getAttribute(`src`)),
                (a !== (i.src == null ? null : i.src) ||
                  e.getAttribute(`type`) !== (i.type == null ? null : i.type) ||
                  e.getAttribute(`crossorigin`) !==
                    (i.crossOrigin == null ? null : i.crossOrigin)) &&
                  a &&
                  e.hasAttribute(`async`) &&
                  !e.hasAttribute(`itemprop`))
              )
                break;
              return e;
            default:
              return e;
          }
        if (((e = lm(e.nextSibling)), e === null)) break;
      }
      return null;
    }
    function im(e, t, n) {
      if (t === ``) return null;
      for (; e.nodeType !== 3;)
        if (
          ((e.nodeType !== 1 || e.nodeName !== `INPUT` || e.type !== `hidden`) && !n) ||
          ((e = lm(e.nextSibling)), e === null)
        )
          return null;
      return e;
    }
    function am(e, t) {
      for (; e.nodeType !== 8;)
        if (
          ((e.nodeType !== 1 || e.nodeName !== `INPUT` || e.type !== `hidden`) && !t) ||
          ((e = lm(e.nextSibling)), e === null)
        )
          return null;
      return e;
    }
    function om(e) {
      return e.data === `$?` || e.data === `$~`;
    }
    function sm(e) {
      return e.data === `$!` || (e.data === `$?` && e.ownerDocument.readyState !== `loading`);
    }
    function cm(e, t) {
      var n = e.ownerDocument;
      if (e.data === `$~`) e._reactRetry = t;
      else if (e.data !== `$?` || n.readyState !== `loading`) t();
      else {
        var r = function () {
          (t(), n.removeEventListener(`DOMContentLoaded`, r));
        };
        (n.addEventListener(`DOMContentLoaded`, r), (e._reactRetry = r));
      }
    }
    function lm(e) {
      for (; e != null; e = e.nextSibling) {
        var t = e.nodeType;
        if (t === 1 || t === 3) break;
        if (t === 8) {
          if (
            ((t = e.data),
            t === `$` ||
              t === `$!` ||
              t === `$?` ||
              t === `$~` ||
              t === `&` ||
              t === `F!` ||
              t === `F`)
          )
            break;
          if (t === `/$` || t === `/&`) return null;
        }
      }
      return e;
    }
    var um = null;
    function dm(e) {
      e = e.nextSibling;
      for (var t = 0; e;) {
        if (e.nodeType === 8) {
          var n = e.data;
          if (n === `/$` || n === `/&`) {
            if (t === 0) return lm(e.nextSibling);
            t--;
          } else (n !== `$` && n !== `$!` && n !== `$?` && n !== `$~` && n !== `&`) || t++;
        }
        e = e.nextSibling;
      }
      return null;
    }
    function fm(e) {
      e = e.previousSibling;
      for (var t = 0; e;) {
        if (e.nodeType === 8) {
          var n = e.data;
          if (n === `$` || n === `$!` || n === `$?` || n === `$~` || n === `&`) {
            if (t === 0) return e;
            t--;
          } else (n !== `/$` && n !== `/&`) || t++;
        }
        e = e.previousSibling;
      }
      return null;
    }
    function pm(e, t) {
      function n() {
        r = !0;
      }
      if (e.ownerDocument.activeElement === e) return !0;
      var r = !1;
      try {
        (e.ownerDocument.addEventListener(`focus`, n, !0),
          (e.focus || HTMLElement.prototype.focus).call(e, t));
      } finally {
        e.ownerDocument.removeEventListener(`focus`, n, !0);
      }
      return r;
    }
    function mm(e) {
      yp(function () {
        yp(function (t) {
          return e(t);
        });
      });
    }
    function hm(e, t, n) {
      switch (((t = lp(n)), e)) {
        case `html`:
          if (((e = t.documentElement), !e)) throw Error(i(452));
          return e;
        case `head`:
          if (((e = t.head), !e)) throw Error(i(453));
          return e;
        case `body`:
          if (((e = t.body), !e)) throw Error(i(454));
          return e;
        default:
          throw Error(i(451));
      }
    }
    function gm(e, t, n) {
      for (var r in n) {
        var i = n[r];
        n.hasOwnProperty(r) && i != null && $(e, t, r, null, rp, i);
      }
      (n.dangerouslySetInnerHTML != null && (e.textContent = ``),
        e.onclick === gn && (e.onclick = null),
        kt(e));
    }
    function _m(e) {
      for (var t = e.attributes; t.length;) e.removeAttributeNode(t[0]);
      kt(e);
    }
    var vm = new Map(),
      ym = new Set();
    function bm(e) {
      if (typeof e.getRootNode == `function`) {
        var t = e.getRootNode();
        if (t.nodeType === 9 || t.nodeType === 11) return t;
      }
      return e.nodeType === 9 ? e : e.ownerDocument;
    }
    var xm = R.d;
    R.d = { f: Sm, r: Cm, D: Em, C: Dm, L: Om, m: km, X: jm, S: Am, M: Mm };
    function Sm() {
      var e = xm.f(),
        t = zd();
      return e || t;
    }
    function Cm(e) {
      var t = jt(e);
      t !== null && t.tag === 5 && t.type === `form` ? ec(t) : xm.r(e);
    }
    var wm = typeof document > `u` ? null : document;
    function Tm(e, t, n) {
      var r = wm;
      if (r && typeof t == `string` && t) {
        var i = en(t);
        ((i = `link[rel="` + e + `"][href="` + i + `"]`),
          typeof n == `string` && (i += `[crossorigin="` + n + `"]`),
          ym.has(i) ||
            (ym.add(i),
            (e = { rel: e, crossOrigin: n, href: t }),
            r.querySelector(i) === null &&
              ((t = r.createElement(`link`)), np(t, `link`, e), Pt(t), r.head.appendChild(t))));
      }
    }
    function Em(e) {
      (xm.D(e), Tm(`dns-prefetch`, e, null));
    }
    function Dm(e, t) {
      (xm.C(e, t), Tm(`preconnect`, e, t));
    }
    function Om(e, t, n) {
      xm.L(e, t, n);
      var r = wm;
      if (r && e && t) {
        var i = `link[rel="preload"][as="` + en(t) + `"]`;
        t === `image` && n && n.imageSrcSet
          ? ((i += `[imagesrcset="` + en(n.imageSrcSet) + `"]`),
            typeof n.imageSizes == `string` && (i += `[imagesizes="` + en(n.imageSizes) + `"]`))
          : (i += `[href="` + en(e) + `"]`);
        var a = i;
        switch (t) {
          case `style`:
            a = Pm(e);
            break;
          case `script`:
            a = Rm(e);
        }
        if (
          !(
            vm.has(a) ||
            ((e = D(
              { rel: `preload`, href: t === `image` && n && n.imageSrcSet ? void 0 : e, as: t },
              n,
            )),
            vm.set(a, e),
            r.querySelector(i) !== null ||
              (t === `style` && r.querySelector(Fm(a))) ||
              (t === `script` && r.querySelector(zm(a))))
          )
        ) {
          var o = r.createElement(`link`);
          (np(o, `link`, e),
            t === `style` &&
              ((o[Ot] = !0),
              (o.onload = o.onerror =
                function () {
                  Ft(o);
                })),
            Pt(o),
            r.head.appendChild(o));
        }
      }
    }
    function km(e, t) {
      xm.m(e, t);
      var n = wm;
      if (n && e) {
        var r = t && typeof t.as == `string` ? t.as : `script`,
          i = `link[rel="modulepreload"][as="` + en(r) + `"][href="` + en(e) + `"]`,
          a = i;
        switch (r) {
          case `audioworklet`:
          case `paintworklet`:
          case `serviceworker`:
          case `sharedworker`:
          case `worker`:
          case `script`:
            a = Rm(e);
        }
        if (
          !vm.has(a) &&
          ((e = D({ rel: `modulepreload`, href: e }, t)), vm.set(a, e), n.querySelector(i) === null)
        ) {
          switch (r) {
            case `audioworklet`:
            case `paintworklet`:
            case `serviceworker`:
            case `sharedworker`:
            case `worker`:
            case `script`:
              if (n.querySelector(zm(a))) return;
          }
          ((r = n.createElement(`link`)), np(r, `link`, e), Pt(r), n.head.appendChild(r));
        }
      }
    }
    function Am(e, t, n) {
      xm.S(e, t, n);
      var r = wm;
      if (r && e) {
        var i = Nt(r).hoistableStyles,
          a = Pm(e);
        t ||= `default`;
        var o = i.get(a);
        if (!o) {
          var s = { loading: 0, preload: null };
          if ((o = r.querySelector(Fm(a)))) s.loading = 5;
          else {
            ((e = D({ rel: `stylesheet`, href: e, "data-precedence": t }, n)),
              (n = vm.get(a)) && Hm(e, n));
            var c = (o = r.createElement(`link`));
            (Pt(c),
              np(c, `link`, e),
              (c._p = new Promise(function (e, t) {
                ((c.onload = e), (c.onerror = t));
              })),
              c.addEventListener(`load`, function () {
                s.loading |= 1;
              }),
              c.addEventListener(`error`, function () {
                s.loading |= 2;
              }),
              (s.loading |= 4),
              Vm(o, t, r));
          }
          ((o = { type: `stylesheet`, instance: o, count: 1, state: s }), i.set(a, o));
        }
      }
    }
    function jm(e, t) {
      xm.X(e, t);
      var n = wm;
      if (n && e) {
        var r = Nt(n).hoistableScripts,
          i = Rm(e),
          a = r.get(i);
        a ||
          ((a = n.querySelector(zm(i))),
          a ||
            ((e = D({ src: e, async: !0 }, t)),
            (t = vm.get(i)) && Um(e, t),
            (a = n.createElement(`script`)),
            Pt(a),
            np(a, `link`, e),
            n.head.appendChild(a)),
          (a = { type: `script`, instance: a, count: 1, state: null }),
          r.set(i, a));
      }
    }
    function Mm(e, t) {
      xm.M(e, t);
      var n = wm;
      if (n && e) {
        var r = Nt(n).hoistableScripts,
          i = Rm(e),
          a = r.get(i);
        a ||
          ((a = n.querySelector(zm(i))),
          a ||
            ((e = D({ src: e, async: !0, type: `module` }, t)),
            (t = vm.get(i)) && Um(e, t),
            (a = n.createElement(`script`)),
            Pt(a),
            np(a, `link`, e),
            n.head.appendChild(a)),
          (a = { type: `script`, instance: a, count: 1, state: null }),
          r.set(i, a));
      }
    }
    function Nm(e, t, n, r) {
      var a = (a = xe.current) ? bm(a) : null;
      if (!a) throw Error(i(446));
      switch (e) {
        case `meta`:
        case `title`:
          return null;
        case `style`:
          return typeof n.precedence == `string` && typeof n.href == `string`
            ? ((n = Pm(n.href)),
              (t = Nt(a).hoistableStyles),
              (r = t.get(n)),
              r || ((r = { type: `style`, instance: null, count: 0, state: null }), t.set(n, r)),
              r)
            : { type: `void`, instance: null, count: 0, state: null };
        case `link`:
          if (
            n.rel === `stylesheet` &&
            typeof n.href == `string` &&
            typeof n.precedence == `string`
          ) {
            e = Pm(n.href);
            var o = Nt(a).hoistableStyles,
              s = o.get(e);
            if (
              (s ||
                ((a = a.ownerDocument || a),
                (s = {
                  type: `stylesheet`,
                  instance: null,
                  count: 0,
                  state: { loading: 0, preload: null },
                }),
                o.set(e, s),
                (o = a.querySelector(Fm(e)))
                  ? o._p || ((s.instance = o), (s.state.loading = 5))
                  : ((o = vm.get(e)),
                    o ||
                      ((o = {
                        rel: `preload`,
                        as: `style`,
                        href: n.href,
                        crossOrigin: n.crossOrigin,
                        integrity: n.integrity,
                        media: n.media,
                        hrefLang: n.hrefLang,
                        referrerPolicy: n.referrerPolicy,
                      }),
                      vm.set(e, o)),
                    Lm(a, e, o, s.state))),
              t && r === null)
            )
              throw Error(i(528, ``));
            return s;
          }
          if (t && r !== null) throw Error(i(529, ``));
          return null;
        case `script`:
          return (
            (t = n.async),
            (n = n.src),
            typeof n == `string` && t && typeof t != `function` && typeof t != `symbol`
              ? ((n = Rm(n)),
                (t = Nt(a).hoistableScripts),
                (r = t.get(n)),
                r || ((r = { type: `script`, instance: null, count: 0, state: null }), t.set(n, r)),
                r)
              : { type: `void`, instance: null, count: 0, state: null }
          );
        default:
          throw Error(i(444, e));
      }
    }
    function Pm(e) {
      return `href="` + en(e) + `"`;
    }
    function Fm(e) {
      return `link[rel="stylesheet"][` + e + `]`;
    }
    function Im(e) {
      return D({}, e, { "data-precedence": e.precedence, precedence: null });
    }
    function Lm(e, t, n, r) {
      if ((t = e.querySelector(`link[rel="preload"][as="style"][` + t + `]`))) {
        if (!0 !== t[Ot]) {
          r.loading = 1;
          return;
        }
      } else
        ((t = e.createElement(`link`)),
          (t[Ot] = !0),
          (t.onload = t.onerror = Ft.bind(null, t)),
          np(t, `link`, n),
          Pt(t),
          e.head.appendChild(t));
      ((r.preload = t),
        t.addEventListener(`load`, function () {
          return (r.loading |= 1);
        }),
        t.addEventListener(`error`, function () {
          return (r.loading |= 2);
        }));
    }
    function Rm(e) {
      return `[src="` + en(e) + `"]`;
    }
    function zm(e) {
      return `script[async]` + e;
    }
    function Bm(e, t, n) {
      if ((t.count++, t.instance === null))
        switch (t.type) {
          case `style`:
            var r = e.querySelector(`style[data-href~="` + en(n.href) + `"]`);
            if (r) return ((t.instance = r), Pt(r), r);
            var a = D({}, n, {
              "data-href": n.href,
              "data-precedence": n.precedence,
              href: null,
              precedence: null,
            });
            return (
              (r = (e.ownerDocument || e).createElement(`style`)),
              Pt(r),
              np(r, `style`, a),
              Vm(r, n.precedence, e),
              (t.instance = r)
            );
          case `stylesheet`:
            a = Pm(n.href);
            var o = e.querySelector(Fm(a));
            if (o) return ((t.state.loading |= 4), (t.instance = o), Pt(o), o);
            ((r = Im(n)),
              (a = vm.get(a)) && Hm(r, a),
              (o = (e.ownerDocument || e).createElement(`link`)),
              Pt(o));
            var s = o;
            return (
              (s._p = new Promise(function (e, t) {
                ((s.onload = e), (s.onerror = t));
              })),
              np(o, `link`, r),
              (t.state.loading |= 4),
              Vm(o, n.precedence, e),
              (t.instance = o)
            );
          case `script`:
            return (
              (o = Rm(n.src)),
              (a = e.querySelector(zm(o)))
                ? ((t.instance = a), Pt(a), a)
                : ((r = n),
                  (a = vm.get(o)) && ((r = D({}, n)), Um(r, a)),
                  (e = e.ownerDocument || e),
                  (a = e.createElement(`script`)),
                  Pt(a),
                  np(a, `link`, r),
                  e.head.appendChild(a),
                  (t.instance = a))
            );
          case `void`:
            return null;
          default:
            throw Error(i(443, t.type));
        }
      else
        t.type === `stylesheet` &&
          !(t.state.loading & 4) &&
          ((r = t.instance), (t.state.loading |= 4), Vm(r, n.precedence, e));
      return t.instance;
    }
    function Vm(e, t, n) {
      for (
        var r = n.querySelectorAll(
            `link[rel="stylesheet"][data-precedence],style[data-precedence]`,
          ),
          i = r.length ? r[r.length - 1] : null,
          a = i,
          o = 0;
        o < r.length;
        o++
      ) {
        var s = r[o];
        if (s.dataset.precedence === t) a = s;
        else if (a !== i) break;
      }
      a
        ? a.parentNode.insertBefore(e, a.nextSibling)
        : ((t = n.nodeType === 9 ? n.head : n), t.insertBefore(e, t.firstChild));
    }
    function Hm(e, t) {
      ((e.crossOrigin ??= t.crossOrigin),
        (e.referrerPolicy ??= t.referrerPolicy),
        (e.title ??= t.title));
    }
    function Um(e, t) {
      ((e.crossOrigin ??= t.crossOrigin),
        (e.referrerPolicy ??= t.referrerPolicy),
        (e.integrity ??= t.integrity));
    }
    var Wm = null;
    function Gm(e, t, n) {
      if (Wm === null) {
        var r = new Map(),
          i = (Wm = new Map());
        i.set(n, r);
      } else ((i = Wm), (r = i.get(n)), r || ((r = new Map()), i.set(n, r)));
      if (r.has(e)) return r;
      for (r.set(e, null), n = n.getElementsByTagName(e), i = 0; i < n.length; i++) {
        var a = n[i];
        if (
          !(a[Dt] || a[bt] || (e === `link` && a.getAttribute(`rel`) === `stylesheet`)) &&
          a.namespaceURI !== `http://www.w3.org/2000/svg`
        ) {
          var o = a.getAttribute(t) || ``;
          o = e + o;
          var s = r.get(o);
          s ? s.push(a) : r.set(o, [a]);
        }
      }
      return r;
    }
    function Km(e, t, n) {
      ((e = e.ownerDocument || e),
        e.head.insertBefore(n, t === `title` ? e.querySelector(`head > title`) : null));
    }
    function qm(e, t, n) {
      if (n === 1 || t.itemProp != null) return !1;
      switch (e) {
        case `meta`:
        case `title`:
          return !0;
        case `style`:
          if (typeof t.precedence != `string` || typeof t.href != `string` || t.href === ``) break;
          return !0;
        case `link`:
          if (
            typeof t.rel != `string` ||
            typeof t.href != `string` ||
            t.href === `` ||
            t.onLoad ||
            t.onError
          )
            break;
          switch (t.rel) {
            case `stylesheet`:
              return ((e = t.disabled), typeof t.precedence == `string` && e == null);
            default:
              return !0;
          }
        case `script`:
          if (
            t.async &&
            typeof t.async != `function` &&
            typeof t.async != `symbol` &&
            !t.onLoad &&
            !t.onError &&
            t.src &&
            typeof t.src == `string`
          )
            return !0;
      }
      return !1;
    }
    function Jm(e, t) {
      return (
        e === `img` && t.src != null && t.src !== `` && t.onLoad == null && t.loading !== `lazy`
      );
    }
    function Ym(e) {
      return !(e.type === `stylesheet` && !(e.state.loading & 3));
    }
    function Xm(e) {
      return (
        (e.width || 100) *
        (e.height || 100) *
        (typeof devicePixelRatio == `number` ? devicePixelRatio : 1) *
        0.25
      );
    }
    function Zm(e, t) {
      typeof t.decode == `function` &&
        (e.imgCount++,
        t.complete || ((e.imgBytes += Xm(t)), e.suspenseyImages.push(t)),
        (e = rh.bind(e)),
        t.decode().then(e, e));
    }
    function Qm(e, t, n, r) {
      if (
        n.type === `stylesheet` &&
        (typeof r.media != `string` || !1 !== matchMedia(r.media).matches) &&
        !(n.state.loading & 4)
      ) {
        if (n.instance === null) {
          var i = Pm(r.href),
            a = t.querySelector(Fm(i));
          if (a) {
            ((t = a._p),
              typeof t == `object` &&
                t &&
                typeof t.then == `function` &&
                (e.count++, (e = nh.bind(e)), t.then(e, e)),
              (n.state.loading |= 4),
              (n.instance = a),
              Pt(a));
            return;
          }
          ((a = t.ownerDocument || t),
            (r = Im(r)),
            (i = vm.get(i)) && Hm(r, i),
            (a = a.createElement(`link`)),
            Pt(a));
          var o = a;
          ((o._p = new Promise(function (e, t) {
            ((o.onload = e), (o.onerror = t));
          })),
            np(a, `link`, r),
            (n.instance = a));
        }
        (e.stylesheets === null && (e.stylesheets = new Map()),
          e.stylesheets.set(n, t),
          (t = n.state.preload) &&
            !(n.state.loading & 3) &&
            (e.count++,
            (n = nh.bind(e)),
            t.addEventListener(`load`, n),
            t.addEventListener(`error`, n)));
      }
    }
    var $m = 0;
    function eh(e, t) {
      return (
        e.stylesheets && e.count === 0 && ah(e, e.stylesheets),
        0 < e.count || 0 < e.imgCount
          ? function (n) {
              var r = setTimeout(function () {
                if ((e.stylesheets && ah(e, e.stylesheets), e.unsuspend)) {
                  var t = e.unsuspend;
                  ((e.unsuspend = null), t());
                }
              }, 6e4 + t);
              0 < e.imgBytes && $m === 0 && ($m = 62500 * op());
              var i = setTimeout(
                function () {
                  if (
                    ((e.waitingForImages = !1),
                    e.count === 0 && (e.stylesheets && ah(e, e.stylesheets), e.unsuspend))
                  ) {
                    var t = e.unsuspend;
                    ((e.unsuspend = null), t());
                  }
                },
                (e.imgBytes > $m ? 50 : 800) + t,
              );
              return (
                (e.unsuspend = n),
                function () {
                  ((e.unsuspend = null), clearTimeout(r), clearTimeout(i));
                }
              );
            }
          : null
      );
    }
    function th(e) {
      if (e.count === 0 && (e.imgCount === 0 || !e.waitingForImages)) {
        if (e.stylesheets) ah(e, e.stylesheets);
        else if (e.unsuspend) {
          var t = e.unsuspend;
          ((e.unsuspend = null), t());
        }
      }
    }
    function nh() {
      (this.count--, th(this));
    }
    function rh() {
      (this.imgCount--, th(this));
    }
    var ih = null;
    function ah(e, t) {
      ((e.stylesheets = null),
        e.unsuspend !== null &&
          (e.count++, (ih = new Map()), t.forEach(oh, e), (ih = null), nh.call(e)));
    }
    function oh(e, t) {
      if (!(t.state.loading & 4)) {
        var n = ih.get(e);
        if (n) var r = n.get(null);
        else {
          ((n = new Map()), ih.set(e, n));
          for (
            var i = e.querySelectorAll(`link[data-precedence],style[data-precedence]`), a = 0;
            a < i.length;
            a++
          ) {
            var o = i[a];
            (o.nodeName === `LINK` || o.getAttribute(`media`) !== `not all`) &&
              (n.set(o.dataset.precedence, o), (r = o));
          }
          r && n.set(null, r);
        }
        ((i = t.instance),
          (o = i.getAttribute(`data-precedence`)),
          (a = n.get(o) || r),
          a === r && n.set(null, i),
          n.set(o, i),
          this.count++,
          (r = nh.bind(this)),
          i.addEventListener(`load`, r),
          i.addEventListener(`error`, r),
          a
            ? a.parentNode.insertBefore(i, a.nextSibling)
            : ((e = e.nodeType === 9 ? e.head : e), e.insertBefore(i, e.firstChild)),
          (t.state.loading |= 4));
      }
    }
    var sh = {
      $$typeof: M,
      Provider: null,
      Consumer: null,
      _currentValue: me,
      _currentValue2: me,
      _threadCount: 0,
    };
    function ch(e, t, n, r, i, a, o, s, c) {
      ((this.tag = 1),
        (this.containerInfo = e),
        (this.pingCache = this.current = this.pendingChildren = null),
        (this.timeoutHandle = -1),
        (this.callbackNode =
          this.next =
          this.pendingContext =
          this.context =
          this.cancelPendingCommit =
            null),
        (this.callbackPriority = 0),
        (this.expirationTimes = lt(-1)),
        (this.entangledLanes =
          this.shellSuspendCounter =
          this.errorRecoveryDisabledLanes =
          this.expiredLanes =
          this.warmLanes =
          this.pingedLanes =
          this.suspendedLanes =
          this.pendingLanes =
            0),
        (this.entanglements = lt(0)),
        (this.hiddenUpdates = lt(null)),
        (this.identifierPrefix = r),
        (this.onUncaughtError = i),
        (this.onCaughtError = a),
        (this.onRecoverableError = o),
        (this.pooledCache = null),
        (this.pooledCacheLanes = 0),
        (this.formState = c),
        (this.transitionTypes = null),
        (this.incompleteTransitions = new Map()));
    }
    function lh(e, t, n, r, i, a, o, s, c, l, u, d) {
      return (
        (e = new ch(e, t, n, o, c, l, u, d, s)),
        (t = 1),
        !0 === a && (t |= 24),
        (a = ki(3, null, null, t)),
        (e.current = a),
        (a.stateNode = e),
        (t = ka()),
        t.refCount++,
        (e.pooledCache = t),
        t.refCount++,
        (a.memoizedState = { element: r, isDehydrated: n, cache: t }),
        fo(a),
        e
      );
    }
    function uh(e) {
      return e ? ((e = Di), e) : Di;
    }
    function dh(e, t, n, r, i, a) {
      ((i = uh(i)),
        r.context === null ? (r.context = i) : (r.pendingContext = i),
        (r = mo(t)),
        (r.payload = { element: n }),
        (a = a === void 0 ? null : a),
        a !== null && (r.callback = a),
        (n = ho(e, r, t)),
        n !== null && (Pd(n, e, t), go(n, e, t)));
    }
    function fh(e, t) {
      if (((e = e.memoizedState), e !== null && e.dehydrated !== null)) {
        var n = e.retryLane;
        e.retryLane = n !== 0 && n < t ? n : t;
      }
    }
    function ph(e, t) {
      (fh(e, t), (e = e.alternate) && fh(e, t));
    }
    function mh(e) {
      if (e.tag === 13 || e.tag === 31) {
        var t = wi(e, 67108864);
        (t !== null && Pd(t, e, 67108864), ph(e, 67108864));
      }
    }
    function hh(e) {
      if (e.tag === 13 || e.tag === 31) {
        var t = jd();
        t = ht(t);
        var n = wi(e, t);
        (n !== null && Pd(n, e, t), ph(e, t));
      }
    }
    var gh = !0;
    function _h(e, t, n, r) {
      var i = L.T;
      L.T = null;
      var a = R.p;
      try {
        ((R.p = 2), yh(e, t, n, r));
      } finally {
        ((R.p = a), (L.T = i));
      }
    }
    function vh(e, t, n, r) {
      var i = L.T;
      L.T = null;
      var a = R.p;
      try {
        ((R.p = 8), yh(e, t, n, r));
      } finally {
        ((R.p = a), (L.T = i));
      }
    }
    function yh(e, t, n, r) {
      if (gh) {
        var i = bh(r);
        if (i === null) (Kf(e, t, r, xh, n), Mh(e, r));
        else if (Ph(i, e, t, n, r)) r.stopPropagation();
        else if ((Mh(e, r), t & 4 && -1 < jh.indexOf(e))) {
          for (; i !== null;) {
            var a = jt(i);
            if (a !== null)
              switch (a.tag) {
                case 3:
                  if (((a = a.stateNode), a.current.memoizedState.isDehydrated)) {
                    var o = B(a.pendingLanes);
                    if (o !== 0) {
                      var s = a;
                      for (s.pendingLanes |= 2, s.entangledLanes |= 2; o;) {
                        var c = 1 << (31 - Ze(o));
                        ((s.entanglements[1] |= c), (o &= ~c));
                      }
                      (Ef(a), !(q & 6) && ((gd = ze() + 500), Df(0, !1)));
                    }
                  }
                  break;
                case 31:
                case 13:
                  ((s = wi(a, 2)), s !== null && Pd(s, a, 2), zd(), ph(a, 2));
              }
            if (((a = bh(r)), a === null && Kf(e, t, r, xh, n), a === i)) break;
            i = a;
          }
          i !== null && r.stopPropagation();
        } else Kf(e, t, r, null, n);
      }
    }
    function bh(e) {
      return ((e = vn(e)), Sh(e));
    }
    var xh = null;
    function Sh(e) {
      if (((xh = null), (e = At(e)), e !== null)) {
        var t = o(e);
        if (t === null) e = null;
        else {
          var n = t.tag;
          if (n === 13) {
            if (((e = s(t)), e !== null)) return e;
            e = null;
          } else if (n === 31) {
            if (((e = c(t)), e !== null)) return e;
            e = null;
          } else if (n === 3) {
            if (t.stateNode.current.memoizedState.isDehydrated)
              return t.tag === 3 ? t.stateNode.containerInfo : null;
            e = null;
          } else t !== e && (e = null);
        }
      }
      return ((xh = e), null);
    }
    function Ch(e) {
      switch (e) {
        case `beforetoggle`:
        case `cancel`:
        case `click`:
        case `close`:
        case `contextmenu`:
        case `copy`:
        case `cut`:
        case `auxclick`:
        case `dblclick`:
        case `dragend`:
        case `dragstart`:
        case `drop`:
        case `focusin`:
        case `focusout`:
        case `input`:
        case `invalid`:
        case `keydown`:
        case `keypress`:
        case `keyup`:
        case `mousedown`:
        case `mouseup`:
        case `paste`:
        case `pause`:
        case `play`:
        case `pointercancel`:
        case `pointerdown`:
        case `pointerup`:
        case `ratechange`:
        case `reset`:
        case `seeked`:
        case `submit`:
        case `toggle`:
        case `touchcancel`:
        case `touchend`:
        case `touchstart`:
        case `volumechange`:
        case `change`:
        case `selectionchange`:
        case `textInput`:
        case `compositionstart`:
        case `compositionend`:
        case `compositionupdate`:
        case `beforeblur`:
        case `afterblur`:
        case `beforeinput`:
        case `blur`:
        case `fullscreenchange`:
        case `fullscreenerror`:
        case `focus`:
        case `hashchange`:
        case `popstate`:
        case `select`:
        case `selectstart`:
          return 2;
        case `drag`:
        case `dragenter`:
        case `dragexit`:
        case `dragleave`:
        case `dragover`:
        case `mousemove`:
        case `mouseout`:
        case `mouseover`:
        case `pointermove`:
        case `pointerout`:
        case `pointerover`:
        case `resize`:
        case `scroll`:
        case `touchmove`:
        case `wheel`:
        case `mouseenter`:
        case `mouseleave`:
        case `pointerenter`:
        case `pointerleave`:
          return 8;
        case `message`:
          switch (Be()) {
            case Ve:
              return 2;
            case He:
              return 8;
            case Ue:
            case We:
              return 32;
            case Ge:
              return 268435456;
            default:
              return 32;
          }
        default:
          return 32;
      }
    }
    var wh = !1,
      Th = null,
      Eh = null,
      Dh = null,
      Oh = new Map(),
      kh = new Map(),
      Ah = [],
      jh =
        `mousedown mouseup touchcancel touchend touchstart auxclick dblclick pointercancel pointerdown pointerup dragend dragstart drop compositionend compositionstart keydown keypress keyup input textInput copy cut paste click change contextmenu reset`.split(
          ` `,
        );
    function Mh(e, t) {
      switch (e) {
        case `focusin`:
        case `focusout`:
          Th = null;
          break;
        case `dragenter`:
        case `dragleave`:
          Eh = null;
          break;
        case `mouseover`:
        case `mouseout`:
          Dh = null;
          break;
        case `pointerover`:
        case `pointerout`:
          Oh.delete(t.pointerId);
          break;
        case `gotpointercapture`:
        case `lostpointercapture`:
          kh.delete(t.pointerId);
      }
    }
    function Nh(e, t, n, r, i, a) {
      return e === null || e.nativeEvent !== a
        ? ((e = {
            blockedOn: t,
            domEventName: n,
            eventSystemFlags: r,
            nativeEvent: a,
            targetContainers: [i],
          }),
          t !== null && ((t = jt(t)), t !== null && mh(t)),
          e)
        : ((e.eventSystemFlags |= r),
          (t = e.targetContainers),
          i !== null && t.indexOf(i) === -1 && t.push(i),
          e);
    }
    function Ph(e, t, n, r, i) {
      switch (t) {
        case `focusin`:
          return ((Th = Nh(Th, e, t, n, r, i)), !0);
        case `dragenter`:
          return ((Eh = Nh(Eh, e, t, n, r, i)), !0);
        case `mouseover`:
          return ((Dh = Nh(Dh, e, t, n, r, i)), !0);
        case `pointerover`:
          var a = i.pointerId;
          return (Oh.set(a, Nh(Oh.get(a) || null, e, t, n, r, i)), !0);
        case `gotpointercapture`:
          return ((a = i.pointerId), kh.set(a, Nh(kh.get(a) || null, e, t, n, r, i)), !0);
      }
      return !1;
    }
    function Fh(e) {
      var t = At(e.target);
      if (t !== null) {
        var n = o(t);
        if (n !== null) {
          if (((t = n.tag), t === 13)) {
            if (((t = s(n)), t !== null)) {
              ((e.blockedOn = t),
                vt(e.priority, function () {
                  hh(n);
                }));
              return;
            }
          } else if (t === 31) {
            if (((t = c(n)), t !== null)) {
              ((e.blockedOn = t),
                vt(e.priority, function () {
                  hh(n);
                }));
              return;
            }
          } else if (t === 3 && n.stateNode.current.memoizedState.isDehydrated) {
            e.blockedOn = n.tag === 3 ? n.stateNode.containerInfo : null;
            return;
          }
        }
      }
      e.blockedOn = null;
    }
    function Ih(e) {
      if (e.blockedOn !== null) return !1;
      for (var t = e.targetContainers; 0 < t.length;) {
        var n = bh(e.nativeEvent);
        if (n === null) {
          n = e.nativeEvent;
          var r = new n.constructor(n.type, n);
          ((_n = r), n.target.dispatchEvent(r), (_n = null));
        } else return ((t = jt(n)), t !== null && mh(t), (e.blockedOn = n), !1);
        t.shift();
      }
      return !0;
    }
    function Lh(e, t, n) {
      Ih(e) && n.delete(t);
    }
    function Rh() {
      ((wh = !1),
        Th !== null && Ih(Th) && (Th = null),
        Eh !== null && Ih(Eh) && (Eh = null),
        Dh !== null && Ih(Dh) && (Dh = null),
        Oh.forEach(Lh),
        kh.forEach(Lh));
    }
    function zh(e, n) {
      e.blockedOn === n &&
        ((e.blockedOn = null),
        wh || ((wh = !0), t.unstable_scheduleCallback(t.unstable_NormalPriority, Rh)));
    }
    var Bh = null;
    function Vh(e) {
      Bh !== e &&
        ((Bh = e),
        t.unstable_scheduleCallback(t.unstable_NormalPriority, function () {
          Bh === e && (Bh = null);
          for (var t = 0; t < e.length; t += 3) {
            var n = e[t],
              r = e[t + 1],
              i = e[t + 2];
            if (typeof r != `function`) {
              if (Sh(r || n) === null) continue;
              break;
            }
            var a = jt(n);
            a !== null &&
              (e.splice(t, 3),
              (t -= 3),
              Qs(a, { pending: !0, data: i, method: n.method, action: r }, r, i));
          }
        }));
    }
    function Hh(e) {
      function t(t) {
        return zh(t, e);
      }
      (Th !== null && zh(Th, e),
        Eh !== null && zh(Eh, e),
        Dh !== null && zh(Dh, e),
        Oh.forEach(t),
        kh.forEach(t));
      for (var n = 0; n < Ah.length; n++) {
        var r = Ah[n];
        r.blockedOn === e && (r.blockedOn = null);
      }
      for (; 0 < Ah.length && ((n = Ah[0]), n.blockedOn === null);)
        (Fh(n), n.blockedOn === null && Ah.shift());
      if (((n = (e.ownerDocument || e).$$reactFormReplay), n != null))
        for (r = 0; r < n.length; r += 3) {
          var i = n[r],
            a = n[r + 1],
            o = i[xt] || null;
          if (typeof a == `function`) o || Vh(n);
          else if (o) {
            var s = null;
            if (a && a.hasAttribute(`formAction`)) {
              if (((i = a), (o = a[xt] || null))) s = o.formAction;
              else if (Sh(i) !== null) continue;
            } else s = o.action;
            (typeof s == `function` ? (n[r + 1] = s) : (n.splice(r, 3), (r -= 3)), Vh(n));
          }
        }
    }
    function Uh() {
      function e(e) {
        e.canIntercept &&
          e.info === `react-transition` &&
          e.intercept({
            handler: function () {
              return new Promise(function (e) {
                return (i = e);
              });
            },
            focusReset: `manual`,
            scroll: `manual`,
          });
      }
      function t() {
        (i !== null && (i(), (i = null)), r || setTimeout(n, 20));
      }
      function n() {
        if (!r && !navigation.transition) {
          var e = navigation.currentEntry;
          e &&
            e.url != null &&
            navigation.navigate(e.url, {
              state: e.getState(),
              info: `react-transition`,
              history: `replace`,
            });
        }
      }
      if (typeof navigation == `object`) {
        var r = !1,
          i = null;
        return (
          navigation.addEventListener(`navigate`, e),
          navigation.addEventListener(`navigatesuccess`, t),
          navigation.addEventListener(`navigateerror`, t),
          setTimeout(n, 100),
          function () {
            ((r = !0),
              navigation.removeEventListener(`navigate`, e),
              navigation.removeEventListener(`navigatesuccess`, t),
              navigation.removeEventListener(`navigateerror`, t),
              i !== null && (i(), (i = null)));
          }
        );
      }
    }
    function Wh(e) {
      this._internalRoot = e;
    }
    ((Gh.prototype.render = Wh.prototype.render =
      function (e) {
        var t = this._internalRoot;
        if (t === null) throw Error(i(409));
        var n = t.current;
        dh(n, jd(), e, t, null, null);
      }),
      (Gh.prototype.unmount = Wh.prototype.unmount =
        function () {
          var e = this._internalRoot;
          if (e !== null) {
            this._internalRoot = null;
            var t = e.containerInfo;
            (dh(e.current, 2, null, e, null, null), zd(), (t[St] = null));
          }
        }));
    function Gh(e) {
      this._internalRoot = e;
    }
    Gh.prototype.unstable_scheduleHydration = function (e) {
      if (e) {
        var t = _t();
        e = { blockedOn: null, target: e, priority: t };
        for (var n = 0; n < Ah.length && t !== 0 && t < Ah[n].priority; n++);
        (Ah.splice(n, 0, e), n === 0 && Fh(e));
      }
    };
    var Kh = n.version;
    if (Kh !== `19.3.1`) throw Error(i(527, Kh, `19.3.1`));
    R.findDOMNode = function (e) {
      var t = e._reactInternals;
      if (t === void 0)
        throw typeof e.render == `function`
          ? Error(i(188))
          : ((e = Object.keys(e).join(`,`)), Error(i(268, e)));
      return ((e = d(t)), (e = e === null ? null : p(e)), (e = e === null ? null : e.stateNode), e);
    };
    var qh = {
      bundleType: 0,
      version: `19.3.1`,
      rendererPackageName: `react-dom`,
      currentDispatcherRef: L,
      reconcilerVersion: `19.3.1`,
    };
    if (typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ < `u`) {
      var Jh = __REACT_DEVTOOLS_GLOBAL_HOOK__;
      if (!Jh.isDisabled && Jh.supportsFiber)
        try {
          ((Je = Jh.inject(qh)), (Ye = Jh));
        } catch {}
    }
    e.createRoot = function (e, t) {
      if (!a(e)) throw Error(i(299));
      var n = !1,
        r = ``,
        o = xc,
        s = Sc,
        c = Cc;
      return (
        t != null &&
          (!0 === t.unstable_strictMode && (n = !0),
          t.identifierPrefix !== void 0 && (r = t.identifierPrefix),
          t.onUncaughtError !== void 0 && (o = t.onUncaughtError),
          t.onCaughtError !== void 0 && (s = t.onCaughtError),
          t.onRecoverableError !== void 0 && (c = t.onRecoverableError)),
        (t = lh(e, 1, !1, null, null, n, r, null, o, s, c, Uh)),
        (e[St] = t.current),
        Wf(e),
        new Wh(t)
      );
    };
  }),
  g = o((e, t) => {
    function n() {
      if (
        typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ < `u` &&
        typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE == `function`
      )
        try {
          __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(n);
        } catch (e) {
          console.error(e);
        }
    }
    (n(), (t.exports = h()));
  }),
  _ = o((e) => {
    var t = Symbol.for(`react.transitional.element`),
      n = Symbol.for(`react.fragment`);
    function r(e, n, r) {
      var i = null;
      if ((r !== void 0 && (i = `` + r), n.key !== void 0 && (i = `` + n.key), `key` in n))
        for (var a in ((r = {}), n)) a !== `key` && (r[a] = n[a]);
      else r = n;
      return (
        (n = r.ref), { $$typeof: t, type: e, key: i, ref: n === void 0 ? null : n, props: r }
      );
    }
    ((e.Fragment = n), (e.jsx = r), (e.jsxs = r));
  }),
  v = o((e, t) => {
    t.exports = _();
  }),
  y = c(u(), 1),
  b = v(),
  x = y.createContext(void 0),
  S = (e) => {
    let t = y.useContext(x);
    if (e) return e;
    if (!t) throw Error(`No QueryClient set, use QueryClientProvider to set one`);
    return t;
  },
  C = ({ client: e, children: t }) => (
    y.useEffect(
      () => (
        e.mount(),
        () => {
          e.unmount();
        }
      ),
      [e],
    ),
    (0, b.jsx)(x.Provider, { value: e, children: t })
  ),
  w = {
    setTimeout: (e, t) => setTimeout(e, t),
    clearTimeout: (e) => clearTimeout(e),
    setInterval: (e, t) => setInterval(e, t),
    clearInterval: (e) => clearInterval(e),
  },
  T = new (class {
    #e = w;
    setTimeoutProvider(e) {
      this.#e = e;
    }
    setTimeout(e, t) {
      return this.#e.setTimeout(e, t);
    }
    clearTimeout(e) {
      this.#e.clearTimeout(e);
    }
    setInterval(e, t) {
      return this.#e.setInterval(e, t);
    }
    clearInterval(e) {
      this.#e.clearInterval(e);
    }
  })();
function E(e) {
  setTimeout(e, 0);
}
var D = typeof window > `u` || `Deno` in globalThis;
function O() {}
function ee(e, t) {
  return typeof e == `function` ? e(t) : e;
}
function te(e) {
  return typeof e == `number` && e >= 0 && e !== 1 / 0;
}
function k(e, t) {
  return Math.max(e + (t || 0) - Date.now(), 0);
}
function A(e, t) {
  return typeof e == `function` ? e(t) : e;
}
function j(e, t) {
  let { type: n = `all`, exact: r, fetchStatus: i, predicate: a, queryKey: o, stale: s } = e;
  if (o) {
    if (r) {
      if (t.queryHash !== M(o, t.options)) return !1;
    } else if (!P(t.queryKey, o)) return !1;
  }
  if (n !== `all`) {
    let e = t.isActive();
    if ((n === `active` && !e) || (n === `inactive` && e)) return !1;
  }
  return !(
    (typeof s == `boolean` && t.isStale() !== s) ||
    (i && i !== t.state.fetchStatus) ||
    (a && !a(t))
  );
}
function ne(e, t) {
  let { exact: n, status: r, predicate: i, mutationKey: a } = e;
  if (a) {
    if (!t.options.mutationKey) return !1;
    if (n) {
      if (N(t.options.mutationKey) !== N(a)) return !1;
    } else if (!P(t.options.mutationKey, a)) return !1;
  }
  return !((r && t.state.status !== r) || (i && !i(t)));
}
function M(e, t) {
  return (t?.queryKeyHashFn || N)(e);
}
function N(e) {
  return JSON.stringify(e, (e, t) =>
    ae(t)
      ? Object.keys(t)
          .sort()
          .reduce((e, n) => ((e[n] = t[n]), e), {})
      : t,
  );
}
function P(e, t) {
  if (e === t) return !0;
  if (typeof e != typeof t) return !1;
  if (e && t && typeof e == `object` && typeof t == `object`) {
    if (Array.isArray(e) && Array.isArray(t)) {
      if (t.length > e.length) return !1;
      for (let n = 0; n < t.length; n++) if (!P(e[n], t[n])) return !1;
      return !0;
    }
    let n = Object.keys(t);
    for (let r of n) if (!P(e[r], t[r])) return !1;
    return !0;
  }
  return !1;
}
var re = Object.prototype.hasOwnProperty;
function F(e, t, n = 0) {
  if (e === t) return e;
  if (n > 500) return t;
  let r = ie(e) && ie(t);
  if (!r && !(ae(e) && ae(t))) return t;
  let i = (r ? e : Object.keys(e)).length,
    a = r ? t : Object.keys(t),
    o = a.length,
    s = r ? Array(o) : {},
    c = 0;
  for (let l = 0; l < o; l++) {
    let o = r ? l : a[l],
      u = e[o],
      d = t[o];
    if (u === d) {
      ((s[o] = u), (r ? l < i : re.call(e, o)) && c++);
      continue;
    }
    if (u === null || d === null || typeof u != `object` || typeof d != `object`) {
      s[o] = d;
      continue;
    }
    let f = F(u, d, n + 1);
    ((s[o] = f), f === u && c++);
  }
  return i === o && c === i ? e : s;
}
function I(e, t) {
  if (!t || Object.keys(e).length !== Object.keys(t).length) return !1;
  for (let n in e) if (e[n] !== t[n]) return !1;
  return !0;
}
function ie(e) {
  return Array.isArray(e) && e.length === Object.keys(e).length;
}
function ae(e) {
  if (!oe(e)) return !1;
  let t = Object.getPrototypeOf(e),
    n = t?.constructor;
  if (n === void 0) return !0;
  if (typeof n != `function`) return !1;
  let r = n.prototype;
  return !(!oe(r) || !r.hasOwnProperty(`isPrototypeOf`) || t !== Object.prototype);
}
function oe(e) {
  return Object.prototype.toString.call(e) === `[object Object]`;
}
function se(e) {
  return new Promise((t) => {
    T.setTimeout(t, e);
  });
}
function ce(e, t, n) {
  return typeof n.structuralSharing == `function`
    ? n.structuralSharing(e, t)
    : n.structuralSharing === !1
      ? t
      : F(e, t);
}
function le(e, t, n = 0) {
  let r = [...e, t];
  return n && r.length > n ? r.slice(1) : r;
}
function ue(e, t, n = 0) {
  let r = [t, ...e];
  return n && r.length > n ? r.slice(0, -1) : r;
}
var de = Symbol();
function fe(e, t) {
  return !e.queryFn && t?.initialPromise
    ? () => t.initialPromise
    : !e.queryFn || e.queryFn === de
      ? () => Promise.reject(Error(`Missing queryFn: '${e.queryHash}'`))
      : e.queryFn;
}
function pe(e, t) {
  return typeof e == `function` ? e(...t) : !!e;
}
function L(e, t, n) {
  let r = !1,
    i;
  return (
    Object.defineProperty(e, "signal", {
      enumerable: !0,
      get: () => (
        (i ??= t()),
        r || ((r = !0), i.aborted ? n() : i.addEventListener(`abort`, n, { once: !0 })),
        i
      ),
    }),
    e
  );
}
var R = () => D,
  me = () => R(),
  he = class {
    constructor() {
      ((this.listeners = new Set()), (this.subscribe = this.subscribe.bind(this)));
    }
    subscribe(e) {
      return (
        this.listeners.add(e),
        this.onSubscribe(),
        () => {
          (this.listeners.delete(e), this.onUnsubscribe());
        }
      );
    }
    hasListeners() {
      return this.listeners.size > 0;
    }
    onSubscribe() {}
    onUnsubscribe() {}
  },
  ge = new (class extends he {
    #e;
    #t;
    #n;
    constructor() {
      (super(),
        (this.#n = (e) => {
          if (typeof window < `u` && window.addEventListener) {
            let t = () => e();
            return (
              window.addEventListener(`visibilitychange`, t, !1),
              () => {
                window.removeEventListener(`visibilitychange`, t);
              }
            );
          }
        }));
    }
    onSubscribe() {
      this.#t || this.setEventListener(this.#n);
    }
    onUnsubscribe() {
      this.hasListeners() || (this.#t?.(), (this.#t = void 0));
    }
    setEventListener(e) {
      ((this.#n = e),
        this.#t?.(),
        (this.#t = e((e) => {
          typeof e == `boolean` ? this.setFocused(e) : this.onFocus();
        })));
    }
    setFocused(e) {
      this.#e !== e && ((this.#e = e), this.onFocus());
    }
    onFocus() {
      let e = this.isFocused();
      this.listeners.forEach((t) => {
        t(e);
      });
    }
    isFocused() {
      return typeof this.#e == `boolean`
        ? this.#e
        : globalThis.document?.visibilityState !== `hidden`;
    }
  })(),
  _e = E;
function ve() {
  let e = [],
    t = 0,
    n = (e) => {
      e();
    },
    r = (e) => {
      e();
    },
    i = _e,
    a = (r) => {
      t
        ? e.push(r)
        : i(() => {
            n(r);
          });
    },
    o = () => {
      let t = e;
      ((e = []),
        t.length &&
          i(() => {
            r(() => {
              t.forEach((e) => {
                n(e);
              });
            });
          }));
    };
  return {
    batch: (e) => {
      let n;
      t++;
      try {
        n = e();
      } finally {
        (t--, t || o());
      }
      return n;
    },
    batchCalls:
      (e) =>
      (...t) => {
        a(() => {
          e(...t);
        });
      },
    schedule: a,
    setNotifyFunction: (e) => {
      n = e;
    },
    setBatchNotifyFunction: (e) => {
      r = e;
    },
    setScheduler: (e) => {
      i = e;
    },
  };
}
var z = ve(),
  ye = new (class extends he {
    #e = !0;
    #t;
    #n;
    constructor() {
      (super(),
        (this.#n = (e) => {
          if (typeof window < `u` && window.addEventListener) {
            let t = () => e(!0),
              n = () => e(!1);
            return (
              window.addEventListener(`online`, t, !1),
              window.addEventListener(`offline`, n, !1),
              () => {
                (window.removeEventListener(`online`, t), window.removeEventListener(`offline`, n));
              }
            );
          }
        }));
    }
    onSubscribe() {
      this.#t || this.setEventListener(this.#n);
    }
    onUnsubscribe() {
      this.hasListeners() || (this.#t?.(), (this.#t = void 0));
    }
    setEventListener(e) {
      ((this.#n = e), this.#t?.(), (this.#t = e(this.setOnline.bind(this))));
    }
    setOnline(e) {
      this.#e !== e &&
        ((this.#e = e),
        this.listeners.forEach((t) => {
          t(e);
        }));
    }
    isOnline() {
      return this.#e;
    }
  })();
function be(e) {
  return Math.min(1e3 * 2 ** e, 3e4);
}
function xe(e) {
  return (e ?? `online`) !== `online` || ye.isOnline();
}
var Se = class extends Error {
  constructor(e) {
    (super(`CancelledError`), (this.revert = e?.revert), (this.silent = e?.silent));
  }
};
function Ce(e) {
  let t = !1,
    n = 0,
    r,
    i = `pending`,
    a,
    o,
    s = new Promise((e, t) => {
      ((a = e), (o = t));
    });
  s.catch(O);
  let c = () => i !== `pending`,
    l = (t) => {
      if (!c()) {
        let n = new Se(t);
        (h(n), e.onCancel?.(n));
      }
    },
    u = () => {
      t = !0;
    },
    d = () => {
      t = !1;
    },
    f = () => ge.isFocused() && (e.networkMode === `always` || ye.isOnline()) && e.canRun(),
    p = () => xe(e.networkMode) && e.canRun(),
    m = (e) => {
      c() || (r?.(), (i = `resolved`), a(e));
    },
    h = (e) => {
      c() || (r?.(), (i = `rejected`), o(e));
    },
    g = (t) =>
      new Promise((n) => {
        ((r = (e) => {
          (c() || t()) && n(e);
        }),
          e.onPause?.());
      }).then(() => {
        ((r = void 0), c() || e.onContinue?.());
      }),
    _ = () => {
      if (c()) return;
      let r,
        i = n === 0 ? e.initialPromise : void 0;
      try {
        r = i ?? e.fn();
      } catch (e) {
        r = Promise.reject(e);
      }
      Promise.resolve(r)
        .then(m)
        .catch((r) => {
          if (c()) return;
          let i = e.retry ?? (me() ? 0 : 3),
            a = e.retryDelay ?? be,
            o = typeof a == `function` ? a(n, r) : a,
            s = i === !0 || (typeof i == `number` && n < i) || (typeof i == `function` && i(n, r));
          t || !s
            ? h(r)
            : (n++,
              e.onFail?.(n, r),
              se(o)
                .then(() => (f() ? void 0 : g(f)))
                .then(() => {
                  t ? h(r) : _();
                }));
        });
    };
  return {
    promise: s,
    status: () => i,
    cancel: l,
    continue: () => (r?.(), s),
    cancelRetry: u,
    continueRetry: d,
    canStart: p,
    start: () => (p() ? _() : g(p).then(_), s),
  };
}
var we = class {
  #e;
  destroy() {
    this.clearGcTimeout();
  }
  scheduleGc() {
    (this.clearGcTimeout(),
      te(this.gcTime) &&
        (this.#e = T.setTimeout(() => {
          this.optionalRemove();
        }, this.gcTime)));
  }
  updateGcTime(e) {
    this.gcTime = Math.max(this.gcTime || 0, e ?? (me() ? 1 / 0 : 3e5));
  }
  clearGcTimeout() {
    this.#e !== void 0 && (T.clearTimeout(this.#e), (this.#e = void 0));
  }
};
function Te(e) {
  return {
    onFetch: (t, n) => {
      let r = t.options,
        i = t.fetchOptions?.meta?.fetchMore?.direction,
        a = t.state.data?.pages || [],
        o = t.state.data?.pageParams || [],
        s = { pages: [], pageParams: [] },
        c = 0,
        l = async () => {
          let n = !1,
            l = (e) => {
              L(
                e,
                () => t.signal,
                () => (n = !0),
              );
            },
            u = fe(t.options, t.fetchOptions),
            d = async (e, r, i) => {
              if (n) return Promise.reject(t.signal.reason);
              if (r == null && e.pages.length) return Promise.resolve(e);
              let a = (() => {
                  let e = {
                    client: t.client,
                    queryKey: t.queryKey,
                    pageParam: r,
                    direction: i ? `backward` : `forward`,
                    meta: t.options.meta,
                  };
                  return (l(e), e);
                })(),
                o = await u(a),
                { maxPages: s } = t.options,
                c = i ? ue : le;
              return { pages: c(e.pages, o, s), pageParams: c(e.pageParams, r, s) };
            };
          if (i && a.length) {
            let e = i === `backward`,
              t = e ? De : Ee,
              n = { pages: a, pageParams: o };
            s = await d(n, t(r, n), e);
          } else {
            let t = e ?? a.length;
            do {
              let e = c === 0 ? (o[0] === void 0 ? r.initialPageParam : o[0]) : Ee(r, s);
              if (c > 0 && e == null) break;
              ((s = await d(s, e)), c++);
            } while (c < t);
          }
          return s;
        };
      t.fetchFn = t.options.persister
        ? () =>
            t.options.persister?.(
              l,
              { client: t.client, queryKey: t.queryKey, meta: t.options.meta, signal: t.signal },
              n,
            )
        : l;
    },
  };
}
function Ee(e, { pages: t, pageParams: n }) {
  let r = t.length - 1;
  return t.length > 0 ? e.getNextPageParam(t[r], t, n[r], n) : void 0;
}
function De(e, { pages: t, pageParams: n }) {
  return t.length > 0 ? e.getPreviousPageParam?.(t[0], t, n[0], n) : void 0;
}
var Oe = class extends we {
  #e;
  #t;
  #n;
  #r;
  #i;
  #a;
  #o;
  #s;
  constructor(e) {
    (super(),
      (this.#s = !1),
      (this.#o = e.defaultOptions),
      this.setOptions(e.options),
      (this.observers = []),
      (this.#i = e.client),
      (this.#r = this.#i.getQueryCache()),
      (this.queryKey = e.queryKey),
      (this.queryHash = e.queryHash),
      (this.#t = je(this.options)),
      (this.state = e.state ?? this.#t),
      this.scheduleGc());
  }
  get meta() {
    return this.options.meta;
  }
  get queryType() {
    return this.#e;
  }
  get promise() {
    return this.#a?.promise;
  }
  setOptions(e) {
    if (
      ((this.options = { ...this.#o, ...e }),
      e?._type && (this.#e = e._type),
      this.updateGcTime(this.options.gcTime),
      this.state && this.state.data === void 0)
    ) {
      let e = je(this.options);
      e.data !== void 0 && (this.setState(Ae(e.data, e.dataUpdatedAt)), (this.#t = e));
    }
  }
  optionalRemove() {
    !this.observers.length && this.state.fetchStatus === `idle` && this.#r.remove(this);
  }
  setData(e, t) {
    let n = ce(this.state.data, e, this.options);
    return (
      this.#c({ data: n, type: `success`, dataUpdatedAt: t?.updatedAt, manual: t?.manual }), n
    );
  }
  setState(e) {
    this.#c({ type: `setState`, state: e });
  }
  cancel(e) {
    let t = this.#a?.promise;
    return (this.#a?.cancel(e), t ? t.then(O).catch(O) : Promise.resolve());
  }
  destroy() {
    (super.destroy(), this.cancel({ silent: !0 }));
  }
  get resetState() {
    return this.#t;
  }
  reset() {
    (this.destroy(), this.setState(this.resetState));
  }
  isActive() {
    return this.observers.some((e) => A(e.options.enabled, this) !== !1);
  }
  isDisabled() {
    return this.getObserversCount() > 0
      ? !this.isActive()
      : this.options.queryFn === de || !this.isFetched();
  }
  isFetched() {
    return this.state.dataUpdateCount + this.state.errorUpdateCount > 0;
  }
  isStatic() {
    return (
      this.getObserversCount() > 0 &&
      this.observers.some((e) => A(e.options.staleTime, this) === `static`)
    );
  }
  isStale() {
    return this.getObserversCount() > 0
      ? this.observers.some((e) => e.getCurrentResult().isStale)
      : this.state.data === void 0 || this.state.isInvalidated;
  }
  isStaleByTime(e = 0) {
    return this.state.data === void 0
      ? !0
      : e === `static`
        ? !1
        : this.state.isInvalidated
          ? !0
          : !k(this.state.dataUpdatedAt, e);
  }
  onFocus() {
    (this.observers.find((e) => e.shouldFetchOnWindowFocus())?.refetch({ cancelRefetch: !1 }),
      this.#a?.continue());
  }
  onOnline() {
    (this.observers.find((e) => e.shouldFetchOnReconnect())?.refetch({ cancelRefetch: !1 }),
      this.#a?.continue());
  }
  addObserver(e) {
    this.observers.includes(e) ||
      (this.observers.push(e),
      this.clearGcTimeout(),
      this.#r.notify({ type: `observerAdded`, query: this, observer: e }));
  }
  removeObserver(e) {
    let t = this.observers.indexOf(e);
    t !== -1 &&
      (this.observers.splice(t, 1),
      this.observers.length ||
        (this.#a &&
          (this.#s || (this.state.fetchStatus === `paused` && this.state.status === `pending`)
            ? this.#a.cancel({ revert: !0 })
            : this.#a.cancelRetry()),
        this.scheduleGc()),
      this.#r.notify({ type: `observerRemoved`, query: this, observer: e }));
  }
  getObserversCount() {
    return this.observers.length;
  }
  invalidate() {
    this.state.isInvalidated || this.#c({ type: `invalidate` });
  }
  async fetch(e, t) {
    if (this.state.fetchStatus !== `idle` && this.#a?.status() !== `rejected`) {
      if (this.state.data !== void 0 && t?.cancelRefetch) this.cancel({ silent: !0 });
      else if (this.#a) return (this.#a.continueRetry(), this.#a.promise);
    }
    if ((e && this.setOptions(e), !this.options.queryFn)) {
      let e = this.observers.find((e) => e.options.queryFn);
      e && this.setOptions(e.options);
    }
    let n = new AbortController(),
      r = (e) => {
        Object.defineProperty(e, "signal", {
          enumerable: !0,
          get: () => ((this.#s = !0), n.signal),
        });
      },
      i = () => {
        let e = fe(this.options, t),
          n = (() => {
            let e = { client: this.#i, queryKey: this.queryKey, meta: this.meta };
            return (r(e), e);
          })();
        return ((this.#s = !1), this.options.persister ? this.options.persister(e, n, this) : e(n));
      },
      a = (() => {
        let e = {
          fetchOptions: t,
          options: this.options,
          queryKey: this.queryKey,
          client: this.#i,
          state: this.state,
          fetchFn: i,
        };
        return (r(e), e);
      })();
    ((this.#e === `infinite` ? Te(this.options.pages) : this.options.behavior)?.onFetch(a, this),
      (this.#n = this.state),
      (this.state.fetchStatus === `idle` || this.state.fetchMeta !== a.fetchOptions?.meta) &&
        this.#c({ type: `fetch`, meta: a.fetchOptions?.meta }));
    let o = (this.#a = Ce({
      initialPromise: t?.initialPromise,
      fn: a.fetchFn,
      onCancel: (e) => {
        (e instanceof Se && e.revert && this.setState({ ...this.#n, fetchStatus: `idle` }),
          n.abort());
      },
      onFail: (e, t) => {
        this.#c({ type: `failed`, failureCount: e, error: t });
      },
      onPause: () => {
        this.#c({ type: `pause` });
      },
      onContinue: () => {
        this.#c({ type: `continue` });
      },
      retry: a.options.retry,
      retryDelay: a.options.retryDelay,
      networkMode: a.options.networkMode,
      canRun: () => !0,
    }));
    try {
      let e = await o.start();
      if (e === void 0) throw Error(`${this.queryHash} data is undefined`);
      return (
        this.setData(e),
        this.#r.config.onSuccess?.(e, this),
        this.#r.config.onSettled?.(e, this.state.error, this),
        e
      );
    } catch (e) {
      if (e instanceof Se) {
        if (e.silent) return this.#a.promise;
        if (e.revert) {
          if (this.state.data === void 0) throw e;
          return this.state.data;
        }
      }
      throw (
        this.#c({ type: `error`, error: e }),
        this.#r.config.onError?.(e, this),
        this.#r.config.onSettled?.(this.state.data, e, this),
        e
      );
    } finally {
      (this.#a === o && (this.#a = void 0), this.scheduleGc());
    }
  }
  #c(e) {
    let t = (t) => {
      switch (e.type) {
        case `failed`:
          return { ...t, fetchFailureCount: e.failureCount, fetchFailureReason: e.error };
        case `pause`:
          return { ...t, fetchStatus: `paused` };
        case `continue`:
          return { ...t, fetchStatus: `fetching` };
        case `fetch`:
          return { ...t, ...ke(t.data, this.options), fetchMeta: e.meta ?? null };
        case `success`:
          let n = {
            ...t,
            ...Ae(e.data, e.dataUpdatedAt),
            dataUpdateCount: t.dataUpdateCount + 1,
            ...(!e.manual && {
              fetchStatus: `idle`,
              fetchFailureCount: 0,
              fetchFailureReason: null,
            }),
          };
          return ((this.#n = e.manual ? n : void 0), n);
        case `error`:
          let r = e.error;
          return {
            ...t,
            error: r,
            errorUpdateCount: t.errorUpdateCount + 1,
            errorUpdatedAt: Date.now(),
            fetchFailureCount: t.fetchFailureCount + 1,
            fetchFailureReason: r,
            fetchStatus: `idle`,
            status: `error`,
            isInvalidated: !0,
          };
        case `invalidate`:
          return { ...t, isInvalidated: !0 };
        case `setState`:
          return { ...t, ...e.state };
      }
    };
    ((this.state = t(this.state)),
      z.batch(() => {
        (this.observers.slice().forEach((e) => {
          e.onQueryUpdate();
        }),
          this.#r.notify({ query: this, type: `updated`, action: e }));
      }));
  }
};
function ke(e, t) {
  return {
    fetchFailureCount: 0,
    fetchFailureReason: null,
    fetchStatus: xe(t.networkMode) ? `fetching` : `paused`,
    ...(e === void 0 && { error: null, status: `pending` }),
  };
}
function Ae(e, t) {
  return {
    data: e,
    dataUpdatedAt: t ?? Date.now(),
    error: null,
    isInvalidated: !1,
    status: `success`,
  };
}
function je(e) {
  let t = typeof e.initialData == `function` ? e.initialData() : e.initialData,
    n = t !== void 0,
    r = n
      ? typeof e.initialDataUpdatedAt == `function`
        ? e.initialDataUpdatedAt()
        : e.initialDataUpdatedAt
      : 0;
  return {
    data: t,
    dataUpdateCount: 0,
    dataUpdatedAt: n ? (r ?? Date.now()) : 0,
    error: null,
    errorUpdateCount: 0,
    errorUpdatedAt: 0,
    fetchFailureCount: 0,
    fetchFailureReason: null,
    fetchMeta: null,
    isInvalidated: !1,
    status: n ? `success` : `pending`,
    fetchStatus: `idle`,
  };
}
var Me = class extends he {
  #e;
  #t = void 0;
  #n = void 0;
  #r = void 0;
  #i;
  #a;
  #o;
  #s;
  #c;
  #l;
  #u;
  #d;
  #f;
  #p = new Set();
  constructor(e, t) {
    (super(),
      (this.options = t),
      (this.#e = e),
      (this.#o = null),
      this.bindMethods(),
      this.setOptions(t));
  }
  bindMethods() {
    this.refetch = this.refetch.bind(this);
  }
  onSubscribe() {
    this.listeners.size === 1 &&
      (this.#t.addObserver(this),
      Pe(this.#t, this.options) ? this.#m() : this.updateResult(),
      this.#y());
  }
  onUnsubscribe() {
    this.hasListeners() || this.destroy();
  }
  shouldFetchOnReconnect() {
    return Fe(this.#t, this.options, this.options.refetchOnReconnect);
  }
  shouldFetchOnWindowFocus() {
    return Fe(this.#t, this.options, this.options.refetchOnWindowFocus);
  }
  destroy() {
    ((this.listeners = new Set()), this.#b(), this.#x(), this.#t.removeObserver(this));
  }
  setOptions(e) {
    let t = this.options,
      n = this.#t;
    if (
      ((this.options = this.#e.defaultQueryOptions(e)),
      this.options.enabled !== void 0 &&
        typeof this.options.enabled != `boolean` &&
        typeof this.options.enabled != `function` &&
        typeof A(this.options.enabled, this.#t) != `boolean`)
    )
      throw Error(`Expected enabled to be a boolean or a callback that returns a boolean`);
    (this.#S(),
      this.#t.setOptions(this.options),
      t._defaulted &&
        !I(this.options, t) &&
        this.#e
          .getQueryCache()
          .notify({ type: `observerOptionsUpdated`, query: this.#t, observer: this }));
    let r = this.hasListeners();
    (r && Ie(this.#t, n, this.options, t) && this.#m(),
      this.updateResult(),
      r &&
        (this.#t !== n ||
          A(this.options.enabled, this.#t) !== A(t.enabled, this.#t) ||
          A(this.options.staleTime, this.#t) !== A(t.staleTime, this.#t)) &&
        this.#g());
    let i = this.#_();
    r &&
      (this.#t !== n ||
        A(this.options.enabled, this.#t) !== A(t.enabled, this.#t) ||
        i !== this.#f) &&
      this.#v(i);
  }
  getOptimisticResult(e) {
    let t = this.#e.getQueryCache().build(this.#e, e),
      n = this.createResult(t, e);
    return (
      I(this.getCurrentResult(), n) ||
        ((this.#r = n), (this.#a = this.options), (this.#i = this.#t.state)),
      n
    );
  }
  getCurrentResult() {
    return this.#r;
  }
  trackResult(e, t) {
    return new Proxy(e, { get: (e, n) => (this.trackProp(n), t?.(n), Reflect.get(e, n)) });
  }
  trackProp(e) {
    this.#p.add(e);
  }
  getCurrentQuery() {
    return this.#t;
  }
  refetch({ ...e } = {}) {
    return this.fetch({ ...e });
  }
  fetchOptimistic(e) {
    let t = this.#e.defaultQueryOptions(e),
      n = this.#e.getQueryCache().build(this.#e, t),
      r = () => {},
      i,
      a = new Promise((e) => {
        ((i = e),
          (r = this.#e.getQueryCache().subscribe((i) => {
            i.type === `updated` &&
              i.query.queryHash === n.queryHash &&
              n.state.data !== void 0 &&
              (r(), e(this.createResult(n, t)));
          })));
      });
    return Promise.race([
      n
        .fetch()
        .then(() => {
          let e = this.createResult(n, t);
          return (i?.(e), e);
        })
        .finally(() => {
          r();
        }),
      a,
    ]);
  }
  fetch(e) {
    return this.#m({ ...e, cancelRefetch: e.cancelRefetch ?? !0 }).then(
      () => (this.updateResult(), this.#r),
    );
  }
  #m(e) {
    this.#S();
    let t = this.#t.fetch(this.options, e);
    return (e?.throwOnError || (t = t.catch(O)), t);
  }
  #h(e) {
    return !me() && A(this.options.enabled, this.#t) !== !1 && te(e);
  }
  #g() {
    this.#b();
    let e = A(this.options.staleTime, this.#t);
    if (this.#r.isStale || !this.#h(e)) return;
    let t = k(this.#r.dataUpdatedAt, e) + 1;
    this.#u = T.setTimeout(() => {
      this.#r.isStale || this.updateResult();
    }, t);
  }
  #_() {
    return A(this.options.refetchInterval, this.#t) ?? !1;
  }
  #v(e) {
    (this.#x(),
      (this.#f = e),
      this.#f !== 0 &&
        this.#h(this.#f) &&
        (this.#d = T.setInterval(() => {
          (this.options.refetchIntervalInBackground || ge.isFocused()) && this.#m();
        }, this.#f)));
  }
  #y() {
    (this.#g(), this.#v(this.#_()));
  }
  #b() {
    this.#u !== void 0 && (T.clearTimeout(this.#u), (this.#u = void 0));
  }
  #x() {
    this.#d !== void 0 && (T.clearInterval(this.#d), (this.#d = void 0));
  }
  createResult(e, t) {
    let n = this.#t,
      r = this.options,
      i = this.#r,
      a = this.#i,
      o = this.#a,
      s = e === n ? this.#n : e.state,
      { state: c } = e,
      l = { ...c },
      u = !1,
      d;
    if (t._optimisticResults) {
      let i = this.hasListeners(),
        a = !i && Pe(e, t),
        o = i && Ie(e, n, t, r);
      ((a || o) && (l = { ...l, ...ke(c.data, e.options) }),
        t._optimisticResults === `isRestoring` && (l.fetchStatus = `idle`));
    }
    let { error: f, errorUpdatedAt: p, status: m } = l;
    d = l.data;
    let h = !1;
    if (t.placeholderData !== void 0 && d === void 0 && m === `pending`) {
      let e;
      (i?.isPlaceholderData && t.placeholderData === o?.placeholderData
        ? ((e = i.data), (h = !0))
        : (e =
            typeof t.placeholderData == `function`
              ? t.placeholderData(this.#l?.state.data, this.#l)
              : t.placeholderData),
        e !== void 0 && ((m = `success`), (d = ce(i?.data, e, t)), (u = !0)));
    }
    if (t.select && d !== void 0 && !h) {
      if (i && d === a?.data && t.select === this.#s) d = this.#c;
      else
        try {
          ((this.#s = t.select),
            (d = t.select(d)),
            (d = ce(i?.data, d, t)),
            (this.#c = d),
            (this.#o = null));
        } catch (e) {
          this.#o = e;
        }
    } else d === void 0 && (this.#o = null);
    this.#o && ((f = this.#o), (d = this.#c), (p = Date.now()), (m = `error`), (u = !1));
    let g = l.fetchStatus === `fetching`,
      _ = m === `pending`,
      v = m === `error`,
      y = _ && g,
      b = d !== void 0;
    return {
      status: m,
      fetchStatus: l.fetchStatus,
      isPending: _,
      isSuccess: m === `success`,
      isError: v,
      isInitialLoading: y,
      isLoading: y,
      data: d,
      dataUpdatedAt: l.dataUpdatedAt,
      error: f,
      errorUpdatedAt: p,
      failureCount: l.fetchFailureCount,
      failureReason: l.fetchFailureReason,
      errorUpdateCount: l.errorUpdateCount,
      isFetched: e.isFetched(),
      isFetchedAfterMount:
        l.dataUpdateCount > s.dataUpdateCount || l.errorUpdateCount > s.errorUpdateCount,
      isFetching: g,
      isRefetching: g && !_,
      isLoadingError: v && !b,
      isPaused: l.fetchStatus === `paused`,
      isPlaceholderData: u,
      isRefetchError: v && b,
      isStale: Le(e, t),
      refetch: this.refetch,
      isEnabled: A(t.enabled, e) !== !1,
    };
  }
  updateResult() {
    let e = this.#r,
      t = this.createResult(this.#t, this.options);
    if (
      ((this.#i = this.#t.state),
      (this.#a = this.options),
      this.#i.data !== void 0 && (this.#l = this.#t),
      I(t, e))
    )
      return;
    this.#r = t;
    let n = (() => {
      if (!e) return !0;
      let { notifyOnChangeProps: t } = this.options,
        n = typeof t == `function` ? t() : t;
      if (n === `all` || (!n && !this.#p.size)) return !0;
      let r = new Set(n ?? this.#p);
      return (
        this.options.throwOnError && r.add(`error`),
        Object.keys(this.#r).some((t) => {
          let n = t;
          return this.#r[n] !== e[n] && r.has(n);
        })
      );
    })();
    z.batch(() => {
      (n &&
        this.listeners.forEach((e) => {
          e(this.#r);
        }),
        this.#e.getQueryCache().notify({ query: this.#t, type: `observerResultsUpdated` }));
    });
  }
  #S() {
    let e = this.#e.getQueryCache().build(this.#e, this.options);
    if (e === this.#t) return;
    let t = this.#t;
    ((this.#t = e),
      (this.#n = e.state),
      this.hasListeners() && (t?.removeObserver(this), e.addObserver(this)));
  }
  onQueryUpdate() {
    (this.updateResult(), this.hasListeners() && this.#y());
  }
};
function Ne(e, t) {
  return (
    A(t.enabled, e) !== !1 &&
    e.state.data === void 0 &&
    (e.state.status !== `error` || A(t.retryOnMount, e) !== !1)
  );
}
function Pe(e, t) {
  return Ne(e, t) || (e.state.data !== void 0 && Fe(e, t, t.refetchOnMount));
}
function Fe(e, t, n) {
  if (A(t.enabled, e) !== !1 && A(t.staleTime, e) !== `static`) {
    let r = A(n, e);
    return r === `always` || (r !== !1 && Le(e, t));
  }
  return !1;
}
function Ie(e, t, n, r) {
  return (
    (e !== t || A(r.enabled, e) === !1) && (!n.suspense || e.state.status !== `error`) && Le(e, n)
  );
}
function Le(e, t) {
  return A(t.enabled, e) !== !1 && e.isStaleByTime(A(t.staleTime, e));
}
var Re = class extends we {
  #e;
  #t;
  #n;
  #r;
  constructor(e) {
    (super(),
      (this.#e = e.client),
      (this.mutationId = e.mutationId),
      (this.#n = e.mutationCache),
      (this.#t = []),
      (this.state = e.state || ze()),
      this.setOptions(e.options),
      this.scheduleGc());
  }
  setOptions(e) {
    ((this.options = e), this.updateGcTime(this.options.gcTime));
  }
  get meta() {
    return this.options.meta;
  }
  addObserver(e) {
    this.#t.includes(e) ||
      (this.#t.push(e),
      this.clearGcTimeout(),
      this.#n.notify({ type: `observerAdded`, mutation: this, observer: e }));
  }
  removeObserver(e) {
    ((this.#t = this.#t.filter((t) => t !== e)),
      this.scheduleGc(),
      this.#n.notify({ type: `observerRemoved`, mutation: this, observer: e }));
  }
  optionalRemove() {
    this.#t.length || (this.state.status === `pending` ? this.scheduleGc() : this.#n.remove(this));
  }
  continue() {
    return (
      this.#r?.continue() ??
      (this.state.status === `pending` ? this.execute(this.state.variables) : Promise.resolve())
    );
  }
  async execute(e) {
    let t = () => {
        this.#i({ type: `continue` });
      },
      n = { client: this.#e, meta: this.options.meta, mutationKey: this.options.mutationKey },
      r = (this.#r = Ce({
        fn: () =>
          this.options.mutationFn
            ? this.options.mutationFn(e, n)
            : Promise.reject(Error(`No mutationFn found`)),
        onFail: (e, t) => {
          this.#i({ type: `failed`, failureCount: e, error: t });
        },
        onPause: () => {
          this.#i({ type: `pause` });
        },
        onContinue: t,
        retry: this.options.retry ?? 0,
        retryDelay: this.options.retryDelay,
        networkMode: this.options.networkMode,
        canRun: () => this.#n.canRun(this),
      })),
      i = this.state.status === `pending`,
      a = !r.canStart();
    try {
      if (i) t();
      else {
        (this.#i({ type: `pending`, variables: e, isPaused: a }),
          this.#n.config.onMutate && (await this.#n.config.onMutate(e, this, n)));
        let t = await this.options.onMutate?.(e, n);
        t !== this.state.context &&
          this.#i({ type: `pending`, context: t, variables: e, isPaused: a });
      }
      let o = await r.start();
      return (
        await this.#n.config.onSuccess?.(o, e, this.state.context, this, n),
        await this.options.onSuccess?.(o, e, this.state.context, n),
        await this.#n.config.onSettled?.(
          o,
          null,
          this.state.variables,
          this.state.context,
          this,
          n,
        ),
        await this.options.onSettled?.(o, null, e, this.state.context, n),
        this.#i({ type: `success`, data: o }),
        o
      );
    } catch (t) {
      try {
        await this.#n.config.onError?.(t, e, this.state.context, this, n);
      } catch (e) {
        Promise.reject(e);
      }
      try {
        await this.options.onError?.(t, e, this.state.context, n);
      } catch (e) {
        Promise.reject(e);
      }
      try {
        await this.#n.config.onSettled?.(
          void 0,
          t,
          this.state.variables,
          this.state.context,
          this,
          n,
        );
      } catch (e) {
        Promise.reject(e);
      }
      try {
        await this.options.onSettled?.(void 0, t, e, this.state.context, n);
      } catch (e) {
        Promise.reject(e);
      }
      throw (this.#i({ type: `error`, error: t }), t);
    } finally {
      (this.#r === r && (this.#r = void 0), this.#n.runNext(this));
    }
  }
  #i(e) {
    let t = (t) => {
      switch (e.type) {
        case `failed`:
          return { ...t, failureCount: e.failureCount, failureReason: e.error };
        case `pause`:
          return { ...t, isPaused: !0 };
        case `continue`:
          return { ...t, isPaused: !1 };
        case `pending`:
          return {
            ...t,
            context: e.context,
            data: void 0,
            failureCount: 0,
            failureReason: null,
            error: null,
            isPaused: e.isPaused,
            status: `pending`,
            variables: e.variables,
            submittedAt: Date.now(),
          };
        case `success`:
          return {
            ...t,
            data: e.data,
            failureCount: 0,
            failureReason: null,
            error: null,
            status: `success`,
            isPaused: !1,
          };
        case `error`:
          return {
            ...t,
            data: void 0,
            error: e.error,
            failureCount: t.failureCount + 1,
            failureReason: e.error,
            isPaused: !1,
            status: `error`,
          };
      }
    };
    ((this.state = t(this.state)),
      z.batch(() => {
        (this.#t.forEach((t) => {
          t.onMutationUpdate(e);
        }),
          this.#n.notify({ mutation: this, type: `updated`, action: e }));
      }));
  }
};
function ze() {
  return {
    context: void 0,
    data: void 0,
    error: null,
    failureCount: 0,
    failureReason: null,
    isPaused: !1,
    status: `idle`,
    variables: void 0,
    submittedAt: 0,
  };
}
var Be = class extends he {
  #e;
  #t;
  #n;
  constructor(e = {}) {
    (super(), (this.config = e), (this.#e = new Set()), (this.#t = new Map()), (this.#n = 0));
  }
  build(e, t, n) {
    let r = new Re({
      client: e,
      mutationCache: this,
      mutationId: ++this.#n,
      options: e.defaultMutationOptions(t),
      state: n,
    });
    return (this.add(r), r);
  }
  add(e) {
    this.#e.add(e);
    let t = Ve(e);
    if (typeof t == `string`) {
      let n = this.#t.get(t);
      n ? n.push(e) : this.#t.set(t, [e]);
    }
    this.notify({ type: `added`, mutation: e });
  }
  remove(e) {
    if (this.#e.delete(e)) {
      let t = Ve(e);
      if (typeof t == `string`) {
        let n = this.#t.get(t);
        if (n) {
          if (n.length > 1) {
            let t = n.indexOf(e);
            t !== -1 && n.splice(t, 1);
          } else n[0] === e && this.#t.delete(t);
        }
      }
    }
    this.notify({ type: `removed`, mutation: e });
  }
  canRun(e) {
    let t = Ve(e);
    if (typeof t == `string`) {
      let n = this.#t.get(t)?.find((e) => e.state.status === `pending`);
      return !n || n === e;
    }
    return !0;
  }
  runNext(e) {
    let t = Ve(e);
    return typeof t == `string`
      ? (this.#t
          .get(t)
          ?.find((t) => t !== e && t.state.isPaused)
          ?.continue() ?? Promise.resolve())
      : Promise.resolve();
  }
  clear() {
    z.batch(() => {
      let e = this.getAll();
      (this.#e.clear(),
        this.#t.clear(),
        e.forEach((e) => {
          this.notify({ type: `removed`, mutation: e });
        }));
    });
  }
  getAll() {
    return Array.from(this.#e);
  }
  find(e) {
    let t = { exact: !0, ...e };
    return this.getAll().find((e) => ne(t, e));
  }
  findAll(e = {}) {
    return this.getAll().filter((t) => ne(e, t));
  }
  notify(e) {
    z.batch(() => {
      this.listeners.forEach((t) => {
        t(e);
      });
    });
  }
  resumePausedMutations() {
    let e = this.getAll().filter((e) => e.state.isPaused);
    return z.batch(() => Promise.all(e.map((e) => e.continue().catch(O))));
  }
};
function Ve(e) {
  return e.options.scope?.id;
}
var He = class extends he {
    #e;
    #t = void 0;
    #n;
    #r;
    constructor(e, t) {
      (super(), (this.#e = e), this.setOptions(t), this.bindMethods(), this.#i());
    }
    bindMethods() {
      ((this.mutate = this.mutate.bind(this)), (this.reset = this.reset.bind(this)));
    }
    setOptions(e) {
      let t = this.options;
      ((this.options = this.#e.defaultMutationOptions(e)),
        I(this.options, t) ||
          this.#e
            .getMutationCache()
            .notify({ type: `observerOptionsUpdated`, mutation: this.#n, observer: this }),
        t?.mutationKey &&
        this.options.mutationKey &&
        N(t.mutationKey) !== N(this.options.mutationKey)
          ? this.reset()
          : this.#n?.state.status === `pending` && this.#n.setOptions(this.options));
    }
    onSubscribe() {
      this.listeners.size === 1 && this.#n && (this.#n.addObserver(this), this.#i());
    }
    onUnsubscribe() {
      this.hasListeners() || this.#n?.removeObserver(this);
    }
    onMutationUpdate(e) {
      (this.#i(), this.#a(e));
    }
    getCurrentResult() {
      return this.#t;
    }
    reset() {
      (this.#n?.removeObserver(this), (this.#n = void 0), this.#i(), this.#a());
    }
    mutate(e, t) {
      return (
        (this.#r = t),
        this.#n?.removeObserver(this),
        (this.#n = this.#e.getMutationCache().build(this.#e, this.options)),
        this.#n.addObserver(this),
        this.#n.execute(e)
      );
    }
    #i() {
      let e = this.#n?.state ?? ze();
      this.#t = {
        ...e,
        isPending: e.status === `pending`,
        isSuccess: e.status === `success`,
        isError: e.status === `error`,
        isIdle: e.status === `idle`,
        mutate: this.mutate,
        reset: this.reset,
      };
    }
    #a(e) {
      z.batch(() => {
        let t = this.#r;
        if (t && this.hasListeners()) {
          let n = this.#t.variables,
            r = this.#t.context,
            i = { client: this.#e, meta: this.options.meta, mutationKey: this.options.mutationKey };
          if (e?.type === `success`) {
            try {
              t.onSuccess?.(e.data, n, r, i);
            } catch (e) {
              Promise.reject(e);
            }
            try {
              t.onSettled?.(e.data, null, n, r, i);
            } catch (e) {
              Promise.reject(e);
            }
          } else if (e?.type === `error`) {
            try {
              t.onError?.(e.error, n, r, i);
            } catch (e) {
              Promise.reject(e);
            }
            try {
              t.onSettled?.(void 0, e.error, n, r, i);
            } catch (e) {
              Promise.reject(e);
            }
          }
        }
        this.listeners.forEach((e) => {
          e(this.#t);
        });
      });
    }
  },
  Ue = class extends he {
    #e;
    constructor(e = {}) {
      (super(), (this.config = e), (this.#e = new Map()));
    }
    build(e, t, n) {
      let r = t.queryKey,
        i = t.queryHash ?? M(r, t),
        a = this.get(i);
      return (
        a ||
          ((a = new Oe({
            client: e,
            queryKey: r,
            queryHash: i,
            options: e.defaultQueryOptions(t),
            state: n,
            defaultOptions: e.getQueryDefaults(r),
          })),
          this.add(a)),
        a
      );
    }
    add(e) {
      this.#e.has(e.queryHash) ||
        (this.#e.set(e.queryHash, e), this.notify({ type: `added`, query: e }));
    }
    remove(e) {
      this.#e.get(e.queryHash) === e &&
        (e.destroy(), this.#e.delete(e.queryHash), this.notify({ type: `removed`, query: e }));
    }
    clear() {
      z.batch(() => {
        this.getAll().forEach((e) => {
          this.remove(e);
        });
      });
    }
    get(e) {
      return this.#e.get(e);
    }
    getAll() {
      return [...this.#e.values()];
    }
    find(e) {
      let t = { exact: !0, ...e };
      return this.getAll().find((e) => j(t, e));
    }
    findAll(e = {}) {
      let t = this.getAll();
      return Object.keys(e).length > 0 ? t.filter((t) => j(e, t)) : t;
    }
    notify(e) {
      z.batch(() => {
        this.listeners.forEach((t) => {
          t(e);
        });
      });
    }
    onFocus() {
      z.batch(() => {
        this.getAll().forEach((e) => {
          e.onFocus();
        });
      });
    }
    onOnline() {
      z.batch(() => {
        this.getAll().forEach((e) => {
          e.onOnline();
        });
      });
    }
  },
  We = class {
    #e;
    #t;
    #n;
    #r;
    #i;
    #a;
    #o;
    #s;
    constructor(e = {}) {
      ((this.#e = e.queryCache || new Ue()),
        (this.#t = e.mutationCache || new Be()),
        (this.#n = e.defaultOptions || {}),
        (this.#r = new Map()),
        (this.#i = new Map()),
        (this.#a = 0));
    }
    mount() {
      (this.#a++,
        this.#a === 1 &&
          ((this.#o = ge.subscribe(async (e) => {
            e && (await this.resumePausedMutations(), this.#e.onFocus());
          })),
          (this.#s = ye.subscribe(async (e) => {
            e && (await this.resumePausedMutations(), this.#e.onOnline());
          }))));
    }
    unmount() {
      (this.#a--,
        this.#a === 0 && (this.#o?.(), (this.#o = void 0), this.#s?.(), (this.#s = void 0)));
    }
    isFetching(e) {
      return this.#e.findAll({ ...e, fetchStatus: `fetching` }).length;
    }
    isMutating(e) {
      return this.#t.findAll({ ...e, status: `pending` }).length;
    }
    getQueryData(e) {
      let t = this.defaultQueryOptions({ queryKey: e });
      return this.#e.get(t.queryHash)?.state.data;
    }
    ensureQueryData(e) {
      let t = this.defaultQueryOptions(e),
        n = this.#e.build(this, t),
        r = n.state.data;
      return r === void 0
        ? this.fetchQuery(e)
        : (e.revalidateIfStale && n.isStaleByTime(A(t.staleTime, n)) && this.prefetchQuery(t),
          Promise.resolve(r));
    }
    getQueriesData(e) {
      return this.#e.findAll(e).map(({ queryKey: e, state: t }) => [e, t.data]);
    }
    setQueryData(e, t, n) {
      let r = this.defaultQueryOptions({ queryKey: e }),
        i = this.#e.get(r.queryHash)?.state.data,
        a = ee(t, i);
      if (a !== void 0) return this.#e.build(this, r).setData(a, { ...n, manual: !0 });
    }
    setQueriesData(e, t, n) {
      return z.batch(() =>
        this.#e.findAll(e).map(({ queryKey: e }) => [e, this.setQueryData(e, t, n)]),
      );
    }
    getQueryState(e) {
      let t = this.defaultQueryOptions({ queryKey: e });
      return this.#e.get(t.queryHash)?.state;
    }
    removeQueries(e) {
      let t = this.#e;
      z.batch(() => {
        t.findAll(e).forEach((e) => {
          t.remove(e);
        });
      });
    }
    resetQueries(e, t) {
      let n = this.#e;
      return z.batch(() => {
        let r = n.findAll(e),
          i = new Set(r);
        return (
          r.forEach((e) => {
            e.reset();
          }),
          this.refetchQueries({ type: `active`, predicate: (e) => i.has(e) }, t)
        );
      });
    }
    cancelQueries(e, t = {}) {
      let n = { revert: !0, ...t },
        r = z.batch(() => this.#e.findAll(e).map((e) => e.cancel(n)));
      return Promise.all(r).then(O).catch(O);
    }
    invalidateQueries(e, t = {}) {
      return z.batch(
        () => (
          this.#e.findAll(e).forEach((e) => {
            e.invalidate();
          }),
          e?.refetchType === `none`
            ? Promise.resolve()
            : this.refetchQueries({ ...e, type: e?.refetchType ?? e?.type ?? `active` }, t)
        ),
      );
    }
    refetchQueries(e, t = {}) {
      let n = { ...t, cancelRefetch: t.cancelRefetch ?? !0 },
        r = z.batch(() =>
          this.#e
            .findAll(e)
            .filter((e) => !e.isDisabled() && !e.isStatic())
            .map((e) => {
              let t = e.fetch(void 0, n);
              return (
                n.throwOnError || (t = t.catch(O)),
                e.state.fetchStatus === `paused` ? Promise.resolve() : t
              );
            }),
        );
      return Promise.all(r).then(O);
    }
    async query(e) {
      let t = this.defaultQueryOptions(e);
      t.retry === void 0 && (t.retry = !1);
      let n = this.#e.build(this, t),
        r = n.isStaleByTime(A(t.staleTime, n)) ? await n.fetch(t) : n.state.data,
        i = t.select;
      return i ? i(r) : r;
    }
    fetchQuery(e) {
      let t = this.defaultQueryOptions(e);
      t.retry === void 0 && (t.retry = !1);
      let n = this.#e.build(this, t);
      return n.isStaleByTime(A(t.staleTime, n)) ? n.fetch(t) : Promise.resolve(n.state.data);
    }
    prefetchQuery(e) {
      return this.fetchQuery(e).then(O).catch(O);
    }
    infiniteQuery(e) {
      return ((e._type = `infinite`), this.query(e));
    }
    fetchInfiniteQuery(e) {
      return ((e._type = `infinite`), this.fetchQuery(e));
    }
    prefetchInfiniteQuery(e) {
      return this.fetchInfiniteQuery(e).then(O).catch(O);
    }
    ensureInfiniteQueryData(e) {
      return ((e._type = `infinite`), this.ensureQueryData(e));
    }
    resumePausedMutations() {
      return ye.isOnline() ? this.#t.resumePausedMutations() : Promise.resolve();
    }
    getQueryCache() {
      return this.#e;
    }
    getMutationCache() {
      return this.#t;
    }
    getDefaultOptions() {
      return this.#n;
    }
    setDefaultOptions(e) {
      this.#n = e;
    }
    setQueryDefaults(e, t) {
      this.#r.set(N(e), { queryKey: e, defaultOptions: t });
    }
    getQueryDefaults(e) {
      let t = [...this.#r.values()],
        n = {};
      return (
        t.forEach((t) => {
          P(e, t.queryKey) && Object.assign(n, t.defaultOptions);
        }),
        n
      );
    }
    setMutationDefaults(e, t) {
      this.#i.set(N(e), { mutationKey: e, defaultOptions: t });
    }
    getMutationDefaults(e) {
      let t = [...this.#i.values()],
        n = {};
      return (
        t.forEach((t) => {
          P(e, t.mutationKey) && Object.assign(n, t.defaultOptions);
        }),
        n
      );
    }
    defaultQueryOptions(e) {
      if (e._defaulted) return e;
      let t = { ...this.#n.queries, ...this.getQueryDefaults(e.queryKey), ...e, _defaulted: !0 };
      return (
        (t.queryHash ||= M(t.queryKey, t)),
        t.refetchOnReconnect === void 0 && (t.refetchOnReconnect = t.networkMode !== `always`),
        t.throwOnError === void 0 && (t.throwOnError = !!t.suspense),
        !t.networkMode && t.persister && (t.networkMode = `offlineFirst`),
        t.queryFn === de && (t.enabled = !1),
        t
      );
    }
    defaultMutationOptions(e) {
      return e?._defaulted
        ? e
        : {
            ...this.#n.mutations,
            ...(e?.mutationKey && this.getMutationDefaults(e.mutationKey)),
            ...e,
            _defaulted: !0,
          };
    }
    clear() {
      (this.#e.clear(), this.#t.clear());
    }
  },
  Ge = y.createContext(!1),
  Ke = () => y.useContext(Ge);
Ge.Provider;
function qe() {
  let e = !1;
  return {
    clearReset: () => {
      e = !1;
    },
    reset: () => {
      e = !0;
    },
    isReset: () => e,
  };
}
var Je = y.createContext(qe()),
  Ye = () => y.useContext(Je),
  Xe = (e, t, n) => {
    let r =
      n?.state.status === `error` && typeof e.throwOnError == `function`
        ? pe(e.throwOnError, [n.state.error, n])
        : e.throwOnError;
    (e.suspense || r) && (t.isReset() || (e.retryOnMount = !1));
  },
  Ze = (e) => {
    y.useEffect(() => {
      e.clearReset();
    }, [e]);
  },
  Qe = ({ result: e, errorResetBoundary: t, throwOnError: n, query: r, suspense: i }) =>
    e.isError &&
    !t.isReset() &&
    !e.isFetching &&
    r &&
    ((i && e.data === void 0) || pe(n, [e.error, r])),
  $e = (e) => {
    if (e.suspense) {
      let t = 1e3,
        n = (e) => (e === `static` ? e : Math.max(e ?? t, t)),
        r = e.staleTime;
      ((e.staleTime = typeof r == `function` ? (...e) => n(r(...e)) : n(r)),
        typeof e.gcTime == `number` && (e.gcTime = Math.max(e.gcTime, t)));
    }
  },
  et = (e, t) => e?.suspense && t.isPending,
  tt = (e, t, n) =>
    t.fetchOptimistic(e).catch(() => {
      n.clearReset();
    });
function nt(e, t, n) {
  let r = Ke(),
    i = Ye(),
    a = S(n),
    o = a.defaultQueryOptions(e),
    s = a.getQueryCache().get(o.queryHash),
    c = e.subscribed !== !1;
  ((o._optimisticResults = r ? `isRestoring` : c ? `optimistic` : void 0),
    $e(o),
    Xe(o, i, s),
    Ze(i));
  let [l] = y.useState(() => new t(a, o)),
    u = l.getOptimisticResult(o),
    d = !r && c;
  if (
    (y.useSyncExternalStore(
      y.useCallback(
        (e) => {
          let t = d ? l.subscribe(z.batchCalls(e)) : O;
          return (l.updateResult(), t);
        },
        [l, d],
      ),
      () => l.getCurrentResult(),
      () => l.getCurrentResult(),
    ),
    y.useEffect(() => {
      l.setOptions(o);
    }, [o, l]),
    et(o, u))
  )
    throw tt(o, l, i);
  if (
    Qe({
      result: u,
      errorResetBoundary: i,
      throwOnError: o.throwOnError,
      query: s,
      suspense: o.suspense,
    })
  )
    throw u.error;
  return o.notifyOnChangeProps ? u : l.trackResult(u);
}
function rt(e, t) {
  return nt(e, Me, t);
}
function B(e, t) {
  let n = S(t),
    [r] = y.useState(() => new He(n, e));
  y.useEffect(() => {
    r.setOptions(e);
  }, [r, e]);
  let i = y.useSyncExternalStore(
      y.useCallback((e) => r.subscribe(z.batchCalls(e)), [r]),
      () => r.getCurrentResult(),
      () => r.getCurrentResult(),
    ),
    a = y.useCallback(
      (...e) => {
        r.mutate(e[0], e[1]).catch(O);
      },
      [r],
    );
  if (i.error && pe(r.options.throwOnError, [i.error])) throw i.error;
  return { ...i, mutate: a, mutateAsync: i.mutate };
}
var it = g();
structuredClone({
  scope: `local`,
  path: `/workspace/.native-whisperx/speakers`,
  library: {
    path: `/workspace/.native-whisperx/speakers/library.json`,
    status: `valid`,
    profileCount: 1,
  },
  profiles: [
    { id: `speaker-a`, label: `Speaker A`, metadata: { status: `confirmed`, note: `fixture` } },
  ],
  trace: {
    path: `/workspace/.native-whisperx/speakers/speaker-trace.json`,
    status: `valid`,
    scanRoot: `/workspace`,
    speakers: [
      {
        kind: `enrolled`,
        profileId: `speaker-a`,
        label: `Speaker A`,
        files: [
          {
            sourceFile: `/workspace/interview.json`,
            segmentCount: 2,
            totalDuration: 7.5,
            spans: [
              { startSeconds: 0, endSeconds: 2.4, snippet: `Welcome to the native-whisperx demo.` },
            ],
          },
        ],
      },
      { kind: `anonymous`, anonymousLabel: `speaker_1`, files: [] },
    ],
    errors: [],
  },
});
function at(e) {
  return typeof e == `object` && !!e && !Array.isArray(e);
}
var ot = (e) => typeof e == `string`,
  st = (e) => e === void 0 || ot(e),
  ct = (e) => typeof e == `number` && Number.isFinite(e),
  lt = (e) => ct(e) && Number.isSafeInteger(e) && e >= 0,
  ut = (e) => e === void 0 || ct(e),
  dt = (e) => e === `valid` || e === `missing` || e === `invalid`;
function ft(e) {
  return at(e) && Object.values(e).every(ot);
}
function pt(e) {
  return at(e) && ot(e.snippet) && ut(e.startSeconds) && ut(e.endSeconds);
}
function mt(e) {
  return (
    at(e) &&
    ot(e.sourceFile) &&
    lt(e.segmentCount) &&
    ct(e.totalDuration) &&
    Array.isArray(e.spans) &&
    e.spans.every(pt)
  );
}
function ht(e) {
  return (
    at(e) &&
    (e.kind === `enrolled` || e.kind === `anonymous`) &&
    st(e.profileId) &&
    st(e.label) &&
    st(e.anonymousLabel) &&
    Array.isArray(e.files) &&
    e.files.every(mt)
  );
}
function gt(e) {
  return at(e) && ot(e.id) && ot(e.label) && ft(e.metadata);
}
function _t(e) {
  return at(e) && ot(e.sourceFile) && ot(e.message);
}
function vt(e) {
  return !at(e) || !at(e.library) || !at(e.trace)
    ? !1
    : (e.scope === `local` || e.scope === `global` || e.scope === `explicit`) &&
        ot(e.path) &&
        ot(e.library.path) &&
        dt(e.library.status) &&
        lt(e.library.profileCount) &&
        st(e.library.message) &&
        Array.isArray(e.profiles) &&
        e.profiles.every(gt) &&
        ot(e.trace.path) &&
        dt(e.trace.status) &&
        st(e.trace.scanRoot) &&
        st(e.trace.message) &&
        Array.isArray(e.trace.speakers) &&
        e.trace.speakers.every(ht) &&
        Array.isArray(e.trace.errors) &&
        e.trace.errors.every(_t);
}
function yt(e) {
  if (!vt(e)) throw Error(`Invalid Speaker Directory response.`);
  return e;
}
var bt = `X-Native-Whisperx-Session-Token`;
async function xt(e, t) {
  let n = await fetch(e, {
    ...t,
    signal: t?.signal ?? AbortSignal.timeout(3e4),
    headers: {
      ...(t?.body ? { "Content-Type": `application/json` } : {}),
      ...(window.nativeWhisperxSessionToken ? { [bt]: window.nativeWhisperxSessionToken } : {}),
      ...t?.headers,
    },
  });
  if (!n.ok) throw Error(await n.text());
  return await n.json();
}
var St = {
    getState: async () => yt(await xt(`/api/state`)),
    updateProfile: async (e, t) => (
      await xt(`/api/profiles/${encodeURIComponent(e)}`, {
        method: `PUT`,
        body: JSON.stringify(t),
      }),
      St.getState()
    ),
    deleteProfile: async (e) => (
      await xt(`/api/profiles/${encodeURIComponent(e)}`, { method: `DELETE` }),
      St.getState()
    ),
    createProfile: () => xt(`/api/profiles`, { method: `POST`, body: JSON.stringify({}) }),
    rebuildTrace: (e) =>
      xt(`/api/trace/rebuild`, { method: `POST`, body: JSON.stringify(e.scanRoot ? e : {}) }),
  },
  Ct = St,
  wt =
    `af.am.ar.as.az.ba.be.bg.bn.bo.br.bs.ca.cs.cy.da.de.el.en.es.et.eu.fa.fi.fo.fr.gl.gu.ha.haw.he.hi.hr.ht.hu.hy.id.is.it.ja.jw.ka.kk.km.kn.ko.la.lb.ln.lo.lt.lv.mg.mi.mk.ml.mn.mr.ms.mt.my.ne.nl.nn.no.oc.pa.pl.ps.pt.ro.ru.sa.sd.si.sk.sl.sn.so.sq.sr.su.sv.sw.ta.te.tg.th.tk.tl.tr.tt.uk.ur.uz.vi.yi.yo.zh`.split(
      `.`,
    ),
  Tt = [`en`, `de`, `fr`, `es`, `it`, `pt`, `nl`, `pl`],
  Et = new Intl.DisplayNames([`en`], { type: `language` });
function Dt(e) {
  try {
    return `${Et.of(e) ?? e} · ${e}`;
  } catch {
    return e;
  }
}
function Ot(e) {
  return e
    .map((e) => ({ code: e, label: Dt(e) }))
    .sort((e, t) => e.label.localeCompare(t.label, `en`));
}
var kt = Uint8Array,
  At = Uint16Array,
  jt = Int32Array,
  Mt = new kt([
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0, 0, 0, 0,
  ]),
  Nt = new kt([
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13, 0, 0,
  ]),
  Pt = new kt([16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15]),
  Ft = function (e, t) {
    for (var n = new At(31), r = 0; r < 31; ++r) n[r] = t += 1 << e[r - 1];
    for (var i = new jt(n[30]), r = 1; r < 30; ++r)
      for (var a = n[r]; a < n[r + 1]; ++a) i[a] = ((a - n[r]) << 5) | r;
    return { b: n, r: i };
  },
  It = Ft(Mt, 2),
  Lt = It.b,
  Rt = It.r;
((Lt[28] = 258), (Rt[258] = 28));
var zt = Ft(Nt, 0),
  Bt = zt.b,
  Vt = zt.r,
  Ht = new At(32768);
for (V = 0; V < 32768; ++V)
  ((Ut = ((V & 43690) >> 1) | ((V & 21845) << 1)),
    (Ut = ((Ut & 52428) >> 2) | ((Ut & 13107) << 2)),
    (Ut = ((Ut & 61680) >> 4) | ((Ut & 3855) << 4)),
    (Ht[V] = (((Ut & 65280) >> 8) | ((Ut & 255) << 8)) >> 1));
var Ut,
  V,
  Wt = function (e, t, n) {
    for (var r = e.length, i = 0, a = new At(t); i < r; ++i) e[i] && ++a[e[i] - 1];
    var o = new At(t);
    for (i = 1; i < t; ++i) o[i] = (o[i - 1] + a[i - 1]) << 1;
    var s;
    if (n) {
      s = new At(1 << t);
      var c = 15 - t;
      for (i = 0; i < r; ++i)
        if (e[i])
          for (
            var l = (i << 4) | e[i], u = t - e[i], d = o[e[i] - 1]++ << u, f = d | ((1 << u) - 1);
            d <= f;
            ++d
          )
            s[Ht[d] >> c] = l;
    } else for (s = new At(r), i = 0; i < r; ++i) e[i] && (s[i] = Ht[o[e[i] - 1]++] >> (15 - e[i]));
    return s;
  },
  Gt = new kt(288);
for (V = 0; V < 144; ++V) Gt[V] = 8;
for (var V = 144; V < 256; ++V) Gt[V] = 9;
for (var V = 256; V < 280; ++V) Gt[V] = 7;
for (var V = 280; V < 288; ++V) Gt[V] = 8;
var V,
  Kt = new kt(32);
for (V = 0; V < 32; ++V) Kt[V] = 5;
var V,
  qt = Wt(Gt, 9, 0),
  Jt = Wt(Gt, 9, 1),
  Yt = Wt(Kt, 5, 0),
  Xt = Wt(Kt, 5, 1),
  Zt = function (e) {
    for (var t = e[0], n = 1; n < e.length; ++n) e[n] > t && (t = e[n]);
    return t;
  },
  Qt = function (e, t, n) {
    var r = (t / 8) | 0;
    return ((e[r] | (e[r + 1] << 8)) >> (t & 7)) & n;
  },
  $t = function (e, t) {
    var n = (t / 8) | 0;
    return (e[n] | (e[n + 1] << 8) | (e[n + 2] << 16)) >> (t & 7);
  },
  en = function (e) {
    return ((e + 7) / 8) | 0;
  },
  tn = function (e, t, n) {
    return (
      (t == null || t < 0) && (t = 0),
      (n == null || n > e.length) && (n = e.length),
      new kt(e.subarray(t, n))
    );
  },
  nn = [
    `unexpected EOF`,
    `invalid block type`,
    `invalid length/literal`,
    `invalid distance`,
    `stream finished`,
    `no stream handler`,
    ,
    `no callback`,
    `invalid UTF-8 data`,
    `extra field too long`,
    `date not in range 1980-2099`,
    `filename too long`,
    `stream finishing`,
    `invalid zip data`,
  ],
  rn = function (e, t, n) {
    var r = Error(t || nn[e]);
    if (((r.code = e), Error.captureStackTrace && Error.captureStackTrace(r, rn), !n)) throw r;
    return r;
  },
  an = function (e, t, n, r) {
    var i = e.length,
      a = r ? r.length : 0;
    if (!i || (t.f && !t.l)) return n || new kt(0);
    var o = !n,
      s = o || t.i != 2,
      c = t.i;
    o && (n = new kt(i * 3));
    var l = function (e) {
        var t = n.length;
        if (e > t) {
          var r = new kt(Math.max(t * 2, e));
          (r.set(n), (n = r));
        }
      },
      u = t.f || 0,
      d = t.p || 0,
      f = t.b || 0,
      { l: p, d: m, m: h, n: g } = t,
      _ = i * 8;
    do {
      if (!p) {
        u = Qt(e, d, 1);
        var v = Qt(e, d + 1, 3);
        if (((d += 3), !v)) {
          var y = en(d) + 4,
            b = e[y - 4] | (e[y - 3] << 8),
            x = y + b;
          if (x > i) {
            c && rn(0);
            break;
          }
          (s && l(f + b), n.set(e.subarray(y, x), f), (t.b = f += b), (t.p = d = x * 8), (t.f = u));
          continue;
        }
        if (v == 1) ((p = Jt), (m = Xt), (h = 9), (g = 5));
        else if (v == 2) {
          var S = Qt(e, d, 31) + 257,
            C = Qt(e, d + 10, 15) + 4,
            w = S + Qt(e, d + 5, 31) + 1;
          d += 14;
          for (var T = new kt(w), E = new kt(19), D = 0; D < C; ++D) E[Pt[D]] = Qt(e, d + D * 3, 7);
          d += C * 3;
          for (var O = Zt(E), ee = (1 << O) - 1, te = Wt(E, O, 1), D = 0; D < w;) {
            var k = te[Qt(e, d, ee)];
            d += k & 15;
            var y = k >> 4;
            if (y < 16) T[D++] = y;
            else {
              var A = 0,
                j = 0;
              for (
                y == 16
                  ? ((j = 3 + Qt(e, d, 3)), (d += 2), (A = T[D - 1]))
                  : y == 17
                    ? ((j = 3 + Qt(e, d, 7)), (d += 3))
                    : y == 18 && ((j = 11 + Qt(e, d, 127)), (d += 7));
                j--;
              )
                T[D++] = A;
            }
          }
          var ne = T.subarray(0, S),
            M = T.subarray(S);
          ((h = Zt(ne)), (g = Zt(M)), (p = Wt(ne, h, 1)), (m = Wt(M, g, 1)));
        } else rn(1);
        if (d > _) {
          c && rn(0);
          break;
        }
      }
      s && l(f + 131072);
      for (var N = (1 << h) - 1, P = (1 << g) - 1, re = d; ; re = d) {
        var A = p[$t(e, d) & N],
          F = A >> 4;
        if (((d += A & 15), d > _)) {
          c && rn(0);
          break;
        }
        if ((A || rn(2), F < 256)) n[f++] = F;
        else if (F == 256) {
          ((re = d), (p = null));
          break;
        } else {
          var I = F - 254;
          if (F > 264) {
            var D = F - 257,
              ie = Mt[D];
            ((I = Qt(e, d, (1 << ie) - 1) + Lt[D]), (d += ie));
          }
          var ae = m[$t(e, d) & P],
            oe = ae >> 4;
          (ae || rn(3), (d += ae & 15));
          var M = Bt[oe];
          if (oe > 3) {
            var ie = Nt[oe];
            ((M += $t(e, d) & ((1 << ie) - 1)), (d += ie));
          }
          if (d > _) {
            c && rn(0);
            break;
          }
          s && l(f + 131072);
          var se = f + I;
          if (f < M) {
            var ce = a - M,
              le = Math.min(M, se);
            for (ce + f < 0 && rn(3); f < le; ++f) n[f] = r[ce + f];
          }
          for (; f < se; ++f) n[f] = n[f - M];
        }
      }
      ((t.l = p),
        (t.p = re),
        (t.b = f),
        (t.f = u),
        p && ((u = 1), (t.m = h), (t.d = m), (t.n = g)));
    } while (!u);
    return f != n.length && o ? tn(n, 0, f) : n.subarray(0, f);
  },
  on = function (e, t, n) {
    n <<= t & 7;
    var r = (t / 8) | 0;
    ((e[r] |= n), (e[r + 1] |= n >> 8));
  },
  sn = function (e, t, n) {
    n <<= t & 7;
    var r = (t / 8) | 0;
    ((e[r] |= n), (e[r + 1] |= n >> 8), (e[r + 2] |= n >> 16));
  },
  cn = function (e, t) {
    for (var n = [], r = 0; r < e.length; ++r) e[r] && n.push({ s: r, f: e[r] });
    var i = n.length,
      a = n.slice();
    if (!i) return { t: hn, l: 0 };
    if (i == 1) {
      var o = new kt(n[0].s + 1);
      return ((o[n[0].s] = 1), { t: o, l: 1 });
    }
    (n.sort(function (e, t) {
      return e.f - t.f;
    }),
      n.push({ s: -1, f: 25001 }));
    var s = n[0],
      c = n[1],
      l = 0,
      u = 1,
      d = 2;
    for (n[0] = { s: -1, f: s.f + c.f, l: s, r: c }; u != i - 1;)
      ((s = n[n[l].f < n[d].f ? l++ : d++]),
        (c = n[l != u && n[l].f < n[d].f ? l++ : d++]),
        (n[u++] = { s: -1, f: s.f + c.f, l: s, r: c }));
    for (var f = a[0].s, r = 1; r < i; ++r) a[r].s > f && (f = a[r].s);
    var p = new At(f + 1),
      m = ln(n[u - 1], p, 0);
    if (m > t) {
      var r = 0,
        h = 0,
        g = m - t,
        _ = 1 << g;
      for (
        a.sort(function (e, t) {
          return p[t.s] - p[e.s] || e.f - t.f;
        });
        r < i;
        ++r
      ) {
        var v = a[r].s;
        if (p[v] > t) ((h += _ - (1 << (m - p[v]))), (p[v] = t));
        else break;
      }
      for (h >>= g; h > 0;) {
        var y = a[r].s;
        p[y] < t ? (h -= 1 << (t - p[y]++ - 1)) : ++r;
      }
      for (; r >= 0 && h; --r) {
        var b = a[r].s;
        p[b] == t && (--p[b], ++h);
      }
      m = t;
    }
    return { t: new kt(p), l: m };
  },
  ln = function (e, t, n) {
    return e.s == -1 ? Math.max(ln(e.l, t, n + 1), ln(e.r, t, n + 1)) : (t[e.s] = n);
  },
  un = function (e) {
    for (var t = e.length; t && !e[--t];);
    for (
      var n = new At(++t),
        r = 0,
        i = e[0],
        a = 1,
        o = function (e) {
          n[r++] = e;
        },
        s = 1;
      s <= t;
      ++s
    )
      if (e[s] == i && s != t) ++a;
      else {
        if (!i && a > 2) {
          for (; a > 138; a -= 138) o(32754);
          a > 2 && (o(a > 10 ? ((a - 11) << 5) | 28690 : ((a - 3) << 5) | 12305), (a = 0));
        } else if (a > 3) {
          for (o(i), --a; a > 6; a -= 6) o(8304);
          a > 2 && (o(((a - 3) << 5) | 8208), (a = 0));
        }
        for (; a--;) o(i);
        ((a = 1), (i = e[s]));
      }
    return { c: n.subarray(0, r), n: t };
  },
  dn = function (e, t) {
    for (var n = 0, r = 0; r < t.length; ++r) n += e[r] * t[r];
    return n;
  },
  fn = function (e, t, n) {
    var r = n.length,
      i = en(t + 2);
    ((e[i] = r & 255), (e[i + 1] = r >> 8), (e[i + 2] = e[i] ^ 255), (e[i + 3] = e[i + 1] ^ 255));
    for (var a = 0; a < r; ++a) e[i + a + 4] = n[a];
    return (i + 4 + r) * 8;
  },
  pn = function (e, t, n, r, i, a, o, s, c, l, u) {
    (on(t, u++, n), ++i[256]);
    for (
      var { t: d, l: f } = cn(i, 15),
        { t: p, l: m } = cn(a, 15),
        { c: h, n: g } = un(d),
        { c: _, n: v } = un(p),
        y = new At(19),
        b = 0;
      b < h.length;
      ++b
    )
      ++y[h[b] & 31];
    for (var b = 0; b < _.length; ++b) ++y[_[b] & 31];
    for (var { t: x, l: S } = cn(y, 7), C = 19; C > 4 && !x[Pt[C - 1]]; --C);
    var w = (l + 5) << 3,
      T = dn(i, Gt) + dn(a, Kt) + o,
      E = dn(i, d) + dn(a, p) + o + 14 + 3 * C + dn(y, x) + 2 * y[16] + 3 * y[17] + 7 * y[18];
    if (c >= 0 && w <= T && w <= E) return fn(t, u, e.subarray(c, c + l));
    var D, O, ee, te;
    if ((on(t, u, 1 + (E < T)), (u += 2), E < T)) {
      ((D = Wt(d, f, 0)), (O = d), (ee = Wt(p, m, 0)), (te = p));
      var k = Wt(x, S, 0);
      (on(t, u, g - 257), on(t, u + 5, v - 1), on(t, u + 10, C - 4), (u += 14));
      for (var b = 0; b < C; ++b) on(t, u + 3 * b, x[Pt[b]]);
      u += 3 * C;
      for (var A = [h, _], j = 0; j < 2; ++j)
        for (var ne = A[j], b = 0; b < ne.length; ++b) {
          var M = ne[b] & 31;
          (on(t, u, k[M]),
            (u += x[M]),
            M > 15 && (on(t, u, (ne[b] >> 5) & 127), (u += ne[b] >> 12)));
        }
    } else ((D = qt), (O = Gt), (ee = Yt), (te = Kt));
    for (var b = 0; b < s; ++b) {
      var N = r[b];
      if (N > 255) {
        var M = (N >> 18) & 31;
        (sn(t, u, D[M + 257]),
          (u += O[M + 257]),
          M > 7 && (on(t, u, (N >> 23) & 31), (u += Mt[M])));
        var P = N & 31;
        (sn(t, u, ee[P]), (u += te[P]), P > 3 && (sn(t, u, (N >> 5) & 8191), (u += Nt[P])));
      } else (sn(t, u, D[N]), (u += O[N]));
    }
    return (sn(t, u, D[256]), u + O[256]);
  },
  mn = new jt([65540, 131080, 131088, 131104, 262176, 1048704, 1048832, 2114560, 2117632]),
  hn = new kt(0),
  gn = function (e, t, n, r, i, a) {
    var o = a.z || e.length,
      s = new kt(r + o + 5 * (1 + Math.ceil(o / 7e3)) + i),
      c = s.subarray(r, s.length - i),
      l = a.l,
      u = (a.r || 0) & 7;
    if (t) {
      u && (c[0] = a.r >> 3);
      for (
        var d = mn[t - 1],
          f = d >> 13,
          p = d & 8191,
          m = (1 << n) - 1,
          h = a.p || new At(32768),
          g = a.h || new At(m + 1),
          _ = Math.ceil(n / 3),
          v = 2 * _,
          y = function (t) {
            return (e[t] ^ (e[t + 1] << _) ^ (e[t + 2] << v)) & m;
          },
          b = new jt(25e3),
          x = new At(288),
          S = new At(32),
          C = 0,
          w = 0,
          T = a.i || 0,
          E = 0,
          D = a.w || 0,
          O = 0;
        T + 2 < o;
        ++T
      ) {
        var ee = y(T),
          te = T & 32767,
          k = g[ee];
        if (((h[te] = k), (g[ee] = te), D <= T)) {
          var A = o - T;
          if ((C > 7e3 || E > 24576) && (A > 423 || !l)) {
            ((u = pn(e, c, 0, b, x, S, w, E, O, T - O, u)), (E = C = w = 0), (O = T));
            for (var j = 0; j < 286; ++j) x[j] = 0;
            for (var j = 0; j < 30; ++j) S[j] = 0;
          }
          var ne = 2,
            M = 0,
            N = p,
            P = (te - k) & 32767;
          if (A > 2 && ee == y(T - P))
            for (
              var re = Math.min(f, A) - 1, F = Math.min(32767, T), I = Math.min(258, A);
              P <= F && --N && te != k;
            ) {
              if (e[T + ne] == e[T + ne - P]) {
                for (var ie = 0; ie < I && e[T + ie] == e[T + ie - P]; ++ie);
                if (ie > ne) {
                  if (((ne = ie), (M = P), ie > re)) break;
                  for (var ae = Math.min(P, ie - 2), oe = 0, j = 0; j < ae; ++j) {
                    var se = (T - P + j) & 32767,
                      ce = (se - h[se]) & 32767;
                    ce > oe && ((oe = ce), (k = se));
                  }
                }
              }
              ((te = k), (k = h[te]), (P += (te - k) & 32767));
            }
          if (M) {
            b[E++] = 268435456 | (Rt[ne] << 18) | Vt[M];
            var le = Rt[ne] & 31,
              ue = Vt[M] & 31;
            ((w += Mt[le] + Nt[ue]), ++x[257 + le], ++S[ue], (D = T + ne), ++C);
          } else ((b[E++] = e[T]), ++x[e[T]]);
        }
      }
      for (T = Math.max(T, D); T < o; ++T) ((b[E++] = e[T]), ++x[e[T]]);
      ((u = pn(e, c, l, b, x, S, w, E, O, T - O, u)),
        l ||
          ((a.r = (u & 7) | (c[(u / 8) | 0] << 3)),
          (u -= 7),
          (a.h = g),
          (a.p = h),
          (a.i = T),
          (a.w = D)));
    } else {
      for (var T = a.w || 0; T < o + l; T += 65535) {
        var de = T + 65535;
        (de >= o && ((c[(u / 8) | 0] = l), (de = o)), (u = fn(c, u + 1, e.subarray(T, de))));
      }
      a.i = o;
    }
    return tn(s, 0, r + en(u) + i);
  },
  _n = (function () {
    for (var e = new Int32Array(256), t = 0; t < 256; ++t) {
      for (var n = t, r = 9; --r;) n = (n & 1 && -306674912) ^ (n >>> 1);
      e[t] = n;
    }
    return e;
  })(),
  vn = function () {
    var e = -1;
    return {
      p: function (t) {
        for (var n = e, r = 0; r < t.length; ++r) n = _n[(n & 255) ^ t[r]] ^ (n >>> 8);
        e = n;
      },
      d: function () {
        return ~e;
      },
    };
  },
  yn = function (e, t, n, r, i) {
    if (!i && ((i = { l: 1 }), t.dictionary)) {
      var a = t.dictionary.subarray(-32768),
        o = new kt(a.length + e.length);
      (o.set(a), o.set(e, a.length), (e = o), (i.w = a.length));
    }
    return gn(
      e,
      t.level == null ? 6 : t.level,
      t.mem == null
        ? i.l
          ? Math.ceil(Math.max(8, Math.min(13, Math.log(e.length))) * 1.5)
          : 20
        : 12 + t.mem,
      n,
      r,
      i,
    );
  },
  bn = function (e, t) {
    var n = {};
    for (var r in e) n[r] = e[r];
    for (var r in t) n[r] = t[r];
    return n;
  },
  xn = function (e, t) {
    return e[t] | (e[t + 1] << 8);
  },
  Sn = function (e, t) {
    return (e[t] | (e[t + 1] << 8) | (e[t + 2] << 16) | (e[t + 3] << 24)) >>> 0;
  },
  Cn = function (e, t) {
    return Sn(e, t) + Sn(e, t + 4) * 4294967296;
  },
  wn = function (e, t, n) {
    for (; n; ++t) ((e[t] = n), (n >>>= 8));
  },
  Tn = function (e) {
    (e[0] != 31 || e[1] != 139 || e[2] != 8) && rn(6, `invalid gzip data`);
    var t = e[3],
      n = 10;
    t & 4 && (n += (e[10] | (e[11] << 8)) + 2);
    for (var r = ((t >> 3) & 1) + ((t >> 4) & 1); r > 0; r -= !e[n++]);
    return n + (t & 2);
  },
  En = function (e) {
    var t = e.length;
    return (e[t - 4] | (e[t - 3] << 8) | (e[t - 2] << 16) | (e[t - 1] << 24)) >>> 0;
  };
function Dn(e, t) {
  return yn(e, t || {}, 0, 0);
}
function On(e, t) {
  return an(e, { i: 2 }, t && t.out, t && t.dictionary);
}
function kn(e, t) {
  var n = Tn(e);
  return (
    n + 8 > e.length && rn(6, `invalid gzip data`),
    an(e.subarray(n, -8), { i: 2 }, (t && t.out) || new kt(En(e)), t && t.dictionary)
  );
}
var An = function (e, t, n, r) {
    for (var i in e) {
      var a = e[i],
        o = t + i,
        s = r;
      (Array.isArray(a) && ((s = bn(r, a[1])), (a = a[0])),
        ArrayBuffer.isView(a)
          ? (n[o] = [a, s])
          : ((n[(o += `/`)] = [new kt(0), s]), An(a, o, n, r)));
    }
  },
  jn = typeof TextEncoder < `u` && new TextEncoder(),
  Mn = typeof TextDecoder < `u` && new TextDecoder();
try {
  Mn.decode(hn, { stream: !0 });
} catch {}
var Nn = function (e) {
  for (var t = ``, n = 0; ;) {
    var r = e[n++],
      i = (r > 127) + (r > 223) + (r > 239);
    if (n + i > e.length) return { s: t, r: tn(e, n - 1) };
    i
      ? i == 3
        ? ((r =
            (((r & 15) << 18) | ((e[n++] & 63) << 12) | ((e[n++] & 63) << 6) | (e[n++] & 63)) -
            65536),
          (t += String.fromCharCode(55296 | (r >> 10), 56320 | (r & 1023))))
        : i & 1
          ? (t += String.fromCharCode(((r & 31) << 6) | (e[n++] & 63)))
          : (t += String.fromCharCode(((r & 15) << 12) | ((e[n++] & 63) << 6) | (e[n++] & 63)))
      : (t += String.fromCharCode(r));
  }
};
function Pn(e, t) {
  if (t) {
    for (var n = new kt(e.length), r = 0; r < e.length; ++r) n[r] = e.charCodeAt(r);
    return n;
  }
  if (jn) return jn.encode(e);
  for (
    var i = e.length,
      a = new kt(e.length + (e.length >> 1)),
      o = 0,
      s = function (e) {
        a[o++] = e;
      },
      r = 0;
    r < i;
    ++r
  ) {
    if (o + 5 > a.length) {
      var c = new kt(o + 8 + ((i - r) << 1));
      (c.set(a), (a = c));
    }
    var l = e.charCodeAt(r);
    l < 128 || t
      ? s(l)
      : l < 2048
        ? (s(192 | (l >> 6)), s(128 | (l & 63)))
        : l > 55295 && l < 57344
          ? ((l = (65536 + (l & 1047552)) | (e.charCodeAt(++r) & 1023)),
            s(240 | (l >> 18)),
            s(128 | ((l >> 12) & 63)),
            s(128 | ((l >> 6) & 63)),
            s(128 | (l & 63)))
          : (s(224 | (l >> 12)), s(128 | ((l >> 6) & 63)), s(128 | (l & 63)));
  }
  return tn(a, 0, o);
}
function Fn(e, t) {
  if (t) {
    for (var n = ``, r = 0; r < e.length; r += 16384)
      n += String.fromCharCode.apply(null, e.subarray(r, r + 16384));
    return n;
  }
  if (Mn) return Mn.decode(e);
  var { s: i, r: n } = Nn(e);
  return (n.length && rn(8), i);
}
var In = function (e, t) {
    return t + 30 + xn(e, t + 26) + xn(e, t + 28);
  },
  Ln = function (e, t, n) {
    var r = xn(e, t + 28),
      i = xn(e, t + 30),
      a = Fn(e.subarray(t + 46, t + 46 + r), !(xn(e, t + 8) & 2048)),
      o = t + 46 + r,
      s = Rn(e, o, i, n, Sn(e, t + 20), Sn(e, t + 24), Sn(e, t + 42)),
      c = s[0],
      l = s[1],
      u = s[2];
    return [xn(e, t + 10), c, l, a, o + i + xn(e, t + 32), u];
  },
  Rn = function (e, t, n, r, i, a, o) {
    var s = i == 4294967295,
      c = a == 4294967295,
      l = o == 4294967295,
      u = t + n,
      d = s + c + l;
    if (r && d) {
      for (; t + 4 < u; t += 4 + xn(e, t + 2))
        if (xn(e, t) == 1)
          return [
            s ? Cn(e, t + 4 + 8 * c) : i,
            c ? Cn(e, t + 4) : a,
            l ? Cn(e, t + 4 + 8 * (c + s)) : o,
            1,
          ];
      r < 2 && rn(13);
    }
    return [i, a, o, 0];
  },
  zn = function (e) {
    var t = 0;
    if (e)
      for (var n in e) {
        var r = e[n].length;
        (r > 65535 && rn(9), (t += r + 4));
      }
    return t;
  },
  Bn = function (e, t, n, r, i, a, o, s) {
    var c = r.length,
      l = n.extra,
      u = s && s.length,
      d = zn(l);
    (wn(e, t, o == null ? 67324752 : 33639248),
      (t += 4),
      o != null && ((e[t++] = 20), (e[t++] = n.os)),
      (e[t] = 20),
      (t += 2),
      (e[t++] = (n.flag << 1) | (a < 0 && 8)),
      (e[t++] = i && 8),
      (e[t++] = n.compression & 255),
      (e[t++] = n.compression >> 8));
    var f = new Date(n.mtime == null ? Date.now() : n.mtime),
      p = f.getFullYear() - 1980;
    if (
      ((p < 0 || p > 119) && rn(10),
      wn(
        e,
        t,
        (p << 25) |
          ((f.getMonth() + 1) << 21) |
          (f.getDate() << 16) |
          (f.getHours() << 11) |
          (f.getMinutes() << 5) |
          (f.getSeconds() >> 1),
      ),
      (t += 4),
      a != -1 && (wn(e, t, n.crc), wn(e, t + 4, a < 0 ? -a - 2 : a), wn(e, t + 8, n.size)),
      wn(e, t + 12, c),
      wn(e, t + 14, d),
      (t += 16),
      o != null && (wn(e, t, u), wn(e, t + 6, n.attrs), wn(e, t + 10, o), (t += 14)),
      e.set(r, t),
      (t += c),
      d)
    )
      for (var m in l) {
        var h = l[m],
          g = h.length;
        (wn(e, t, +m), wn(e, t + 2, g), e.set(h, t + 4), (t += 4 + g));
      }
    return (u && (e.set(s, t), (t += u)), t);
  },
  Vn = function (e, t, n, r, i) {
    (wn(e, t, 101010256), wn(e, t + 8, n), wn(e, t + 10, n), wn(e, t + 12, r), wn(e, t + 16, i));
  };
function Hn(e, t) {
  t ||= {};
  var n = {},
    r = [];
  An(e, ``, n, t);
  var i = 0,
    a = 0;
  for (var o in n) {
    var s = n[o],
      c = s[0],
      l = s[1],
      u = l.level == 0 ? 0 : 8,
      d = Pn(o),
      f = d.length,
      p = l.comment,
      m = p && Pn(p),
      h = m && m.length,
      g = zn(l.extra);
    f > 65535 && rn(11);
    var _ = u ? Dn(c, l) : c,
      v = _.length,
      y = vn();
    (y.p(c),
      r.push(
        bn(l, {
          size: c.length,
          crc: y.d(),
          c: _,
          f: d,
          m,
          u: f != o.length || (m && p.length != h),
          o: i,
          compression: u,
        }),
      ),
      (i += 30 + f + g + v),
      (a += 76 + 2 * (f + g) + (h || 0) + v));
  }
  for (var b = new kt(a + 22), x = i, S = a - i, C = 0; C < r.length; ++C) {
    var d = r[C];
    Bn(b, d.o, d, d.f, d.u, d.c.length);
    var w = 30 + d.f.length + zn(d.extra);
    (b.set(d.c, d.o + w),
      Bn(b, i, d, d.f, d.u, d.c.length, d.o, d.m),
      (i += 16 + w + (d.m ? d.m.length : 0)));
  }
  return (Vn(b, i, r.length, S, x), b);
}
function Un(e, t) {
  for (var n = {}, r = e.length - 22; Sn(e, r) != 101010256; --r)
    (!r || e.length - r > 65558) && rn(13);
  var i = xn(e, r + 8);
  if (!i) return {};
  var a = Sn(e, r + 16),
    o = Sn(e, r - 20) == 117853008;
  if (o) {
    var s = Sn(e, r - 12);
    ((o = Sn(e, s) == 101075792), o && ((i = Sn(e, s + 32)), (a = Sn(e, s + 48))));
  }
  for (var c = t && t.filter, l = 0; l < i; ++l) {
    var u = Ln(e, a, o),
      d = u[0],
      f = u[1],
      p = u[2],
      m = u[3],
      h = u[4],
      g = u[5],
      _ = In(e, g);
    ((a = h),
      (!c || c({ name: m, size: f, originalSize: p, compression: d })) &&
        (d
          ? d == 8
            ? (n[m] = On(e.subarray(_, _ + f), { out: new kt(p) }))
            : rn(14, `unknown compression type ` + d)
          : (n[m] = tn(e, _, _ + f))));
  }
  return n;
}
var Wn = class extends String {
    constructor(e, t, n) {
      let r = new Uint32Array(e, t, n),
        i = Array.from(r),
        a = String.fromCodePoint(...i);
      super(a);
    }
  },
  Gn = new TextDecoder(`latin1`);
function Kn(e) {
  let t = new DataView(e);
  if (
    String.fromCharCode(
      t.getUint8(0),
      t.getUint8(1),
      t.getUint8(2),
      t.getUint8(3),
      t.getUint8(4),
      t.getUint8(5),
    ) !== `NUMPY`
  )
    throw Error(`Not an .npy file`);
  let n = t.getUint8(6),
    r = t.getUint8(7),
    i;
  return n <= 1
    ? ((i = t.getUint16(8, !0)), { headerOffset: 10, headerLen: i, version: [n, r] })
    : ((i = Number(t.getUint32(8, !0))), { headerOffset: 12, headerLen: i, version: [n, r] });
}
function qn(e) {
  let t = /'descr'\s*:\s*'([^']+)'/.exec(e)?.[1],
    n = /'fortran_order'\s*:\s*(True|False)/.exec(e)?.[1] === `True`,
    r = (/'shape'\s*:\s*\(([^)]*)\)/.exec(e)?.[1] ?? ``)
      .split(`,`)
      .map((e) => e.trim())
      .filter(Boolean)
      .map((e) => Number(e));
  return (r.length === 1 && e.includes(`(n,)`), { dtype: t, fortranOrder: n, shape: r });
}
function Jn(e, t, n, r) {
  let i = e.startsWith(`<`) || e.startsWith(`|`),
    a = i ? e.substring(1) : e;
  if (a[0] === `U`) {
    let e = parseInt(a.substring(1)),
      r = String(new Wn(t, n)),
      i = [];
    for (let t = 0; t < r.length; t += e) i.push(r.substring(t, t + e).replace(/\0/g, ``));
    return i;
  }
  switch (a) {
    case `b1`:
      if (i) {
        let e = new Uint8Array(t, n),
          r = Array(e.length);
        for (let t = 0; t < e.length; t++) r[t] = e[t] !== 0;
        return r;
      }
      return new Uint8Array(t, n);
    case `i1`:
      return new Int8Array(t, n);
    case `u1`:
      return new Uint8Array(t, n);
    case `i2`:
      return new Int16Array(t, n);
    case `u2`:
      return new Uint16Array(t, n);
    case `i4`:
      return new Int32Array(t, n);
    case `u4`:
      return new Uint32Array(t, n);
    case `i8`:
      return new BigInt64Array(t, n);
    case `u8`:
      return new BigUint64Array(t, n);
    case `f4`:
      return new Float32Array(t, n);
    case `f8`:
      return new Float64Array(t, n);
    case `c8`:
      return new Float32Array(t, n);
    case `c16`:
      return new Float64Array(t, n);
    case `f2`:
      if (r.convertFloat16 !== !1) {
        let e = new Uint16Array(t, n),
          r = new Float32Array(e.length);
        for (let t = 0; t < e.length; t++) r[t] = Yn(e[t]);
        return r;
      }
      return new Uint16Array(t, n);
    default:
      throw Error(`Unsupported dtype: ${e}`);
  }
}
function Yn(e) {
  let t = (e & 32768) >> 15,
    n = (e & 31744) >> 10,
    r = e & 1023;
  return n === 0
    ? (t ? -1 : 1) * 2 ** -14 * (r / 1024)
    : n === 31
      ? r
        ? NaN
        : t
          ? -1 / 0
          : 1 / 0
      : (t ? -1 : 1) * 2 ** (n - 15) * (1 + r / 1024);
}
function Xn(e, t = {}) {
  let { headerOffset: n, headerLen: r } = Kn(e),
    i = new Uint8Array(e, n, r),
    { dtype: a, fortranOrder: o, shape: s } = qn(Gn.decode(i).trim());
  if (!a || !s) throw Error(`Malformed .npy header`);
  return { data: Jn(a, e, n + r, t), shape: s, dtype: a.slice(1), fortranOrder: o };
}
var Zn = {
  manifest: {
    artifactSetSha256: `0a12189874dace9b590af9b09ef4637552006130716db57c35d72f984b36c577`,
    clustering: {
      constrainedAssignment: !0,
      fa: 0.07,
      fb: 0.8,
      kind: `vbx`,
      maxIters: 20,
      minActiveRatio: 0.2,
      threshold: 0.6,
    },
    conversion: {
      command: `python scripts/convert_pyannote_community.py --source-dir <standard-hugging-face-cache-snapshot> --two-speaker-fixture <local-two-speaker-wav>`,
      inputHashes: {
        "README.md": `2db91f9265bd81f1653ff088b5bff22bf6aebebea03328513af65501643f8a31`,
        "config.yaml": `5ce2bfa9a938dc132cec1172592d65173cbb8f444ea1e4133f10f9391de155be`,
        "embedding/README.md": `fa9e5105ae95edb231d841476cdb91eef4be0621c372ed4f7d3421294b5f8ad7`,
        "embedding/pytorch_model.bin": `6f10ff60898a1d185fa22e1d11e0bfa8a92efec811f11bca48cb8cafebefd929`,
        "plda/README.md": `e1316dbbeb3261431478d48ceebbd4bba395c3587e7b80c254dbab00f1209d0a`,
        "plda/plda.npz": `9b77bcd840692710dd3496f62ecfeed8d8e5f002fd991b785079b244eab7d255`,
        "plda/xvec_transform.npz": `325f1ce8e48f7e55e9c8aa47e05d2766b7c48c4b25b8de8dd751e7a4cc5fbe8f`,
        "segmentation/pytorch_model.bin": `7ad24338d844fb95985486eb1a464e32d229f6d7a03c9abe60f978bacf3f816e`,
      },
      onnxOpset: 17,
      packages: {
        numpy: `2.4.6`,
        onnx: `1.22.0`,
        onnxruntime: `1.27.0`,
        "pyannote.audio": `4.0.4`,
        torch: `2.8.0`,
        torchaudio: `2.8.0`,
      },
      python: `3.11.15`,
    },
    embedding: {
      dimension: 256,
      maskFrames: 589,
      maskInputName: `masks`,
      outputName: `embeddings`,
      waveformInputName: `waveform`,
    },
    endToEndComparison: {
      assignedSpeakers: 2,
      embeddingsFinite: !0,
      fixtureSha256: `d57df4e868dbaccaffb45a2569a6fa8f619847b9e1ae2ea66c465dba4252afbb`,
      requestedSpeakers: 2,
      turnCount: 20,
    },
    files: {
      "LICENSE.md": `a4f9ca9a094f311ad6ce9c021f164aa5d150fb5517df1fb9d31288f0a4e4a4c0`,
      "MODEL_PROVENANCE.md": `bb89c24e8b95b0bb5cca36428aa588662f80b18894bf47eb2b61932dfbe80d13`,
      "clustering.json": `ed79fc68111e9ac92b306d2064461425c175359592f8447a0ec3f21ec265f073`,
      "embedding.onnx": `ea25e8c1c6d08c79e4b812dd00f7044969115f475b5cb94d9601f670ef1d1c47`,
      "plda_model.json": `0c58a94bdbc1dfbb1d1d067deefcc8f7fd288b3d0bc098670e230323075c5530`,
      "plda_transform.json": `154c5ba3e1bcafa8fa9b42dbfa0a82e49e141dd89a41cb54ea8e2b43d2ccf46c`,
      "segmentation.onnx": `cbfb788c5156fc802ecf2a9772c0dd88e3dadc4ca8645f11b565870749a5259b`,
    },
    kind: `pyannote-diarization`,
    labelFormat: `SPEAKER_{:02}`,
    modelId: `pyannote/speaker-diarization-community-1`,
    numericalComparison: {
      embeddingMaxAbsoluteDifference: 1862645149230957e-21,
      fixtureSeed: 218,
      segmentationMaxAbsoluteDifference: 0,
      tolerance: 1e-4,
    },
    sampleRate: 16e3,
    schemaVersion: 1,
    segmentation: {
      durationSeconds: 10,
      frames: 589,
      inputName: `waveform`,
      localSpeakers: 3,
      outputName: `segmentations`,
      powerset: !0,
      stepRatio: 0.1,
    },
    source: {
      license: `CC-BY-4.0`,
      modelId: `pyannote/speaker-diarization-community-1`,
      revision: `3533c8cf8e369892e6b79ff1bf80f7b0286a54ee`,
    },
  },
  donors: {
    "segmentation-community-1.onnx": {
      url: `https://huggingface.co/altunenes/speaker-diarization-community-1-onnx/resolve/e2e09da94ae093a56cd5a60a09b138ae3da1959c/segmentation-community-1.onnx`,
      sha256: `62d59a487d8ba877d0bd1638c53aa06a810bdb104fb776a27ec2204521711006`,
    },
    "wespeaker.onnx": {
      url: `https://huggingface.co/onnx-community/wespeaker-voxceleb-resnet34-LM/resolve/6a61a1833ff2583aabeba044f5c8221f00b67ceb/onnx/model.onnx`,
      sha256: `3955447b0499dc9e0a4541a895df08b03c69098eba4e56c02b5603e9f7f4fcbb`,
    },
  },
  models: {
    "segmentation.onnx": {
      donor: `segmentation-community-1.onnx`,
      sha256: `cbfb788c5156fc802ecf2a9772c0dd88e3dadc4ca8645f11b565870749a5259b`,
      bytes: 5928616,
      templateSha256: `87830fd2ac28e9a1aaef1be9d33d3b86a18cc154182447ba5a5aa2676ed33542`,
      recipeSha256: `569e34f2a5b6fa88832dcb794085c2af23c5ef753a5c87854219ac405e0d4401`,
      templateBase64: `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AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPF2zi2UqU1Qpirro51PO2647O78YV+3xuyFiqREAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADwmGYXKUWZoqiLvp5y3nbduN6Mt1er9rgdIpJGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAwINmF1WK4u7VRV9POW+7blxvxturVXvyeoioNAIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAOCP5sdhiiI9q4u+nnLedt243oy3V6v25GiIQ4EAAAD+D++Xs/Rt/fXy4/TlevFytkjFYj4rU3H/qxS/nn94Ndu/ufx0fZk3683nKd8sXswO7tb271f20s/qbtjry/rgNz2nZCuodloA`,
      recipe: {
        targetSha256: `cbfb788c5156fc802ecf2a9772c0dd88e3dadc4ca8645f11b565870749a5259b`,
        parts: [
          {
            targetStart: 32707,
            targetEnd: 32711,
            donor: `segmentation-community-1.onnx`,
            start: 20527,
            end: 20531,
          },
          {
            targetStart: 32750,
            targetEnd: 32754,
            donor: `segmentation-community-1.onnx`,
            start: 20564,
            end: 20568,
          },
          {
            targetStart: 32809,
            targetEnd: 32969,
            donor: `segmentation-community-1.onnx`,
            start: 20617,
            end: 20777,
          },
          {
            targetStart: 33025,
            targetEnd: 33185,
            donor: `segmentation-community-1.onnx`,
            start: 20827,
            end: 20987,
          },
          {
            targetStart: 33238,
            targetEnd: 33738,
            donor: `segmentation-community-1.onnx`,
            start: 21034,
            end: 21534,
          },
          {
            targetStart: 33788,
            targetEnd: 34288,
            donor: `segmentation-community-1.onnx`,
            start: 21578,
            end: 22078,
          },
          {
            targetStart: 34335,
            targetEnd: 130335,
            donor: `segmentation-community-1.onnx`,
            start: 22119,
            end: 118119,
          },
          {
            targetStart: 130374,
            targetEnd: 130614,
            donor: `segmentation-community-1.onnx`,
            start: 118152,
            end: 118392,
          },
          {
            targetStart: 130661,
            targetEnd: 202661,
            donor: `segmentation-community-1.onnx`,
            start: 118433,
            end: 190433,
          },
          {
            targetStart: 202700,
            targetEnd: 202940,
            donor: `segmentation-community-1.onnx`,
            start: 190466,
            end: 190706,
          },
          {
            targetStart: 202981,
            targetEnd: 203301,
            donor: `segmentation-community-1.onnx`,
            start: 190741,
            end: 191061,
          },
          {
            targetStart: 203340,
            targetEnd: 203660,
            donor: `segmentation-community-1.onnx`,
            start: 191094,
            end: 191414,
          },
          {
            targetStart: 203701,
            targetEnd: 203941,
            donor: `segmentation-community-1.onnx`,
            start: 191449,
            end: 191689,
          },
          {
            targetStart: 203980,
            targetEnd: 204220,
            donor: `segmentation-community-1.onnx`,
            start: 191722,
            end: 191962,
          },
          {
            targetStart: 204261,
            targetEnd: 204501,
            donor: `segmentation-community-1.onnx`,
            start: 191997,
            end: 192237,
          },
          {
            targetStart: 204540,
            targetEnd: 204780,
            donor: `segmentation-community-1.onnx`,
            start: 192270,
            end: 192510,
          },
          {
            targetStart: 204812,
            targetEnd: 205324,
            donor: `segmentation-community-1.onnx`,
            start: 192536,
            end: 193048,
          },
          {
            targetStart: 205356,
            targetEnd: 205868,
            donor: `segmentation-community-1.onnx`,
            start: 193074,
            end: 193586,
          },
          {
            targetStart: 205899,
            targetEnd: 205927,
            donor: `segmentation-community-1.onnx`,
            start: 193611,
            end: 193639,
          },
          {
            targetStart: 205954,
            targetEnd: 206454,
            donor: `segmentation-community-1.onnx`,
            start: 193666,
            end: 194166,
          },
          {
            targetStart: 206483,
            targetEnd: 214675,
            donor: `segmentation-community-1.onnx`,
            start: 194195,
            end: 202387,
          },
          {
            targetStart: 214708,
            targetEnd: 460468,
            donor: `segmentation-community-1.onnx`,
            start: 202420,
            end: 448180,
          },
          {
            targetStart: 460502,
            targetEnd: 984790,
            donor: `segmentation-community-1.onnx`,
            start: 448214,
            end: 972502,
          },
          {
            targetStart: 984819,
            targetEnd: 993011,
            donor: `segmentation-community-1.onnx`,
            start: 972531,
            end: 980723,
          },
          {
            targetStart: 993045,
            targetEnd: 2041621,
            donor: `segmentation-community-1.onnx`,
            start: 980757,
            end: 2029333,
          },
          {
            targetStart: 2041655,
            targetEnd: 2565943,
            donor: `segmentation-community-1.onnx`,
            start: 2029367,
            end: 2553655,
          },
          {
            targetStart: 2565972,
            targetEnd: 2574164,
            donor: `segmentation-community-1.onnx`,
            start: 2553684,
            end: 2561876,
          },
          {
            targetStart: 2574198,
            targetEnd: 3622774,
            donor: `segmentation-community-1.onnx`,
            start: 2561910,
            end: 3610486,
          },
          {
            targetStart: 3622808,
            targetEnd: 4147096,
            donor: `segmentation-community-1.onnx`,
            start: 3610520,
            end: 4134808,
          },
          {
            targetStart: 4147125,
            targetEnd: 4155317,
            donor: `segmentation-community-1.onnx`,
            start: 4134837,
            end: 4143029,
          },
          {
            targetStart: 4155351,
            targetEnd: 5203927,
            donor: `segmentation-community-1.onnx`,
            start: 4143063,
            end: 5191639,
          },
          {
            targetStart: 5203961,
            targetEnd: 5728249,
            donor: `segmentation-community-1.onnx`,
            start: 5191673,
            end: 5715961,
          },
          {
            targetStart: 5728283,
            targetEnd: 5859355,
            donor: `segmentation-community-1.onnx`,
            start: 5715995,
            end: 5847067,
          },
          {
            targetStart: 5859389,
            targetEnd: 5924925,
            donor: `segmentation-community-1.onnx`,
            start: 5847101,
            end: 5912637,
          },
          {
            targetStart: 5924956,
            targetEnd: 5928540,
            donor: `segmentation-community-1.onnx`,
            start: 5912668,
            end: 5916252,
          },
        ],
      },
    },
    "embedding.onnx": {
      donor: `wespeaker.onnx`,
      sha256: `ea25e8c1c6d08c79e4b812dd00f7044969115f475b5cb94d9601f670ef1d1c47`,
      bytes: 27225490,
      templateSha256: `86993df6492c2142e3af30e5d38d3fc695307f65052f69a6a7cb246fb5fe4b8d`,
      recipeSha256: `42e1b5275589e1f7585071650799205922f0c68f53b6fe7f30eadea31047037b`,
      templateBase64: `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kFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsisCTJrgsyaUGZNkFkTA7MmyKwJMmuCzJpQZk2QWRMDsybIrAkya4LMmlBmTZBZEwOzJsmsCTJrksyaVGZNkFkTA7MmyawJMmuSzJpUZk2QWRMDsybJrAkya5LMmlRmTZBZEwOzJsmsCTJrksyaVGZNkFkTA7MmyawJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7MmyaxJMmuSzJpUZk2SWZMDsybJrEkya5LMmlRmTZJZkwOzJsmsSTJrksyaVGZNklmTA7OmyKxJMmuKzJpSZk2SWZMDs6bIrEkya4rMmlJmTZJZkwOzpsisSTJrisyaUmZNklmTA7OmyKxJMmuKzJpSZk2SWZMDs6bIrEkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bIrCkya4rMmlJmTZFZUwOzpsisKTJrisyaUmZNkVlTA7OmyKwpMmuKzJpSZk2RWVMDs6bJrCkya5rMmlZmTZFZUwOzpsmsKTJrmsyaVmZNkVlTA7OmyawpMmuazJpWZk2RWVMDs6bJrCkya5rMmlZmTZFZUwOzpsmsKTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzpsmsaTJrmsyaVmZNk1nTA7OmyaxpMmuazJpWZk2TWdMDs6bJrGkya5rMmlZmTZNZ0wOzZpFZ02TWLDJrljJrmsyaHpg1i8yaJrNmkVmzlFnTZNb0wKxZZNY0mTWLzJqlzJoms6YHZs0is6bJrFlk1ixl1jSZNT0waxaZNU1mzSKzZimzZpFZswZmzSKzZpFZs8isWcqsWWTWrIFZs8isWWTWLDJrljJrFpk1a2DWLDJrFpk1i8yapcyaRWbNGpg1i8yaRWbNIrNmKbNmkVmzBmbNIrNmkVmzyKxZyqxZZNasgVmzyKxZZNYsMmuWMmsWmTVrYNYsMmsWmTWLzJqlzJpFZs0amDWLzJpFZs0is2Yps2aRWbMGZs0is2aRWbPIrFnKrFlk1qyBWbPIrFlk1iwya5YyaxaZNWtg1iwyaxaZNYvMmqXMmkVmzRqYNYvMmkVmzSKzZimzZpFZswZmzSKzZpFZs8isWcqsWWTWrIFZs8isWWTWLDJrljJrFpk1a2DWLDJrFpk1i8yapcyaRWbNGpg1i8yaRWbNIrNmKbNmkVmzBmbNIrNmkVmzyKxZyqxZZNasgVmzyKxZZNYsMmuWMmsWmTVrYNYsMmsWmTWLzJqlzJpFZs0amDWLzJpFZs0is2Yps2aRWbMGZs0is2aRWbPIrFnKrFlk1qyBWbPIrFlk1iwya5YyaxaZNWtg1iwyaxaZNYvMmqXMmkVmzRqYNYvMmkVmzSKzZimzZpFZswZmzSKzZpFZs8isWcqsWWTWrIFZs8isWWTWLDJrljJrFpk1a2DWLDJrFpk1i8yapcyaRWbNGpg1i8yaRWbNIrNmKbNmkVmzBmbNIrNm/b/s3QmcZFVhNnyHRbFExUbRuCCORhFF6FvbOcrURcUNRVE0YlxwkAGJw4AwiBviloS4ZdXXvDGGIMYlLvGLSxKNEjXG+GpcY9xiNHFL9FM0RrOpX/Us0tPTz53q+pLI7+3/32SYqbpddc+te049twaeSp01JXXWlK7OmpI6a8ocnTUlddaU1FlTUmdN6eqsKamzpszRWVNSZ01JnTUlddaUrs6akjpryhydNSV11pTUWVNSZ03p6qwpqbOmzNFZU1JnTUmdNSV11pSuzpqSOmvKHJ01JXXWlNRZU1JnTenqrCmps6bM0VlTUmdNSZ01JXXWlK7OmpI6a8ocnTUlddaU1FlTUmdN6eqsKamzpszRWVNSZ01JnTUlddaUrs6akjpryhydNSV11pTUWVNSZ03p6qwpqbOmzNFZU1JnTUmdNSV11pSuzpqSOmvKHJ01JXXWlNRZU1JnTenqrCmps6bM0VlTUmdNSZ01JXXWlK7OmpI6a8ocnTUlddaU1FlTUmdN6eqsKamzpszRWVNSZ01JnTUlddaUrs6akjpryhydNSV11pTUWVNSZ03p6qwpqbOmzNFZU1JnTUmdNSV11pSuzpqSOmvKHJ01JXXWlNRZU1JnTenqrCmps6bM0VlTUmdNSZ01JXXWlK7OmpI6a8ocnTUlddaU1FlTUmdN6eqsKamzpszRWVNSZ01JnTUlddaUrs6akjpryhydNSV11pTUWVNSZ03p6qwpqbOmzNFZU1JnTUmdNSV11pSuzpqSOmvKHJ01JXXWlNRZU1JnTenqrCmps6bM0VlTUmdNSZ01JXXWlK7OmpI6a8ocnTUlddaU1FlTUmdN6eqsKamzpszRWVNSZ01JnTUlddaUrs6akjpryhydNSV11pTUWVNSZ03p6qwpqbOmzNFZU1JnTUmdNSV11pSuzpqSOmvKHJ01JXXWlNRZU1JnTenqrCmps6bM0VlTUmdNSZ01JXXWlK7OmpI6a8ocnTUlddaU1FlTUmdN6eqsKamzpszRWVNSZ01JnTUlddaUrs6akjpryhydNSV11pTUWVNSZ03p6qwpqbOmzNFZU1JnTUmdNSV11pSuzpqSOmvKHJ01JXXWlNRZU1JnTenqrCmps6bM0VlTUmdNSZ01JXXWlK7OmpI6a8ocnTUlddaU1FlTUmdN6eqsKamzpszRWVNSZ01JnTUlddaUrs6akjpryhydNSV11pTUWVNSZ03p6qwpqbOmzNFZU1JnTUmdNSV11pSuzpqSOmvKHJ01JXXWlNRZU1JnTenqrCmps6bM0VlTUmdNSZ01JXXWlK7OmpI6a8ocnTUlddaU1FlTUmdN6eqsKamzpszRWVNSZ01JnTUlddaUrs6akjpryhydNSV11pTUWVNSZ03p6qwpqbOmzNFZU1JnTUmdNSV11pSuzpqSOmvKHJ01JXXWlNRZU1JnTenqrCmps6bM0VlTUmdNSZ01JXXWlK7OmpI6a8ocnTUlddaU1FlTUmdN6eqsKamzpszRWVNSZ01JnTUlddaUrs6akjpryhydNSV11pTUWVNSZ03p6qwpqbOmzNFZU1JnTUmdNSV11pSuzpqSOmvKHJ01JXXWlNRZU1JnTenqrCmps6bM0VlTUmdNSZ01JXXWlK7OmpI6a8ocnTUlddaU1FlTUmdN6eqsKamzpszRWVNSZ01JnTUlddaUrs6akjpryhydNSV11pTUWVNSZ03p6qwpqbOmzNFZU1JnTUmdNSV11pSuzpqSOmvKHJ01JXXWlNRZU1JnTenqrCmps6bM0VlTUmdNSZ01JXXWlK7OmpI6a8ocnTUlddaU1FlTUmdN6eqsKamzpszRWVNSZ01JnTUlddaUrs6akjpryhydNSV11pTUWVNSZ03p6qwpqbOmzNFZU1JnTUmdNSV11pSuzpqSOmvKHJ01JXXWlNRZU1JnTenqrCmps6bM0VlTUmdNSZ01JXXWlK7OmpI6a8ocnTU1ddaU1FlTU2dN7eqsKamzpszRWVNTZ01JnTU1ddbUrs6akjpryhydNTV11pTUWVNTZ03t6qwpqbOmzNFZU1NnTUmdNTV11tSuzpqSOmvKHJ01NXXWlNRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUrs6amjpr6hydNTV11tTUWVNTZ03t6qypqbOmztFZU1NnTU2dNTV11tSuzpqaOmvqHJ01NXXW1NRZU1NnTe3qrKmps6bO0VlTU2dNTZ01NXXW1K7Ompo6a+ocnTU1ddbU1FlTU2dN7eqsqamzps7RWVNTZ01NnTU1ddbUPTtr3vruDdNnufruHz/LTZbduLj6zc3qN/dXv3mw+s3D1W8erX7zePWby+o3L3tzO2z5cI4Nty+G25twez/cPgi3D8Pto3D7ONxewu1hvE0YbxPG24TxNmG8TRhvE8bbhPE2YbxNGG8TxtsP4+2H8fbDePthvP0w3n4Ybz+Mtx/G2w/j7YfxDsJ4B2G8gzDeQRjvIIx3EMY7COMdhPEOwngHYbzDMN5hGO8wjHcYxjsM4x2G8Q7DeIdhvMMw3mEY7yiMdxTGOwrjHYXxjsJ4R2G8ozDeURjvKIx3FMY7DuMdh/GOw3jHYbzjMN5xGO84jHccxjsO4x2H8ZYw3hLGW8J4SxhvCeMtYbwljLeE8ZYw3hLGW8N4axhvDeOtYbw1jLeG8dYw3hrGW8N496y7W543jk13LKY7mnRHP90xSHcM0x2jdMc43VHSHWnki2nki2nki2nki2nki2nki2nki2nki2nki2nki2nkTRp5k0bepJE3aeRNGnmTRt6kkTdp5E0aeZNG3k8j76eR99PI+2nk/TTyfhp5P428n0beTyPvp5EP0sgHaeSDNPJBGvkgjXyQRj5IIx+kkQ/SyAdp5MM08mEa+TCNfJhGPkwjH6aRD9PIh2nkwzTyYRr5KI18lEY+SiMfpZGP0shHaeSjNPJRGvkojXyURj5OIx+nkY/TyMdp5OM08nEa+TiNfJxGPk4jH6eRlzTykkZe0shLGnlJIy9p5CWNvKSRlzTykkZe08hrGnlNI69p5DWNvKaR1zTymkZe08hThmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShmtShuunDNdPGa6fMlw/Zbh+ynD9lOH6KcP1U4brpwzXTxmunzJcP2W4fspw/ZTh+inD9VOG66cM108Zrp8yXD9luH7KcP2U4fopw/VThuunDNdPGa6fMlw/Zbh+ynD9lOH6KcP1U4brpwzXTxmunzJcP2W4fspw/ZTh+inD9VOG66cM108Zrp8yXD9luH7KcP2U4fopw/VThuunDNdPGa6fMlw/Zbh+ynD9lOH6KcP1U4brpwzXTxmunzJcP2W4fspw/ZTh+inD9VOG66cM108Zrp8yXD9luH7KcP2U4fopw/VThuunDNdPGa6fMlw/Zbh+ynD9lOH6KcP1U4brpwzXTxmunzJcP2W4fspw/ZTh+inD9VOG66cM108Zrp8yXD9luH7KcP2U4fopw/VThuunDNdPGa6fMlw/ZbhBynCDlOEGKcMNUoYbpAw3SBlukDLcIGW4Qcpwg5ThBinDDVKGG6QMN0gZbpAy3CBluEHKcIOU4QYpww1ShhukDDdIGW6QMtwgZbhBynCDlOEGKcMNUoYbpAw3SBlukDLcIGW4Qcpwg5ThBinDDVKGG6QMN0gZ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AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPx/7NoxrttGFAXQDOliVEneAWsWhr8k0jYrQepmE0QCfBhphkC+IaicZQgIYHEpXlqUmkmVQkl0ztvAxX2DVw0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA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AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABfUd8PKYpzMZyWnLeuG5e8zm1z/TvFwTo7b+MjLo84Pr9K1VMjb7+niIuZAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA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AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALzj+Mgl0iHNfav1Pk3XVtfbOJ7GJbJ3PryNLX5K/Nr7KF3pXhv5XiJ+3AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD8Z6cOkRuEojCMDsFcF9aBKp2WTpHEsQl28HCZyLuEWGSz0oKKiK3oDOe8J6/6xQcAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA8FfaNSJPTTWel1Juw3BZynXu+7fvKcM6cOA2PPJebXXYfx31SyS6bsrtwlAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAMB/064ReWqq8byUchuGy1Kuc99371OGdeDAbfjJPQ6RVWzvtREfU4oEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA8NSuEXlqqvG8lHIbhstSrnPfd59ThnXgwG145L3a6rD/OurXSHxNuV0YCgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB+2alDHIRhMAzD2WrqxjlQDJKGVILrJXaDzpHJHmEWy0kpmguQ8Dz55a8+8QIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADwa47PGNt4GG7TWuuW832tjyWl+VpatA78cRtebR96HT4XYviKxPlUWv8wFAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA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AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgJ/m4CWElBfZeNA2zdNodNY2y2lVDY8nKbgOrPA2/E/PWbcOH18v9L6PRDVJ3QuHAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAfpqDlxBSXmTjQds0T6PRWdssp1U1PJmk4DqwyttQhNT/2obLq/OrbhuOh5NUrK2lUxEREVmtXO/G8HiznP9tH+pyM5YhK3/HPGSf6Yf0tnG9Hdfrm8W/RTmIv7p647Pqhdf+7VGM8/p2PpvdN3eLcj/udfVO3Co3L+Z1/dVMZ/f19LD7JeXjvPjzDjY/7fWSbZ8B`,
      recipe: {
        targetSha256: `ea25e8c1c6d08c79e4b812dd00f7044969115f475b5cb94d9601f670ef1d1c47`,
        parts: [
          {
            targetStart: 700653,
            targetEnd: 5943533,
            donor: `wespeaker.onnx`,
            start: 21285144,
            end: 26528024,
          },
          {
            targetStart: 5943563,
            targetEnd: 5944587,
            donor: `wespeaker.onnx`,
            start: 26528053,
            end: 26529077,
          },
          {
            targetStart: 5944620,
            targetEnd: 5945772,
            donor: `wespeaker.onnx`,
            start: 7228,
            end: 8380,
          },
          {
            targetStart: 5945799,
            targetEnd: 5945927,
            donor: `wespeaker.onnx`,
            start: 8395,
            end: 8523,
          },
          {
            targetStart: 5945962,
            targetEnd: 5982826,
            donor: `wespeaker.onnx`,
            start: 8546,
            end: 45410,
          },
          {
            targetStart: 5982853,
            targetEnd: 5982981,
            donor: `wespeaker.onnx`,
            start: 45425,
            end: 45553,
          },
          {
            targetStart: 5983016,
            targetEnd: 6019880,
            donor: `wespeaker.onnx`,
            start: 45576,
            end: 82440,
          },
          {
            targetStart: 6019907,
            targetEnd: 6020035,
            donor: `wespeaker.onnx`,
            start: 82455,
            end: 82583,
          },
          {
            targetStart: 6020070,
            targetEnd: 6056934,
            donor: `wespeaker.onnx`,
            start: 82606,
            end: 119470,
          },
          {
            targetStart: 6056961,
            targetEnd: 6057089,
            donor: `wespeaker.onnx`,
            start: 119485,
            end: 119613,
          },
          {
            targetStart: 6057124,
            targetEnd: 6093988,
            donor: `wespeaker.onnx`,
            start: 119636,
            end: 156500,
          },
          {
            targetStart: 6094015,
            targetEnd: 6094143,
            donor: `wespeaker.onnx`,
            start: 156515,
            end: 156643,
          },
          {
            targetStart: 6094178,
            targetEnd: 6131042,
            donor: `wespeaker.onnx`,
            start: 156666,
            end: 193530,
          },
          {
            targetStart: 6131069,
            targetEnd: 6131197,
            donor: `wespeaker.onnx`,
            start: 193545,
            end: 193673,
          },
          {
            targetStart: 6131232,
            targetEnd: 6168096,
            donor: `wespeaker.onnx`,
            start: 193696,
            end: 230560,
          },
          {
            targetStart: 6168123,
            targetEnd: 6168251,
            donor: `wespeaker.onnx`,
            start: 230575,
            end: 230703,
          },
          {
            targetStart: 6168286,
            targetEnd: 6242014,
            donor: `wespeaker.onnx`,
            start: 230726,
            end: 304454,
          },
          {
            targetStart: 6242041,
            targetEnd: 6242297,
            donor: `wespeaker.onnx`,
            start: 304469,
            end: 304725,
          },
          {
            targetStart: 6242332,
            targetEnd: 6389788,
            donor: `wespeaker.onnx`,
            start: 313232,
            end: 460688,
          },
          {
            targetStart: 6389815,
            targetEnd: 6390071,
            donor: `wespeaker.onnx`,
            start: 460703,
            end: 460959,
          },
          {
            targetStart: 6390104,
            targetEnd: 6398296,
            donor: `wespeaker.onnx`,
            start: 304746,
            end: 312938,
          },
          {
            targetStart: 6398323,
            targetEnd: 6398579,
            donor: `wespeaker.onnx`,
            start: 312953,
            end: 313209,
          },
          {
            targetStart: 6398614,
            targetEnd: 6546070,
            donor: `wespeaker.onnx`,
            start: 460982,
            end: 608438,
          },
          {
            targetStart: 6546097,
            targetEnd: 6546353,
            donor: `wespeaker.onnx`,
            start: 608453,
            end: 608709,
          },
          {
            targetStart: 6546388,
            targetEnd: 6693844,
            donor: `wespeaker.onnx`,
            start: 608732,
            end: 756188,
          },
          {
            targetStart: 6693871,
            targetEnd: 6694127,
            donor: `wespeaker.onnx`,
            start: 756203,
            end: 756459,
          },
          {
            targetStart: 6694162,
            targetEnd: 6841618,
            donor: `wespeaker.onnx`,
            start: 756482,
            end: 903938,
          },
          {
            targetStart: 6841645,
            targetEnd: 6841901,
            donor: `wespeaker.onnx`,
            start: 903953,
            end: 904209,
          },
          {
            targetStart: 6841936,
            targetEnd: 6989392,
            donor: `wespeaker.onnx`,
            start: 904232,
            end: 1051688,
          },
          {
            targetStart: 6989419,
            targetEnd: 6989675,
            donor: `wespeaker.onnx`,
            start: 1051703,
            end: 1051959,
          },
          {
            targetStart: 6989710,
            targetEnd: 7137166,
            donor: `wespeaker.onnx`,
            start: 1051982,
            end: 1199438,
          },
          {
            targetStart: 7137193,
            targetEnd: 7137449,
            donor: `wespeaker.onnx`,
            start: 1199453,
            end: 1199709,
          },
          {
            targetStart: 7137484,
            targetEnd: 7284940,
            donor: `wespeaker.onnx`,
            start: 1199732,
            end: 1347188,
          },
          {
            targetStart: 7284967,
            targetEnd: 7285223,
            donor: `wespeaker.onnx`,
            start: 1347203,
            end: 1347459,
          },
          {
            targetStart: 7285259,
            targetEnd: 7580171,
            donor: `wespeaker.onnx`,
            start: 1347483,
            end: 1642395,
          },
          {
            targetStart: 7580199,
            targetEnd: 7580711,
            donor: `wespeaker.onnx`,
            start: 1642411,
            end: 1642923,
          },
          {
            targetStart: 7580748,
            targetEnd: 8170572,
            donor: `wespeaker.onnx`,
            start: 1676268,
            end: 2266092,
          },
          {
            targetStart: 8170600,
            targetEnd: 8171112,
            donor: `wespeaker.onnx`,
            start: 2266108,
            end: 2266620,
          },
          {
            targetStart: 8171148,
            targetEnd: 8203916,
            donor: `wespeaker.onnx`,
            start: 1642947,
            end: 1675715,
          },
          {
            targetStart: 8203944,
            targetEnd: 8204456,
            donor: `wespeaker.onnx`,
            start: 1675731,
            end: 1676243,
          },
          {
            targetStart: 8204493,
            targetEnd: 8794317,
            donor: `wespeaker.onnx`,
            start: 2266645,
            end: 2856469,
          },
          {
            targetStart: 8794345,
            targetEnd: 8794857,
            donor: `wespeaker.onnx`,
            start: 2856485,
            end: 2856997,
          },
          {
            targetStart: 8794894,
            targetEnd: 9384718,
            donor: `wespeaker.onnx`,
            start: 2857022,
            end: 3446846,
          },
          {
            targetStart: 9384746,
            targetEnd: 9385258,
            donor: `wespeaker.onnx`,
            start: 3446862,
            end: 3447374,
          },
          {
            targetStart: 9385295,
            targetEnd: 9975119,
            donor: `wespeaker.onnx`,
            start: 3447399,
            end: 4037223,
          },
          {
            targetStart: 9975147,
            targetEnd: 9975659,
            donor: `wespeaker.onnx`,
            start: 4037239,
            end: 4037751,
          },
          {
            targetStart: 9975696,
            targetEnd: 10565520,
            donor: `wespeaker.onnx`,
            start: 4037776,
            end: 4627600,
          },
          {
            targetStart: 10565548,
            targetEnd: 10566060,
            donor: `wespeaker.onnx`,
            start: 4627616,
            end: 4628128,
          },
          {
            targetStart: 10566097,
            targetEnd: 11155921,
            donor: `wespeaker.onnx`,
            start: 4628153,
            end: 5217977,
          },
          {
            targetStart: 11155949,
            targetEnd: 11156461,
            donor: `wespeaker.onnx`,
            start: 5217993,
            end: 5218505,
          },
          {
            targetStart: 11156498,
            targetEnd: 11746322,
            donor: `wespeaker.onnx`,
            start: 5218530,
            end: 5808354,
          },
          {
            targetStart: 11746350,
            targetEnd: 11746862,
            donor: `wespeaker.onnx`,
            start: 5808370,
            end: 5808882,
          },
          {
            targetStart: 11746899,
            targetEnd: 12336723,
            donor: `wespeaker.onnx`,
            start: 5808907,
            end: 6398731,
          },
          {
            targetStart: 12336751,
            targetEnd: 12337263,
            donor: `wespeaker.onnx`,
            start: 6398747,
            end: 6399259,
          },
          {
            targetStart: 12337300,
            targetEnd: 12927124,
            donor: `wespeaker.onnx`,
            start: 6399284,
            end: 6989108,
          },
          {
            targetStart: 12927152,
            targetEnd: 12927664,
            donor: `wespeaker.onnx`,
            start: 6989124,
            end: 6989636,
          },
          {
            targetStart: 12927701,
            targetEnd: 13517525,
            donor: `wespeaker.onnx`,
            start: 6989661,
            end: 7579485,
          },
          {
            targetStart: 13517553,
            targetEnd: 13518065,
            donor: `wespeaker.onnx`,
            start: 7579501,
            end: 7580013,
          },
          {
            targetStart: 13518102,
            targetEnd: 14107926,
            donor: `wespeaker.onnx`,
            start: 7580038,
            end: 8169862,
          },
          {
            targetStart: 14107954,
            targetEnd: 14108466,
            donor: `wespeaker.onnx`,
            start: 8169878,
            end: 8170390,
          },
          {
            targetStart: 14108503,
            targetEnd: 15288151,
            donor: `wespeaker.onnx`,
            start: 8170415,
            end: 9350063,
          },
          {
            targetStart: 15288179,
            targetEnd: 15289203,
            donor: `wespeaker.onnx`,
            start: 9350079,
            end: 9351103,
          },
          {
            targetStart: 15289242,
            targetEnd: 17648538,
            donor: `wespeaker.onnx`,
            start: 9483267,
            end: 11842563,
          },
          {
            targetStart: 17648566,
            targetEnd: 17649590,
            donor: `wespeaker.onnx`,
            start: 11842579,
            end: 11843603,
          },
          {
            targetStart: 17649627,
            targetEnd: 17780699,
            donor: `wespeaker.onnx`,
            start: 9351128,
            end: 9482200,
          },
          {
            targetStart: 17780727,
            targetEnd: 17781751,
            donor: `wespeaker.onnx`,
            start: 9482216,
            end: 9483240,
          },
          {
            targetStart: 17781790,
            targetEnd: 20141086,
            donor: `wespeaker.onnx`,
            start: 11843630,
            end: 14202926,
          },
          {
            targetStart: 20141114,
            targetEnd: 20142138,
            donor: `wespeaker.onnx`,
            start: 14202942,
            end: 14203966,
          },
          {
            targetStart: 20142177,
            targetEnd: 22501473,
            donor: `wespeaker.onnx`,
            start: 14203993,
            end: 16563289,
          },
          {
            targetStart: 22501501,
            targetEnd: 22502525,
            donor: `wespeaker.onnx`,
            start: 16563305,
            end: 16564329,
          },
          {
            targetStart: 22502564,
            targetEnd: 24861860,
            donor: `wespeaker.onnx`,
            start: 16564356,
            end: 18923652,
          },
          {
            targetStart: 24861888,
            targetEnd: 24862912,
            donor: `wespeaker.onnx`,
            start: 18923668,
            end: 18924692,
          },
          {
            targetStart: 24862951,
            targetEnd: 27222247,
            donor: `wespeaker.onnx`,
            start: 18924719,
            end: 21284015,
          },
          {
            targetStart: 27222275,
            targetEnd: 27223299,
            donor: `wespeaker.onnx`,
            start: 21284031,
            end: 21285055,
          },
        ],
      },
    },
  },
  files: {
    "pyannote_diarization_manifest.json": `{
  "artifactSetSha256": "0a12189874dace9b590af9b09ef4637552006130716db57c35d72f984b36c577",
  "clustering": {
    "constrainedAssignment": true,
    "fa": 0.07,
    "fb": 0.8,
    "kind": "vbx",
    "maxIters": 20,
    "minActiveRatio": 0.2,
    "threshold": 0.6
  },
  "conversion": {
    "command": "python scripts/convert_pyannote_community.py --source-dir <standard-hugging-face-cache-snapshot> --two-speaker-fixture <local-two-speaker-wav>",
    "inputHashes": {
      "README.md": "2db91f9265bd81f1653ff088b5bff22bf6aebebea03328513af65501643f8a31",
      "config.yaml": "5ce2bfa9a938dc132cec1172592d65173cbb8f444ea1e4133f10f9391de155be",
      "embedding/README.md": "fa9e5105ae95edb231d841476cdb91eef4be0621c372ed4f7d3421294b5f8ad7",
      "embedding/pytorch_model.bin": "6f10ff60898a1d185fa22e1d11e0bfa8a92efec811f11bca48cb8cafebefd929",
      "plda/README.md": "e1316dbbeb3261431478d48ceebbd4bba395c3587e7b80c254dbab00f1209d0a",
      "plda/plda.npz": "9b77bcd840692710dd3496f62ecfeed8d8e5f002fd991b785079b244eab7d255",
      "plda/xvec_transform.npz": "325f1ce8e48f7e55e9c8aa47e05d2766b7c48c4b25b8de8dd751e7a4cc5fbe8f",
      "segmentation/pytorch_model.bin": "7ad24338d844fb95985486eb1a464e32d229f6d7a03c9abe60f978bacf3f816e"
    },
    "onnxOpset": 17,
    "packages": {
      "numpy": "2.4.6",
      "onnx": "1.22.0",
      "onnxruntime": "1.27.0",
      "pyannote.audio": "4.0.4",
      "torch": "2.8.0",
      "torchaudio": "2.8.0"
    },
    "python": "3.11.15"
  },
  "embedding": {
    "dimension": 256,
    "maskFrames": 589,
    "maskInputName": "masks",
    "outputName": "embeddings",
    "waveformInputName": "waveform"
  },
  "endToEndComparison": {
    "assignedSpeakers": 2,
    "embeddingsFinite": true,
    "fixtureSha256": "d57df4e868dbaccaffb45a2569a6fa8f619847b9e1ae2ea66c465dba4252afbb",
    "requestedSpeakers": 2,
    "turnCount": 20
  },
  "files": {
    "LICENSE.md": "a4f9ca9a094f311ad6ce9c021f164aa5d150fb5517df1fb9d31288f0a4e4a4c0",
    "MODEL_PROVENANCE.md": "bb89c24e8b95b0bb5cca36428aa588662f80b18894bf47eb2b61932dfbe80d13",
    "clustering.json": "ed79fc68111e9ac92b306d2064461425c175359592f8447a0ec3f21ec265f073",
    "embedding.onnx": "ea25e8c1c6d08c79e4b812dd00f7044969115f475b5cb94d9601f670ef1d1c47",
    "plda_model.json": "0c58a94bdbc1dfbb1d1d067deefcc8f7fd288b3d0bc098670e230323075c5530",
    "plda_transform.json": "154c5ba3e1bcafa8fa9b42dbfa0a82e49e141dd89a41cb54ea8e2b43d2ccf46c",
    "segmentation.onnx": "cbfb788c5156fc802ecf2a9772c0dd88e3dadc4ca8645f11b565870749a5259b"
  },
  "kind": "pyannote-diarization",
  "labelFormat": "SPEAKER_{:02}",
  "modelId": "pyannote/speaker-diarization-community-1",
  "numericalComparison": {
    "embeddingMaxAbsoluteDifference": 1.862645149230957e-06,
    "fixtureSeed": 218,
    "segmentationMaxAbsoluteDifference": 0.0,
    "tolerance": 0.0001
  },
  "sampleRate": 16000,
  "schemaVersion": 1,
  "segmentation": {
    "durationSeconds": 10.0,
    "frames": 589,
    "inputName": "waveform",
    "localSpeakers": 3,
    "outputName": "segmentations",
    "powerset": true,
    "stepRatio": 0.1
  },
  "source": {
    "license": "CC-BY-4.0",
    "modelId": "pyannote/speaker-diarization-community-1",
    "revision": "3533c8cf8e369892e6b79ff1bf80f7b0286a54ee"
  }
}
`,
    "clustering.json": `{"kind":"vbx","threshold":0.6,"fa":0.07,"fb":0.8,"maxIters":20,"minActiveRatio":0.2,"constrainedAssignment":true}
`,
    "MODEL_PROVENANCE.md":
      "# Native pyannote community diarization bundle provenance\n\n- Source: `pyannote/speaker-diarization-community-1`\n- Pinned revision: `3533c8cf8e369892e6b79ff1bf80f7b0286a54ee`\n- License: `CC-BY-4.0`\n- Distribution: caller-owned checksum-addressed local cache snapshot\n- Conversion command: `python scripts/convert_pyannote_community.py --source-dir <standard-hugging-face-cache-snapshot> --two-speaker-fixture <local-two-speaker-wav>`\n- Python: `3.11.15`\n- ONNX opset: `17`\n- Segmentation contract: `waveform` [1, 1, 160000] -> `segmentations` [1, 589, 3]\n- Embedding contract: `waveform` [1, 1, 160000] + `masks` [1, 589] -> `embeddings` [1, 256]\n- Segmentation max absolute difference: 0.0\n- Embedding max absolute difference: 1.862645149230957e-06\n- Numerical tolerance: 0.0001\n- PLDA: 256-to-128 x-vector transform plus 128-dimensional model\n- Clustering: pinned VBx threshold/Fa/Fb configuration\n\nSource and generated SHA-256 values are retained in `pyannote_diarization_manifest.json`. No credentials, source weights, audio, or absolute source paths are stored in this bundle.\n\nAutomatic remote hydration is intentionally out of scope. Verify and install this immutable local snapshot before cache-only execution.\n",
    "LICENSE.md": `# Model license

\`pyannote/speaker-diarization-community-1\` at \`3533c8cf8e369892e6b79ff1bf80f7b0286a54ee\` declares \`CC-BY-4.0\`.

License reference: https://creativecommons.org/licenses/by/4.0/

This file documents the license of the converted model artifacts; native-whisperx itself retains its repository license.
`,
  },
};
async function Qn(e) {
  return Array.from(new Uint8Array(await crypto.subtle.digest(`SHA-256`, e)), (e) =>
    e.toString(16).padStart(2, `0`),
  ).join(``);
}
async function $n(e, t) {
  if ((await Qn(e)) !== t) throw Error(`Pyannote resource checksum mismatch.`);
  return e;
}
async function er(e, t, n, r = !1) {
  n.signal?.throwIfAborted();
  let i;
  try {
    i = await fetch(e, {
      credentials: `omit`,
      ...(n.signal ? { signal: n.signal } : {}),
      ...(r ? { headers: { Authorization: `Bearer ${n.token}` }, cache: `no-store` } : {}),
    });
  } catch {
    throw (
      n.signal?.throwIfAborted(),
      Error(`Unable to download pyannote resources. Check your network connection.`)
    );
  }
  if (!i.ok)
    throw r && [401, 403].includes(i.status)
      ? Error(
          `Check your HF_TOKEN and accept the pyannote community-1 model terms on Hugging Face.`,
        )
      : Error(`Pyannote resource download failed (HTTP ${i.status}).`);
  return $n(new Uint8Array(await i.arrayBuffer()), t);
}
function tr(e, t) {
  (e.signal?.throwIfAborted(), e.onProgress?.({ stage: `resources`, message: t }));
}
function nr(e, t, n) {
  let r = e[`${t}.npy`];
  if (!r) throw Error(`Missing approved PLDA array.`);
  let i = Xn(Uint8Array.from(r).buffer);
  if (
    !(i.data instanceof Float32Array || i.data instanceof Float64Array) ||
    i.fortranOrder ||
    i.shape.length !== n.length ||
    i.shape.some((e, t) => e !== n[t]) ||
    i.data.some((e) => !Number.isFinite(e))
  )
    throw Error(`Invalid approved PLDA array.`);
  return Array.from(i.data);
}
function rr(e, t, n, r) {
  let i = nr(e, t, [n, r]);
  return Array.from({ length: n }, (e, t) => i.slice(t * r, (t + 1) * r));
}
function ir(e) {
  if (!Number.isFinite(e)) throw Error(`Invalid PLDA number.`);
  if (Object.is(e, -0)) return `-0.0`;
  let t = Math.abs(e);
  if (t === 0) return `0.0`;
  if (t >= 1e-4 && t < 0x2386f26fc10000) {
    let t = e.toString();
    return t.includes(`.`) ? t : `${t}.0`;
  }
  let [n, r] = e.toExponential().split(`e`),
    i = Number(r);
  return `${n}e${i < 0 ? `-` : `+`}${Math.abs(i).toString().padStart(2, `0`)}`;
}
function ar(e) {
  return `[${e.map(ir).join(`,`)}]`;
}
function or(e) {
  return `[${e.map(ar).join(`,`)}]`;
}
function sr(e) {
  return new TextEncoder().encode(e);
}
function cr(e, t) {
  let n = Un(e),
    r = Un(t);
  return {
    "plda_transform.json":
      sr(`{"schemaVersion":1,"inputDimension":256,"outputDimension":128,"mean1":${ar(nr(n, `mean1`, [256]))},"mean2":${ar(nr(n, `mean2`, [128]))},"lda":${or(rr(n, `lda`, 256, 128))}}
`),
    "plda_model.json":
      sr(`{"schemaVersion":1,"dimension":128,"mean":${ar(nr(r, `mu`, [128]))},"transform":${or(rr(r, `tr`, 128, 128))},"psi":${ar(nr(r, `psi`, [128]))}}
`),
  };
}
async function lr(e, t) {
  let n = Zn.models[e],
    r = Zn.donors[n.donor],
    i = await er(r.url, r.sha256, t),
    a = Uint8Array.from(atob(n.templateBase64), (e) => e.charCodeAt(0));
  await $n(a, n.templateSha256);
  let o = Uint8Array.from(kn(a));
  if (o.length !== n.bytes) throw Error(`Invalid pyannote graph template.`);
  for (let e of n.recipe.parts) {
    if (
      e.targetStart < 0 ||
      e.targetEnd > o.length ||
      e.start < 0 ||
      e.end > i.length ||
      e.targetEnd - e.targetStart !== e.end - e.start
    )
      throw Error(`Invalid pyannote reconstruction span.`);
    o.set(i.subarray(e.start, e.end), e.targetStart);
  }
  return (t.signal?.throwIfAborted(), $n(o, n.sha256));
}
async function ur(e) {
  if (
    typeof e.token != `string` ||
    !e.token.trim() ||
    /[^\x21-\x7e]/.test(e.token) ||
    e.token.length > 4096
  )
    throw Error(`Enter an HF_TOKEN with access to the pyannote community-1 model.`);
  let t = Zn.manifest.source,
    n = Zn.manifest.conversion.inputHashes;
  tr(e, `Checking licensed pyannote resources…`);
  let r = `https://huggingface.co/${t.modelId}/resolve/${t.revision}/plda/`,
    i = cr(
      await er(`${r}xvec_transform.npz`, n[`plda/xvec_transform.npz`], e, !0),
      await er(`${r}plda.npz`, n[`plda/plda.npz`], e, !0),
    );
  (await $n(i[`plda_transform.json`], Zn.manifest.files[`plda_transform.json`]),
    await $n(i[`plda_model.json`], Zn.manifest.files[`plda_model.json`]));
  let a = { ...i };
  for (let [e, t] of Object.entries(Zn.files)) a[e] = sr(t);
  for (let t of [`segmentation.onnx`, `embedding.onnx`])
    (tr(
      e,
      `Preparing pyannote ${t === `segmentation.onnx` ? `segmentation` : `speaker embedding`} model…`,
    ),
      (a[t] = await lr(t, e)));
  return (
    tr(e, `Approved pyannote model bundle is ready.`),
    {
      modelId: t.modelId,
      sourceRevision: t.revision,
      artifactSetSha256: Zn.manifest.artifactSetSha256,
      files: a,
    }
  );
}
async function dr(e) {
  if (
    e.modelId !== Zn.manifest.modelId ||
    e.sourceRevision !== Zn.manifest.source.revision ||
    e.artifactSetSha256 !== Zn.manifest.artifactSetSha256
  )
    throw Error(`Unsupported pyannote model bundle.`);
  let t = {
    ...Zn.manifest.files,
    "pyannote_diarization_manifest.json": await Qn(
      sr(Zn.files[`pyannote_diarization_manifest.json`]),
    ),
  };
  if (Object.keys(e.files).length !== Object.keys(t).length)
    throw Error(`Incomplete pyannote model bundle.`);
  let n = {};
  for (let [r, i] of Object.entries(t)) {
    let t = e.files[r];
    if (!(t instanceof Uint8Array)) throw Error(`Missing pyannote model resource.`);
    n[r] = await $n(Uint8Array.from(t), i);
  }
  return Uint8Array.from(Hn(n, { level: 0 }));
}
function fr(e) {
  if (!e || typeof e != `object` || Array.isArray(e)) throw Error(`Invalid Review response.`);
  return Object.fromEntries(Object.entries(e));
}
function pr(e) {
  if (typeof e != `string`) throw Error(`Invalid Review text.`);
  return e;
}
function mr(e) {
  return e == null ? null : pr(e);
}
function hr(e) {
  if (typeof e != `boolean`) throw Error(`Invalid Review state.`);
  return e;
}
function gr(e) {
  if (!Array.isArray(e)) throw Error(`Invalid Review list.`);
  return e;
}
function _r(e) {
  if (e == null) return null;
  if (typeof e != `number` || !Number.isFinite(e) || e < 0) throw Error(`Invalid Review timing.`);
  return e;
}
function vr(e) {
  let t = fr(e);
  return {
    language: mr(t.language),
    segments: gr(t.segments).map((e) => {
      let t = fr(e),
        n = t.wordCount;
      if (typeof n != `number` || !Number.isSafeInteger(n) || n < 0)
        throw Error(`Invalid alignment count.`);
      return {
        id: pr(t.id),
        text: pr(t.text),
        startSeconds: _r(t.startSeconds),
        endSeconds: _r(t.endSeconds),
        speaker: mr(t.speaker),
        wordCount: n,
        overlapsPrevious: hr(t.overlapsPrevious),
      };
    }),
  };
}
function yr(e) {
  let t = fr(e),
    n = mr(t.mediaUrl);
  if (n && !/^\/api\/review\/media\/[a-f0-9]{64}$/.test(n))
    throw Error(`Invalid local recording capability.`);
  let r = t.translationProvenance == null ? null : fr(t.translationProvenance);
  return {
    resultId: pr(t.resultId),
    snapshotId: pr(t.snapshotId),
    originalName: mr(t.originalName),
    sourceName: mr(t.sourceName),
    mediaUrl: n,
    mediaType: pr(t.mediaType),
    mediaError: mr(t.mediaError),
    root: pr(t.root),
    transcript: vr(t.transcript),
    sourceTranscript: t.sourceTranscript == null ? null : vr(t.sourceTranscript),
    isTranslation: hr(t.isTranslation),
    translationProvenance: r
      ? {
          modelId: mr(r.modelId),
          sourceLanguage: mr(r.sourceLanguage),
          targetLanguage: mr(r.targetLanguage),
        }
      : null,
  };
}
async function br(e, t) {
  let n = JSON.stringify(t);
  if (new TextEncoder().encode(n).length > 1048576)
    throw Error(`Too many corrections for one copy. Save a smaller set of edits.`);
  let r = await fetch(`/api/review/${e}`, {
    method: `POST`,
    signal: AbortSignal.timeout(3e4),
    headers: {
      "Content-Type": `application/json`,
      "X-Native-Whisperx-Session-Token": window.nativeWhisperxSessionToken ?? ``,
    },
    body: n,
  });
  if (!r.ok) throw Error(await r.text());
  return r.json();
}
var xr = {
  async open(e) {
    return yr(await br(`open`, { resultId: e }));
  },
  async reset(e) {
    return yr(await br(`reset`, { resultId: e }));
  },
  async recording(e) {
    return yr(await br(`recording`, e));
  },
  async preview(e) {
    let t = fr(await br(`preview`, e));
    return {
      previewId: pr(t.previewId),
      transcript: vr(t.transcript),
      sourceTranscript: t.sourceTranscript == null ? null : vr(t.sourceTranscript),
      clearedAlignment: gr(t.clearedAlignment).map(pr),
      clearedSourceAlignment: gr(t.clearedSourceAlignment).map(pr),
    };
  },
  async save(e) {
    let t = fr(await br(`save`, e));
    return {
      resultId: pr(t.resultId),
      outputs: gr(t.outputs).map((e) => {
        let t = fr(e);
        return { path: pr(t.path), format: pr(t.format) };
      }),
    };
  },
};
function Sr() {
  return {
    align: !1,
    identifySpeakers: !1,
    autoUpdateSpeakers: !1,
    translation: null,
    model: `small`,
    language: null,
    device: `auto`,
    modelDir: null,
    vadBundle: null,
    diarizationBundle: null,
    outputDir: null,
    formats: [`json`],
    speakerDirectory: null,
  };
}
function Cr(e) {
  if (!e || typeof e != `object` || Array.isArray(e))
    throw Error(`Invalid local workbench response.`);
  return Object.fromEntries(Object.entries(e));
}
function H(e) {
  if (typeof e != `string`) throw Error(`Invalid workbench text.`);
  return e;
}
function wr(e) {
  if (typeof e != `boolean`) throw Error(`Invalid workbench capability.`);
  return e;
}
function Tr(e) {
  return e == null ? null : H(e);
}
function Er(e) {
  switch (e) {
    case `auto`:
    case `cpu`:
    case `cuda`:
      return e;
    default:
      throw Error(`Invalid saved device choice.`);
  }
}
function Dr(e) {
  switch (e) {
    case `json`:
    case `native-json`:
    case `srt`:
    case `vtt`:
    case `txt`:
    case `tsv`:
    case `aud`:
    case `all`:
      return e;
    default:
      throw Error(`Invalid saved output format.`);
  }
}
function Or(e) {
  let t = Cr(e),
    n = t.translation == null ? null : Cr(t.translation),
    r = t.speakerDirectory == null ? null : Cr(t.speakerDirectory),
    i = null;
  if (r) {
    let e = r.scope;
    if (e !== `auto` && e !== `local` && e !== `global`)
      throw Error(`Invalid saved speaker index scope.`);
    i = { scope: e, explicitPath: Tr(r.explicitPath) };
  }
  if (!Array.isArray(t.formats)) throw Error(`Invalid saved output formats.`);
  return {
    align: wr(t.align),
    identifySpeakers: wr(t.identifySpeakers),
    autoUpdateSpeakers: wr(t.autoUpdateSpeakers),
    translation: n
      ? {
          modelId: H(n.modelId),
          sourceLanguage: H(n.sourceLanguage),
          targetLanguage: H(n.targetLanguage),
        }
      : null,
    model: H(t.model),
    language: Tr(t.language),
    device: Er(t.device),
    modelDir: Tr(t.modelDir),
    vadBundle: Tr(t.vadBundle),
    diarizationBundle: Tr(t.diarizationBundle),
    outputDir: Tr(t.outputDir),
    formats: t.formats.map(Dr),
    speakerDirectory: i,
  };
}
function kr(e) {
  let t = Cr(e),
    n = Cr(t.preset);
  return { id: H(t.id), preset: { name: H(n.name), settings: Or(n.settings) } };
}
function Ar(e) {
  let t = Cr(e);
  if (!Array.isArray(t.presets) || !Array.isArray(t.builtins))
    throw Error(`Invalid saved presets response.`);
  return {
    presets: t.presets.map(kr),
    activeId: Tr(t.activeId),
    problem: Tr(t.problem),
    canReset: wr(t.canReset),
    backup: Tr(t.backup),
    builtins: t.builtins.map((e) => {
      let t = Cr(e);
      return { ...kr(e), supported: wr(t.supported), note: H(t.note) };
    }),
  };
}
function jr(e) {
  let t = Cr(e);
  if (!Array.isArray(t.checks)) throw Error(`Invalid preflight checks.`);
  return {
    ready: wr(t.ready),
    checks: t.checks.map((e) => {
      let t = Cr(e);
      return { resource: H(t.resource), available: wr(t.available), message: H(t.message) };
    }),
  };
}
async function Mr(e, t) {
  let n = await fetch(`/api/workbench/${e}`, {
    method: t === void 0 ? `GET` : `POST`,
    headers: {
      "X-Native-Whisperx-Session-Token": window.nativeWhisperxSessionToken ?? ``,
      ...(t === void 0 ? {} : { "Content-Type": `application/json` }),
    },
    ...(t === void 0 ? {} : { body: JSON.stringify(t) }),
  });
  if (!n.ok) throw Error(await n.text());
  return n.json();
}
function Nr(e) {
  let t = Cr(e),
    n = H(t.status);
  if (![`idle`, `running`, `cancelling`, `completed`, `failed`, `cancelled`].includes(n))
    throw Error(`Unknown workbench job state.`);
  let r = Object.fromEntries(
      Object.entries(Cr(t.tasks ?? {})).map(([e, t]) => [e, { status: H(Cr(t).status) }]),
    ),
    i = t.outputs ?? [];
  if (!Array.isArray(i)) throw Error(`Invalid workbench outputs.`);
  return {
    status: n,
    reviewId: t.reviewId == null ? null : H(t.reviewId),
    tasks: r,
    outputs: i.map((e) => {
      let t = Cr(e);
      return { path: H(t.path), format: H(t.format) };
    }),
    input: t.input == null ? null : H(Cr(t.input).path),
    currentTask: t.currentTask == null ? null : H(t.currentTask),
    activity: t.activity == null ? `` : H(t.activity),
    error: t.error == null ? `` : H(t.error),
  };
}
var Pr = {
  async installCommunityBundle(e, t) {
    let n = await fetch(`/api/workbench/install-community-bundle`, {
      method: `POST`,
      headers: {
        "Content-Type": `application/zip`,
        "X-Native-Whisperx-Session-Token": window.nativeWhisperxSessionToken ?? ``,
      },
      body: e,
      signal: t,
    });
    if (!n.ok)
      throw Error(
        `Could not install the verified pyannote models. Check that the app is idle and the model cache is writable.`,
      );
    let r = Cr(await n.json());
    return { bundlePath: H(r.bundlePath), vadBundlePath: H(r.vadBundlePath) };
  },
  async presets() {
    return Ar(await Mr(`presets`));
  },
  async savePreset(e) {
    return Ar(await Mr(`preset-save`, e));
  },
  async selectPreset(e) {
    return Ar(await Mr(`preset-select`, { id: e }));
  },
  async deletePreset(e) {
    return Ar(await Mr(`preset-delete`, { id: e }));
  },
  async resetPresets() {
    return Ar(await Mr(`preset-reset`, {}));
  },
  async preflight(e) {
    return jr(await Mr(`preflight`, e));
  },
  async capabilities() {
    let e = Cr(await Mr(`capabilities`));
    return {
      native: wr(e.native),
      align: wr(e.align),
      identifySpeakers: wr(e.identifySpeakers),
      translate: wr(e.translate),
      cuda: wr(e.cuda),
      root: H(e.root),
      speakerDirectory: H(e.speakerDirectory),
    };
  },
  async state() {
    return Nr(await Mr(`state`));
  },
  async run(e) {
    return Nr(await Mr(`run`, e));
  },
  async cancel() {
    return Nr(await Mr(`cancel`, {}));
  },
  async result(e) {
    let t = Cr(await Mr(`result`, { index: e }));
    return {
      text: H(t.text),
      content: H(t.content),
      path: H(t.path),
      transcript: t.transcript == null ? null : vr(t.transcript),
    };
  },
  async files(e) {
    let t = Cr(await Mr(`files`, { directory: e }));
    if (!Array.isArray(t.entries)) throw Error(`Invalid workbench file list.`);
    return {
      root: H(t.root),
      directory: H(t.directory),
      truncated: wr(t.truncated),
      entries: t.entries.map((e) => {
        let t = Cr(e);
        return { path: H(t.path), name: H(t.name), directory: wr(t.directory) };
      }),
    };
  },
};
function Fr({ onPrepared: e }) {
  let t = (0, y.useRef)(null),
    n = (0, y.useRef)(null),
    [r, i] = (0, y.useState)(``);
  (0, y.useEffect)(() => () => n.current?.abort(), []);
  let a = B({
    mutationFn: async () => {
      let e = new AbortController();
      n.current = e;
      try {
        let n = await ur({
          token: t.current?.value.trim() ?? ``,
          signal: e.signal,
          onProgress: ({ message: e }) => i(e),
        });
        (e.signal.throwIfAborted(), i(`Installing verified models in the local cache…`));
        let r = await dr(n);
        e.signal.throwIfAborted();
        let a = await Pr.installCommunityBundle(r, e.signal);
        return (e.signal.throwIfAborted(), a);
      } finally {
        (t.current && (t.current.value = ``), (n.current = null));
      }
    },
    onSuccess: (t) => {
      (e(t), i(`Pyannote models are ready for VAD and diarization.`));
    },
  });
  return (0, b.jsxs)(`section`, {
    "aria-label": `Pyannote model setup`,
    children: [
      (0, b.jsxs)(`p`, {
        children: [
          `Accept the`,
          ` `,
          (0, b.jsx)(`a`, {
            href: `https://huggingface.co/pyannote/speaker-diarization-community-1`,
            target: `_blank`,
            rel: `noreferrer`,
            children: `community-1 model conditions`,
          }),
          `, then use a Hugging Face read token. Downloads run in your browser; verified models are stored in this app’s local cache.`,
        ],
      }),
      (0, b.jsxs)(`label`, {
        children: [
          `HF_TOKEN`,
          (0, b.jsx)(`input`, {
            ref: t,
            type: `password`,
            autoComplete: `off`,
            spellCheck: !1,
            disabled: a.isPending,
          }),
        ],
      }),
      (0, b.jsx)(`p`, {
        className: `muted`,
        children: `The token is used only for Hugging Face downloads and is cleared after setup. It is never saved in presets or sent to the local app.`,
      }),
      (0, b.jsx)(`button`, {
        type: `button`,
        disabled: a.isPending,
        onClick: () => a.mutate(),
        children: `Prepare pyannote models`,
      }),
      a.isPending
        ? (0, b.jsx)(`button`, {
            type: `button`,
            onClick: () => n.current?.abort(),
            children: `Cancel model setup`,
          })
        : null,
      r ? (0, b.jsx)(`p`, { role: `status`, children: r }) : null,
      a.error
        ? (0, b.jsx)(`p`, {
            role: `alert`,
            children: a.error.name === `AbortError` ? `Model setup cancelled.` : a.error.message,
          })
        : null,
    ],
  });
}
function Ir(e) {
  let t = e.trim(),
    n = () => Error(`Enter a finite nonnegative time in seconds or HH:MM:SS.mmm.`);
  if (!t) throw n();
  let r = t.split(`:`),
    i;
  if (r.length === 1) {
    if (!/^(?:\d+(?:\.\d*)?|\.\d+)(?:e[+-]?\d+)?$/i.test(t)) throw n();
    i = Number(t);
  } else if (r.length === 2 || r.length === 3) {
    let e = r.at(-1);
    if (!e || !/^\d+(?:\.\d*)?$/.test(e)) throw n();
    let t = r.slice(0, -1);
    if (t.some((e) => !/^\d+$/.test(e))) throw n();
    let a = t.map(Number),
      o = Number(e);
    if (o >= 60 || (r.length === 3 && (a[1] ?? 0) >= 60)) throw n();
    i = r.length === 3 ? (a[0] ?? 0) * 3600 + (a[1] ?? 0) * 60 + o : (a[0] ?? 0) * 60 + o;
  } else throw n();
  if (!Number.isFinite(i) || i < 0) throw n();
  return i;
}
function Lr(e) {
  if (!Number.isFinite(e)) return `—`;
  let t = Math.floor(e / 3600),
    n = Math.floor((e % 3600) / 60),
    r = (e % 60).toFixed(3).padStart(6, `0`);
  return `${String(t).padStart(2, `0`)}:${String(n).padStart(2, `0`)}:${r}`;
}
function Rr({ transcript: e, fallbackText: t }) {
  return e?.segments.length
    ? (0, b.jsx)(`ol`, {
        className: `timedTranscript`,
        "aria-label": `Timestamped transcript`,
        children: e.segments.map((e) =>
          (0, b.jsxs)(
            `li`,
            {
              children: [
                (0, b.jsxs)(`div`, {
                  className: `segmentTiming`,
                  children: [
                    (0, b.jsxs)(`span`, {
                      children: [
                        e.startSeconds == null ? `Untimed` : Lr(e.startSeconds),
                        ` →`,
                        ` `,
                        e.endSeconds == null ? `Untimed` : Lr(e.endSeconds),
                      ],
                    }),
                    e.speaker ? (0, b.jsx)(`strong`, { children: e.speaker }) : null,
                  ],
                }),
                (0, b.jsx)(`p`, { children: e.text }),
              ],
            },
            e.id,
          ),
        ),
      })
    : (0, b.jsx)(`p`, { children: t || `No speech detected.` });
}
function zr(e) {
  if (!at(e)) throw Error(`Invalid Speaker Library response.`);
  return e;
}
function Br(e) {
  if (typeof e != `string`) throw Error(`Invalid Speaker Library path or identifier.`);
  return e;
}
function Vr(e) {
  if (typeof e != `number` || !Number.isSafeInteger(e) || e < 0)
    throw Error(`Invalid Speaker Library profile count.`);
  return e;
}
function Hr(e) {
  if (typeof e != `boolean`) throw Error(`Invalid Speaker Library operation state.`);
  return e;
}
function Ur(e) {
  return e === null ? null : Br(e);
}
function Wr(e) {
  let t = zr(e);
  return {
    path: Br(t.path),
    profileCount: Vr(t.profileCount),
    changed: Hr(t.changed),
    backupPath: Ur(t.backupPath),
  };
}
async function Gr(e, t) {
  let n = await fetch(`/api/library/${e}`, {
    method: `POST`,
    signal: AbortSignal.timeout(3e4),
    headers: {
      "Content-Type": `application/json`,
      "X-Native-Whisperx-Session-Token": window.nativeWhisperxSessionToken ?? ``,
    },
    body: JSON.stringify(t),
  });
  if (!n.ok) throw Error(await n.text());
  return n.json();
}
function Kr(e) {
  let t = zr(e),
    n = zr(t.summary),
    r = Br(t.id);
  switch (t.kind) {
    case `new`:
      return {
        id: r,
        kind: `new`,
        summary: {
          directory: Br(n.directory),
          destination: Br(n.destination),
          profileCount: Vr(n.profileCount),
        },
      };
    case `export`:
      return {
        id: r,
        kind: `export`,
        summary: {
          source: Br(n.source),
          destination: Br(n.destination),
          profileCount: Vr(n.profileCount),
          confirmedProfiles: Vr(n.confirmedProfiles),
          draftProfiles: Vr(n.draftProfiles),
          unclassifiedProfiles: Vr(n.unclassifiedProfiles),
        },
      };
    case `import`:
      if (n.mode !== `merge` && n.mode !== `replace`)
        throw Error(`Unknown Speaker Library import mode.`);
      return {
        id: r,
        kind: `import`,
        summary: {
          source: Br(n.source),
          destination: Br(n.destination),
          mode: n.mode,
          existingProfiles: Vr(n.existingProfiles),
          incomingProfiles: Vr(n.incomingProfiles),
          confirmedProfiles: Vr(n.confirmedProfiles),
          draftProfiles: Vr(n.draftProfiles),
          unclassifiedProfiles: Vr(n.unclassifiedProfiles),
          addedProfiles: Vr(n.addedProfiles),
          unchangedProfiles: Vr(n.unchangedProfiles),
          replacedProfiles: Vr(n.replacedProfiles),
          removedProfiles: Vr(n.removedProfiles),
          resultingProfiles: Vr(n.resultingProfiles),
          changed: Hr(n.changed),
          backupPath: Ur(n.backupPath),
        },
      };
    default:
      throw Error(`Unknown Speaker Library preview.`);
  }
}
var qr = {
    async preview(e) {
      let t = Kr(await Gr(`preview`, e));
      if (t.kind !== e.kind)
        throw Error(`Speaker Library preview does not match the requested operation.`);
      return t;
    },
    async confirm(e) {
      let t = zr(await Gr(`confirm`, { id: e, confirmed: !0 }));
      switch (t.kind) {
        case `export`:
          return { kind: `export`, receipt: Wr(t.receipt) };
        case `new`:
        case `import`:
          return { kind: t.kind, receipt: Wr(t.receipt), state: yt(t.state) };
        default:
          throw Error(`Unknown Speaker Library confirmation result.`);
      }
    },
    async load(e) {
      let t = zr(await Gr(`load`, { selection: e }));
      if (t.kind !== `load`) throw Error(`Invalid selected Speaker Library.`);
      return { kind: `load`, state: yt(t.state) };
    },
    async cancel(e) {
      Hr(zr(await Gr(`cancel`, { id: e })).cancelled);
    },
  },
  Jr = [`workbench`, `presets`];
function Yr(e, t, n, r) {
  let i = S(),
    a = rt({ queryKey: Jr, queryFn: Pr.presets, retry: !1 }),
    o = rt({ queryKey: [`speaker-directory-state`], queryFn: Ct.getState, enabled: !1 }),
    [s, c] = (0, y.useState)(Sr),
    [l, u] = (0, y.useState)(null),
    [d, f] = (0, y.useState)(``),
    [p, m] = (0, y.useState)(!1),
    h = (0, y.useRef)(!1),
    g = [...(a.data?.builtins ?? []), ...(a.data?.presets ?? [])],
    _ = g.find((e) => e.id === l),
    v = a.data?.presets.some((e) => e.id === l) ?? !1,
    b =
      _ != null &&
      (d.trim() !== _.preset.name || JSON.stringify(s) !== JSON.stringify(_.preset.settings)),
    x = B({
      mutationFn: async (e) => {
        (u(e?.id ?? null), f(e?.preset.name ?? ``), e && c(e.preset.settings));
        let t = await Pr.selectPreset(e?.id ?? null);
        i.setQueryData(Jr, t);
        let n = e?.preset.settings.speakerDirectory;
        if (n && e?.preset.settings.identifySpeakers) {
          let e = await qr.load(
            n.explicitPath ? { scope: `explicit`, path: n.explicitPath } : { scope: n.scope },
          );
          (i.setQueryData([`speaker-directory-state`], e.state),
            i.invalidateQueries({ queryKey: [`workbench`, `preflight`] }),
            i.invalidateQueries({ queryKey: [`workbench`, `capabilities`] }));
        }
        return t;
      },
    });
  (0, y.useEffect)(() => {
    if (h.current || !a.data || a.isFetching || n) return;
    h.current = !0;
    let e = g.find((e) => e.id === a.data?.activeId);
    e ? x.mutate(e, { onSettled: () => m(!0) }) : m(!0);
  }, [a.data, a.isFetching, n, g, x]);
  let C = B({
      mutationFn: (e) =>
        Pr.savePreset({
          ...(e && v && l ? { id: l } : {}),
          preset: { name: d.trim(), settings: s },
        }),
      onSuccess: (e) => {
        (i.setQueryData(Jr, e), u(e.activeId));
        let t = e.presets.find((t) => t.id === e.activeId);
        t && (c(t.preset.settings), f(t.preset.name));
      },
    }),
    w = B({
      mutationFn: Pr.deletePreset,
      onSuccess: (e) => {
        (i.setQueryData(Jr, e), u(null), f(``));
      },
    }),
    T = B({
      mutationFn: Pr.resetPresets,
      onSuccess: (e) => {
        (i.setQueryData(Jr, e), u(null), f(``));
      },
    }),
    E = x.isPending || C.isPending || w.isPending || T.isPending,
    D = t ? `` : e,
    O = rt({
      queryKey: [`workbench`, `preflight`, D, s],
      queryFn: () => Pr.preflight({ ...s, input: D }),
      enabled: p && r && !n && !E,
      retry: !1,
    });
  return {
    settings: s,
    setSettings: c,
    name: d,
    setName: f,
    selectedId: l,
    selected: _,
    custom: v,
    modified: b,
    initialized: p,
    presets: a,
    preflight: O,
    pending: E,
    directory: o.data,
    select: (e) => x.mutate(g.find((t) => t.id === e) ?? null),
    save: (e) => C.mutate(e),
    remove: () => {
      v && l && w.mutate(l);
    },
    recover: () => T.mutate(),
    error: a.error ?? x.error ?? C.error ?? w.error ?? T.error ?? O.error,
    backup: T.data?.backup,
  };
}
function Xr({ configuration: e, disabled: t }) {
  let n = e,
    r = n.selected
      ? `${n.modified ? `Modified preset` : n.custom ? `Saved preset` : `Preset`}: ${n.selected.preset.name}`
      : `Unsaved configuration`;
  return (0, b.jsxs)(`section`, {
    "aria-label": `Presets`,
    className: `presetControls`,
    children: [
      (0, b.jsxs)(`label`, {
        children: [
          `Preset`,
          (0, b.jsxs)(`select`, {
            "aria-label": `Preset`,
            disabled: t || n.pending || !n.initialized,
            value: n.selectedId ?? ``,
            onChange: (e) => n.select(e.currentTarget.value),
            children: [
              (0, b.jsx)(`option`, { value: ``, children: `Custom configuration` }),
              (0, b.jsx)(`optgroup`, {
                label: `Built-in · check local models`,
                children: n.presets.data?.builtins.map((e) =>
                  (0, b.jsxs)(
                    `option`,
                    {
                      value: e.id,
                      disabled: !e.supported,
                      children: [e.preset.name, e.supported ? `` : ` (unavailable in this build)`],
                    },
                    e.id,
                  ),
                ),
              }),
              (0, b.jsx)(`optgroup`, {
                label: `Saved presets`,
                children: n.presets.data?.presets.map((e) =>
                  (0, b.jsx)(`option`, { value: e.id, children: e.preset.name }, e.id),
                ),
              }),
            ],
          }),
        ],
      }),
      (0, b.jsx)(`p`, { className: `muted presetIndicator`, role: `status`, children: r }),
      (0, b.jsxs)(`details`, {
        children: [
          (0, b.jsx)(`summary`, { children: `Save or manage presets` }),
          (0, b.jsxs)(`label`, {
            children: [
              `Preset name`,
              (0, b.jsx)(`input`, {
                "aria-label": `Preset name`,
                value: n.name,
                maxLength: 128,
                disabled: t || n.pending,
                onChange: (e) => n.setName(e.currentTarget.value),
              }),
            ],
          }),
          (0, b.jsx)(`button`, {
            type: `button`,
            disabled: t || n.pending || !n.name.trim() || !!n.presets.data?.problem,
            onClick: () => n.save(!1),
            children: `Save preset`,
          }),
          n.custom
            ? (0, b.jsxs)(b.Fragment, {
                children: [
                  (0, b.jsx)(`button`, {
                    type: `button`,
                    disabled: t || n.pending || !n.modified,
                    onClick: () => n.save(!0),
                    children: `Update preset`,
                  }),
                  (0, b.jsx)(`button`, {
                    type: `button`,
                    disabled: t || n.pending,
                    onClick: n.remove,
                    children: `Delete preset`,
                  }),
                ],
              })
            : null,
          n.settings.identifySpeakers && n.directory
            ? (0, b.jsxs)(`label`, {
                children: [
                  (0, b.jsx)(`input`, {
                    type: `checkbox`,
                    checked: n.settings.speakerDirectory !== null,
                    disabled: t || n.pending,
                    onChange: (e) => {
                      let t = e.currentTarget.checked,
                        r = n.directory;
                      n.setSettings((e) => ({
                        ...e,
                        speakerDirectory:
                          t && r
                            ? {
                                scope: r.scope === `explicit` ? `auto` : r.scope,
                                explicitPath: r.scope === `explicit` ? r.path : null,
                              }
                            : null,
                      }));
                    },
                  }),
                  `Remember active speaker index in this preset`,
                ],
              })
            : null,
          n.settings.speakerDirectory
            ? (0, b.jsxs)(b.Fragment, {
                children: [
                  (0, b.jsxs)(`p`, {
                    className: `muted`,
                    children: [
                      `Preset speaker index:`,
                      ` `,
                      n.settings.speakerDirectory.explicitPath ?? n.settings.speakerDirectory.scope,
                    ],
                  }),
                  (0, b.jsx)(`button`, {
                    type: `button`,
                    disabled: t || n.pending,
                    onClick: () => n.setSettings((e) => ({ ...e, speakerDirectory: null })),
                    children: `Use active speaker index instead`,
                  }),
                ],
              })
            : null,
        ],
      }),
      n.presets.data?.problem
        ? (0, b.jsxs)(`div`, {
            role: `alert`,
            children: [
              (0, b.jsx)(`p`, { children: n.presets.data.problem }),
              n.presets.data.canReset
                ? (0, b.jsx)(`button`, {
                    type: `button`,
                    disabled: t || n.pending,
                    onClick: n.recover,
                    children: `Back up and reset saved presets`,
                  })
                : null,
            ],
          })
        : null,
      n.backup
        ? (0, b.jsxs)(`p`, {
            role: `status`,
            children: [`Original presets backed up to `, n.backup],
          })
        : null,
    ],
  });
}
function Zr(e) {
  if (!e || typeof e != `object` || Array.isArray(e)) throw Error(`Invalid Jobs response.`);
  return Object.fromEntries(Object.entries(e));
}
function Qr(e) {
  if (typeof e != `string`) throw Error(`Invalid Jobs text.`);
  return e;
}
function $r(e) {
  if (typeof e != `boolean`) throw Error(`Invalid Jobs setting.`);
  return e;
}
function ei(e) {
  return e == null ? null : Qr(e);
}
function ti(e) {
  if (!Array.isArray(e)) throw Error(`Invalid Jobs list.`);
  return e;
}
function ni(e) {
  switch (e) {
    case `queued`:
    case `running`:
    case `completed`:
    case `failed`:
    case `cancelled`:
    case `notRun`:
    case `interrupted`:
      return e;
    default:
      throw Error(`Unknown Jobs status.`);
  }
}
function ri(e) {
  return {
    reviews: ti(Zr(e).reviews ?? []).map((e) => {
      let t = Zr(e);
      return {
        resultId: Qr(t.resultId),
        originalResultId: Qr(t.originalResultId),
        input: Qr(t.input),
        outputs: ti(t.outputs).map((e) => {
          let t = Zr(e);
          return { path: Qr(t.path), format: Qr(t.format) };
        }),
      };
    }),
    jobs: ti(Zr(e).jobs).map((e) => {
      let t = Zr(e),
        n = Zr(t.settings);
      return {
        id: Qr(t.id),
        status: ni(t.status),
        error: ei(t.error),
        speakerDirectory: Qr(t.speakerDirectory),
        settings: {
          model: Qr(n.model),
          device: Qr(n.device),
          align: $r(n.align),
          identifySpeakers: $r(n.identifySpeakers),
          translate: n.translation != null,
        },
        files: ti(t.files).map((e) => {
          let t = Zr(e);
          if (
            t.seconds != null &&
            (typeof t.seconds != `number` || !Number.isFinite(t.seconds) || t.seconds < 0)
          )
            throw Error(`Invalid job duration.`);
          return {
            input: Qr(t.input),
            status: ni(t.status),
            currentTask: ei(t.currentTask),
            seconds: t.seconds == null ? null : t.seconds,
            error: ei(t.error),
            tasks: Object.fromEntries(
              Object.entries(Zr(t.tasks)).map(([e, t]) => [e, { status: ni(Zr(t).status) }]),
            ),
            outputs: ti(t.outputs).map((e) => {
              let t = Zr(e);
              return { path: Qr(t.path), format: Qr(t.format) };
            }),
          };
        }),
      };
    }),
  };
}
async function ii(e, t) {
  let n = await fetch(`/api/workbench/${e}`, {
    method: t === void 0 ? `GET` : `POST`,
    signal: AbortSignal.timeout(3e4),
    headers: {
      "X-Native-Whisperx-Session-Token": window.nativeWhisperxSessionToken ?? ``,
      ...(t === void 0 ? {} : { "Content-Type": `application/json` }),
    },
    ...(t === void 0 ? {} : { body: JSON.stringify(t) }),
  });
  if (!n.ok) throw Error(await n.text());
  return n.json();
}
var ai = {
    async history() {
      return ri(await ii(`jobs`));
    },
    async queue(e) {
      return ri(await ii(`queue`, e));
    },
    async retry(e) {
      return ri(await ii(`retry`, e));
    },
    async result(e) {
      let t = Zr(await ii(`job-result`, e));
      return {
        path: Qr(t.path),
        text: Qr(t.text),
        content: Qr(t.content),
        transcript: t.transcript == null ? null : vr(t.transcript),
      };
    },
  },
  oi = [`workbench`, `jobs`],
  si = {
    queued: `Queued`,
    running: `Running`,
    completed: `Completed`,
    failed: `Failed`,
    cancelled: `Cancelled`,
    notRun: `Not run`,
    interrupted: `Interrupted`,
  },
  ci = {
    decode: `Read media`,
    vad: `Detect speech`,
    transcribe: `Transcribe`,
    align: `Align timestamps`,
    identifySpeakers: `Identify speakers`,
    translate: `Translate`,
    writeOutputs: `Write outputs`,
  };
function li(e) {
  return e.split(/[\\/]/).at(-1) ?? e;
}
function ui(e) {
  return e.length <= 180
    ? e
    : /decode|WAV|RIFF/i.test(e)
      ? `Could not read this media file. Check its format before retrying.`
      : /model/i.test(e)
        ? `A required model could not be loaded. Check local model resources before retrying.`
        : `Native processing failed. Open error details before retrying.`;
}
function di({ message: e }) {
  let t = ui(e);
  return (0, b.jsxs)(b.Fragment, {
    children: [
      (0, b.jsx)(`p`, { children: t }),
      t === e
        ? null
        : (0, b.jsxs)(`details`, {
            children: [
              (0, b.jsx)(`summary`, { children: `Error details` }),
              (0, b.jsx)(`pre`, { className: `resultContent`, children: e }),
            ],
          }),
    ],
  });
}
function fi() {
  let e = S(),
    [t, n] = (0, y.useState)(``),
    [r, i] = (0, y.useState)(20),
    a = rt({
      queryKey: oi,
      queryFn: ai.history,
      retry: !1,
      refetchInterval: (e) =>
        e.state.data?.jobs.some((e) => e.status === `running` || e.status === `queued`) ? 750 : !1,
    }),
    o = B({ mutationFn: ai.retry, onSuccess: (t) => e.setQueryData(oi, t) }),
    s = B({
      mutationFn: Pr.cancel,
      onSuccess: () => {
        (n(`Cancellation requested; waiting for a native safe boundary.`),
          e.invalidateQueries({ queryKey: oi }));
      },
    }),
    c = B({ mutationFn: ai.result }),
    l = a.data?.jobs.some((e) => e.status === `running` || e.status === `queued`) ?? !1,
    u = a.error ?? o.error ?? s.error ?? c.error;
  return (0, b.jsxs)(`main`, {
    className: `page jobsPage`,
    children: [
      (0, b.jsxs)(`header`, {
        children: [
          (0, b.jsx)(`h1`, { children: `Jobs` }),
          (0, b.jsx)(`p`, {
            children: `Files run sequentially with the task settings recorded below. Completed outputs remain available after reopening this local workbench.`,
          }),
        ],
      }),
      u ? (0, b.jsx)(`p`, { role: `alert`, children: u.message }) : null,
      t ? (0, b.jsx)(`p`, { role: `status`, children: t }) : null,
      a.isPending ? (0, b.jsx)(`p`, { children: `Loading jobs…` }) : null,
      a.data?.jobs.length === 0
        ? (0, b.jsxs)(`p`, {
            children: [
              `No jobs yet. `,
              (0, b.jsx)(`a`, {
                href: `?view=transcribe`,
                children: `Select several files or a folder`,
              }),
              ` to start a batch.`,
            ],
          })
        : null,
      l
        ? (0, b.jsx)(`button`, {
            type: `button`,
            disabled: s.isPending,
            onClick: () => s.mutate(),
            children: `Cancel current batch`,
          })
        : null,
      a.data?.jobs.slice(0, r).map((e) =>
        (0, b.jsxs)(
          `section`,
          {
            className: `jobBatch`,
            "aria-label": `Batch ${e.id}`,
            children: [
              (0, b.jsx)(`h2`, { children: si[e.status] }),
              (0, b.jsxs)(`p`, {
                className: `muted`,
                children: [
                  e.settings.model,
                  ` · `,
                  e.settings.device,
                  ` · Transcribe`,
                  e.settings.align ? ` → Align` : ``,
                  e.settings.identifySpeakers ? ` → Identify speakers` : ``,
                  e.settings.translate ? ` → Translate` : ``,
                ],
              }),
              (0, b.jsxs)(`p`, {
                className: `path`,
                children: [`Speaker index: `, e.speakerDirectory],
              }),
              e.error && !e.files.some((e) => e.error)
                ? (0, b.jsx)(di, { message: e.error })
                : null,
              (0, b.jsx)(`div`, {
                className: `jobsTableScroll`,
                children: (0, b.jsxs)(`table`, {
                  children: [
                    (0, b.jsx)(`thead`, {
                      children: (0, b.jsxs)(`tr`, {
                        children: [
                          (0, b.jsx)(`th`, { children: `File` }),
                          (0, b.jsx)(`th`, { children: `Status / stage` }),
                          (0, b.jsx)(`th`, { children: `Run time` }),
                          (0, b.jsx)(`th`, { children: `Outputs / action` }),
                        ],
                      }),
                    }),
                    (0, b.jsx)(`tbody`, {
                      children: e.files.map((t, r) =>
                        (0, b.jsxs)(
                          `tr`,
                          {
                            children: [
                              (0, b.jsx)(`th`, {
                                scope: `row`,
                                className: `path`,
                                children: t.input,
                              }),
                              (0, b.jsxs)(`td`, {
                                children: [
                                  (0, b.jsxs)(`span`, {
                                    role: t.status === `running` ? `status` : void 0,
                                    children: [
                                      si[t.status],
                                      t.currentTask
                                        ? ` · ${ci[t.currentTask] ?? t.currentTask}…`
                                        : ``,
                                    ],
                                  }),
                                  t.error ? (0, b.jsx)(di, { message: t.error }) : null,
                                  Object.keys(t.tasks).length
                                    ? (0, b.jsxs)(`details`, {
                                        children: [
                                          (0, b.jsx)(`summary`, { children: `Task outcomes` }),
                                          (0, b.jsx)(`ul`, {
                                            children: Object.entries(t.tasks).map(([e, t]) =>
                                              (0, b.jsxs)(
                                                `li`,
                                                { children: [ci[e] ?? e, `: `, si[t.status]] },
                                                e,
                                              ),
                                            ),
                                          }),
                                        ],
                                      })
                                    : null,
                                ],
                              }),
                              (0, b.jsx)(`td`, {
                                children: t.seconds == null ? `—` : `${t.seconds.toFixed(2)} s`,
                              }),
                              (0, b.jsxs)(`td`, {
                                children: [
                                  t.status === `completed` &&
                                  t.outputs.some((e) => [`json`, `native-json`].includes(e.format))
                                    ? (0, b.jsx)(`p`, {
                                        children: (0, b.jsx)(`a`, {
                                          href: `?view=review&result=${encodeURIComponent(`${e.id}:${r}`)}`,
                                          children: `Review & Edit`,
                                        }),
                                      })
                                    : null,
                                  t.status === `completed`
                                    ? t.outputs.map((t, n) =>
                                        (0, b.jsxs)(
                                          `button`,
                                          {
                                            type: `button`,
                                            disabled: c.isPending,
                                            onClick: () =>
                                              c.mutate({
                                                batchId: e.id,
                                                fileIndex: r,
                                                outputIndex: n,
                                              }),
                                            children: [`Open `, li(t.path)],
                                          },
                                          `${n}:${t.path}`,
                                        ),
                                      )
                                    : null,
                                  [`failed`, `cancelled`, `interrupted`, `notRun`].includes(
                                    t.status,
                                  )
                                    ? (0, b.jsxs)(`button`, {
                                        type: `button`,
                                        disabled: l || o.isPending,
                                        onClick: () => {
                                          (n(`Retry requested for ${li(t.input)}.`),
                                            o.mutate({ batchId: e.id, fileIndex: r }));
                                        },
                                        children: [
                                          t.status === `notRun` ? `Run` : `Retry`,
                                          ` `,
                                          li(t.input),
                                        ],
                                      })
                                    : null,
                                ],
                              }),
                            ],
                          },
                          `${r}:${t.input}`,
                        ),
                      ),
                    }),
                  ],
                }),
              }),
              (0, b.jsx)(`p`, {
                className: `muted`,
                children: `Retry runs only the selected file with its recorded task settings and a new output folder. The original batch stays unchanged.`,
              }),
            ],
          },
          e.id,
        ),
      ),
      a.data && a.data.jobs.length > r
        ? (0, b.jsx)(`button`, {
            type: `button`,
            onClick: () => i((e) => e + 20),
            children: `Show older batches`,
          })
        : null,
      c.data
        ? (0, b.jsxs)(`section`, {
            "aria-label": `Job result`,
            children: [
              (0, b.jsx)(`h2`, { children: `Result` }),
              (0, b.jsx)(`p`, { className: `path`, children: c.data.path }),
              (0, b.jsx)(Rr, { transcript: c.data.transcript, fallbackText: c.data.text }),
              (0, b.jsxs)(`details`, {
                children: [
                  (0, b.jsx)(`summary`, { children: `Exact output file` }),
                  (0, b.jsx)(`pre`, { className: `resultContent`, children: c.data.content }),
                ],
              }),
            ],
          })
        : null,
    ],
  });
}
var pi = [`json`, `srt`, `vtt`, `txt`, `tsv`, `native-json`, `aud`, `all`],
  mi = [`workbench`, `current-job`],
  hi = {
    pending: `Waiting`,
    running: `Running`,
    completed: `Completed`,
    failed: `Failed`,
    cancelled: `Cancelled`,
    notRun: `Not run`,
  },
  gi = {
    decode: `Read media`,
    vad: `Detect speech`,
    transcribe: `Transcribe`,
    align: `Align timestamps`,
    identifySpeakers: `Identify speakers`,
    translate: `Translate`,
    writeOutputs: `Write outputs`,
  };
function _i({ speakerIndex: e }) {
  let t = S(),
    n = rt({ queryKey: [`workbench`, `capabilities`], queryFn: Pr.capabilities, retry: !1 }),
    r = rt({
      queryKey: mi,
      queryFn: Pr.state,
      retry: !1,
      refetchInterval: (e) =>
        [`running`, `cancelling`].includes(e.state.data?.status ?? ``) ? 750 : !1,
    }),
    [i, a] = (0, y.useState)(``),
    [o, s] = (0, y.useState)(!1),
    [c, l] = (0, y.useState)(``),
    [u, d] = (0, y.useState)(!1),
    f = B({ mutationFn: Pr.files }),
    p = B({
      mutationFn: Pr.run,
      onSuccess: (e) => {
        (t.setQueryData(mi, e), g.reset());
      },
    }),
    m = B({
      mutationFn: ai.queue,
      onSuccess: (e) => {
        (t.setQueryData(oi, e), t.invalidateQueries({ queryKey: mi }), g.reset());
      },
    }),
    h = B({ mutationFn: Pr.cancel, onSuccess: (e) => t.setQueryData(mi, e) }),
    g = B({ mutationFn: Pr.result });
  (0, y.useEffect)(() => {
    [`completed`, `failed`, `cancelled`].includes(r.data?.status ?? ``) &&
      t.invalidateQueries({ queryKey: [`speaker-directory-state`] });
  }, [t, r.data?.status]);
  let _ = p.isPending || m.isPending || [`running`, `cancelling`].includes(r.data?.status ?? ``),
    v = n.data,
    x = Yr(i, o, _, v?.native ?? !1),
    { settings: C, setSettings: w } = x,
    { align: T, identifySpeakers: E, autoUpdateSpeakers: D, model: O, device: ee, formats: te } = C,
    k = C.language ?? ``,
    A = C.modelDir ?? ``,
    j = C.outputDir ?? ``,
    ne = C.vadBundle ?? ``,
    M = C.diarizationBundle ?? ``,
    N = C.translation !== null,
    P = C.translation?.targetLanguage ?? `de`,
    re = C.translation?.modelId ?? ``,
    F = _ || x.pending || !x.initialized;
  function I(e, t) {
    w((n) => ({ ...n, [e]: t }));
  }
  function ie(e) {
    w((t) => ({ ...t, identifySpeakers: e, autoUpdateSpeakers: e && t.autoUpdateSpeakers }));
  }
  function ae(e) {
    w((t) => {
      let n = t.language || `en`,
        r = n === `de` ? `en` : `de`;
      return {
        ...t,
        language: e ? n : t.language,
        translation: e
          ? { modelId: `Helsinki-NLP/opus-mt-${n}-${r}`, sourceLanguage: n, targetLanguage: r }
          : null,
      };
    });
  }
  function oe(e) {
    w((t) => ({
      ...t,
      language: e || null,
      translation: t.translation ? { ...t.translation, sourceLanguage: e } : null,
    }));
  }
  function se(e, t) {
    w((n) => ({ ...n, translation: n.translation ? { ...n.translation, [e]: t } : null }));
  }
  function ce(e, t) {
    w((n) => ({ ...n, formats: t ? [...n.formats, e] : n.formats.filter((t) => t !== e) }));
  }
  let le = x.error ?? n.error ?? r.error ?? p.error ?? m.error ?? h.error ?? f.error ?? g.error;
  return (0, b.jsxs)(`main`, {
    className: `page workbench`,
    children: [
      (0, b.jsx)(`h1`, { children: `Transcribe` }),
      le ? (0, b.jsx)(`p`, { role: `alert`, children: le.message }) : null,
      m.isSuccess
        ? (0, b.jsxs)(`p`, {
            role: `status`,
            children: [
              `Batch added. `,
              (0, b.jsx)(`a`, { href: `?view=jobs`, children: `View jobs` }),
            ],
          })
        : null,
      (0, b.jsx)(Xr, { configuration: x, disabled: F }),
      (0, b.jsxs)(`form`, {
        className: `workbenchForm`,
        onSubmit: (e) => {
          e.preventDefault();
          let t = { ...C, input: i };
          o
            ? m.mutate({
                inputs: [
                  i,
                  ...c
                    .split(`
`)
                    .map((e) => e.trim())
                    .filter(Boolean),
                ],
                settings: t,
              })
            : p.mutate(t);
        },
        children: [
          (0, b.jsxs)(`fieldset`, {
            className: `inputOptions`,
            disabled: F || !v,
            children: [
              (0, b.jsx)(`legend`, { children: `Input` }),
              (0, b.jsxs)(`label`, {
                children: [
                  (0, b.jsx)(`input`, {
                    type: `checkbox`,
                    checked: o,
                    onChange: (e) => s(e.currentTarget.checked),
                  }),
                  `Process several files or a folder`,
                ],
              }),
              (0, b.jsxs)(`label`, {
                children: [
                  `Audio or video file`,
                  (0, b.jsx)(`input`, {
                    required: !0,
                    value: i,
                    onChange: (e) => a(e.currentTarget.value),
                    placeholder: `recording.wav`,
                  }),
                ],
              }),
              o
                ? (0, b.jsxs)(b.Fragment, {
                    children: [
                      (0, b.jsxs)(`label`, {
                        children: [
                          `Additional files or folders (one per line)`,
                          (0, b.jsx)(`textarea`, {
                            value: c,
                            onChange: (e) => l(e.currentTarget.value),
                          }),
                        ],
                      }),
                      (0, b.jsx)(`p`, {
                        className: `muted`,
                        children: `Folders add supported media in filename order, without subfolders. Up to 100 files run sequentially. Outputs use fresh run folders beside each input or inside your selected output folder.`,
                      }),
                    ],
                  })
                : null,
              (0, b.jsx)(`button`, {
                type: `button`,
                onClick: () => {
                  (d(!u), u || f.mutate(`.`));
                },
                children: `Browse files`,
              }),
              u && f.data
                ? (0, b.jsxs)(`div`, {
                    className: `filePicker`,
                    "aria-label": `Local files`,
                    children: [
                      (0, b.jsx)(`button`, {
                        type: `button`,
                        onClick: () => f.mutate(f.data?.directory ? `${f.data.directory}/..` : `.`),
                        children: `Parent folder`,
                      }),
                      o
                        ? (0, b.jsx)(`button`, {
                            type: `button`,
                            onClick: () => {
                              (a(f.data?.directory || `.`), d(!1));
                            },
                            children: `Use this folder`,
                          })
                        : null,
                      (0, b.jsx)(`ul`, {
                        children: f.data.entries.map((e) =>
                          (0, b.jsx)(
                            `li`,
                            {
                              children: (0, b.jsxs)(`button`, {
                                type: `button`,
                                onClick: () => {
                                  e.directory ? f.mutate(e.path) : (a(e.path), d(!1));
                                },
                                children: [e.name, e.directory ? `/` : ``],
                              }),
                            },
                            e.path,
                          ),
                        ),
                      }),
                      f.data.truncated
                        ? (0, b.jsx)(`p`, {
                            children: `More files exist. Enter a file path above.`,
                          })
                        : null,
                    ],
                  })
                : null,
            ],
          }),
          (0, b.jsxs)(`fieldset`, {
            className: `taskOptions`,
            disabled: F || !v,
            children: [
              (0, b.jsx)(`legend`, { children: `Tasks · in order` }),
              (0, b.jsx)(`p`, { children: `1. Transcribe` }),
              (0, b.jsxs)(`label`, {
                children: [
                  `Source language`,
                  (0, b.jsx)(`input`, {
                    list: `source-language-choices`,
                    required: N,
                    value: k,
                    onChange: (e) => oe(e.currentTarget.value),
                    placeholder: `Auto-detect`,
                  }),
                ],
              }),
              (0, b.jsx)(`datalist`, {
                id: `source-language-choices`,
                children: Ot(N ? Tt : wt).map(({ code: e, label: t }) =>
                  (0, b.jsx)(`option`, { value: e, children: t }, e),
                ),
              }),
              (0, b.jsxs)(`label`, {
                children: [
                  (0, b.jsx)(`input`, {
                    type: `checkbox`,
                    checked: T,
                    disabled: !v?.align && !T,
                    onChange: (e) => I(`align`, e.currentTarget.checked),
                  }),
                  `Align timestamps`,
                ],
              }),
              (0, b.jsxs)(`label`, {
                children: [
                  (0, b.jsx)(`input`, {
                    type: `checkbox`,
                    checked: E,
                    disabled: !v?.identifySpeakers && !E,
                    onChange: (e) => ie(e.currentTarget.checked),
                  }),
                  `Diarize speakers (pyannote)`,
                ],
              }),
              E && v?.identifySpeakers
                ? (0, b.jsx)(Fr, {
                    onPrepared: ({ bundlePath: e, vadBundlePath: t }) =>
                      w((n) => ({ ...n, diarizationBundle: e, vadBundle: t })),
                  })
                : null,
              E
                ? (0, b.jsxs)(`label`, {
                    children: [
                      (0, b.jsx)(`input`, {
                        type: `checkbox`,
                        checked: D,
                        onChange: (e) => I(`autoUpdateSpeakers`, e.currentTarget.checked),
                      }),
                      `Auto-update with new speakers (save compatible voices locally as drafts)`,
                    ],
                  })
                : null,
              v?.identifySpeakers
                ? null
                : (0, b.jsx)(`p`, {
                    className: `muted`,
                    children: `Speaker identification is unavailable in this build.`,
                  }),
              (0, b.jsxs)(`label`, {
                children: [
                  (0, b.jsx)(`input`, {
                    type: `checkbox`,
                    checked: N,
                    disabled: !v?.translate && !N,
                    onChange: (e) => ae(e.currentTarget.checked),
                  }),
                  `Translate`,
                ],
              }),
              N
                ? (0, b.jsx)(`div`, {
                    className: `settingsRow taskLanguages`,
                    children: (0, b.jsxs)(`label`, {
                      children: [
                        `Target language`,
                        (0, b.jsx)(`select`, {
                          required: !0,
                          value: P,
                          onChange: (e) => se(`targetLanguage`, e.currentTarget.value),
                          children: Ot(Tt).map(({ code: e, label: t }) =>
                            (0, b.jsx)(`option`, { value: e, children: t }, e),
                          ),
                        }),
                      ],
                    }),
                  })
                : null,
              e
                ? null
                : (0, b.jsxs)(`p`, {
                    children: [
                      `Active Speaker Directory:`,
                      ` `,
                      (0, b.jsx)(`span`, {
                        className: `mono`,
                        children: v?.speakerDirectory ?? `Loading…`,
                      }),
                    ],
                  }),
            ],
          }),
          (0, b.jsxs)(`fieldset`, {
            className: `outputOptions`,
            disabled: F || !v,
            children: [
              (0, b.jsx)(`legend`, { children: `Output` }),
              (0, b.jsx)(`p`, {
                className: `muted`,
                children: `A canonical Native JSON result is also saved for Review & Edit.`,
              }),
              (0, b.jsx)(`div`, {
                className: `formatChoices`,
                children: pi.map((e) =>
                  (0, b.jsxs)(
                    `label`,
                    {
                      children: [
                        (0, b.jsx)(`input`, {
                          type: `checkbox`,
                          checked: te.includes(e),
                          onChange: (t) => ce(e, t.currentTarget.checked),
                        }),
                        e.toUpperCase(),
                      ],
                    },
                    e,
                  ),
                ),
              }),
            ],
          }),
          (0, b.jsxs)(`details`, {
            className: `advancedOptions`,
            children: [
              (0, b.jsx)(`summary`, { children: `Advanced settings` }),
              (0, b.jsxs)(`div`, {
                className: `settingsRow`,
                children: [
                  (0, b.jsxs)(`label`, {
                    children: [
                      `Output folder`,
                      (0, b.jsx)(`input`, {
                        value: j,
                        onChange: (e) => I(`outputDir`, e.currentTarget.value || null),
                        placeholder: `Beside input`,
                      }),
                    ],
                  }),
                  v
                    ? (0, b.jsxs)(`p`, {
                        className: `muted`,
                        children: [`Files and output folders in `, v.root],
                      })
                    : null,
                  (0, b.jsx)(`p`, {
                    className: `muted`,
                    children: `Models must already be cached; this app does not download them.`,
                  }),
                  N
                    ? (0, b.jsxs)(`label`, {
                        children: [
                          `Translation model`,
                          (0, b.jsx)(`input`, {
                            disabled: F,
                            required: !0,
                            value: re,
                            onChange: (e) => se(`modelId`, e.currentTarget.value),
                          }),
                        ],
                      })
                    : null,
                  (0, b.jsxs)(`label`, {
                    children: [
                      `Model cache folder`,
                      (0, b.jsx)(`input`, {
                        disabled: F,
                        value: A,
                        onChange: (e) => I(`modelDir`, e.currentTarget.value || null),
                        placeholder: `Standard model cache`,
                      }),
                    ],
                  }),
                  (0, b.jsxs)(b.Fragment, {
                    children: [
                      (0, b.jsxs)(`label`, {
                        children: [
                          `Verified VAD bundle`,
                          (0, b.jsx)(`input`, {
                            disabled: F,
                            value: ne,
                            onChange: (e) => I(`vadBundle`, e.currentTarget.value || null),
                            placeholder: `Auto-select cached bundle`,
                          }),
                        ],
                      }),
                      (0, b.jsxs)(`label`, {
                        children: [
                          `Verified diarization bundle`,
                          (0, b.jsx)(`input`, {
                            disabled: F,
                            value: M,
                            onChange: (e) => I(`diarizationBundle`, e.currentTarget.value || null),
                            placeholder: `Auto-select cached bundle`,
                          }),
                        ],
                      }),
                    ],
                  }),
                  (0, b.jsxs)(`label`, {
                    children: [
                      `Transcription model`,
                      (0, b.jsx)(`input`, {
                        disabled: F,
                        required: !0,
                        value: O,
                        onChange: (e) => I(`model`, e.currentTarget.value),
                      }),
                    ],
                  }),
                  (0, b.jsxs)(`label`, {
                    children: [
                      `Device`,
                      (0, b.jsxs)(`select`, {
                        disabled: F,
                        value: ee,
                        onChange: (e) => I(`device`, Er(e.currentTarget.value)),
                        children: [
                          (0, b.jsx)(`option`, { value: `auto`, children: `Automatic` }),
                          (0, b.jsx)(`option`, { value: `cpu`, children: `CPU` }),
                          (0, b.jsxs)(`option`, {
                            value: `cuda`,
                            disabled: !v?.cuda,
                            children: [`CUDA`, v?.cuda ? `` : ` (unavailable)`],
                          }),
                        ],
                      }),
                    ],
                  }),
                ],
              }),
            ],
          }),
          (0, b.jsxs)(`section`, {
            "aria-label": `Resource preflight`,
            className: `resourcePreflight`,
            children: [
              x.preflight.isFetching
                ? (0, b.jsx)(`p`, { role: `status`, children: `Checking local resources…` })
                : x.preflight.data?.ready
                  ? (0, b.jsx)(`p`, {
                      className: `muted`,
                      children: `Local resource checks passed. Model execution is validated when the run starts.`,
                    })
                  : null,
              x.preflight.data && !x.preflight.data.ready
                ? (0, b.jsxs)(`div`, {
                    role: `alert`,
                    children: [
                      (0, b.jsx)(`ul`, {
                        children: x.preflight.data.checks
                          .filter((e) => !e.available)
                          .map((e) =>
                            (0, b.jsxs)(
                              `li`,
                              { children: [e.resource, `: `, e.message] },
                              e.resource,
                            ),
                          ),
                      }),
                      (0, b.jsx)(`button`, {
                        type: `button`,
                        disabled: x.preflight.isFetching || F,
                        onClick: () => void x.preflight.refetch(),
                        children: `Recheck resources`,
                      }),
                    ],
                  })
                : null,
            ],
          }),
          (0, b.jsx)(`button`, {
            className: `startRun`,
            type: `submit`,
            disabled:
              F ||
              !v?.native ||
              !i.trim() ||
              te.length === 0 ||
              !x.preflight.data?.ready ||
              x.preflight.isFetching,
            children: `Start`,
          }),
          !v?.native && v
            ? (0, b.jsx)(`p`, {
                role: `alert`,
                children: `Native transcription is unavailable in this build.`,
              })
            : null,
        ],
      }),
      e,
      r.data && r.data.status !== `idle`
        ? (0, b.jsxs)(`section`, {
            "aria-label": `Current transcription`,
            children: [
              (0, b.jsx)(`h2`, { children: r.data.status }),
              r.data.input ? (0, b.jsx)(`p`, { className: `mono`, children: r.data.input }) : null,
              r.data.currentTask
                ? (0, b.jsx)(`p`, {
                    role: `status`,
                    children: gi[r.data.currentTask] ?? r.data.currentTask,
                  })
                : null,
              r.data.activity && _ ? (0, b.jsx)(`p`, { children: r.data.activity }) : null,
              (0, b.jsx)(`ol`, {
                children: Object.keys(gi)
                  .filter((e) => r.data?.tasks[e])
                  .map((e) =>
                    (0, b.jsxs)(
                      `li`,
                      {
                        children: [
                          gi[e] ?? e,
                          `:`,
                          ` `,
                          hi[r.data?.tasks[e]?.status ?? ``] ?? `Unknown`,
                        ],
                      },
                      e,
                    ),
                  ),
              }),
              r.data.error ? (0, b.jsx)(`p`, { role: `alert`, children: r.data.error }) : null,
              _
                ? (0, b.jsx)(`button`, {
                    type: `button`,
                    disabled: h.isPending || r.data.status === `cancelling`,
                    onClick: () => h.mutate(),
                    children: `Cancel`,
                  })
                : null,
              r.data.status === `cancelling`
                ? (0, b.jsx)(`p`, { children: `Stopping at the next safe boundary.` })
                : null,
              r.data.status === `completed`
                ? r.data.outputs.map((e, t) =>
                    (0, b.jsxs)(
                      `p`,
                      {
                        children: [
                          (0, b.jsx)(`span`, { className: `mono`, children: e.path }),
                          ` `,
                          (0, b.jsx)(`button`, {
                            type: `button`,
                            onClick: () => g.mutate(t),
                            children: t === 0 ? `Open result` : `Open ${e.format.toUpperCase()}`,
                          }),
                        ],
                      },
                      e.path,
                    ),
                  )
                : null,
            ],
          })
        : null,
      r.data?.status === `completed` && r.data.reviewId
        ? (0, b.jsx)(`p`, {
            children: (0, b.jsx)(`a`, {
              href: `?view=review&result=${encodeURIComponent(r.data.reviewId)}`,
              children: `Review & Edit this result`,
            }),
          })
        : null,
      g.data
        ? (0, b.jsxs)(`section`, {
            "aria-label": `Transcript result`,
            children: [
              (0, b.jsx)(`h2`, { children: `Result` }),
              (0, b.jsx)(Rr, { transcript: g.data.transcript, fallbackText: g.data.text }),
              (0, b.jsxs)(`details`, {
                children: [
                  (0, b.jsx)(`summary`, { children: g.data.path }),
                  (0, b.jsx)(`pre`, { children: g.data.content }),
                ],
              }),
            ],
          })
        : null,
    ],
  });
}
var vi = JSON.stringify({ transcript: {}, source: {} });
function yi(e) {
  return {
    text: e.text,
    start: e.startSeconds == null ? `` : String(e.startSeconds),
    end: e.endSeconds == null ? `` : String(e.endSeconds),
    speaker: e.speaker ?? ``,
  };
}
function bi(e, t) {
  return Object.entries(e).map(([e, n]) => {
    if (!t?.segments.some((t) => t.id === e))
      throw Error(`Selected segment no longer exists. Reopen the result.`);
    return {
      segmentId: e,
      text: n.text,
      startSeconds: Ir(n.start),
      endSeconds: Ir(n.end),
      speaker: n.speaker || null,
    };
  });
}
function xi(e) {
  return `?view=review&result=${encodeURIComponent(e)}`;
}
function Si({ api: e = Ct }) {
  let t = new URLSearchParams(window.location.search).get(`result`),
    n = rt({
      queryKey: [`review`, t],
      queryFn: () => xr.open(t ?? ``),
      enabled: !!t,
      retry: !1,
      staleTime: 1 / 0,
      refetchOnWindowFocus: !1,
    });
  return t
    ? n.isPending
      ? (0, b.jsxs)(`main`, {
          className: `page reviewPage`,
          children: [
            (0, b.jsx)(`h1`, { children: `Review & Edit` }),
            (0, b.jsx)(`p`, { children: `Opening recorded result…` }),
          ],
        })
      : n.error || !n.data
        ? (0, b.jsxs)(`main`, {
            className: `page reviewPage`,
            children: [
              (0, b.jsx)(`h1`, { children: `Review & Edit` }),
              (0, b.jsx)(`p`, {
                role: `alert`,
                children: n.error?.message ?? `Result is unavailable.`,
              }),
              (0, b.jsx)(`a`, { href: `?view=jobs`, children: `Open a completed result in Jobs` }),
              (0, b.jsx)(`p`, {
                children: (0, b.jsx)(`a`, {
                  href: `?view=transcribe`,
                  children: `Transcribe a recording`,
                }),
              }),
            ],
          })
        : (0, b.jsx)(wi, { original: n.data, api: e }, n.data.snapshotId)
    : (0, b.jsx)(Ci, {});
}
function Ci() {
  let e = rt({ queryKey: oi, queryFn: ai.history, retry: !1 }),
    [t, n] = (0, y.useState)(20);
  return (0, b.jsxs)(`main`, {
    className: `page reviewPage`,
    children: [
      (0, b.jsx)(`h1`, { children: `Review & Edit` }),
      e.data?.reviews.length
        ? (0, b.jsxs)(`section`, {
            children: [
              (0, b.jsx)(`h2`, { children: `Corrected copies` }),
              (0, b.jsx)(`ul`, {
                children: e.data.reviews.slice(0, t).map((e) =>
                  (0, b.jsx)(
                    `li`,
                    {
                      children: (0, b.jsx)(`a`, {
                        href: xi(e.resultId),
                        children:
                          e.outputs
                            .find((e) => e.format === `native-json`)
                            ?.path.split(/[\\/]/)
                            .at(-1) ?? `Corrected copy`,
                      }),
                    },
                    e.resultId,
                  ),
                ),
              }),
              e.data.reviews.length > t
                ? (0, b.jsx)(`button`, {
                    type: `button`,
                    onClick: () => n((e) => e + 20),
                    children: `Show older copies`,
                  })
                : null,
            ],
          })
        : null,
      (0, b.jsxs)(`p`, {
        children: [
          `Open a completed native result from `,
          (0, b.jsx)(`a`, { href: `?view=jobs`, children: `Jobs` }),
          `, or`,
          ` `,
          (0, b.jsx)(`a`, { href: `?view=transcribe`, children: `transcribe a recording` }),
          `.`,
        ],
      }),
      e.error ? (0, b.jsx)(`p`, { role: `alert`, children: e.error.message }) : null,
      (0, b.jsx)(`ul`, {
        children: e.data?.jobs
          .flatMap((e) => e.files.map((t, n) => ({ batch: e, file: t, index: n })))
          .filter(
            ({ file: e }) =>
              e.status === `completed` &&
              e.outputs.some((e) => [`json`, `native-json`].includes(e.format)),
          )
          .slice(0, 20)
          .map(({ batch: e, file: t, index: n }) =>
            (0, b.jsx)(
              `li`,
              {
                children: (0, b.jsx)(`a`, {
                  href: xi(`${e.id}:${n}`),
                  children: t.input.split(/[\\/]/).at(-1),
                }),
              },
              `${e.id}:${n}`,
            ),
          ),
      }),
    ],
  });
}
function wi({ original: e, api: t }) {
  let n = S(),
    r = rt({ queryKey: [`speaker-directory-state`], queryFn: () => t.getState(), retry: !1 }),
    i = new URLSearchParams(window.location.search),
    [a, o] = (0, y.useState)(
      i.get(`layer`) === `source` && e.sourceTranscript ? `source` : `transcript`,
    ),
    [s, c] = (0, y.useState)(i.get(`segment`) ?? e.transcript.segments[0]?.id ?? ``),
    [l, u] = (0, y.useState)({ transcript: {}, source: {} }),
    d = JSON.stringify(l),
    [f, p] = (0, y.useState)(vi),
    m = d !== f,
    [h, g] = (0, y.useState)(`.`),
    [_, v] = (0, y.useState)(
      `${(e.originalName ?? `result`).replace(/(?:\.native)?\.json$/i, ``)}.corrected`,
    ),
    [x, C] = (0, y.useState)([`json`, `srt`]),
    [w, T] = (0, y.useState)(`transcript`),
    [E, D] = (0, y.useState)(``),
    [O, ee] = (0, y.useState)(``),
    [te, k] = (0, y.useState)(0),
    A = (0, y.useRef)(null),
    j = (0, y.useRef)(new Map()),
    ne = (0, y.useRef)(null),
    M = B({
      mutationFn: async (t) => ({
        fingerprint: t.fingerprint,
        preview: await xr.preview({
          resultId: e.resultId,
          corrections: bi(t.drafts.transcript, e.transcript),
          sourceCorrections: bi(t.drafts.source, e.sourceTranscript),
        }),
      }),
    }),
    N = M.data?.fingerprint === d ? M.data.preview : null,
    P = B({
      mutationFn: async (t) =>
        xr.save({
          resultId: e.resultId,
          previewId: t.previewId,
          directory: h,
          basename: _,
          formats: x,
          exportLayer: w,
        }),
      onSuccess: (e, t) => {
        (p(t.fingerprint), n.invalidateQueries({ queryKey: oi }));
      },
    }),
    re = B({
      mutationFn: () => xr.reset(e.resultId),
      onSuccess: () => {
        (u({ transcript: {}, source: {} }), p(vi), M.reset(), P.reset());
      },
    }),
    F = B({
      mutationFn: () => xr.recording({ resultId: e.resultId, path: E }),
      onSuccess: (t) => {
        (ee(``), n.setQueryData([`review`, e.resultId], t));
      },
    });
  (0, y.useEffect)(() => {
    if (!m) return;
    let e = (e) => {
      (e.preventDefault(), (e.returnValue = ``));
    };
    return (
      window.addEventListener(`beforeunload`, e),
      () => window.removeEventListener(`beforeunload`, e)
    );
  }, [m]);
  let I =
      (a === `source`
        ? (N?.sourceTranscript ?? e.sourceTranscript)
        : (N?.transcript ?? e.transcript)
      )?.segments ?? [],
    ie = a === `source` ? e.sourceTranscript : e.transcript,
    ae = new Set(
      I.filter(
        (e) =>
          e.startSeconds != null &&
          e.endSeconds != null &&
          te >= e.startSeconds &&
          te < e.endSeconds,
      ).map((e) => e.id),
    ),
    oe = r.data?.profiles ?? [],
    se = new Map();
  for (let t of [...e.transcript.segments, ...(e.sourceTranscript?.segments ?? [])])
    t.speaker && se.set(t.speaker, t.speaker);
  for (let e of oe)
    (se.set(e.label, `${e.label} (${e.metadata.status ?? `unclassified`})`),
      se.has(e.id) &&
        se.set(e.id, `${e.label} (${e.metadata.status ?? `unclassified`}; stored ID)`));
  for (let e of Object.values(l))
    for (let t of Object.values(e)) t.speaker && !se.has(t.speaker) && se.set(t.speaker, t.speaker);
  let ce = M.error ?? P.error ?? re.error ?? F.error ?? r.error;
  function le(e) {
    (c(e.id),
      A.current &&
        e.startSeconds != null &&
        ((A.current.currentTime = e.startSeconds), k(A.current.currentTime)));
    let t = new URL(window.location.href);
    (t.searchParams.set(`segment`, e.id),
      t.searchParams.set(`layer`, a),
      window.history.replaceState(null, ``, t));
  }
  function ue(e, t, n) {
    let r = ie?.segments.find((t) => t.id === e.id);
    r &&
      (u((i) => {
        let o = { ...(i[a][e.id] ?? yi(r)), [t]: n },
          s = { ...i[a] };
        return (
          JSON.stringify(o) === JSON.stringify(yi(r)) ? delete s[e.id] : (s[e.id] = o),
          { ...i, [a]: s }
        );
      }),
      M.reset(),
      P.reset());
  }
  function de() {
    let e = I.find((e) => e.id === s);
    A.current &&
      e?.startSeconds != null &&
      ((A.current.currentTime = e.startSeconds), k(A.current.currentTime));
  }
  let fe = {
    "aria-label": `Source recording`,
    controls: !0,
    src: e.mediaUrl ?? void 0,
    onLoadedMetadata: de,
    onTimeUpdate: () => {
      if (A.current) {
        let e = A.current.currentTime;
        k(e);
        let t = I.find(
          (t) =>
            t.startSeconds != null &&
            t.endSeconds != null &&
            e >= t.startSeconds &&
            e < t.endSeconds,
        );
        t &&
          ne.current !== t.id &&
          ((ne.current = t.id),
          [`TEXTAREA`, `INPUT`, `SELECT`].includes(document.activeElement?.tagName ?? ``) ||
            j.current.get(t.id)?.scrollIntoView?.({ block: `nearest` }));
      }
    },
    onError: () =>
      ee(
        `This browser could not play the recording. Locate a compatible local audio/video file below.`,
      ),
  };
  return (0, b.jsxs)(`main`, {
    className: `page reviewPage`,
    children: [
      (0, b.jsxs)(`header`, {
        className: `reviewHeader`,
        children: [
          (0, b.jsxs)(`div`, {
            children: [
              (0, b.jsx)(`h1`, { children: `Review & Edit` }),
              (0, b.jsx)(`p`, { className: `path`, children: e.originalName }),
            ],
          }),
          (0, b.jsx)(`p`, {
            role: `status`,
            children: m ? `Unsaved changes` : `No unsaved changes`,
          }),
        ],
      }),
      ce ? (0, b.jsx)(`p`, { role: `alert`, children: ce.message }) : null,
      (0, b.jsxs)(`div`, {
        className: `reviewActions`,
        children: [
          (0, b.jsx)(`button`, {
            type: `button`,
            disabled: re.isPending || P.isPending || (!m && !N),
            onClick: () => re.mutate(),
            children: `Discard changes`,
          }),
          (0, b.jsx)(`a`, { href: `?view=jobs`, children: `Jobs` }),
        ],
      }),
      (0, b.jsxs)(`div`, {
        className: `reviewWorkspace`,
        children: [
          (0, b.jsxs)(`section`, {
            className: `reviewMedia`,
            "aria-label": `Recording playback`,
            children: [
              (0, b.jsx)(`h2`, { children: e.sourceName ?? `Source recording` }),
              e.mediaUrl
                ? e.mediaType.startsWith(`video/`)
                  ? (0, b.jsx)(`video`, {
                      ...fe,
                      ref: (e) => {
                        A.current = e;
                      },
                    })
                  : (0, b.jsx)(`audio`, {
                      ...fe,
                      ref: (e) => {
                        A.current = e;
                      },
                    })
                : null,
              (0, b.jsxs)(`p`, { children: [`Playback `, Lr(te)] }),
              e.mediaError || O
                ? (0, b.jsx)(`p`, { role: `alert`, children: e.mediaError ?? O })
                : null,
              (0, b.jsxs)(`details`, {
                open: !!e.mediaError || !!O,
                children: [
                  (0, b.jsx)(`summary`, { children: `Locate source recording` }),
                  (0, b.jsxs)(`label`, {
                    children: [
                      `Matching local recording path`,
                      (0, b.jsx)(`input`, { value: E, onChange: (e) => D(e.currentTarget.value) }),
                    ],
                  }),
                  (0, b.jsx)(`button`, {
                    type: `button`,
                    disabled: !E.trim() || F.isPending,
                    onClick: () => F.mutate(),
                    children: `Use recording`,
                  }),
                  (0, b.jsxs)(`p`, {
                    className: `muted path`,
                    children: [
                      `Paths must stay inside `,
                      e.root,
                      `. The recording file is never modified.`,
                    ],
                  }),
                ],
              }),
              e.translationProvenance
                ? (0, b.jsxs)(`p`, {
                    className: `muted`,
                    children: [
                      `Translation: `,
                      e.translationProvenance.sourceLanguage ?? `source`,
                      ` →`,
                      ` `,
                      e.translationProvenance.targetLanguage ?? `target`,
                      ` ·`,
                      ` `,
                      e.translationProvenance.modelId ?? `recorded model`,
                    ],
                  })
                : null,
              e.isTranslation && !e.sourceTranscript
                ? (0, b.jsxs)(`p`, {
                    children: [
                      `This older translated result has no retained source transcript.`,
                      ` `,
                      (0, b.jsx)(`a`, {
                        href: `?view=transcribe`,
                        children: `Run Transcribe again`,
                      }),
                      ` to retain both languages.`,
                    ],
                  })
                : null,
            ],
          }),
          (0, b.jsxs)(`section`, {
            className: `reviewTranscript`,
            "aria-label": `Timed transcript`,
            children: [
              e.sourceTranscript
                ? (0, b.jsx)(`div`, {
                    className: `reviewLayers`,
                    "aria-label": `Transcript layer`,
                    children: [`transcript`, `source`].map((e) =>
                      (0, b.jsx)(
                        `button`,
                        {
                          type: `button`,
                          "aria-pressed": a === e,
                          onClick: () => {
                            o(e);
                            let t = new URL(window.location.href);
                            (t.searchParams.set(`layer`, e),
                              window.history.replaceState(null, ``, t));
                          },
                          children: e === `source` ? `Source transcript` : `Translation`,
                        },
                        e,
                      ),
                    ),
                  })
                : null,
              (0, b.jsx)(`p`, {
                className: `muted`,
                children: `Select a segment to seek and edit. Arrow keys move between segment buttons. Times accept exact seconds or HH:MM:SS.mmm.`,
              }),
              I.length > 0 && s && !I.some((e) => e.id === s)
                ? (0, b.jsx)(`p`, {
                    children: `The selected segment was not found. Choose another segment below.`,
                  })
                : null,
              I.length
                ? null
                : (0, b.jsxs)(`p`, {
                    children: [
                      `No transcript segments are available.`,
                      ` `,
                      (0, b.jsx)(`a`, {
                        href: `?view=jobs`,
                        children: `Choose another result in Jobs`,
                      }),
                      ` or`,
                      ` `,
                      (0, b.jsx)(`a`, {
                        href: `?view=transcribe`,
                        children: `transcribe a recording`,
                      }),
                      `.`,
                    ],
                  }),
              I.map((e, t) => {
                let n = ie?.segments.find((t) => t.id === e.id) ?? e,
                  r = l[a][e.id] ?? yi(n),
                  i = s === e.id;
                return (0, b.jsxs)(
                  `article`,
                  {
                    className: `reviewSegment${i ? ` isSelected` : ``}${ae.has(e.id) ? ` isPlaying` : ``}`,
                    children: [
                      (0, b.jsxs)(`button`, {
                        type: `button`,
                        ref: (t) => {
                          t ? j.current.set(e.id, t) : j.current.delete(e.id);
                        },
                        tabIndex: i || (t === 0 && !I.some((e) => e.id === s)) ? 0 : -1,
                        "aria-label": `Seek to segment ${t + 1}`,
                        "aria-current": ae.has(e.id) ? `time` : void 0,
                        onClick: () => le(e),
                        onKeyDown: (e) => {
                          if (e.altKey || e.ctrlKey || e.metaKey) return;
                          let n =
                            e.key === `ArrowDown`
                              ? I[t + 1]
                              : e.key === `ArrowUp`
                                ? I[t - 1]
                                : e.key === `Home`
                                  ? I[0]
                                  : e.key === `End`
                                    ? I.at(-1)
                                    : void 0;
                          n && (e.preventDefault(), le(n), j.current.get(n.id)?.focus());
                        },
                        children: [
                          `Segment `,
                          t + 1,
                          ` ·`,
                          ` `,
                          e.startSeconds == null ? `Untimed` : Lr(e.startSeconds),
                          ` →`,
                          ` `,
                          e.endSeconds == null ? `Untimed` : Lr(e.endSeconds),
                          ` ·`,
                          ` `,
                          r.speaker || `Unassigned`,
                        ],
                      }),
                      e.overlapsPrevious
                        ? (0, b.jsx)(`p`, {
                            className: `reviewOverlap`,
                            children: `Overlapping speech with the previous segment; source order is preserved.`,
                          })
                        : null,
                      i
                        ? (0, b.jsxs)(b.Fragment, {
                            children: [
                              (0, b.jsxs)(`label`, {
                                children: [
                                  `Segment `,
                                  t + 1,
                                  ` text`,
                                  (0, b.jsx)(`textarea`, {
                                    "aria-label": `Segment ${t + 1} text`,
                                    rows: 3,
                                    value: r.text,
                                    onChange: (t) => ue(e, `text`, t.currentTarget.value),
                                  }),
                                ],
                              }),
                              (0, b.jsxs)(`div`, {
                                className: `reviewExactTimes`,
                                children: [
                                  (0, b.jsxs)(`label`, {
                                    children: [
                                      `Segment `,
                                      t + 1,
                                      ` start (seconds or timecode)`,
                                      (0, b.jsx)(`input`, {
                                        inputMode: `decimal`,
                                        value: r.start,
                                        onChange: (t) => ue(e, `start`, t.currentTarget.value),
                                      }),
                                    ],
                                  }),
                                  (0, b.jsxs)(`label`, {
                                    children: [
                                      `Segment `,
                                      t + 1,
                                      ` end (seconds or timecode)`,
                                      (0, b.jsx)(`input`, {
                                        inputMode: `decimal`,
                                        value: r.end,
                                        onChange: (t) => ue(e, `end`, t.currentTarget.value),
                                      }),
                                    ],
                                  }),
                                ],
                              }),
                              (0, b.jsxs)(`label`, {
                                children: [
                                  `Segment `,
                                  t + 1,
                                  ` speaker annotation`,
                                  (0, b.jsxs)(`select`, {
                                    "aria-label": `Segment ${t + 1} speaker annotation`,
                                    value: r.speaker,
                                    onChange: (t) => ue(e, `speaker`, t.currentTarget.value),
                                    children: [
                                      (0, b.jsx)(`option`, { value: ``, children: `Unassigned` }),
                                      [...se].map(([e, t]) =>
                                        (0, b.jsx)(`option`, { value: e, children: t }, e),
                                      ),
                                    ],
                                  }),
                                ],
                              }),
                              (0, b.jsxs)(`p`, {
                                className: `muted`,
                                children: [
                                  `Annotation does not confirm a Speaker Profile.`,
                                  ` `,
                                  (0, b.jsx)(`a`, {
                                    href: `?view=speakers`,
                                    target: `_blank`,
                                    rel: `noreferrer`,
                                    children: `Confirm or correct identity in Speakers`,
                                  }),
                                  `.`,
                                ],
                              }),
                            ],
                          })
                        : (0, b.jsx)(`p`, { children: r.text }),
                    ],
                  },
                  e.id,
                );
              }),
            ],
          }),
        ],
      }),
      (0, b.jsxs)(`section`, {
        className: `reviewExport`,
        "aria-label": `Corrected copy`,
        children: [
          (0, b.jsx)(`h2`, { children: `Preview and save` }),
          (0, b.jsx)(`button`, {
            type: `button`,
            disabled: M.isPending || !I.length,
            onClick: () => {
              (P.reset(), M.mutate({ fingerprint: d, drafts: l }));
            },
            children: `Preview changes`,
          }),
          N
            ? (0, b.jsxs)(b.Fragment, {
                children: [
                  (0, b.jsx)(`p`, {
                    role: `status`,
                    children: `Preview validated. Segment order is unchanged.`,
                  }),
                  N.clearedAlignment.length || N.clearedSourceAlignment.length
                    ? (0, b.jsx)(`p`, {
                        children: `Word alignment cleared for manually edited text/timing. Neighboring segments retain their alignment.`,
                      })
                    : null,
                ],
              })
            : null,
          (0, b.jsxs)(`p`, {
            className: `muted`,
            children: [
              `A new canonical Native JSON copy always retains source/translation metadata. Text and subtitle exports use`,
              ` `,
              w === `source` ? `Source` : e.isTranslation ? `Translation` : `Transcript`,
              `. Existing files and the original recording/result are never overwritten.`,
            ],
          }),
          e.sourceTranscript
            ? (0, b.jsxs)(`label`, {
                children: [
                  `Text/subtitle export layer`,
                  (0, b.jsxs)(`select`, {
                    "aria-label": `Text/subtitle export layer`,
                    value: w,
                    onChange: (e) => {
                      let t = e.currentTarget.value;
                      (t === `source` || t === `transcript`) && T(t);
                    },
                    children: [
                      (0, b.jsx)(`option`, { value: `transcript`, children: `Translation` }),
                      (0, b.jsx)(`option`, { value: `source`, children: `Source transcript` }),
                    ],
                  }),
                ],
              })
            : null,
          (0, b.jsxs)(`div`, {
            className: `reviewExportFields`,
            children: [
              (0, b.jsxs)(`label`, {
                children: [
                  `Output folder`,
                  (0, b.jsx)(`input`, { value: h, onChange: (e) => g(e.currentTarget.value) }),
                ],
              }),
              (0, b.jsxs)(`label`, {
                children: [
                  `Corrected-copy name`,
                  (0, b.jsx)(`input`, { value: _, onChange: (e) => v(e.currentTarget.value) }),
                ],
              }),
            ],
          }),
          (0, b.jsxs)(`fieldset`, {
            children: [
              (0, b.jsx)(`legend`, { children: `Additional exports` }),
              [
                [`json`, `WhisperX JSON`],
                [`txt`, `TXT`],
                [`srt`, `SRT`],
                [`vtt`, `WebVTT`],
                [`tsv`, `TSV`],
                [`aud`, `Audacity`],
              ].map(([e, t]) =>
                e && t
                  ? (0, b.jsxs)(
                      `label`,
                      {
                        children: [
                          (0, b.jsx)(`input`, {
                            type: `checkbox`,
                            checked: x.includes(e),
                            onChange: (t) =>
                              C((n) =>
                                t.currentTarget.checked ? [...n, e] : n.filter((t) => t !== e),
                              ),
                          }),
                          t,
                        ],
                      },
                      e,
                    )
                  : null,
              ),
            ],
          }),
          (0, b.jsx)(`button`, {
            type: `button`,
            disabled: !N || P.isPending || P.isSuccess,
            onClick: () => {
              N && P.mutate({ fingerprint: d, previewId: N.previewId });
            },
            children: `Save corrected copy`,
          }),
          P.data
            ? (0, b.jsxs)(`div`, {
                role: `status`,
                children: [
                  (0, b.jsxs)(`p`, {
                    children: [
                      `Corrected copy saved. `,
                      (0, b.jsx)(`a`, {
                        href: xi(P.data.resultId),
                        children: `Open corrected copy`,
                      }),
                    ],
                  }),
                  (0, b.jsx)(`ul`, {
                    children: P.data.outputs.map((e) =>
                      (0, b.jsx)(`li`, { className: `path`, children: e.path }, e.path),
                    ),
                  }),
                ],
              })
            : null,
        ],
      }),
    ],
  });
}
function Ti({ kind: e, onChoose: t }) {
  let n = B({ mutationFn: Pr.files });
  return (0, b.jsxs)(`div`, {
    className: `filePicker`,
    "aria-label": `Local library paths`,
    children: [
      (0, b.jsx)(`button`, {
        type: `button`,
        disabled: n.isPending,
        onClick: () => n.mutate(`.`),
        children: `Browse launched directory`,
      }),
      n.error ? (0, b.jsx)(`p`, { role: `alert`, children: n.error.message }) : null,
      n.data
        ? (0, b.jsxs)(b.Fragment, {
            children: [
              (0, b.jsxs)(`p`, {
                className: `indexPath`,
                children: [n.data.root, `/`, n.data.directory],
              }),
              (0, b.jsx)(`button`, {
                type: `button`,
                disabled: !n.data.directory || n.isPending,
                onClick: () => n.mutate(`${n.data?.directory}/..`),
                children: `Parent folder`,
              }),
              e === `directory`
                ? (0, b.jsx)(`button`, {
                    type: `button`,
                    onClick: () => t(n.data?.directory || `.`),
                    children: `Use this folder`,
                  })
                : null,
              (0, b.jsx)(`ul`, {
                children: n.data.entries
                  .filter(
                    (t) =>
                      t.directory ||
                      (e === `file` && (t.name.endsWith(`.json`) || t.name.endsWith(`.backup`))),
                  )
                  .map((e) =>
                    (0, b.jsx)(
                      `li`,
                      {
                        children: (0, b.jsxs)(`button`, {
                          type: `button`,
                          disabled: n.isPending,
                          onClick: () => {
                            e.directory ? n.mutate(e.path) : t(e.path);
                          },
                          children: [e.name, e.directory ? `/` : ``],
                        }),
                      },
                      e.name,
                    ),
                  ),
              }),
              n.data.truncated
                ? (0, b.jsx)(`p`, {
                    children: `More files exist. Enter an exact path in the field above.`,
                  })
                : null,
            ],
          })
        : null,
    ],
  });
}
var Ei = [`speaker-directory-state`],
  Di = {
    load: `Load speaker index`,
    new: `New speaker index`,
    export: `Save speaker index`,
    import: `Import speaker index`,
  },
  Oi = { new: `Create empty library`, export: `Save copy`, import: `Import library` };
function ki(e) {
  throw Error(`Unsupported Speaker Library operation.`);
}
function Ai({ api: e = Ct, manageHref: t, onIndexReplaced: n }) {
  let r = S(),
    i = rt({ queryKey: Ei, queryFn: () => e.getState(), refetchOnMount: !1 }),
    [a, o] = (0, y.useState)(null),
    [s, c] = (0, y.useState)(``);
  return (0, b.jsxs)(`section`, {
    className: `speakerIndex`,
    "aria-label": `Speaker index`,
    children: [
      (0, b.jsxs)(`div`, {
        className: `indexActions`,
        children: [
          (0, b.jsx)(`h2`, { children: `Speaker index` }),
          (0, b.jsx)(`button`, { type: `button`, onClick: () => o(`load`), children: `Load…` }),
          (0, b.jsx)(`button`, { type: `button`, onClick: () => o(`new`), children: `New…` }),
          (0, b.jsx)(`button`, {
            type: `button`,
            disabled: i.data?.library.status !== `valid`,
            onClick: () => o(`export`),
            children: `Save…`,
          }),
          (0, b.jsx)(`button`, {
            type: `button`,
            disabled: i.data?.library.status === `invalid`,
            onClick: () => o(`import`),
            children: `Import…`,
          }),
          t ? (0, b.jsx)(`a`, { href: t, children: `Manage speakers` }) : null,
        ],
      }),
      i.data
        ? (0, b.jsxs)(b.Fragment, {
            children: [
              (0, b.jsxs)(`p`, {
                children: [
                  i.data.scope,
                  ` · `,
                  i.data.library.profileCount,
                  ` `,
                  i.data.library.profileCount === 1 ? `profile` : `profiles`,
                  ` ·`,
                  ` `,
                  i.data.library.status,
                ],
              }),
              (0, b.jsx)(`code`, { className: `indexPath`, children: i.data.library.path }),
              i.data.library.message ? (0, b.jsx)(`p`, { children: i.data.library.message }) : null,
            ],
          })
        : (0, b.jsx)(`p`, {
            children: i.isError
              ? `Could not read the active index. Load a valid directory or create one.`
              : `Loading Speaker Directory…`,
          }),
      s ? (0, b.jsx)(`p`, { role: `status`, children: s }) : null,
      a
        ? (0, b.jsx)(Mi, {
            action: a,
            onApplied: async (e) => {
              if (`state` in e) {
                let t = e.kind !== `import` || e.receipt.changed;
                (t && n?.(),
                  r.setQueryData(Ei, e.state),
                  t && (await r.invalidateQueries({ queryKey: [`workbench`, `preflight`] })),
                  await r.invalidateQueries({ queryKey: [`workbench`, `capabilities`] }));
              }
              switch (e.kind) {
                case `export`:
                  c(`Saved copy: ${e.receipt.path}`);
                  break;
                case `load`:
                  c(`Loaded: ${e.state.library.path}`);
                  break;
                case `new`:
                  c(`Created: ${e.receipt.path}`);
                  break;
                case `import`:
                  c(
                    e.receipt.changed
                      ? `Imported: ${e.receipt.path}`
                      : `Speaker Library is unchanged.`,
                  );
                  break;
                default:
                  ki(e);
              }
              o(null);
            },
            onDismiss: () => o(null),
          })
        : null,
    ],
  });
}
function ji({ title: e, onCancel: t, children: n }) {
  let r = (0, y.useRef)(null),
    i = (0, y.useId)();
  return (
    (0, y.useEffect)(() => {
      let e = r.current,
        t = document.activeElement;
      return (
        e?.showModal(),
        () => {
          (e?.open && e.close(), t instanceof HTMLElement && t.focus());
        }
      );
    }, []),
    (0, b.jsxs)(`dialog`, {
      className: `libraryDialog`,
      ref: r,
      "aria-labelledby": i,
      onCancel: (e) => {
        (e.preventDefault(), t());
      },
      children: [(0, b.jsx)(`h2`, { id: i, children: e }), n],
    })
  );
}
function Mi({ action: e, onApplied: t, onDismiss: n }) {
  let [r, i] = (0, y.useState)(`explicit`),
    [a, o] = (0, y.useState)(``),
    [s, c] = (0, y.useState)(`merge`),
    [l, u] = (0, y.useState)(!1),
    [d, f] = (0, y.useState)(!1),
    p = B({ mutationFn: qr.preview }),
    m = B({ mutationFn: qr.confirm, onSuccess: t }),
    h = B({ mutationFn: qr.load, onSuccess: t }),
    g = B({ mutationFn: qr.cancel }),
    _ = m.isPending || h.isPending,
    v = _ || p.isPending,
    x = () => {
      (p.data && g.mutate(p.data.id), p.reset(), m.reset(), u(!1));
    },
    S = () => {
      _ || (p.data && g.mutate(p.data.id), n());
    },
    C = r === `explicit` ? { scope: r, path: a } : { scope: r },
    w = () => {
      if (e === `load`) {
        h.mutate(C);
        return;
      }
      let t;
      switch (e) {
        case `new`:
          t = { kind: `new`, selection: C };
          break;
        case `export`:
          t = { kind: `export`, destination: a };
          break;
        case `import`:
          t = { kind: `import`, source: a, mode: s };
      }
      p.mutate(t);
    },
    T = p.error ?? m.error ?? h.error ?? g.error,
    E = p.data?.kind === `import` && !p.data.summary.changed;
  return (0, b.jsxs)(ji, {
    title: Di[e],
    onCancel: S,
    children: [
      (0, b.jsxs)(`form`, {
        onSubmit: (e) => {
          (e.preventDefault(), e.stopPropagation(), w());
        },
        children: [
          (0, b.jsxs)(`fieldset`, {
            disabled: v,
            onChange: x,
            children: [
              e === `load` || e === `new`
                ? (0, b.jsxs)(b.Fragment, {
                    children: [
                      (0, b.jsxs)(`label`, {
                        children: [
                          `Directory scope`,
                          (0, b.jsxs)(`select`, {
                            "aria-label": `Directory scope`,
                            value: r,
                            onChange: (e) => {
                              let t = e.currentTarget.value;
                              (t === `explicit` ||
                                t === `auto` ||
                                t === `local` ||
                                t === `global`) &&
                                i(t);
                            },
                            children: [
                              (0, b.jsx)(`option`, {
                                value: `explicit`,
                                children: `Choose a directory`,
                              }),
                              (0, b.jsx)(`option`, {
                                value: `auto`,
                                children: `Automatic (local first)`,
                              }),
                              (0, b.jsx)(`option`, { value: `local`, children: `Project local` }),
                              (0, b.jsx)(`option`, { value: `global`, children: `Global` }),
                            ],
                          }),
                        ],
                      }),
                      r === `explicit`
                        ? (0, b.jsxs)(`label`, {
                            children: [
                              e === `load` ? `Directory or library.json` : `Directory`,
                              (0, b.jsx)(`input`, {
                                required: !0,
                                autoFocus: !0,
                                value: a,
                                onChange: (e) => o(e.currentTarget.value),
                              }),
                            ],
                          })
                        : null,
                    ],
                  })
                : (0, b.jsxs)(`label`, {
                    children: [
                      e === `export` ? `Destination file` : `Source file`,
                      (0, b.jsx)(`input`, {
                        required: !0,
                        autoFocus: !0,
                        value: a,
                        onChange: (e) => o(e.currentTarget.value),
                      }),
                    ],
                  }),
              (e !== `load` && e !== `new`) || r === `explicit`
                ? (0, b.jsxs)(b.Fragment, {
                    children: [
                      (0, b.jsx)(`button`, {
                        type: `button`,
                        onClick: () => f(!d),
                        children: `Browse…`,
                      }),
                      d
                        ? (0, b.jsx)(Ti, {
                            kind: e === `import` ? `file` : `directory`,
                            onChoose: (t) => {
                              (x(), o(e === `export` ? `${t}/speaker-library.json` : t), f(!1));
                            },
                          })
                        : null,
                    ],
                  })
                : null,
              e === `import`
                ? (0, b.jsxs)(`label`, {
                    children: [
                      `Import mode`,
                      (0, b.jsxs)(`select`, {
                        "aria-label": `Import mode`,
                        value: s,
                        onChange: (e) => {
                          let t = e.currentTarget.value;
                          (t === `merge` || t === `replace`) && c(t);
                        },
                        children: [
                          (0, b.jsx)(`option`, {
                            value: `merge`,
                            children: `Merge compatible new profiles`,
                          }),
                          (0, b.jsx)(`option`, {
                            value: `replace`,
                            children: `Replace the complete library`,
                          }),
                        ],
                      }),
                    ],
                  })
                : null,
            ],
          }),
          (0, b.jsx)(`p`, {
            className: `muted`,
            children: `Explicit paths stay inside the launched directory. Saved copies use a new filename.`,
          }),
          T ? (0, b.jsx)(`p`, { role: `alert`, children: T.message }) : null,
          (0, b.jsx)(`button`, {
            type: `submit`,
            disabled: v,
            children: e === `load` ? `Load index` : `Review`,
          }),
        ],
      }),
      p.data
        ? (0, b.jsxs)(`div`, {
            className: `libraryConsent`,
            children: [
              (0, b.jsx)(Pi, { preview: p.data }),
              E
                ? (0, b.jsx)(`p`, {
                    children: `The library already contains this exact snapshot. No files will change.`,
                  })
                : (0, b.jsxs)(b.Fragment, {
                    children: [
                      (0, b.jsxs)(`label`, {
                        children: [
                          (0, b.jsx)(`input`, {
                            type: `checkbox`,
                            checked: l,
                            disabled: _,
                            onChange: (e) => u(e.currentTarget.checked),
                          }),
                          Ni(p.data),
                        ],
                      }),
                      (0, b.jsx)(`button`, {
                        type: `button`,
                        disabled: !l || _,
                        onClick: () => {
                          p.data && m.mutate(p.data.id);
                        },
                        children: Oi[p.data.kind],
                      }),
                    ],
                  }),
            ],
          })
        : null,
      (0, b.jsx)(`button`, {
        type: `button`,
        disabled: _,
        onClick: S,
        children: E ? `Close` : `Cancel`,
      }),
      v
        ? (0, b.jsx)(`p`, {
            role: `status`,
            children: _
              ? `Applying the selected operation…`
              : `Reading the library and checking the selection…`,
          })
        : null,
    ],
  });
}
function Ni(e) {
  switch (e.kind) {
    case `new`:
      return `Create this empty Speaker Library`;
    case `export`:
      return e.summary.profileCount === 0
        ? `Save this empty Speaker Library copy`
        : `I agree to save voice embeddings locally to this destination`;
    case `import`:
      return e.summary.incomingProfiles === 0
        ? `Replace the active library with this empty snapshot`
        : `I agree to import voice embeddings into this Speaker Library`;
    default:
      return ki(e);
  }
}
function Pi({ preview: e }) {
  switch (e.kind) {
    case `new`:
      return (0, b.jsxs)(b.Fragment, {
        children: [
          (0, b.jsx)(`p`, { children: `Create an empty canonical library:` }),
          (0, b.jsx)(`p`, { className: `indexPath`, children: e.summary.destination }),
          (0, b.jsx)(`p`, { children: `Existing libraries will not be overwritten.` }),
        ],
      });
    case `export`:
      return (0, b.jsxs)(b.Fragment, {
        children: [
          (0, b.jsxs)(`p`, {
            children: [
              e.summary.profileCount,
              ` profiles · `,
              e.summary.confirmedProfiles,
              ` confirmed · `,
              e.summary.draftProfiles,
              ` drafts`,
              e.summary.unclassifiedProfiles > 0
                ? ` · ${e.summary.unclassifiedProfiles} other statuses`
                : ``,
            ],
          }),
          (0, b.jsx)(`p`, { className: `indexPath`, children: e.summary.source }),
          (0, b.jsx)(`p`, {
            children:
              e.summary.profileCount === 0
                ? `Save this empty portable library to:`
                : `Save a portable copy containing voice embeddings to:`,
          }),
          (0, b.jsx)(`p`, { className: `indexPath`, children: e.summary.destination }),
        ],
      });
    case `import`:
      return (0, b.jsxs)(b.Fragment, {
        children: [
          (0, b.jsxs)(`p`, {
            children: [
              e.summary.incomingProfiles,
              ` incoming profiles ·`,
              ` `,
              e.summary.confirmedProfiles,
              ` confirmed · `,
              e.summary.draftProfiles,
              ` drafts`,
              e.summary.unclassifiedProfiles > 0
                ? ` · ${e.summary.unclassifiedProfiles} other statuses`
                : ``,
            ],
          }),
          (0, b.jsx)(`p`, { className: `indexPath`, children: e.summary.source }),
          (0, b.jsxs)(`p`, {
            children: [
              e.summary.addedProfiles,
              ` added · `,
              e.summary.replacedProfiles,
              ` replaced ·`,
              ` `,
              e.summary.removedProfiles,
              ` removed · `,
              e.summary.unchangedProfiles,
              ` `,
              `unchanged`,
            ],
          }),
          (0, b.jsx)(`p`, { children: `Destination:` }),
          (0, b.jsx)(`p`, { className: `indexPath`, children: e.summary.destination }),
          e.summary.backupPath
            ? (0, b.jsxs)(b.Fragment, {
                children: [
                  (0, b.jsx)(`p`, { children: `Previous library backup:` }),
                  (0, b.jsx)(`p`, { className: `indexPath`, children: e.summary.backupPath }),
                ],
              })
            : null,
        ],
      });
    default:
      return ki(e);
  }
}
function Fi({ api: e = Ct }) {
  let [t] = (0, y.useState)(() => new We()),
    n = new URLSearchParams(window.location.search).get(`view`) ?? `speakers`;
  return (0, b.jsxs)(C, {
    client: t,
    children: [
      (0, b.jsxs)(`nav`, {
        className: `appNav`,
        "aria-label": `Workbench`,
        children: [
          (0, b.jsx)(`a`, {
            href: `?view=transcribe`,
            "aria-current": n === `transcribe` ? `page` : void 0,
            children: `Transcribe`,
          }),
          (0, b.jsx)(`a`, {
            href: `?view=jobs`,
            "aria-current": n === `jobs` ? `page` : void 0,
            children: `Jobs`,
          }),
          (0, b.jsx)(`a`, {
            href: `?view=review`,
            "aria-current": n === `review` ? `page` : void 0,
            children: `Review & Edit`,
          }),
          (0, b.jsx)(`a`, {
            href: `?view=speakers`,
            "aria-current": n === `speakers` ? `page` : void 0,
            children: `Speakers`,
          }),
        ],
      }),
      n === `transcribe`
        ? (0, b.jsx)(_i, { speakerIndex: (0, b.jsx)(Ai, { api: e, manageHref: `?view=speakers` }) })
        : n === `jobs`
          ? (0, b.jsx)(fi, {})
          : n === `review`
            ? (0, b.jsx)(Si, { api: e })
            : (0, b.jsx)(Ii, { api: e }),
    ],
  });
}
function Ii({ api: e }) {
  let [t, n] = (0, y.useState)(0),
    r = rt({ queryKey: [`speaker-directory-state`], queryFn: () => e.getState() });
  return r.isLoading
    ? (0, b.jsx)(`main`, { className: `page`, children: `Loading Speaker Directory...` })
    : r.isError || !r.data
      ? (0, b.jsxs)(`main`, {
          className: `page`,
          children: [
            (0, b.jsx)(`h1`, { children: `Speaker Directory` }),
            (0, b.jsx)(`p`, { role: `alert`, children: `Failed to load Speaker Directory state.` }),
          ],
        })
      : (0, b.jsx)(Li, {
          api: e,
          state: r.data,
          editorGeneration: t,
          onIndexReplaced: () => n((e) => e + 1),
        });
}
function Li({ api: e, state: t, editorGeneration: n, onIndexReplaced: r }) {
  return (0, b.jsxs)(`main`, {
    className: `page`,
    children: [
      (0, b.jsxs)(`header`, {
        className: `header`,
        children: [
          (0, b.jsx)(`p`, { className: `eyebrow`, children: `CLI workspace` }),
          (0, b.jsx)(`h1`, { children: `Speaker Directory` }),
          (0, b.jsx)(`p`, { className: `path`, children: t.path }),
        ],
      }),
      (0, b.jsx)(Ai, { api: e, onIndexReplaced: r }),
      (0, b.jsxs)(`section`, {
        className: `summaryGrid`,
        "aria-label": `Speaker Directory summary`,
        children: [
          (0, b.jsx)(Ri, {
            title: `Speaker Library`,
            status: t.library.status,
            detail: `${t.library.profileCount} profile${t.library.profileCount === 1 ? `` : `s`}`,
          }),
          (0, b.jsx)(Ri, {
            title: `Speaker Trace`,
            status: t.trace.status,
            detail: t.trace.scanRoot ?? `No scan root`,
          }),
          (0, b.jsx)(Ri, { title: `Scope`, status: t.scope, detail: `Speaker Directory` }),
        ],
      }),
      (0, b.jsxs)(`section`, {
        children: [
          (0, b.jsxs)(`div`, {
            className: `sectionHeading`,
            children: [
              (0, b.jsx)(`h2`, { children: `Speaker Library profiles` }),
              (0, b.jsx)(`span`, { children: t.profiles.length }),
            ],
          }),
          (0, b.jsx)(`div`, {
            className: `profileList`,
            children: t.profiles.map((r) =>
              (0, b.jsx)(zi, { api: e, profile: r }, JSON.stringify([t.path, n, r.id])),
            ),
          }),
        ],
      }),
      (0, b.jsx)(Bi, { api: e, trace: t.trace }),
    ],
  });
}
function Ri({ title: e, status: t, detail: n }) {
  return (0, b.jsxs)(`article`, {
    className: `statusPanel`,
    children: [
      (0, b.jsx)(`h2`, { children: e }),
      (0, b.jsx)(`p`, { className: `status`, children: t }),
      (0, b.jsx)(`p`, { children: n }),
    ],
  });
}
function zi({ api: e, profile: t }) {
  let n = S(),
    [r, i] = (0, y.useState)(t.label),
    [a, o] = (0, y.useState)(Gi(t.metadata)),
    [s, c] = (0, y.useState)(null),
    l = B({
      mutationFn: () => {
        let n = Ki(a);
        return (c(null), e.updateProfile(t.id, { id: t.id, label: r, metadata: n }));
      },
      onSuccess: (e) => {
        n.setQueryData([`speaker-directory-state`], e);
      },
      onError: (e) => {
        c(e instanceof Error ? e.message : `Failed to save Speaker Library profile.`);
      },
    }),
    u = B({
      mutationFn: () => (c(null), e.deleteProfile(t.id)),
      onSuccess: (e) => {
        n.setQueryData([`speaker-directory-state`], e);
      },
      onError: (e) => {
        c(e instanceof Error ? e.message : `Failed to delete Speaker Library profile.`);
      },
    });
  return (0, b.jsxs)(`article`, {
    className: `profile`,
    children: [
      (0, b.jsxs)(`div`, {
        className: `profileIdentity`,
        children: [
          (0, b.jsxs)(`div`, {
            children: [
              (0, b.jsx)(`h3`, { children: t.label }),
              (0, b.jsx)(`p`, { className: `identityLabel`, children: `Stable profile id` }),
              (0, b.jsx)(`p`, { className: `mono profileId`, children: t.id }),
            ],
          }),
          (0, b.jsx)(`span`, { className: `identityBadge`, children: `Speaker Library profile` }),
        ],
      }),
      (0, b.jsx)(`dl`, {
        children: Object.entries(t.metadata).map(([e, t]) =>
          (0, b.jsxs)(
            `div`,
            { children: [(0, b.jsx)(`dt`, { children: e }), (0, b.jsx)(`dd`, { children: t })] },
            e,
          ),
        ),
      }),
      (0, b.jsxs)(`div`, {
        className: `profileForm`,
        children: [
          (0, b.jsxs)(`label`, {
            children: [
              `Label`,
              (0, b.jsx)(`input`, {
                "aria-label": `${t.id} label`,
                value: r,
                onChange: (e) => i(e.currentTarget.value),
              }),
            ],
          }),
          (0, b.jsxs)(`label`, {
            children: [
              `Metadata`,
              (0, b.jsx)(`textarea`, {
                "aria-label": `${t.id} metadata`,
                rows: 4,
                value: a,
                onChange: (e) => o(e.currentTarget.value),
              }),
            ],
          }),
          s ? (0, b.jsx)(`p`, { role: `alert`, children: s }) : null,
          (0, b.jsxs)(`div`, {
            className: `profileActions`,
            children: [
              (0, b.jsx)(`button`, {
                disabled: l.isPending,
                type: `button`,
                onClick: () => {
                  try {
                    Ki(a);
                  } catch (e) {
                    c(
                      e instanceof Error
                        ? e.message
                        : `Speaker Library profile metadata is malformed.`,
                    );
                    return;
                  }
                  l.mutate();
                },
                children: `Save profile`,
              }),
              (0, b.jsx)(`button`, {
                disabled: u.isPending,
                type: `button`,
                onClick: () => u.mutate(),
                children: `Delete profile`,
              }),
            ],
          }),
        ],
      }),
    ],
  });
}
function Bi({ api: e, trace: t }) {
  let n = S(),
    [r, i] = (0, y.useState)(``),
    [a, o] = (0, y.useState)(null),
    [s, c] = (0, y.useState)(null),
    l = t.speakers.filter((e) => e.kind === `enrolled`),
    u = t.speakers.filter((e) => e.kind === `anonymous`),
    d = B({
      mutationFn: () => e.rebuildTrace(r.trim() ? { scanRoot: r.trim() } : {}),
      onSuccess: (e) => {
        (o(null), c(e.report), n.setQueryData([`speaker-directory-state`], e.state));
      },
      onError: (e) => {
        o(e instanceof Error ? e.message : `Failed to rebuild Speaker Trace.`);
      },
    });
  return (0, b.jsxs)(`section`, {
    children: [
      (0, b.jsxs)(`div`, {
        className: `sectionHeading`,
        children: [
          (0, b.jsx)(`h2`, { children: `Speaker Trace` }),
          (0, b.jsx)(`span`, { children: t.speakers.length }),
        ],
      }),
      (0, b.jsxs)(`div`, {
        className: `traceMeta`,
        children: [
          (0, b.jsx)(`span`, { children: `Scan root` }),
          (0, b.jsx)(`code`, { children: t.scanRoot || `Not available` }),
        ],
      }),
      t.message ? (0, b.jsx)(`p`, { className: `muted`, children: t.message }) : null,
      (0, b.jsxs)(`div`, {
        className: `profile traceRebuild`,
        children: [
          (0, b.jsxs)(`label`, {
            children: [
              `Trace rebuild scan root`,
              (0, b.jsx)(`input`, {
                "aria-label": `Trace rebuild scan root`,
                placeholder: `Optional transcript scan root`,
                value: r,
                onChange: (e) => i(e.currentTarget.value),
              }),
            ],
          }),
          a ? (0, b.jsx)(`p`, { role: `alert`, children: a }) : null,
          (0, b.jsx)(`button`, {
            disabled: d.isPending,
            type: `button`,
            onClick: () => d.mutate(),
            children: `Rebuild Speaker Trace`,
          }),
        ],
      }),
      s ? (0, b.jsx)(Vi, { report: s }) : null,
      (0, b.jsxs)(`div`, {
        className: `sectionHeading`,
        children: [
          (0, b.jsx)(`h3`, { children: `Enrolled Speaker Trace` }),
          (0, b.jsx)(`span`, { children: l.length }),
        ],
      }),
      (0, b.jsx)(`div`, {
        className: `profileList`,
        children: l.map((e) =>
          (0, b.jsxs)(
            `article`,
            {
              className: `profile`,
              children: [
                (0, b.jsxs)(`div`, {
                  className: `profileIdentity`,
                  children: [
                    (0, b.jsxs)(`div`, {
                      children: [
                        (0, b.jsx)(`h4`, { children: e.label ?? e.profileId }),
                        (0, b.jsx)(`p`, {
                          className: `identityLabel`,
                          children: `Stable profile id`,
                        }),
                        (0, b.jsx)(`p`, { className: `mono profileId`, children: e.profileId }),
                      ],
                    }),
                    (0, b.jsx)(`span`, {
                      className: `identityBadge`,
                      children: `Speaker Library profile`,
                    }),
                  ],
                }),
                (0, b.jsx)(Hi, { files: e.files }),
              ],
            },
            e.profileId,
          ),
        ),
      }),
      (0, b.jsxs)(`div`, {
        className: `sectionHeading`,
        children: [
          (0, b.jsx)(`h3`, { children: `Anonymous Speaker Labels` }),
          (0, b.jsx)(`span`, { children: u.length }),
        ],
      }),
      (0, b.jsx)(`p`, {
        className: `muted`,
        children: `Anonymous Speaker Labels are Speaker Trace data, not enrolled Speaker Library identities.`,
      }),
      (0, b.jsx)(`div`, {
        className: `profileList`,
        children: u.map((e) =>
          (0, b.jsxs)(
            `article`,
            {
              className: `profile`,
              children: [
                (0, b.jsxs)(`div`, {
                  className: `profileIdentity`,
                  children: [
                    (0, b.jsxs)(`div`, {
                      children: [
                        (0, b.jsx)(`h4`, {
                          children: e.anonymousLabel ?? `Anonymous Speaker Label`,
                        }),
                        (0, b.jsx)(`p`, {
                          className: `identityLabel`,
                          children: `Anonymous Speaker Label`,
                        }),
                      ],
                    }),
                    (0, b.jsx)(`span`, {
                      className: `identityBadge traceBadge`,
                      children: `Trace data only`,
                    }),
                  ],
                }),
                (0, b.jsx)(Hi, { files: e.files }),
              ],
            },
            e.anonymousLabel,
          ),
        ),
      }),
      (0, b.jsxs)(`div`, {
        className: `sectionHeading`,
        children: [
          (0, b.jsx)(`h3`, { children: `Malformed transcript JSON` }),
          (0, b.jsx)(`span`, { children: t.errors.length }),
        ],
      }),
      t.errors.length === 0
        ? (0, b.jsx)(`p`, {
            className: `muted`,
            children: `No malformed transcript JSON errors recorded.`,
          })
        : (0, b.jsx)(`div`, {
            className: `profileList`,
            children: t.errors.map((e) =>
              (0, b.jsxs)(
                `article`,
                {
                  className: `profile`,
                  children: [
                    (0, b.jsx)(`p`, { className: `mono profileId`, children: e.sourceFile }),
                    (0, b.jsx)(`p`, { children: e.message }),
                  ],
                },
                `${e.sourceFile}:${e.message}`,
              ),
            ),
          }),
    ],
  });
}
function Vi({ report: e }) {
  return (0, b.jsxs)(`article`, {
    className: `profile rebuildReport`,
    children: [
      (0, b.jsx)(`h3`, { children: `Rebuild report` }),
      (0, b.jsx)(`p`, { className: `mono profileId`, children: e.tracePath }),
      (0, b.jsxs)(`dl`, {
        children: [
          (0, b.jsxs)(`div`, {
            children: [
              (0, b.jsx)(`dt`, { children: `Scanned files` }),
              (0, b.jsx)(`dd`, { children: e.stats.scannedFiles }),
            ],
          }),
          (0, b.jsxs)(`div`, {
            children: [
              (0, b.jsx)(`dt`, { children: `Accepted entries` }),
              (0, b.jsx)(`dd`, { children: e.stats.acceptedEntries }),
            ],
          }),
          (0, b.jsxs)(`div`, {
            children: [
              (0, b.jsx)(`dt`, { children: `Ignored non-JSON files` }),
              (0, b.jsx)(`dd`, { children: e.stats.ignoredNonJsonFiles }),
            ],
          }),
          (0, b.jsxs)(`div`, {
            children: [
              (0, b.jsx)(`dt`, { children: `Malformed JSON errors` }),
              (0, b.jsx)(`dd`, { children: e.stats.malformedJsonErrors }),
            ],
          }),
        ],
      }),
    ],
  });
}
function Hi({ files: e }) {
  return e.length === 0
    ? (0, b.jsx)(`p`, { className: `muted`, children: `No traced files recorded.` })
    : (0, b.jsx)(`div`, {
        className: `traceFileList`,
        children: e.map((e) =>
          (0, b.jsxs)(
            `div`,
            {
              className: `traceFile`,
              children: [
                (0, b.jsx)(`p`, { className: `mono profileId`, children: e.sourceFile }),
                (0, b.jsxs)(`div`, {
                  className: `traceStats`,
                  children: [
                    (0, b.jsxs)(`span`, {
                      children: [e.segmentCount, ` segment`, e.segmentCount === 1 ? `` : `s`],
                    }),
                    (0, b.jsxs)(`span`, { children: [e.totalDuration.toFixed(2), ` seconds`] }),
                  ],
                }),
                (0, b.jsx)(Ui, { spans: e.spans }),
              ],
            },
            e.sourceFile,
          ),
        ),
      });
}
function Ui({ spans: e }) {
  return e.length === 0
    ? (0, b.jsx)(`p`, { className: `muted`, children: `No spans recorded.` })
    : (0, b.jsx)(`ol`, {
        className: `traceSpanList`,
        children: e.map((e, t) =>
          (0, b.jsxs)(
            `li`,
            {
              children: [
                (0, b.jsx)(`span`, { className: `mono`, children: Wi(e) }),
                (0, b.jsx)(`p`, { children: e.snippet }),
              ],
            },
            `${e.startSeconds ?? `unknown`}:${e.endSeconds ?? `unknown`}:${t}`,
          ),
        ),
      });
}
function Wi(e) {
  return e.startSeconds === void 0 || e.endSeconds === void 0
    ? `Timing unavailable`
    : `${e.startSeconds.toFixed(2)}s - ${e.endSeconds.toFixed(2)}s`;
}
function Gi(e) {
  return Object.entries(e).map(([e, t]) => `${e}=${t}`).join(`
`);
}
function Ki(e) {
  return e
    ? e
        .split(`
`)
        .reduce((e, t, n) => {
          if (!t.trim())
            throw Error(`Speaker Library profile metadata line ${n + 1} must be key=value.`);
          let r = t.indexOf(`=`);
          if (r <= 0 || r === t.length - 1)
            throw Error(`Speaker Library profile metadata line ${n + 1} must be key=value.`);
          let i = t.slice(0, r).trim(),
            a = t.slice(r + 1).trim();
          if (!i || !a)
            throw Error(`Speaker Library profile metadata line ${n + 1} must be key=value.`);
          return ((e[i] = a), e);
        }, {})
    : {};
}
(0, it.createRoot)(document.getElementById(`root`)).render(
  (0, b.jsx)(y.StrictMode, { children: (0, b.jsx)(Fi, {}) }),
);
