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
    var ee = (x.prototype = new b());
    ((ee.constructor = x), _(ee, y.prototype), (ee.isPureReactComponent = !0));
    var S = Array.isArray;
    function te() {}
    var C = { H: null, A: null, T: null, S: null },
      ne = Object.prototype.hasOwnProperty;
    function w(e, n, r) {
      var i = r.ref;
      return { $$typeof: t, type: e, key: n, ref: i === void 0 ? null : i, props: r };
    }
    function T(e, t) {
      return w(e.type, t, e.props);
    }
    function re(e) {
      return typeof e == `object` && !!e && e.$$typeof === t;
    }
    function ie(e) {
      var t = { "=": `=0`, ":": `=2` };
      return (
        `$` +
        e.replace(/[=:]/g, function (e) {
          return t[e];
        })
      );
    }
    var ae = /\/+/g;
    function E(e, t) {
      return typeof e == `object` && e && e.key != null ? ie(`` + e.key) : t.toString(36);
    }
    function oe(e) {
      switch (e.status) {
        case `fulfilled`:
          return e.value;
        case `rejected`:
          throw e.reason;
        default:
          switch (
            (typeof e.status == `string`
              ? e.then(te, te)
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
    function se(e, r, i, a, o) {
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
                return ((c = e._init), se(c(e._payload), r, i, a, o));
            }
        }
      if (c)
        return (
          (o = o(e)),
          (c = a === `` ? `.` + E(e, 0) : a),
          S(o)
            ? ((i = ``),
              c != null && (i = c.replace(ae, `$&/`) + `/`),
              se(o, r, i, ``, function (e) {
                return e;
              }))
            : o != null &&
              (re(o) &&
                (o = T(
                  o,
                  i +
                    (o.key == null || (e && e.key === o.key)
                      ? ``
                      : (`` + o.key).replace(ae, `$&/`) + `/`) +
                    c,
                )),
              r.push(o)),
          1
        );
      c = 0;
      var l = a === `` ? `.` : a + `:`;
      if (S(e))
        for (var u = 0; u < e.length; u++)
          ((a = e[u]), (s = l + E(a, u)), (c += se(a, r, i, s, o)));
      else if (((u = h(e)), typeof u == `function`))
        for (e = u.call(e), u = 0; !(a = e.next()).done;)
          ((a = a.value), (s = l + E(a, u++)), (c += se(a, r, i, s, o)));
      else if (s === `object`) {
        if (typeof e.then == `function`) return se(oe(e), r, i, a, o);
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
    function D(e, t, n) {
      if (e == null) return e;
      var r = [],
        i = 0;
      return (
        se(e, r, ``, ``, function (e) {
          return t.call(n, e, i++);
        }),
        r
      );
    }
    function O(e) {
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
    var k =
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
    function ce(e) {
      var t = C.T,
        n = {};
      ((n.types = t === null ? null : t.types), (C.T = n));
      try {
        var r = e(),
          i = C.S;
        (i !== null && i(n, r),
          typeof r == `object` && r && typeof r.then == `function` && r.then(te, k));
      } catch (e) {
        k(e);
      } finally {
        (t !== null && n.types !== null && (t.types = n.types), (C.T = t));
      }
    }
    function A(e) {
      var t = C.T;
      if (t !== null) {
        var n = t.types;
        n === null ? (t.types = [e]) : n.indexOf(e) === -1 && n.push(e);
      } else ce(A.bind(null, e));
    }
    var j = {
      map: D,
      forEach: function (e, t, n) {
        D(
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
          D(e, function () {
            t++;
          }),
          t
        );
      },
      toArray: function (e) {
        return (
          D(e, function (e) {
            return e;
          }) || []
        );
      },
      only: function (e) {
        if (!re(e))
          throw Error(`React.Children.only expected to receive a single React element child.`);
        return e;
      },
    };
    ((e.Activity = f),
      (e.Children = j),
      (e.Component = y),
      (e.Fragment = r),
      (e.Profiler = a),
      (e.PureComponent = x),
      (e.StrictMode = i),
      (e.Suspense = l),
      (e.ViewTransition = p),
      (e.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE = C),
      (e.__COMPILER_RUNTIME = {
        __proto__: null,
        c: function (e) {
          return C.H.useMemoCache(e);
        },
      }),
      (e.addTransitionType = A),
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
            !ne.call(t, a) ||
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
        return w(e.type, i, r);
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
            ne.call(t, r) && r !== `key` && r !== `__self` && r !== `__source` && (i[r] = t[r]);
        var o = arguments.length - 2;
        if (o === 1) i.children = n;
        else if (1 < o) {
          for (var s = Array(o), c = 0; c < o; c++) s[c] = arguments[c + 2];
          i.children = s;
        }
        if (e && e.defaultProps)
          for (r in ((o = e.defaultProps), o)) i[r] === void 0 && (i[r] = o[r]);
        return w(e, a, i);
      }),
      (e.createRef = function () {
        return { current: null };
      }),
      (e.forwardRef = function (e) {
        return { $$typeof: c, render: e };
      }),
      (e.isValidElement = re),
      (e.lazy = function (e) {
        return { $$typeof: d, _payload: { _status: -1, _result: e }, _init: O };
      }),
      (e.memo = function (e, t) {
        return { $$typeof: u, type: e, compare: t === void 0 ? null : t };
      }),
      (e.startTransition = ce),
      (e.unstable_useCacheRefresh = function () {
        return C.H.useCacheRefresh();
      }),
      (e.use = function (e) {
        return C.H.use(e);
      }),
      (e.useActionState = function (e, t, n) {
        return C.H.useActionState(e, t, n);
      }),
      (e.useCallback = function (e, t) {
        return C.H.useCallback(e, t);
      }),
      (e.useContext = function (e) {
        return C.H.useContext(e);
      }),
      (e.useDebugValue = function () {}),
      (e.useDeferredValue = function (e, t) {
        return C.H.useDeferredValue(e, t);
      }),
      (e.useEffect = function (e, t) {
        return C.H.useEffect(e, t);
      }),
      (e.useEffectEvent = function (e) {
        return C.H.useEffectEvent(e);
      }),
      (e.useId = function () {
        return C.H.useId();
      }),
      (e.useImperativeHandle = function (e, t, n) {
        return C.H.useImperativeHandle(e, t, n);
      }),
      (e.useInsertionEffect = function (e, t) {
        return C.H.useInsertionEffect(e, t);
      }),
      (e.useLayoutEffect = function (e, t) {
        return C.H.useLayoutEffect(e, t);
      }),
      (e.useMemo = function (e, t) {
        return C.H.useMemo(e, t);
      }),
      (e.useOptimistic = function (e, t) {
        return C.H.useOptimistic(e, t);
      }),
      (e.useReducer = function (e, t, n) {
        return C.H.useReducer(e, t, n);
      }),
      (e.useRef = function (e) {
        return C.H.useRef(e);
      }),
      (e.useState = function (e) {
        return C.H.useState(e);
      }),
      (e.useSyncExternalStore = function (e, t, n) {
        return C.H.useSyncExternalStore(e, t, n);
      }),
      (e.useTransition = function () {
        return C.H.useTransition();
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
        if (n(c) !== null) ((m = !0), ee || ((ee = !0), T()));
        else {
          var t = n(l);
          t !== null && ae(x, t.startTime - e);
        }
      }
    }
    var ee = !1,
      S = -1,
      te = 5,
      C = -1;
    function ne() {
      return g ? !0 : !(e.unstable_now() - C < te);
    }
    function w() {
      if (((g = !1), ee)) {
        var t = e.unstable_now();
        C = t;
        var i = !0;
        try {
          a: {
            ((m = !1), h && ((h = !1), v(S), (S = -1)), (p = !0));
            var a = f;
            try {
              b: {
                for (b(t), d = n(c); d !== null && !(d.expirationTime > t && ne());) {
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
                  (u !== null && ae(x, u.startTime - t), (i = !1));
                }
              }
              break a;
            } finally {
              ((d = null), (f = a), (p = !1));
            }
            i = void 0;
          }
        } finally {
          i ? T() : (ee = !1);
        }
      }
    }
    var T;
    if (typeof y == `function`)
      T = function () {
        y(w);
      };
    else if (typeof MessageChannel < `u`) {
      var re = new MessageChannel(),
        ie = re.port2;
      ((re.port1.onmessage = w),
        (T = function () {
          ie.postMessage(null);
        }));
    } else
      T = function () {
        _(w, 0);
      };
    function ae(t, n) {
      S = _(function () {
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
          : (te = 0 < e ? Math.floor(1e3 / e) : 5);
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
              n(c) === null && r === n(l) && (h ? (v(S), (S = -1)) : (h = !0), ae(x, a - o)))
            : ((r.sortIndex = s), t(c, r), m || p || ((m = !0), ee || ((ee = !0), T()))),
          r
        );
      }),
      (e.unstable_shouldYield = ne),
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
      ee = null;
    function S(e, t, n) {
      return e === n || (e === t && ((x = e), !0));
    }
    function te(e, t, n) {
      return e === n ? ((ee = e), !1) : e === t && (ee !== null && (x = e), !0);
    }
    function C(e) {
      if (e === null) return null;
      do e = e === null ? null : e.return;
      while (e && e.tag !== 5 && e.tag !== 27 && e.tag !== 3);
      return e || null;
    }
    function ne(e, t, n) {
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
    var w = Object.assign,
      T = Symbol.for(`react.element`),
      re = Symbol.for(`react.transitional.element`),
      ie = Symbol.for(`react.portal`),
      ae = Symbol.for(`react.fragment`),
      E = Symbol.for(`react.strict_mode`),
      oe = Symbol.for(`react.profiler`),
      se = Symbol.for(`react.consumer`),
      D = Symbol.for(`react.context`),
      O = Symbol.for(`react.forward_ref`),
      k = Symbol.for(`react.suspense`),
      ce = Symbol.for(`react.suspense_list`),
      A = Symbol.for(`react.memo`),
      j = Symbol.for(`react.lazy`),
      le = Symbol.for(`react.activity`),
      ue = Symbol.for(`react.legacy_hidden`),
      de = Symbol.for(`react.memo_cache_sentinel`),
      fe = Symbol.for(`react.view_transition`),
      pe = Symbol.for(`react.recoverable`),
      me = Symbol.iterator;
    function he(e) {
      return typeof e != `object` || !e
        ? null
        : ((e = (me && e[me]) || e[`@@iterator`]), typeof e == `function` ? e : null);
    }
    var ge = Symbol.for(`react.client.reference`);
    function _e(e) {
      if (e == null) return null;
      if (typeof e == `function`) return e.$$typeof === ge ? null : e.displayName || e.name || null;
      if (typeof e == `string`) return e;
      switch (e) {
        case ae:
          return `Fragment`;
        case oe:
          return `Profiler`;
        case E:
          return `StrictMode`;
        case k:
          return `Suspense`;
        case ce:
          return `SuspenseList`;
        case le:
          return `Activity`;
        case fe:
          return `ViewTransition`;
      }
      if (typeof e == `object`)
        switch (e.$$typeof) {
          case ie:
            return `Portal`;
          case D:
            return e.displayName || `Context`;
          case se:
            return (e._context.displayName || `Context`) + `.Consumer`;
          case O:
            var t = e.render;
            return (
              (e = e.displayName),
              (e ||=
                ((e = t.displayName || t.name || ``),
                e === `` ? `ForwardRef` : `ForwardRef(` + e + `)`)),
              e
            );
          case A:
            return ((t = e.displayName || null), t === null ? _e(e.type) || `Memo` : t);
          case j:
            ((t = e._payload), (e = e._init));
            try {
              return _e(e(t));
            } catch {}
        }
      return null;
    }
    var ve = Array.isArray,
      M = n.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,
      N = r.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,
      ye = { pending: !1, data: null, method: null, action: null },
      be = [],
      xe = -1;
    function Se(e) {
      return { current: e };
    }
    function Ce(e) {
      0 > xe || ((e.current = be[xe]), (be[xe] = null), xe--);
    }
    function P(e, t) {
      (xe++, (be[xe] = e.current), (e.current = t));
    }
    var we = Se(null),
      Te = Se(null),
      Ee = Se(null),
      De = Se(null);
    function Oe(e, t) {
      switch ((P(Ee, t), P(Te, e), P(we, null), t.nodeType)) {
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
      (Ce(we), P(we, e));
    }
    function ke() {
      (Ce(we), Ce(Te), Ce(Ee));
    }
    function Ae(e) {
      var t = e.memoizedState;
      (t !== null && ((sh._currentValue = t.memoizedState), P(De, e)), (t = we.current));
      var n = dp(t, e.type);
      t !== n && (P(Te, e), P(we, n));
    }
    function je(e) {
      (Te.current === e && (Ce(we), Ce(Te)), De.current === e && (Ce(De), (sh._currentValue = ye)));
    }
    var Me, Ne;
    function Pe(e) {
      if (Me === void 0)
        try {
          throw Error();
        } catch (e) {
          var t = e.stack.trim().match(/\n( *(at )?)/);
          ((Me = (t && t[1]) || ``),
            (Ne =
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
        Me +
        e +
        Ne
      );
    }
    var Fe = !1;
    function Ie(e, t) {
      if (!e || Fe) return ``;
      Fe = !0;
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
        ((Fe = !1), (Error.prepareStackTrace = n));
      }
      return (n = e ? e.displayName || e.name : ``) ? Pe(n) : ``;
    }
    function Le(e, t) {
      switch (e.tag) {
        case 26:
        case 27:
        case 5:
          return Pe(e.type);
        case 16:
          return Pe(`Lazy`);
        case 13:
          return e.child !== t && t !== null ? Pe(`Suspense Fallback`) : Pe(`Suspense`);
        case 19:
          return Pe(`SuspenseList`);
        case 0:
        case 15:
          return Ie(e.type, !1);
        case 11:
          return Ie(e.type.render, !1);
        case 1:
          return Ie(e.type, !0);
        case 31:
          return Pe(`Activity`);
        case 30:
          return Pe(`ViewTransition`);
        default:
          return ``;
      }
    }
    function Re(e) {
      try {
        var t = ``,
          n = null;
        do ((t += Le(e, n)), (n = e), (e = e.return));
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
    var ze = Object.prototype.hasOwnProperty,
      Be = t.unstable_scheduleCallback,
      Ve = t.unstable_cancelCallback,
      He = t.unstable_shouldYield,
      Ue = t.unstable_requestPaint,
      We = t.unstable_now,
      Ge = t.unstable_getCurrentPriorityLevel,
      Ke = t.unstable_ImmediatePriority,
      qe = t.unstable_UserBlockingPriority,
      Je = t.unstable_NormalPriority,
      Ye = t.unstable_LowPriority,
      Xe = t.unstable_IdlePriority,
      Ze = t.log,
      Qe = t.unstable_setDisableYieldValue,
      $e = null,
      et = null;
    function tt(e) {
      if ((typeof Ze == `function` && Qe(e), et && typeof et.setStrictMode == `function`))
        try {
          et.setStrictMode($e, e);
        } catch {}
    }
    var nt = Math.clz32 ? Math.clz32 : at,
      rt = Math.log,
      it = Math.LN2;
    function at(e) {
      return ((e >>>= 0), e === 0 ? 32 : (31 - ((rt(e) / it) | 0)) | 0);
    }
    var ot = 256,
      st = 262144,
      ct = 4194304;
    function F(e) {
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
    function lt(e, t, n) {
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
                ? n || ((n = r & ~e), n !== 0 && (i = F(n)))
                : (i = F(o))
              : (i = F(s)))
          : ((r = s & ~a),
            r === 0
              ? ((o &= s), o === 0 ? n || ((n = s & ~e), n !== 0 && (i = F(n))) : (i = F(o)))
              : (i = F(r))),
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
    function ut(e, t) {
      return (e.pendingLanes & ~(e.suspendedLanes & ~e.pingedLanes) & t) === 0;
    }
    function dt(e, t) {
      t & 8 && (t |= t & 32);
      var n = e.entangledLanes;
      if (n !== 0)
        for (e = e.entanglements, n &= t; 0 < n;) {
          var r = 31 - nt(n),
            i = 1 << r;
          ((t |= e[r]), (n &= ~i));
        }
      return t;
    }
    function ft(e, t) {
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
    function pt() {
      var e = ct;
      return ((ct <<= 1), !(ct & 62914560) && (ct = 4194304), e);
    }
    function mt(e) {
      for (var t = [], n = 0; 31 > n; n++) t.push(e);
      return t;
    }
    function ht(e, t) {
      ((e.pendingLanes |= t),
        t !== 268435456 && ((e.suspendedLanes = 0), (e.pingedLanes = 0), (e.warmLanes = 0)));
    }
    function gt(e, t, n, r, i, a) {
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
        var u = 31 - nt(n),
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
      (r !== 0 && _t(e, r, 0),
        a !== 0 && i === 0 && e.tag !== 0 && (e.suspendedLanes |= a & ~(o & ~t)));
    }
    function _t(e, t, n) {
      ((e.pendingLanes |= t), (e.suspendedLanes &= ~t));
      var r = 31 - nt(t);
      ((e.entangledLanes |= t),
        (e.entanglements[r] = e.entanglements[r] | 1073741824 | (n & 261930)));
    }
    function vt(e, t) {
      var n = (e.entangledLanes |= t);
      for (e = e.entanglements; n;) {
        var r = 31 - nt(n),
          i = 1 << r;
        ((i & t) | (e[r] & t) && (e[r] |= t), (n &= ~i));
      }
    }
    function yt(e, t) {
      var n = t & -t;
      return ((n = n & 42 ? 1 : bt(n)), (n & (e.suspendedLanes | t)) === 0 ? n : 0);
    }
    function bt(e) {
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
    function xt(e) {
      return ((e &= -e), 2 < e ? (8 < e ? (e & 134217727 ? 32 : 268435456) : 8) : 2);
    }
    function St() {
      var e = N.p;
      return e === 0 ? ((e = window.event), e === void 0 ? 32 : Ch(e.type)) : e;
    }
    function Ct(e, t) {
      var n = N.p;
      try {
        return ((N.p = e), t());
      } finally {
        N.p = n;
      }
    }
    var wt = Math.random().toString(36).slice(2),
      Tt = `__reactFiber$` + wt,
      Et = `__reactProps$` + wt,
      Dt = `__reactContainer$` + wt,
      Ot = `__reactEvents$` + wt,
      kt = `__reactListeners$` + wt,
      At = `__reactHandles$` + wt,
      I = `__reactResources$` + wt,
      jt = `__reactMarker$` + wt,
      Mt = `__reactLoad$` + wt;
    function Nt(e) {
      (delete e[Tt], delete e[Et], delete e[kt], delete e[At]);
    }
    function Pt(e) {
      var t;
      if ((t = e[Tt])) return t;
      for (var n = e.parentNode; n;) {
        if ((t = n[Dt] || n[Tt])) {
          if (((n = t.alternate), t.child !== null || (n !== null && n.child !== null)))
            for (e = fm(e); e !== null;) {
              if ((n = e[Tt])) return n;
              e = fm(e);
            }
          return t;
        }
        ((e = n), (n = e.parentNode));
      }
      return null;
    }
    function Ft(e) {
      if ((e = e[Tt] || e[Dt])) {
        var t = e.tag;
        if (t === 5 || t === 6 || t === 13 || t === 31 || t === 26 || t === 27 || t === 3) return e;
      }
      return null;
    }
    function It(e) {
      var t = e.tag;
      if (t === 5 || t === 26 || t === 27 || t === 6) return e.stateNode;
      throw Error(i(33));
    }
    function Lt(e) {
      var t = e[I];
      return ((t ||= e[I] = { hoistableStyles: new Map(), hoistableScripts: new Map() }), t);
    }
    function Rt(e) {
      e[jt] = !0;
    }
    function zt(e) {
      e[Mt] = void 0;
    }
    var Bt = new Set(),
      Vt = {};
    function Ht(e, t) {
      (Ut(e, t), Ut(e + `Capture`, t));
    }
    function Ut(e, t) {
      for (Vt[e] = t, e = 0; e < t.length; e++) Bt.add(t[e]);
    }
    var Wt = RegExp(
        `^[:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD][:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040]*$`,
      ),
      Gt = {},
      Kt = {};
    function qt(e) {
      return ze.call(Kt, e)
        ? !0
        : ze.call(Gt, e)
          ? !1
          : Wt.test(e)
            ? (Kt[e] = !0)
            : ((Gt[e] = !0), !1);
    }
    var L = !1;
    function Jt() {
      var e = L;
      return ((L = !1), e);
    }
    function Yt(e, t, n) {
      if (qt(t)) {
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
    function Xt(e, t, n) {
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
    function Zt(e, t, n, r) {
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
    function Qt(e) {
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
    function $t(e) {
      var t = e.type;
      return (e = e.nodeName) && e.toLowerCase() === `input` && (t === `checkbox` || t === `radio`);
    }
    function en(e, t, n) {
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
    function tn(e) {
      if (!e._valueTracker) {
        var t = $t(e) ? `checked` : `value`;
        e._valueTracker = en(e, t, `` + e[t]);
      }
    }
    function nn(e) {
      if (!e) return !1;
      var t = e._valueTracker;
      if (!t) return !0;
      var n = t.getValue(),
        r = ``;
      return (
        e && (r = $t(e) ? (e.checked ? `true` : `false`) : e.value),
        (e = r),
        e !== n && (t.setValue(e), !0)
      );
    }
    var rn = /[\n"\\]/g;
    function an(e) {
      return e.replace(rn, function (e) {
        return `\\` + e.charCodeAt(0).toString(16) + ` `;
      });
    }
    function on(e, t, n, r, i, a, o, s) {
      ((e.name = ``),
        o != null && typeof o != `function` && typeof o != `symbol` && typeof o != `boolean`
          ? (e.type = o)
          : e.removeAttribute(`type`),
        t == null
          ? (o !== `submit` && o !== `reset`) || e.removeAttribute(`value`)
          : o === `number`
            ? ((t === 0 && e.value === ``) || e.value != t) && (e.value = `` + Qt(t))
            : e.value !== `` + Qt(t) && (e.value = `` + Qt(t)),
        t == null
          ? n == null
            ? r != null && e.removeAttribute(`value`)
            : cn(e, Qt(n))
          : o === `number` && e.value == t
            ? cn(e, Qt(e.value))
            : cn(e, Qt(t)),
        i == null && a != null && (e.defaultChecked = !!a),
        i != null && (e.checked = i && typeof i != `function` && typeof i != `symbol`),
        s != null && typeof s != `function` && typeof s != `symbol` && typeof s != `boolean`
          ? (e.name = `` + Qt(s))
          : e.removeAttribute(`name`));
    }
    function sn(e, t, n, r, i, a, o, s) {
      if (
        (a != null &&
          typeof a != `function` &&
          typeof a != `symbol` &&
          typeof a != `boolean` &&
          (e.type = a),
        t != null || n != null)
      ) {
        if (!((a !== `submit` && a !== `reset`) || t != null)) {
          tn(e);
          return;
        }
        ((n = n == null ? `` : `` + Qt(n)),
          (t = t == null ? n : `` + Qt(t)),
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
        tn(e));
    }
    function cn(e, t) {
      e.defaultValue !== `` + t && (e.defaultValue = `` + t);
    }
    function ln(e, t, n, r) {
      if (((e = e.options), t)) {
        t = {};
        for (var i = 0; i < n.length; i++) t[`$` + n[i]] = !0;
        for (n = 0; n < e.length; n++)
          ((i = t.hasOwnProperty(`$` + e[n].value)),
            e[n].selected !== i && (e[n].selected = i),
            i && r && (e[n].defaultSelected = !0));
      } else {
        for (n = `` + Qt(n), t = null, i = 0; i < e.length; i++) {
          if (e[i].value === n) {
            ((e[i].selected = !0), r && (e[i].defaultSelected = !0));
            return;
          }
          t !== null || e[i].disabled || (t = e[i]);
        }
        t !== null && (t.selected = !0);
      }
    }
    function un(e, t, n) {
      t != null && ((t = `` + Qt(t)), t !== e.value && (e.value = t), n == null)
        ? e.defaultValue !== t && (e.defaultValue = t)
        : (e.defaultValue = n == null ? `` : `` + Qt(n));
    }
    function dn(e, t, n, r) {
      if (t == null) {
        if (r != null) {
          if (n != null) throw Error(i(92));
          if (ve(r)) {
            if (1 < r.length) throw Error(i(93));
            r = r[0];
          }
          n = r;
        }
        ((n ??= ``), (t = n));
      }
      ((n = Qt(t)),
        (e.defaultValue = n),
        (r = e.textContent),
        r === n && r !== `` && r !== null && (e.value = r),
        tn(e));
    }
    function fn(e, t) {
      if (t) {
        var n = e.firstChild;
        if (n && n === e.lastChild && n.nodeType === 3) {
          n.nodeValue = t;
          return;
        }
      }
      e.textContent = t;
    }
    var pn = new Set(
      `animationIterationCount aspectRatio borderImageOutset borderImageSlice borderImageWidth boxFlex boxFlexGroup boxOrdinalGroup columnCount columns flex flexGrow flexPositive flexShrink flexNegative flexOrder gridArea gridRow gridRowEnd gridRowSpan gridRowStart gridColumn gridColumnEnd gridColumnSpan gridColumnStart fontWeight lineClamp lineHeight opacity order orphans scale tabSize widows zIndex zoom fillOpacity floodOpacity stopOpacity strokeDasharray strokeDashoffset strokeMiterlimit strokeOpacity strokeWidth MozAnimationIterationCount MozBoxFlex MozBoxFlexGroup MozLineClamp msAnimationIterationCount msFlex msZoom msFlexGrow msFlexNegative msFlexOrder msFlexPositive msFlexShrink msGridColumn msGridColumnSpan msGridRow msGridRowSpan WebkitAnimationIterationCount WebkitBoxFlex WebKitBoxFlexGroup WebkitBoxOrdinalGroup WebkitColumnCount WebkitColumns WebkitFlex WebkitFlexGrow WebkitFlexPositive WebkitFlexShrink WebkitLineClamp`.split(
        ` `,
      ),
    );
    function mn(e, t, n) {
      var r = t.indexOf(`--`) === 0;
      n == null || typeof n == `boolean` || n === ``
        ? r
          ? e.setProperty(t, ``)
          : t === `float`
            ? (e.cssFloat = ``)
            : (e[t] = ``)
        : r
          ? e.setProperty(t, n)
          : typeof n != `number` || n === 0 || pn.has(t)
            ? t === `float`
              ? (e.cssFloat = n)
              : (e[t] = (`` + n).trim())
            : (e[t] = n + `px`);
    }
    function hn(e, t, n) {
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
            (L = !0));
        for (var a in t) ((r = t[a]), t.hasOwnProperty(a) && n[a] !== r && (mn(e, a, r), (L = !0)));
      } else for (var o in t) t.hasOwnProperty(o) && mn(e, o, t[o]);
    }
    function gn(e) {
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
    var _n = new Map([
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
      vn =
        /^[\u0000-\u001F ]*j[\r\n\t]*a[\r\n\t]*v[\r\n\t]*a[\r\n\t]*s[\r\n\t]*c[\r\n\t]*r[\r\n\t]*i[\r\n\t]*p[\r\n\t]*t[\r\n\t]*:/i;
    function yn(e) {
      return vn.test(`` + e)
        ? `javascript:throw new Error('React has blocked a javascript: URL as a security precaution.')`
        : e;
    }
    function bn() {}
    var xn = null;
    function Sn(e) {
      return (
        (e = e.target || e.srcElement || window),
        e.correspondingUseElement && (e = e.correspondingUseElement),
        e.nodeType === 3 ? e.parentNode : e
      );
    }
    var Cn = null,
      wn = null;
    function Tn(e) {
      var t = Ft(e);
      if (t && (e = t.stateNode)) {
        var n = e[Et] || null;
        a: switch (((e = t.stateNode), t.type)) {
          case `input`:
            if (
              (on(
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
                n = n.querySelectorAll(`input[name="` + an(`` + t) + `"][type="radio"]`), t = 0;
                t < n.length;
                t++
              ) {
                var r = n[t];
                if (r !== e && r.form === e.form) {
                  var a = r[Et] || null;
                  if (!a) throw Error(i(90));
                  on(
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
              for (t = 0; t < n.length; t++) ((r = n[t]), r.form === e.form && nn(r));
            }
            break a;
          case `textarea`:
            un(e, n.value, n.defaultValue);
            break a;
          case `select`:
            ((t = n.value), t != null && ln(e, !!n.multiple, t, !1));
        }
      }
    }
    var En = !1;
    function Dn(e, t, n) {
      if (En) return e(t, n);
      En = !0;
      try {
        return e(t);
      } finally {
        if (
          ((En = !1),
          (Cn !== null || wn !== null) &&
            (zd(), Cn && ((t = Cn), (e = wn), (wn = Cn = null), Tn(t), e)))
        )
          for (t = 0; t < e.length; t++) Tn(e[t]);
      }
    }
    function On(e, t) {
      var n = e.stateNode;
      if (n === null) return null;
      var r = n[Et] || null;
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
    var kn =
        typeof window < `u` &&
        window.document !== void 0 &&
        window.document.createElement !== void 0,
      An = !1;
    if (kn)
      try {
        var jn = {};
        (Object.defineProperty(jn, "passive", {
          get: function () {
            An = !0;
          },
        }),
          window.addEventListener(`test`, jn, jn),
          window.removeEventListener(`test`, jn, jn));
      } catch {
        An = !1;
      }
    var Mn = null,
      Nn = null,
      Pn = null;
    function Fn() {
      if (Pn) return Pn;
      var e,
        t = Nn,
        n = t.length,
        r,
        i = `value` in Mn ? Mn.value : Mn.textContent,
        a = i.length;
      for (e = 0; e < n && t[e] === i[e]; e++);
      var o = n - e;
      for (r = 1; r <= o && t[n - r] === i[a - r]; r++);
      return (Pn = i.slice(e, 1 < r ? 1 - r : void 0));
    }
    function In(e) {
      var t = e.keyCode;
      return (
        `charCode` in e ? ((e = e.charCode), e === 0 && t === 13 && (e = 13)) : (e = t),
        e === 10 && (e = 13),
        32 <= e || e === 13 ? e : 0
      );
    }
    function Ln() {
      return !0;
    }
    function Rn() {
      return !1;
    }
    function zn(e) {
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
            ? Ln
            : Rn),
          (this.isPropagationStopped = Rn),
          this
        );
      }
      return (
        w(t.prototype, {
          preventDefault: function () {
            this.defaultPrevented = !0;
            var e = this.nativeEvent;
            e &&
              (e.preventDefault
                ? e.preventDefault()
                : typeof e.returnValue != `unknown` && (e.returnValue = !1),
              (this.isDefaultPrevented = Ln));
          },
          stopPropagation: function () {
            var e = this.nativeEvent;
            e &&
              (e.stopPropagation
                ? e.stopPropagation()
                : typeof e.cancelBubble != `unknown` && (e.cancelBubble = !0),
              (this.isPropagationStopped = Ln));
          },
          persist: function () {},
          isPersistent: Ln,
        }),
        t
      );
    }
    var Bn = {
        eventPhase: 0,
        bubbles: 0,
        cancelable: 0,
        timeStamp: function (e) {
          return e.timeStamp || Date.now();
        },
        defaultPrevented: 0,
        isTrusted: 0,
      },
      Vn = zn(Bn),
      Hn = w({}, Bn, { view: 0, detail: 0 }),
      Un = zn(Hn),
      Wn,
      Gn,
      Kn,
      qn = w({}, Hn, {
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
        getModifierState: ir,
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
            : (e !== Kn &&
                (Kn && e.type === `mousemove`
                  ? ((Wn = e.screenX - Kn.screenX), (Gn = e.screenY - Kn.screenY))
                  : (Gn = Wn = 0),
                (Kn = e)),
              Wn);
        },
        movementY: function (e) {
          return `movementY` in e ? e.movementY : Gn;
        },
      }),
      Jn = zn(qn),
      Yn = zn(w({}, qn, { dataTransfer: 0 })),
      Xn = zn(w({}, Hn, { relatedTarget: 0 })),
      Zn = zn(w({}, Bn, { animationName: 0, elapsedTime: 0, pseudoElement: 0 })),
      Qn = zn(
        w({}, Bn, {
          clipboardData: function (e) {
            return `clipboardData` in e ? e.clipboardData : window.clipboardData;
          },
        }),
      ),
      $n = zn(w({}, Bn, { data: 0 })),
      er = {
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
      tr = {
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
      nr = { Alt: `altKey`, Control: `ctrlKey`, Meta: `metaKey`, Shift: `shiftKey` };
    function rr(e) {
      var t = this.nativeEvent;
      return t.getModifierState ? t.getModifierState(e) : (e = nr[e]) ? !!t[e] : !1;
    }
    function ir() {
      return rr;
    }
    var ar = zn(
        w({}, Hn, {
          key: function (e) {
            if (e.key) {
              var t = er[e.key] || e.key;
              if (t !== `Unidentified`) return t;
            }
            return e.type === `keypress`
              ? ((e = In(e)), e === 13 ? `Enter` : String.fromCharCode(e))
              : e.type === `keydown` || e.type === `keyup`
                ? tr[e.keyCode] || `Unidentified`
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
          getModifierState: ir,
          charCode: function (e) {
            return e.type === `keypress` ? In(e) : 0;
          },
          keyCode: function (e) {
            return e.type === `keydown` || e.type === `keyup` ? e.keyCode : 0;
          },
          which: function (e) {
            return e.type === `keypress`
              ? In(e)
              : e.type === `keydown` || e.type === `keyup`
                ? e.keyCode
                : 0;
          },
        }),
      ),
      or = zn(
        w({}, qn, {
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
      sr = zn(w({}, Bn, { submitter: 0 })),
      cr = zn(
        w({}, Hn, {
          touches: 0,
          targetTouches: 0,
          changedTouches: 0,
          altKey: 0,
          metaKey: 0,
          ctrlKey: 0,
          shiftKey: 0,
          getModifierState: ir,
        }),
      ),
      lr = zn(w({}, Bn, { propertyName: 0, elapsedTime: 0, pseudoElement: 0 })),
      ur = zn(
        w({}, qn, {
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
      dr = zn(w({}, Bn, { newState: 0, oldState: 0, source: 0 })),
      fr = [9, 13, 27, 32],
      pr = kn && `CompositionEvent` in window,
      mr = null;
    kn && `documentMode` in document && (mr = document.documentMode);
    var hr = kn && `TextEvent` in window && !mr,
      gr = kn && (!pr || (mr && 8 < mr && 11 >= mr)),
      _r = ` `,
      vr = !1;
    function yr(e, t) {
      switch (e) {
        case `keyup`:
          return fr.indexOf(t.keyCode) !== -1;
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
    function br(e) {
      return ((e = e.detail), typeof e == `object` && `data` in e ? e.data : null);
    }
    var xr = !1;
    function Sr(e, t) {
      switch (e) {
        case `compositionend`:
          return br(t);
        case `keypress`:
          return t.which === 32 ? ((vr = !0), _r) : null;
        case `textInput`:
          return ((e = t.data), e === _r && vr ? null : e);
        default:
          return null;
      }
    }
    function Cr(e, t) {
      if (xr)
        return e === `compositionend` || (!pr && yr(e, t))
          ? ((e = Fn()), (Pn = Nn = Mn = null), (xr = !1), e)
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
          return gr && t.locale !== `ko` ? null : t.data;
        default:
          return null;
      }
    }
    var wr = {
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
    function Tr(e) {
      var t = e && e.nodeName && e.nodeName.toLowerCase();
      return t === `input` ? !!wr[e.type] : t === `textarea`;
    }
    function Er(e, t, n, r) {
      (Cn ? (wn ? wn.push(r) : (wn = [r])) : (Cn = r),
        (t = Jf(t, `onChange`)),
        0 < t.length &&
          ((n = new Vn(`onChange`, `change`, null, n, r)), e.push({ event: n, listeners: t })));
    }
    var Dr = null,
      Or = null;
    function kr(e) {
      Vf(e, 0);
    }
    function Ar(e) {
      if (nn(It(e))) return e;
    }
    function jr(e, t) {
      if (e === `change`) return t;
    }
    var Mr = !1;
    if (kn) {
      var Nr;
      if (kn) {
        var Pr = `oninput` in document;
        if (!Pr) {
          var Fr = document.createElement(`div`);
          (Fr.setAttribute(`oninput`, `return;`), (Pr = typeof Fr.oninput == `function`));
        }
        Nr = Pr;
      } else Nr = !1;
      Mr = Nr && (!document.documentMode || 9 < document.documentMode);
    }
    function Ir() {
      Dr && (Dr.detachEvent(`onpropertychange`, Lr), (Or = Dr = null));
    }
    function Lr(e) {
      if (e.propertyName === `value` && Ar(Or)) {
        var t = [];
        (Er(t, Or, e, Sn(e)), Dn(kr, t));
      }
    }
    function Rr(e, t, n) {
      e === `focusin`
        ? (Ir(), (Dr = t), (Or = n), Dr.attachEvent(`onpropertychange`, Lr))
        : e === `focusout` && Ir();
    }
    function zr(e) {
      if (e === `selectionchange` || e === `keyup` || e === `keydown`) return Ar(Or);
    }
    function Br(e, t) {
      if (e === `click`) return Ar(t);
    }
    function Vr(e, t) {
      if (e === `input` || e === `change`) return Ar(t);
    }
    function Hr(e, t) {
      return (e === t && (e !== 0 || 1 / e == 1 / t)) || (e !== e && t !== t);
    }
    var Ur = typeof Object.is == `function` ? Object.is : Hr;
    function Wr(e, t) {
      if (Ur(e, t)) return !0;
      if (typeof e != `object` || !e || typeof t != `object` || !t) return !1;
      var n = Object.keys(e),
        r = Object.keys(t);
      if (n.length !== r.length) return !1;
      for (r = 0; r < n.length; r++) {
        var i = n[r];
        if (!ze.call(t, i) || !Ur(e[i], t[i])) return !1;
      }
      return !0;
    }
    function Gr(e) {
      if (((e ||= typeof document < `u` ? document : void 0), e === void 0)) return null;
      try {
        return e.activeElement || e.body;
      } catch {
        return e.body;
      }
    }
    function Kr(e) {
      for (; e && e.firstChild;) e = e.firstChild;
      return e;
    }
    function qr(e, t) {
      var n = Kr(e);
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
        n = Kr(n);
      }
    }
    function Jr(e, t) {
      return e && t
        ? e === t
          ? !0
          : e && e.nodeType === 3
            ? !1
            : t && t.nodeType === 3
              ? Jr(e, t.parentNode)
              : `contains` in e
                ? e.contains(t)
                : e.compareDocumentPosition
                  ? !!(e.compareDocumentPosition(t) & 16)
                  : !1
        : !1;
    }
    function Yr(e) {
      e =
        e != null && e.ownerDocument != null && e.ownerDocument.defaultView != null
          ? e.ownerDocument.defaultView
          : window;
      for (var t = Gr(e.document); t instanceof e.HTMLIFrameElement;) {
        try {
          var n = typeof t.contentWindow.location.href == `string`;
        } catch {
          n = !1;
        }
        if (n) e = t.contentWindow;
        else break;
        t = Gr(e.document);
      }
      return t;
    }
    function Xr(e) {
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
    var Zr = kn && `documentMode` in document && 11 >= document.documentMode,
      Qr = null,
      $r = null,
      ei = null,
      ti = !1;
    function ni(e, t, n) {
      var r = n.window === n ? n.document : n.nodeType === 9 ? n : n.ownerDocument;
      ti ||
        Qr == null ||
        Qr !== Gr(r) ||
        ((r = Qr),
        `selectionStart` in r && Xr(r)
          ? (r = { start: r.selectionStart, end: r.selectionEnd })
          : ((r = ((r.ownerDocument && r.ownerDocument.defaultView) || window).getSelection()),
            (r = {
              anchorNode: r.anchorNode,
              anchorOffset: r.anchorOffset,
              focusNode: r.focusNode,
              focusOffset: r.focusOffset,
            })),
        (ei && Wr(ei, r)) ||
          ((ei = r),
          (r = Jf($r, `onSelect`)),
          0 < r.length &&
            ((t = new Vn(`onSelect`, `select`, null, t, n)),
            e.push({ event: t, listeners: r }),
            (t.target = Qr))));
    }
    function ri(e, t) {
      var n = {};
      return (
        (n[e.toLowerCase()] = t.toLowerCase()),
        (n[`Webkit` + e] = `webkit` + t),
        (n[`Moz` + e] = `moz` + t),
        n
      );
    }
    var ii = {
        animationend: ri(`Animation`, `AnimationEnd`),
        animationiteration: ri(`Animation`, `AnimationIteration`),
        animationstart: ri(`Animation`, `AnimationStart`),
        transitionrun: ri(`Transition`, `TransitionRun`),
        transitionstart: ri(`Transition`, `TransitionStart`),
        transitioncancel: ri(`Transition`, `TransitionCancel`),
        transitionend: ri(`Transition`, `TransitionEnd`),
      },
      ai = {},
      oi = {};
    kn &&
      ((oi = document.createElement(`div`).style),
      `AnimationEvent` in window ||
        (delete ii.animationend.animation,
        delete ii.animationiteration.animation,
        delete ii.animationstart.animation),
      `TransitionEvent` in window || delete ii.transitionend.transition);
    function si(e) {
      if (ai[e]) return ai[e];
      if (!ii[e]) return e;
      var t = ii[e],
        n;
      for (n in t) if (t.hasOwnProperty(n) && n in oi) return (ai[e] = t[n]);
      return e;
    }
    var ci = si(`animationend`),
      li = si(`animationiteration`),
      ui = si(`animationstart`),
      di = si(`transitionrun`),
      fi = si(`transitionstart`),
      pi = si(`transitioncancel`),
      mi = si(`transitionend`),
      hi = new Map(),
      gi =
        `abort auxClick beforeToggle cancel canPlay canPlayThrough click close contextMenu copy cut drag dragEnd dragEnter dragExit dragLeave dragOver dragStart drop durationChange emptied encrypted ended error fullscreenChange fullscreenError gotPointerCapture input invalid keyDown keyPress keyUp load loadedData loadedMetadata loadStart lostPointerCapture mouseDown mouseMove mouseOut mouseOver mouseUp paste pause play playing pointerCancel pointerDown pointerMove pointerOut pointerOver pointerUp progress rateChange reset resize seeked seeking stalled submit suspend timeUpdate touchCancel touchEnd touchStart volumeChange scroll toggle touchMove waiting wheel`.split(
          ` `,
        );
    gi.push(`scrollEnd`);
    function _i(e, t) {
      (hi.set(e, t), Ht(t, [e]));
    }
    var vi = 0;
    function yi(e, t) {
      if (e.name != null && e.name !== `auto`) return e.name;
      if (t.autoName !== null) return t.autoName;
      e = bd.identifierPrefix;
      var n = vi++;
      return ((e = `_` + e + `t_` + n.toString(32) + `_`), (t.autoName = e));
    }
    function bi(e) {
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
    function xi(e, t) {
      return (
        (e = bi(e)), (t = bi(t)), t == null ? (e === `auto` ? null : e) : t === `auto` ? null : t
      );
    }
    var Si =
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
      Ci = [],
      wi = 0,
      Ti = 0;
    function Ei() {
      for (var e = wi, t = (Ti = wi = 0); t < e;) {
        var n = Ci[t];
        Ci[t++] = null;
        var r = Ci[t];
        Ci[t++] = null;
        var i = Ci[t];
        Ci[t++] = null;
        var a = Ci[t];
        if (((Ci[t++] = null), r !== null && i !== null)) {
          var o = r.pending;
          (o === null ? (i.next = i) : ((i.next = o.next), (o.next = i)), (r.pending = i));
        }
        a !== 0 && Ai(n, i, a);
      }
    }
    function Di(e, t, n, r) {
      ((Ci[wi++] = e),
        (Ci[wi++] = t),
        (Ci[wi++] = n),
        (Ci[wi++] = r),
        (Ti |= r),
        (e.lanes |= r),
        (e = e.alternate),
        e !== null && (e.lanes |= r));
    }
    function Oi(e, t, n, r) {
      return (Di(e, t, n, r), ji(e));
    }
    function ki(e, t) {
      return (Di(e, null, null, t), ji(e));
    }
    function Ai(e, t, n) {
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
            ((i = 31 - nt(n)),
            (e = a.hiddenUpdates),
            (r = e[i]),
            r === null ? (e[i] = [t]) : r.push(t),
            (t.lane = n | 536870912)),
          a)
        : null;
    }
    function ji(e) {
      if (50 < kd) throw ((kd = 0), (Ad = null), Error(i(185)));
      for (var t = e.return; t !== null;) ((e = t), (t = e.return));
      return e.tag === 3 ? e.stateNode : null;
    }
    var Mi = {};
    function Ni(e, t, n, r) {
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
    function Pi(e, t, n, r) {
      return new Ni(e, t, n, r);
    }
    function Fi(e) {
      return ((e = e.prototype), !(!e || !e.isReactComponent));
    }
    function Ii(e, t) {
      var n = e.alternate;
      return (
        n === null
          ? ((n = Pi(e.tag, t, e.key, e.mode)),
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
    function Li(e, t) {
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
    function Ri(e, t, n, r, a, o) {
      var s = 0;
      if (((r = e), typeof r == `function`)) Fi(r) && (s = 1);
      else if (typeof r == `string`)
        s = qm(e, n, we.current) ? 26 : e === `html` || e === `head` || e === `body` ? 27 : 5;
      else
        a: switch (r) {
          case le:
            return ((e = Pi(31, n, t, a)), (e.elementType = le), (e.lanes = o), e);
          case ae:
            return zi(n.children, a, o, t);
          case E:
            ((s = 8), (a |= 24));
            break;
          case oe:
            return ((e = Pi(12, n, t, a | 2)), (e.elementType = oe), (e.lanes = o), e);
          case k:
            return ((e = Pi(13, n, t, a)), (e.elementType = k), (e.lanes = o), e);
          case ce:
            return ((e = Pi(19, n, t, a)), (e.elementType = ce), (e.lanes = o), e);
          case ue:
          case fe:
            return (
              (e = a | 32),
              (e = Pi(30, n, t, e)),
              (e.elementType = fe),
              (e.lanes = o),
              (e.stateNode = { autoName: null, paired: null, clones: null, ref: null }),
              e
            );
          default:
            if (typeof r == `object` && r)
              switch (r.$$typeof) {
                case D:
                  s = 10;
                  break a;
                case se:
                  s = 9;
                  break a;
                case O:
                  s = 11;
                  break a;
                case A:
                  s = 14;
                  break a;
                case j:
                  ((s = 16), (r = null));
                  break a;
              }
            ((s = 29), (n = Error(i(130, e === null ? `null` : typeof e, ``))), (r = null));
        }
      return ((t = Pi(s, n, t, a)), (t.elementType = e), (t.type = r), (t.lanes = o), t);
    }
    function zi(e, t, n, r) {
      return ((e = Pi(7, e, r, t)), (e.lanes = n), e);
    }
    function Bi(e, t, n) {
      return ((e = Pi(6, e, null, t)), (e.lanes = n), e);
    }
    function Vi(e) {
      var t = Pi(18, null, null, 0);
      return ((t.stateNode = e), t);
    }
    function Hi(e, t, n) {
      return (
        (t = Pi(4, e.children === null ? [] : e.children, e.key, t)),
        (t.lanes = n),
        (t.stateNode = {
          containerInfo: e.containerInfo,
          pendingChildren: null,
          implementation: e.implementation,
        }),
        t
      );
    }
    var Ui = new WeakMap();
    function Wi(e, t) {
      if (typeof e == `object` && e) {
        var n = Ui.get(e);
        return n === void 0 ? ((t = { value: e, source: t, stack: Re(t) }), Ui.set(e, t), t) : n;
      }
      return { value: e, source: t, stack: Re(t) };
    }
    var Gi = [],
      Ki = 0,
      qi = null,
      Ji = 0,
      Yi = [],
      Xi = 0,
      Zi = null,
      Qi = 1,
      $i = ``;
    function ea(e, t) {
      ((Gi[Ki++] = Ji), (Gi[Ki++] = qi), (qi = e), (Ji = t));
    }
    function ta(e, t, n) {
      ((Yi[Xi++] = Qi), (Yi[Xi++] = $i), (Yi[Xi++] = Zi), (Zi = e));
      var r = Qi;
      e = $i;
      var i = 32 - nt(r) - 1;
      ((r &= ~(1 << i)), (n += 1));
      var a = 32 - nt(t) + i;
      if (30 < a) {
        var o = i - (i % 5);
        ((a = (r & ((1 << o) - 1)).toString(32)),
          (r >>= o),
          (i -= o),
          (Qi = (1 << (32 - nt(t) + i)) | (n << i) | r),
          ($i = a + e));
      } else ((Qi = (1 << a) | (n << i) | r), ($i = e));
    }
    function na(e) {
      e.return !== null && (ea(e, 1), ta(e, 1, 0));
    }
    function ra(e) {
      for (; e === qi;) ((qi = Gi[--Ki]), (Gi[Ki] = null), (Ji = Gi[--Ki]), (Gi[Ki] = null));
      for (; e === Zi;)
        ((Zi = Yi[--Xi]),
          (Yi[Xi] = null),
          ($i = Yi[--Xi]),
          (Yi[Xi] = null),
          (Qi = Yi[--Xi]),
          (Yi[Xi] = null));
    }
    function ia(e, t) {
      ((Yi[Xi++] = Qi), (Yi[Xi++] = $i), (Yi[Xi++] = Zi), (Qi = t.id), ($i = t.overflow), (Zi = e));
    }
    var aa = null,
      R = null,
      z = !1,
      oa = null,
      sa = !1,
      ca = Error(i(519));
    function la(e) {
      throw (
        ha(
          Wi(
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
        ca
      );
    }
    function ua(e) {
      var t = e.stateNode,
        n = e.type,
        r = e.memoizedProps;
      switch (((t[Tt] = e), (t[Et] = r), n)) {
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
            sn(t, r.value, r.defaultValue, r.checked, r.defaultChecked, r.type, r.name, !0));
          break;
        case `select`:
          Q(`invalid`, t);
          break;
        case `textarea`:
          (Q(`invalid`, t), dn(t, r.value, r.defaultValue, r.children));
      }
      ((n = r.children),
        (typeof n != `string` && typeof n != `number` && typeof n != `bigint`) ||
        t.textContent === `` + n ||
        !0 === r.suppressHydrationWarning ||
        ep(t.textContent, n)
          ? (r.popover != null && (Q(`beforetoggle`, t), Q(`toggle`, t)),
            r.onScroll != null && Q(`scroll`, t),
            r.onScrollEnd != null && Q(`scrollend`, t),
            r.onClick != null && (t.onclick = bn),
            (t = !0))
          : (t = !1),
        t || la(e, !0));
    }
    function da(e) {
      for (aa = e.return; aa;)
        switch (aa.tag) {
          case 5:
          case 31:
          case 13:
            sa = !1;
            return;
          case 27:
          case 3:
            sa = !0;
            return;
          default:
            aa = aa.return;
        }
    }
    function fa(e) {
      if (e !== aa) return !1;
      if (!z) return (da(e), (z = !0), !1);
      var t = e.tag,
        n;
      if (
        ((n = t !== 3 && t !== 27) &&
          ((n = t === 5) &&
            ((n = e.type), (n = n === `form` || n === `button` || pp(e.type, e.memoizedProps))),
          (n = !n)),
        n && R && la(e),
        da(e),
        t === 13)
      ) {
        if (((e = e.memoizedState), (e = e === null ? null : e.dehydrated), !e))
          throw Error(i(317));
        R = dm(e);
      } else if (t === 31) {
        if (((e = e.memoizedState), (e = e === null ? null : e.dehydrated), !e))
          throw Error(i(317));
        R = dm(e);
      } else
        t === 27
          ? ((t = R), Sp(e.type) ? ((e = um), (um = null), (R = e)) : (R = t))
          : (R = aa ? lm(e.stateNode.nextSibling) : null);
      return !0;
    }
    function pa() {
      ((R = aa = null), (z = !1));
    }
    function ma() {
      var e = oa;
      return (e !== null && (pd === null ? (pd = e) : pd.push.apply(pd, e), (oa = null)), e);
    }
    function ha(e) {
      oa === null ? (oa = [e]) : oa.push(e);
    }
    var ga = Se(null),
      _a = null,
      va = null;
    function ya(e, t, n) {
      (P(ga, t._currentValue), (t._currentValue = n));
    }
    function ba(e) {
      ((e._currentValue = ga.current), Ce(ga));
    }
    function xa(e, t, n) {
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
    function Sa(e, t, n, r) {
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
                  xa(o.return, n, e),
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
            xa(s, n, e),
            (s = null));
        } else
          a.tag === 13 && a.memoizedState !== null && a.memoizedState.dehydrated === null
            ? ((a.lanes |= n),
              (s = a.alternate),
              s !== null && (s.lanes |= n),
              xa(a.return, n, e),
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
    function Ca(e, t, n, r) {
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
            Ur(a.pendingProps.value, s.value) || (e === null ? (e = [c]) : e.push(c));
          }
        } else if (a === De.current) {
          if (((s = a.alternate), s === null)) throw Error(i(387));
          s.memoizedState.memoizedState !== a.memoizedState.memoizedState &&
            (e === null ? (e = [sh]) : e.push(sh));
        }
        a = a.return;
      }
      return (e !== null && Sa(t, e, n, r), (t.flags |= 262144), e !== null);
    }
    function wa(e) {
      for (e = e.firstContext; e !== null;) {
        if (!Ur(e.context._currentValue, e.memoizedValue)) return !0;
        e = e.next;
      }
      return !1;
    }
    function Ta(e) {
      ((_a = e), (va = null), (e = e.dependencies), e !== null && (e.firstContext = null));
    }
    function Ea(e) {
      return Oa(_a, e);
    }
    function Da(e, t) {
      return (_a === null && Ta(e), Oa(e, t));
    }
    function Oa(e, t) {
      var n = t._currentValue;
      if (((t = { context: t, memoizedValue: n, next: null }), va === null)) {
        if (e === null) throw Error(i(308));
        ((va = t), (e.dependencies = { lanes: 0, firstContext: t }), (e.flags |= 524288));
      } else va = va.next = t;
      return n;
    }
    var ka =
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
      Aa = t.unstable_scheduleCallback,
      ja = t.unstable_NormalPriority,
      Ma = {
        $$typeof: D,
        Consumer: null,
        Provider: null,
        _currentValue: null,
        _currentValue2: null,
        _threadCount: 0,
      };
    function Na() {
      return { controller: new ka(), data: new Map(), refCount: 0 };
    }
    function Pa(e) {
      (e.refCount--,
        e.refCount === 0 &&
          Aa(ja, function () {
            e.controller.abort();
          }));
    }
    function Fa(e, t) {
      if (e.pendingLanes & 4194048) {
        var n = e.transitionTypes;
        for (n === null && (n = e.transitionTypes = []), e = 0; e < t.length; e++) {
          var r = t[e];
          n.indexOf(r) === -1 && n.push(r);
        }
      }
    }
    var Ia = null;
    function La(e) {
      var t = e.transitionTypes;
      return ((e.transitionTypes = null), t);
    }
    var Ra = null,
      za = 0,
      Ba = 0,
      Va = null;
    function Ha(e, t) {
      if (Ra === null) {
        var n = (Ra = []);
        ((za = 0),
          (Ba = Pf()),
          (Va = {
            status: `pending`,
            value: void 0,
            then: function (e) {
              n.push(e);
            },
          }));
      }
      return (za++, t.then(Ua, Ua), t);
    }
    function Ua() {
      if (--za === 0 && ((Ia = null), Ra !== null)) {
        Va !== null && (Va.status = `fulfilled`);
        var e = Ra;
        ((Ra = null), (Ba = 0), (Va = null));
        for (var t = 0; t < e.length; t++) (0, e[t])();
      }
    }
    function Wa(e, t) {
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
    var Ga = M.S;
    M.S = function (e, t) {
      if (
        ((gd = We()),
        typeof t == `object` && t && typeof t.then == `function` && Ha(e, t),
        Ia !== null)
      )
        for (var n = bf; n !== null;) (Fa(n, Ia), (n = n.next));
      if (((n = e.types), n !== null)) {
        for (var r = bf; r !== null;) (Fa(r, n), (r = r.next));
        if (Ba !== 0) {
          ((r = Ia), r === null && (r = Ia = []));
          for (var i = 0; i < n.length; i++) {
            var a = n[i];
            r.indexOf(a) === -1 && r.push(a);
          }
        }
      }
      Ga !== null && Ga(e, t);
    };
    var Ka = Se(null);
    function qa() {
      var e = Ka.current;
      return e === null ? K.pooledCache : e;
    }
    function Ja(e, t) {
      t === null ? P(Ka, Ka.current) : P(Ka, t.pool);
    }
    function Ya() {
      var e = qa();
      return e === null ? null : { parent: Ma._currentValue, pool: e };
    }
    var Xa = Error(i(460)),
      Za = Error(i(474)),
      Qa = Error(i(542)),
      $a = { then: function () {} };
    function eo(e) {
      return ((e = e.status), e === `fulfilled` || e === `rejected`);
    }
    function to(e, t, n) {
      switch (
        ((n = e[n]), n === void 0 ? e.push(t) : n !== t && (t.then(bn, bn), (t = n)), t.status)
      ) {
        case `fulfilled`:
          return t.value;
        case `rejected`:
          throw ((e = t.reason), ao(e), e === void 0 && !(`reason` in t) ? Error(i(600)) : e);
        default:
          if (typeof t.status == `string`) t.then(bn, bn);
          else {
            if (((e = K), e !== null && 100 < e.shellSuspendCounter)) throw Error(i(482));
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
              throw ((e = t.reason), ao(e), e);
          }
          throw ((ro = t), Xa);
      }
    }
    function no(e) {
      try {
        var t = e._init;
        return t(e._payload);
      } catch (e) {
        throw typeof e == `object` && e && typeof e.then == `function` ? ((ro = e), Xa) : e;
      }
    }
    var ro = null;
    function io() {
      if (ro === null) throw Error(i(459));
      var e = ro;
      return ((ro = null), e);
    }
    function ao(e) {
      if (e === Xa || e === Qa) throw Error(i(483));
    }
    var oo = null,
      so = 0;
    function co(e) {
      var t = so;
      return ((so += 1), oo === null && (oo = []), to(oo, e, t));
    }
    function lo(e, t) {
      ((t = t.props.ref), (e.ref = t === void 0 ? null : t));
    }
    function uo(e, t) {
      throw t.$$typeof === T
        ? Error(i(525))
        : ((e = Object.prototype.toString.call(t)),
          Error(
            i(
              31,
              e === `[object Object]` ? `object with keys {` + Object.keys(t).join(`, `) + `}` : e,
            ),
          ));
    }
    function fo(e) {
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
        return ((e = Ii(e, t)), (e.index = 0), (e.sibling = null), e);
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
          ? ((t = Bi(n, e.mode, r)), (t.return = e), t)
          : ((t = a(t, n)), (t.return = e), t);
      }
      function l(e, t, n, r) {
        var i = n.type;
        return i === ae
          ? ((e = d(e, t, n.props.children, r, n.key)), lo(e, n), e)
          : t !== null &&
              (t.elementType === i ||
                (typeof i == `object` && i && i.$$typeof === j && no(i) === t.type))
            ? ((t = a(t, n.props)), lo(t, n), (t.return = e), t)
            : ((t = Ri(n.type, n.key, n.props, null, e.mode, r)), lo(t, n), (t.return = e), t);
      }
      function u(e, t, n, r) {
        return t === null ||
          t.tag !== 4 ||
          t.stateNode.containerInfo !== n.containerInfo ||
          t.stateNode.implementation !== n.implementation
          ? ((t = Hi(n, e.mode, r)), (t.return = e), t)
          : ((t = a(t, n.children || [])), (t.return = e), t);
      }
      function d(e, t, n, r, i) {
        return t === null || t.tag !== 7
          ? ((t = zi(n, e.mode, r, i)), (t.return = e), t)
          : ((t = a(t, n)), (t.return = e), t);
      }
      function f(e, t, n) {
        if ((typeof t == `string` && t !== ``) || typeof t == `number` || typeof t == `bigint`)
          return ((t = Bi(`` + t, e.mode, n)), (t.return = e), t);
        if (typeof t == `object` && t) {
          switch (t.$$typeof) {
            case re:
              return (
                (n = Ri(t.type, t.key, t.props, null, e.mode, n)), lo(n, t), (n.return = e), n
              );
            case ie:
              return ((t = Hi(t, e.mode, n)), (t.return = e), t);
            case j:
              return ((t = no(t)), f(e, t, n));
          }
          if (ve(t) || he(t)) return ((t = zi(t, e.mode, n, null)), (t.return = e), t);
          if (typeof t.then == `function`) return f(e, co(t), n);
          if (t.$$typeof === D) return f(e, Da(e, t), n);
          uo(e, t);
        }
        return null;
      }
      function p(e, t, n, r) {
        var i = t === null ? null : t.key;
        if ((typeof n == `string` && n !== ``) || typeof n == `number` || typeof n == `bigint`)
          return i === null ? c(e, t, `` + n, r) : null;
        if (typeof n == `object` && n) {
          switch (n.$$typeof) {
            case re:
              return n.key === i ? l(e, t, n, r) : null;
            case ie:
              return n.key === i ? u(e, t, n, r) : null;
            case j:
              return ((n = no(n)), p(e, t, n, r));
          }
          if (ve(n) || he(n)) return i === null ? d(e, t, n, r, null) : null;
          if (typeof n.then == `function`) return p(e, t, co(n), r);
          if (n.$$typeof === D) return p(e, t, Da(e, n), r);
          uo(e, n);
        }
        return null;
      }
      function m(e, t, n, r, i) {
        if ((typeof r == `string` && r !== ``) || typeof r == `number` || typeof r == `bigint`)
          return ((e = e.get(n) || null), c(t, e, `` + r, i));
        if (typeof r == `object` && r) {
          switch (r.$$typeof) {
            case re:
              return ((e = e.get(r.key === null ? n : r.key) || null), l(t, e, r, i));
            case ie:
              return ((e = e.get(r.key === null ? n : r.key) || null), u(t, e, r, i));
            case j:
              return ((r = no(r)), m(e, t, n, r, i));
          }
          if (ve(r) || he(r)) return ((e = e.get(n) || null), d(t, e, r, i, null));
          if (typeof r.then == `function`) return m(e, t, n, co(r), i);
          if (r.$$typeof === D) return m(e, t, n, Da(t, r), i);
          uo(t, r);
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
        if (h === s.length) return (n(i, d), z && ea(i, h), l);
        if (d === null) {
          for (; h < s.length; h++)
            ((d = f(i, s[h], c)),
              d !== null && ((a = o(d, a, h)), u === null ? (l = d) : (u.sibling = d), (u = d)));
          return (z && ea(i, h), l);
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
          z && ea(i, h),
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
        if (v.done) return (n(a, h), z && ea(a, g), u);
        if (h === null) {
          for (; !v.done; g++, v = c.next())
            ((v = f(a, v.value, l)),
              v !== null && ((s = o(v, s, g)), d === null ? (u = v) : (d.sibling = v), (d = v)));
          return (z && ea(a, g), u);
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
          z && ea(a, g),
          u
        );
      }
      function _(e, r, o, c) {
        if (
          (typeof o == `object` &&
            o &&
            o.type === ae &&
            o.key === null &&
            o.props.ref === void 0 &&
            (o = o.props.children),
          typeof o == `object` && o)
        ) {
          switch (o.$$typeof) {
            case re:
              a: {
                for (var l = o.key; r !== null;) {
                  if (r.key === l) {
                    if (((l = o.type), l === ae)) {
                      if (r.tag === 7) {
                        (n(e, r.sibling),
                          (c = a(r, o.props.children)),
                          lo(c, o),
                          (c.return = e),
                          (e = c));
                        break a;
                      }
                    } else if (
                      r.elementType === l ||
                      (typeof l == `object` && l && l.$$typeof === j && no(l) === r.type)
                    ) {
                      (n(e, r.sibling), (c = a(r, o.props)), lo(c, o), (c.return = e), (e = c));
                      break a;
                    }
                    n(e, r);
                    break;
                  }
                  (t(e, r), (r = r.sibling));
                }
                o.type === ae
                  ? ((c = zi(o.props.children, e.mode, c, o.key)),
                    lo(c, o),
                    (c.return = e),
                    (e = c))
                  : ((c = Ri(o.type, o.key, o.props, null, e.mode, c)),
                    lo(c, o),
                    (c.return = e),
                    (e = c));
              }
              return s(e);
            case ie:
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
                ((c = Hi(o, e.mode, c)), (c.return = e), (e = c));
              }
              return s(e);
            case j:
              return ((o = no(o)), _(e, r, o, c));
          }
          if (ve(o)) return h(e, r, o, c);
          if (he(o)) {
            if (((l = he(o)), typeof l != `function`)) throw Error(i(150));
            return ((o = l.call(o)), g(e, r, o, c));
          }
          if (typeof o.then == `function`) return _(e, r, co(o), c);
          if (o.$$typeof === D) return _(e, r, Da(e, o), c);
          uo(e, o);
        }
        return (typeof o == `string` && o !== ``) || typeof o == `number` || typeof o == `bigint`
          ? ((o = `` + o),
            r !== null && r.tag === 6
              ? (n(e, r.sibling), (c = a(r, o)), (c.return = e), (e = c))
              : (n(e, r), (c = Bi(o, e.mode, c)), (c.return = e), (e = c)),
            s(e))
          : n(e, r);
      }
      return function (e, t, n, r) {
        try {
          so = 0;
          var i = _(e, t, n, r);
          return ((oo = null), i);
        } catch (t) {
          if (t === Xa || t === Qa) throw t;
          var a = Pi(29, t, null, e.mode);
          return ((a.lanes = r), (a.return = e), a);
        }
      };
    }
    var po = fo(!0),
      mo = fo(!1),
      ho = !1;
    function go(e) {
      e.updateQueue = {
        baseState: e.memoizedState,
        firstBaseUpdate: null,
        lastBaseUpdate: null,
        shared: { pending: null, lanes: 0, hiddenCallbacks: null },
        callbacks: null,
      };
    }
    function _o(e, t) {
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
    function vo(e) {
      return { lane: e, tag: 0, payload: null, callback: null, next: null };
    }
    function yo(e, t, n) {
      var r = e.updateQueue;
      if (r === null) return null;
      if (((r = r.shared), G & 2)) {
        var i = r.pending;
        return (
          i === null ? (t.next = t) : ((t.next = i.next), (i.next = t)),
          (r.pending = t),
          (t = ji(e)),
          Ai(e, null, n),
          t
        );
      }
      return (Di(e, r, t, n), ji(e));
    }
    function bo(e, t, n) {
      if (((t = t.updateQueue), t !== null && ((t = t.shared), n & 4194048))) {
        var r = t.lanes;
        ((r &= e.pendingLanes), (n |= r), (t.lanes = n), vt(e, n));
      }
    }
    function xo(e, t) {
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
    var So = !1;
    function Co() {
      if (So) {
        var e = Va;
        if (e !== null) throw e;
      }
    }
    function wo(e, t, n, r) {
      So = !1;
      var i = e.updateQueue;
      ho = !1;
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
          if (p ? (J & f) === f : (r & f) === f) {
            (f !== 0 && f === Ba && (So = !0),
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
                  d = w({}, d, f);
                  break a;
                case 2:
                  ho = !0;
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
          (sd |= o),
          (e.lanes = o),
          (e.memoizedState = d));
      }
    }
    function To(e, t) {
      if (typeof e != `function`) throw Error(i(191, e));
      e.call(t);
    }
    function Eo(e, t) {
      var n = e.callbacks;
      if (n !== null) for (e.callbacks = null, e = 0; e < n.length; e++) To(n[e], t);
    }
    var Do = Se(null),
      Oo = Se(0);
    function ko(e, t) {
      ((e = ad), P(Oo, e), P(Do, t), (ad = e | t.baseLanes));
    }
    function Ao() {
      (P(Oo, ad), P(Do, Do.current));
    }
    function jo() {
      ((ad = Oo.current), Ce(Do), Ce(Oo));
    }
    var Mo = Se(null),
      No = null;
    function Po(e) {
      var t = e.alternate;
      (P(zo, zo.current & 1),
        P(Mo, e),
        No === null && (t === null || Do.current !== null || t.memoizedState !== null) && (No = e));
    }
    function Fo(e) {
      (P(zo, zo.current), P(Mo, e), No === null && (No = e));
    }
    function Io(e) {
      e.tag === 22 ? (P(zo, zo.current), P(Mo, e), No === null && (No = e)) : Lo();
    }
    function Lo() {
      (P(zo, zo.current), P(Mo, Mo.current));
    }
    function Ro(e) {
      (Ce(Mo), No === e && (No = null), Ce(zo));
    }
    var zo = Se(0);
    function Bo(e, t) {
      (P(Mo, Mo.current), P(zo, t));
    }
    function Vo(e) {
      (Ce(zo), Ce(Mo), No === e && (No = null));
    }
    function Ho(e) {
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
    var Uo = 0,
      B = null,
      V = null,
      Wo = null,
      Go = !1,
      Ko = !1,
      qo = !1,
      Jo = 0,
      Yo = 0,
      Xo = null,
      Zo = 0;
    function Qo() {
      throw Error(i(321));
    }
    function $o(e, t) {
      if (t === null) return !1;
      for (var n = 0; n < t.length && n < e.length; n++) if (!Ur(e[n], t[n])) return !1;
      return !0;
    }
    function es(e, t, n, r, i, a) {
      return (
        (Uo = a),
        (B = t),
        (t.memoizedState = null),
        (t.updateQueue = null),
        (t.lanes = 0),
        (M.H = e === null || e.memoizedState === null ? _c : vc),
        (qo = !1),
        (a = n(r, i)),
        (qo = !1),
        Ko && (a = ns(t, n, r, i)),
        ts(e),
        a
      );
    }
    function ts(e) {
      M.H = gc;
      var t = V !== null && V.next !== null;
      if (((Uo = 0), (Wo = V = B = null), (Go = !1), (Yo = 0), (Xo = null), t)) throw Error(i(300));
      e === null || Fc || ((e = e.dependencies), e !== null && wa(e) && (Fc = !0));
    }
    function ns(e, t, n, r) {
      B = e;
      var a = 0;
      do {
        if ((Ko && (Xo = null), (Yo = 0), (Ko = !1), 25 <= a)) throw Error(i(301));
        if (((a += 1), (Wo = V = null), e.updateQueue != null)) {
          var o = e.updateQueue;
          ((o.lastEffect = null),
            (o.events = null),
            (o.stores = null),
            o.memoCache != null && (o.memoCache.index = 0));
        }
        ((M.H = yc), (o = t(n, r)));
      } while (Ko);
      return o;
    }
    function rs() {
      var e = M.H,
        t = e.useState()[0];
      return (
        (t = typeof t.then == `function` ? us(t) : t),
        (e = e.useState()[0]),
        (V === null ? null : V.memoizedState) !== e && (B.flags |= 1024),
        t
      );
    }
    function is() {
      var e = Jo !== 0;
      return ((Jo = 0), e);
    }
    function as(e, t, n) {
      ((t.updateQueue = e.updateQueue), (t.flags &= -2053), (e.lanes &= ~n));
    }
    function os(e) {
      if (Go) {
        for (e = e.memoizedState; e !== null;) {
          var t = e.queue;
          (t !== null && (t.pending = null), (e = e.next));
        }
        Go = !1;
      }
      ((Uo = 0), (Wo = V = B = null), (Ko = !1), (Yo = Jo = 0), (Xo = null));
    }
    function ss() {
      var e = { memoizedState: null, baseState: null, baseQueue: null, queue: null, next: null };
      return (Wo === null ? (B.memoizedState = Wo = e) : (Wo = Wo.next = e), Wo);
    }
    function cs() {
      if (V === null) {
        var e = B.alternate;
        e = e === null ? null : e.memoizedState;
      } else e = V.next;
      var t = Wo === null ? B.memoizedState : Wo.next;
      if (t !== null) ((Wo = t), (V = e));
      else {
        if (e === null) throw B.alternate === null ? Error(i(467)) : Error(i(310));
        ((V = e),
          (e = {
            memoizedState: V.memoizedState,
            baseState: V.baseState,
            baseQueue: V.baseQueue,
            queue: V.queue,
            next: null,
          }),
          Wo === null ? (B.memoizedState = Wo = e) : (Wo = Wo.next = e));
      }
      return Wo;
    }
    function ls() {
      return { lastEffect: null, events: null, stores: null, memoCache: null };
    }
    function us(e) {
      var t = Yo;
      return (
        (Yo += 1),
        Xo === null && (Xo = []),
        (e = to(Xo, e, t)),
        (t = B),
        (Wo === null ? t.memoizedState : Wo.next) === null &&
          ((t = t.alternate), (M.H = t === null || t.memoizedState === null ? _c : vc)),
        e
      );
    }
    function ds(e) {
      if (typeof e == `object` && e) {
        if (typeof e.then == `function`) return us(e);
        if (e.$$typeof === pe) return;
        if (e.$$typeof === D) return Ea(e);
      }
      throw Error(i(438, String(e)));
    }
    function fs(e) {
      var t = null,
        n = B.updateQueue;
      if ((n !== null && (t = n.memoCache), t == null)) {
        var r = B.alternate;
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
        n === null && ((n = ls()), (B.updateQueue = n)),
        (n.memoCache = t),
        (n = t.data[t.index]),
        n === void 0)
      )
        for (n = t.data[t.index] = Array(e), r = 0; r < e; r++) n[r] = de;
      return (t.index++, n);
    }
    function ps(e, t) {
      return typeof t == `function` ? t(e) : t;
    }
    function ms(e) {
      return hs(cs(), V, e);
    }
    function hs(e, t, n) {
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
          if (f === u.lane ? (Uo & f) === f : (J & f) === f) {
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
                f === Ba && (d = !0));
            else if ((Uo & p) === p) {
              ((u = u.next), p === Ba && (d = !0));
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
                (B.lanes |= p),
                (sd |= p));
            ((f = u.action), qo && n(o, f), (o = u.hasEagerState ? u.eagerState : n(o, f)));
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
              (B.lanes |= f),
              (sd |= f));
          u = u.next;
        } while (u !== null && u !== t);
        if (
          (l === null ? (s = o) : (l.next = c),
          !Ur(o, e.memoizedState) && ((Fc = !0), d && ((n = Va), n !== null)))
        )
          throw n;
        ((e.memoizedState = o), (e.baseState = s), (e.baseQueue = l), (r.lastRenderedState = o));
      }
      return (a === null && (r.lanes = 0), [e.memoizedState, r.dispatch]);
    }
    function gs(e) {
      var t = cs(),
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
        (Ur(o, t.memoizedState) || (Fc = !0),
          (t.memoizedState = o),
          t.baseQueue === null && (t.baseState = o),
          (n.lastRenderedState = o));
      }
      return [o, r];
    }
    function _s(e, t, n) {
      var r = B,
        a = cs(),
        o = z;
      if (o) {
        if (n === void 0) throw Error(i(407));
        n = n();
      } else n = t();
      var s = !Ur((V || a).memoizedState, n);
      if (
        (s && ((a.memoizedState = n), (Fc = !0)),
        (a = a.queue),
        Hs(bs.bind(null, r, a, e), [e]),
        (e = a.getSnapshot !== t || s || (Wo !== null && !!(Wo.memoizedState.tag & 1))),
        Ls(e ? 9 : 8, { destroy: void 0 }, ys.bind(null, r, a, n, t), null),
        e)
      ) {
        if (((r.flags |= 2048), K === null)) throw Error(i(349));
        o || Uo & 127 || vs(r, t, n);
      }
      return n;
    }
    function vs(e, t, n) {
      ((e.flags |= 16384),
        (e = { getSnapshot: t, value: n }),
        (t = B.updateQueue),
        t === null
          ? ((t = ls()), (B.updateQueue = t), (t.stores = [e]))
          : ((n = t.stores), n === null ? (t.stores = [e]) : n.push(e)));
    }
    function ys(e, t, n, r) {
      ((t.value = n), (t.getSnapshot = r), xs(t) && Ss(e));
    }
    function bs(e, t, n) {
      return n(function () {
        xs(t) && Ss(e);
      });
    }
    function xs(e) {
      var t = e.getSnapshot;
      e = e.value;
      try {
        var n = t();
        return !Ur(e, n);
      } catch {
        return !0;
      }
    }
    function Ss(e) {
      var t = ki(e, 2);
      t !== null && Pd(t, e, 2);
    }
    function Cs(e) {
      var t = ss();
      if (typeof e == `function`) {
        var n = e;
        if (((e = n()), qo)) {
          tt(!0);
          try {
            n();
          } finally {
            tt(!1);
          }
        }
      }
      return (
        (t.memoizedState = t.baseState = e),
        (t.queue = {
          pending: null,
          lanes: 0,
          dispatch: null,
          lastRenderedReducer: ps,
          lastRenderedState: e,
        }),
        t
      );
    }
    function ws(e, t, n, r) {
      return ((e.baseState = n), hs(e, V, typeof r == `function` ? r : ps));
    }
    function Ts(e, t, n, r, a) {
      if (pc(e)) throw Error(i(485));
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
        (M.T === null ? (o.isTransition = !1) : n(!0),
          r(o),
          (n = t.pending),
          n === null
            ? ((o.next = t.pending = o), Es(t, o))
            : ((o.next = n.next), (t.pending = n.next = o)));
      }
    }
    function Es(e, t) {
      var n = t.action,
        r = t.payload,
        i = e.state;
      if (t.isTransition) {
        var a = M.T,
          o = {};
        ((o.types = a === null ? null : a.types), (M.T = o));
        try {
          var s = n(i, r),
            c = M.S;
          (c !== null && c(o, s), Ds(e, t, s));
        } catch (n) {
          ks(e, t, n);
        } finally {
          (a !== null && o.types !== null && (a.types = o.types), (M.T = a));
        }
      } else
        try {
          ((a = n(i, r)), Ds(e, t, a));
        } catch (n) {
          ks(e, t, n);
        }
    }
    function Ds(e, t, n) {
      typeof n == `object` && n && typeof n.then == `function`
        ? n.then(
            function (n) {
              Os(e, t, n);
            },
            function (n) {
              return ks(e, t, n);
            },
          )
        : Os(e, t, n);
    }
    function Os(e, t, n) {
      ((t.status = `fulfilled`),
        (t.value = n),
        As(t),
        (e.state = n),
        (t = e.pending),
        t !== null &&
          ((n = t.next), n === t ? (e.pending = null) : ((n = n.next), (t.next = n), Es(e, n))));
    }
    function ks(e, t, n) {
      var r = e.pending;
      if (((e.pending = null), r !== null)) {
        r = r.next;
        do ((t.status = `rejected`), (t.reason = n), As(t), (t = t.next));
        while (t !== r);
      }
      e.action = null;
    }
    function As(e) {
      e = e.listeners;
      for (var t = 0; t < e.length; t++) (0, e[t])();
    }
    function js(e, t) {
      return t;
    }
    function Ms(e, t) {
      if (z) {
        var n = K.formState;
        if (n !== null) {
          a: {
            var r = B;
            if (z) {
              if (R) {
                b: {
                  for (var i = R, a = sa; i.nodeType !== 8;)
                    if (!a || ((i = lm(i.nextSibling)), i === null)) {
                      i = null;
                      break b;
                    }
                  ((a = i.data), (i = a === `F!` || a === `F` ? i : null));
                }
                if (i) {
                  ((R = lm(i.nextSibling)), (r = i.data === `F!`));
                  break a;
                }
              }
              la(r);
            }
            r = !1;
          }
          r && (t = n[0]);
        }
      }
      return (
        (n = ss()),
        (n.memoizedState = n.baseState = t),
        (r = {
          pending: null,
          lanes: 0,
          dispatch: null,
          lastRenderedReducer: js,
          lastRenderedState: t,
        }),
        (n.queue = r),
        (n = uc.bind(null, B, r)),
        (r.dispatch = n),
        (r = Cs(!1)),
        (a = fc.bind(null, B, !1, r.queue)),
        (r = ss()),
        (i = { state: t, dispatch: null, action: e, pending: null }),
        (r.queue = i),
        (n = Ts.bind(null, B, i, a, n)),
        (i.dispatch = n),
        (r.memoizedState = e),
        [t, n, !1]
      );
    }
    function Ns(e) {
      return Ps(cs(), V, e);
    }
    function Ps(e, t, n) {
      if (
        ((t = hs(e, t, js)[0]),
        (e = ms(ps)[0]),
        typeof t == `object` && t && typeof t.then == `function`)
      )
        try {
          var r = us(t);
        } catch (e) {
          throw e === Xa ? Qa : e;
        }
      else r = t;
      t = cs();
      var i = t.queue,
        a = i.dispatch;
      return (
        n !== t.memoizedState &&
          ((B.flags |= 2048), Ls(9, { destroy: void 0 }, Fs.bind(null, i, n), null)),
        [r, a, e]
      );
    }
    function Fs(e, t) {
      e.action = t;
    }
    function Is(e) {
      var t = cs(),
        n = V;
      if (n !== null) return Ps(t, n, e);
      (cs(), (t = t.memoizedState), (n = cs()));
      var r = n.queue.dispatch;
      return ((n.memoizedState = e), [t, r, !1]);
    }
    function Ls(e, t, n, r) {
      return (
        (e = { tag: e, create: n, deps: r, inst: t, next: null }),
        (t = B.updateQueue),
        t === null && ((t = ls()), (B.updateQueue = t)),
        (n = t.lastEffect),
        n === null
          ? (t.lastEffect = e.next = e)
          : ((r = n.next), (n.next = e), (e.next = r), (t.lastEffect = e)),
        e
      );
    }
    function Rs() {
      return cs().memoizedState;
    }
    function zs(e, t, n, r) {
      var i = ss();
      ((B.flags |= e),
        (i.memoizedState = Ls(1 | t, { destroy: void 0 }, n, r === void 0 ? null : r)));
    }
    function Bs(e, t, n, r) {
      var i = cs();
      r = r === void 0 ? null : r;
      var a = i.memoizedState.inst;
      V !== null && r !== null && $o(r, V.memoizedState.deps)
        ? (i.memoizedState = Ls(t, a, n, r))
        : ((B.flags |= e), (i.memoizedState = Ls(1 | t, a, n, r)));
    }
    function Vs(e, t) {
      zs(8390656, 8, e, t);
    }
    function Hs(e, t) {
      Bs(2048, 8, e, t);
    }
    function Us(e) {
      B.flags |= 4;
      var t = B.updateQueue;
      if (t === null) ((t = ls()), (B.updateQueue = t), (t.events = [e]));
      else {
        var n = t.events;
        n === null ? (t.events = [e]) : n.push(e);
      }
    }
    function Ws(e) {
      var t = cs().memoizedState;
      return (
        Us({ ref: t, nextImpl: e }),
        function () {
          if (G & 2) throw Error(i(440));
          return t.impl.apply(void 0, arguments);
        }
      );
    }
    function Gs(e, t) {
      return Bs(4, 2, e, t);
    }
    function Ks(e, t) {
      return Bs(4, 4, e, t);
    }
    function qs(e, t) {
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
    function Js(e, t, n) {
      ((n = n == null ? null : n.concat([e])), Bs(4, 4, qs.bind(null, t, e), n));
    }
    function Ys() {}
    function Xs(e, t) {
      var n = cs();
      t = t === void 0 ? null : t;
      var r = n.memoizedState;
      return t !== null && $o(t, r[1]) ? r[0] : ((n.memoizedState = [e, t]), e);
    }
    function Zs(e, t) {
      var n = cs();
      t = t === void 0 ? null : t;
      var r = n.memoizedState;
      if (t !== null && $o(t, r[1])) return r[0];
      if (((r = e()), qo)) {
        tt(!0);
        try {
          e();
        } finally {
          tt(!1);
        }
      }
      return ((n.memoizedState = [r, t]), r);
    }
    function Qs(e, t, n) {
      return n === void 0 || (Uo & 1073741824 && !(J & 261930))
        ? (e.memoizedState = t)
        : ((e.memoizedState = n), (e = Md()), (B.lanes |= e), (sd |= e), n);
    }
    function $s(e, t, n, r) {
      return Ur(n, t)
        ? n
        : Do.current === null
          ? !(Uo & 106) || (Uo & 1073741824 && !(J & 261930))
            ? ((Fc = !0), (e.memoizedState = n))
            : ((e = Md()), (B.lanes |= e), (sd |= e), t)
          : ((e = Qs(e, n, r)), Ur(e, t) || (Fc = !0), e);
    }
    function ec(e, t, n, r, i) {
      var a = N.p;
      N.p = a !== 0 && 8 > a ? a : 8;
      var o = M.T,
        s = {};
      ((s.types = o === null ? null : o.types), (M.T = s), fc(e, !1, t, n));
      try {
        var c = i(),
          l = M.S;
        (l !== null && l(s, c),
          typeof c == `object` && c && typeof c.then == `function`
            ? dc(e, t, Wa(c, r), jd(e))
            : dc(e, t, r, jd(e)));
      } catch (n) {
        dc(e, t, { then: function () {}, status: `rejected`, reason: n }, jd());
      } finally {
        ((N.p = a), o !== null && s.types !== null && (o.types = s.types), (M.T = o));
      }
    }
    function tc() {}
    function nc(e, t, n, r) {
      if (e.tag !== 5) throw Error(i(476));
      var a = rc(e).queue;
      ec(
        e,
        a,
        t,
        ye,
        n === null
          ? tc
          : function () {
              return (ic(e), n(r));
            },
      );
    }
    function rc(e) {
      var t = e.memoizedState;
      if (t !== null) return t;
      t = {
        memoizedState: ye,
        baseState: ye,
        baseQueue: null,
        queue: {
          pending: null,
          lanes: 0,
          dispatch: null,
          lastRenderedReducer: ps,
          lastRenderedState: ye,
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
            lastRenderedReducer: ps,
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
    function ic(e) {
      var t = rc(e);
      (t.next === null && (t = e.alternate.memoizedState), dc(e, t.next.queue, {}, jd()));
    }
    function ac() {
      return Ea(sh);
    }
    function oc() {
      return cs().memoizedState;
    }
    function sc() {
      return cs().memoizedState;
    }
    function cc(e) {
      for (var t = e.return; t !== null;) {
        switch (t.tag) {
          case 24:
          case 3:
            var n = jd();
            e = vo(n);
            var r = yo(t, e, n);
            (r !== null && (Pd(r, t, n), bo(r, t, n)), (t = { cache: Na() }), (e.payload = t));
            return;
        }
        t = t.return;
      }
    }
    function lc(e, t, n) {
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
        pc(e) ? mc(t, n) : ((n = Oi(e, t, n, r)), n !== null && (Pd(n, e, r), hc(n, t, r))));
    }
    function uc(e, t, n) {
      dc(e, t, n, jd());
    }
    function dc(e, t, n, r) {
      var i = {
        lane: r,
        revertLane: 0,
        gesture: null,
        action: n,
        hasEagerState: !1,
        eagerState: null,
        next: null,
      };
      if (pc(e)) mc(t, i);
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
            if (((i.hasEagerState = !0), (i.eagerState = s), Ur(s, o)))
              return (Di(e, t, i, 0), K === null && Ei(), !1);
          } catch {}
        if (((n = Oi(e, t, i, r)), n !== null)) return (Pd(n, e, r), hc(n, t, r), !0);
      }
      return !1;
    }
    function fc(e, t, n, r) {
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
        pc(e))
      ) {
        if (t) throw Error(i(479));
      } else ((t = Oi(e, n, r, 2)), t !== null && Pd(t, e, 2));
    }
    function pc(e) {
      var t = e.alternate;
      return e === B || (t !== null && t === B);
    }
    function mc(e, t) {
      Ko = Go = !0;
      var n = e.pending;
      (n === null ? (t.next = t) : ((t.next = n.next), (n.next = t)), (e.pending = t));
    }
    function hc(e, t, n) {
      if (n & 4194048) {
        var r = t.lanes;
        ((r &= e.pendingLanes), (n |= r), (t.lanes = n), vt(e, n));
      }
    }
    var gc = {
        readContext: Ea,
        use: ds,
        useCallback: Qo,
        useContext: Qo,
        useEffect: Qo,
        useImperativeHandle: Qo,
        useLayoutEffect: Qo,
        useInsertionEffect: Qo,
        useMemo: Qo,
        useReducer: Qo,
        useRef: Qo,
        useState: Qo,
        useDebugValue: Qo,
        useDeferredValue: Qo,
        useTransition: Qo,
        useSyncExternalStore: Qo,
        useId: Qo,
        useHostTransitionStatus: Qo,
        useFormState: Qo,
        useActionState: Qo,
        useOptimistic: Qo,
        useMemoCache: Qo,
        useCacheRefresh: Qo,
        useEffectEvent: Qo,
      },
      _c = {
        readContext: Ea,
        use: ds,
        useCallback: function (e, t) {
          return ((ss().memoizedState = [e, t === void 0 ? null : t]), e);
        },
        useContext: Ea,
        useEffect: Vs,
        useImperativeHandle: function (e, t, n) {
          ((n = n == null ? null : n.concat([e])), zs(4194308, 4, qs.bind(null, t, e), n));
        },
        useLayoutEffect: function (e, t) {
          return zs(4194308, 4, e, t);
        },
        useInsertionEffect: function (e, t) {
          zs(4, 2, e, t);
        },
        useMemo: function (e, t) {
          var n = ss();
          t = t === void 0 ? null : t;
          var r = e();
          if (qo) {
            tt(!0);
            try {
              e();
            } finally {
              tt(!1);
            }
          }
          return ((n.memoizedState = [r, t]), r);
        },
        useReducer: function (e, t, n) {
          var r = ss();
          if (n !== void 0) {
            var i = n(t);
            if (qo) {
              tt(!0);
              try {
                n(t);
              } finally {
                tt(!1);
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
            (e = e.dispatch = lc.bind(null, B, e)),
            [r.memoizedState, e]
          );
        },
        useRef: function (e) {
          var t = ss();
          return ((e = { current: e }), (t.memoizedState = e));
        },
        useState: function (e) {
          e = Cs(e);
          var t = e.queue,
            n = uc.bind(null, B, t);
          return ((t.dispatch = n), [e.memoizedState, n]);
        },
        useDebugValue: Ys,
        useDeferredValue: function (e, t) {
          return Qs(ss(), e, t);
        },
        useTransition: function () {
          var e = Cs(!1);
          return ((e = ec.bind(null, B, e.queue, !0, !1)), (ss().memoizedState = e), [!1, e]);
        },
        useSyncExternalStore: function (e, t, n) {
          var r = B,
            a = ss();
          if (z) {
            if (n === void 0) throw Error(i(407));
            n = n();
          } else {
            if (((n = t()), K === null)) throw Error(i(349));
            J & 127 || vs(r, t, n);
          }
          a.memoizedState = n;
          var o = { value: n, getSnapshot: t };
          return (
            (a.queue = o),
            Vs(bs.bind(null, r, o, e), [e]),
            (r.flags |= 2048),
            Ls(9, { destroy: void 0 }, ys.bind(null, r, o, n, t), null),
            n
          );
        },
        useId: function () {
          var e = ss(),
            t = K.identifierPrefix;
          if (z) {
            var n = $i,
              r = Qi;
            ((n = (r & ~(1 << (32 - nt(r) - 1))).toString(32) + n),
              (t = `_` + t + `R_` + n),
              (n = Jo++),
              0 < n && (t += `H` + n.toString(32)),
              (t += `_`));
          } else ((n = Zo++), (t = `_` + t + `r_` + n.toString(32) + `_`));
          return (e.memoizedState = t);
        },
        useHostTransitionStatus: ac,
        useFormState: Ms,
        useActionState: Ms,
        useOptimistic: function (e) {
          var t = ss();
          t.memoizedState = t.baseState = e;
          var n = {
            pending: null,
            lanes: 0,
            dispatch: null,
            lastRenderedReducer: null,
            lastRenderedState: null,
          };
          return ((t.queue = n), (t = fc.bind(null, B, !0, n)), (n.dispatch = t), [e, t]);
        },
        useMemoCache: fs,
        useCacheRefresh: function () {
          return (ss().memoizedState = cc.bind(null, B));
        },
        useEffectEvent: function (e) {
          var t = ss(),
            n = { impl: e };
          return (
            (t.memoizedState = n),
            function () {
              if (G & 2) throw Error(i(440));
              return n.impl.apply(void 0, arguments);
            }
          );
        },
      },
      vc = {
        readContext: Ea,
        use: ds,
        useCallback: Xs,
        useContext: Ea,
        useEffect: Hs,
        useImperativeHandle: Js,
        useInsertionEffect: Gs,
        useLayoutEffect: Ks,
        useMemo: Zs,
        useReducer: ms,
        useRef: Rs,
        useState: function () {
          return ms(ps);
        },
        useDebugValue: Ys,
        useDeferredValue: function (e, t) {
          return $s(cs(), V.memoizedState, e, t);
        },
        useTransition: function () {
          var e = ms(ps)[0],
            t = cs().memoizedState;
          return [typeof e == `boolean` ? e : us(e), t];
        },
        useSyncExternalStore: _s,
        useId: oc,
        useHostTransitionStatus: ac,
        useFormState: Ns,
        useActionState: Ns,
        useOptimistic: function (e, t) {
          return ws(cs(), V, e, t);
        },
        useMemoCache: fs,
        useCacheRefresh: sc,
        useEffectEvent: Ws,
      },
      yc = {
        readContext: Ea,
        use: ds,
        useCallback: Xs,
        useContext: Ea,
        useEffect: Hs,
        useImperativeHandle: Js,
        useInsertionEffect: Gs,
        useLayoutEffect: Ks,
        useMemo: Zs,
        useReducer: gs,
        useRef: Rs,
        useState: function () {
          return gs(ps);
        },
        useDebugValue: Ys,
        useDeferredValue: function (e, t) {
          var n = cs();
          return V === null ? Qs(n, e, t) : $s(n, V.memoizedState, e, t);
        },
        useTransition: function () {
          var e = gs(ps)[0],
            t = cs().memoizedState;
          return [typeof e == `boolean` ? e : us(e), t];
        },
        useSyncExternalStore: _s,
        useId: oc,
        useHostTransitionStatus: ac,
        useFormState: Is,
        useActionState: Is,
        useOptimistic: function (e, t) {
          var n = cs();
          return V === null ? ((n.baseState = e), [e, n.queue.dispatch]) : ws(n, V, e, t);
        },
        useMemoCache: fs,
        useCacheRefresh: sc,
        useEffectEvent: Ws,
      };
    function bc(e, t, n, r) {
      ((t = e.memoizedState),
        (n = n(r, t)),
        (n = n == null ? t : w({}, t, n)),
        (e.memoizedState = n),
        e.lanes === 0 && (e.updateQueue.baseState = n));
    }
    var xc = {
      enqueueSetState: function (e, t, n) {
        e = e._reactInternals;
        var r = jd(),
          i = vo(r);
        ((i.payload = t),
          n != null && (i.callback = n),
          (t = yo(e, i, r)),
          t !== null && (Pd(t, e, r), bo(t, e, r)));
      },
      enqueueReplaceState: function (e, t, n) {
        e = e._reactInternals;
        var r = jd(),
          i = vo(r);
        ((i.tag = 1),
          (i.payload = t),
          n != null && (i.callback = n),
          (t = yo(e, i, r)),
          t !== null && (Pd(t, e, r), bo(t, e, r)));
      },
      enqueueForceUpdate: function (e, t) {
        e = e._reactInternals;
        var n = jd(),
          r = vo(n);
        ((r.tag = 2),
          t != null && (r.callback = t),
          (t = yo(e, r, n)),
          t !== null && (Pd(t, e, n), bo(t, e, n)));
      },
    };
    function Sc(e, t, n, r, i, a, o) {
      return (
        (e = e.stateNode),
        typeof e.shouldComponentUpdate == `function`
          ? e.shouldComponentUpdate(r, a, o)
          : t.prototype && t.prototype.isPureReactComponent
            ? !Wr(n, r) || !Wr(i, a)
            : !0
      );
    }
    function Cc(e, t, n, r) {
      ((e = t.state),
        typeof t.componentWillReceiveProps == `function` && t.componentWillReceiveProps(n, r),
        typeof t.UNSAFE_componentWillReceiveProps == `function` &&
          t.UNSAFE_componentWillReceiveProps(n, r),
        t.state !== e && xc.enqueueReplaceState(t, t.state, null));
    }
    function wc(e, t) {
      var n = t;
      if (`ref` in t) for (var r in ((n = {}), t)) r !== `ref` && (n[r] = t[r]);
      if ((e = e.defaultProps))
        for (var i in (n === t && (n = w({}, n)), e)) n[i] === void 0 && (n[i] = e[i]);
      return n;
    }
    function Tc(e) {
      Si(e);
    }
    function Ec(e) {
      console.error(e);
    }
    function Dc(e) {
      Si(e);
    }
    function Oc(e, t) {
      try {
        var n = e.onUncaughtError;
        n(t.value, { componentStack: t.stack });
      } catch (e) {
        setTimeout(function () {
          throw e;
        });
      }
    }
    function kc(e, t, n) {
      try {
        var r = e.onCaughtError;
        r(n.value, { componentStack: n.stack, errorBoundary: t.tag === 1 ? t.stateNode : null });
      } catch (e) {
        setTimeout(function () {
          throw e;
        });
      }
    }
    function Ac(e, t, n) {
      return (
        (n = vo(n)),
        (n.tag = 3),
        (n.payload = { element: null }),
        (n.callback = function () {
          Oc(e, t);
        }),
        n
      );
    }
    function jc(e) {
      return ((e = vo(e)), (e.tag = 3), e);
    }
    function Mc(e, t, n, r) {
      var i = n.type.getDerivedStateFromError;
      if (typeof i == `function`) {
        var a = r.value;
        ((e.payload = function () {
          return i(a);
        }),
          (e.callback = function () {
            kc(t, n, r);
          }));
      }
      var o = n.stateNode;
      o !== null &&
        typeof o.componentDidCatch == `function` &&
        (e.callback = function () {
          (kc(t, n, r),
            typeof i != `function` && (yd === null ? (yd = new Set([this])) : yd.add(this)));
          var e = r.stack;
          this.componentDidCatch(r.value, { componentStack: e === null ? `` : e });
        });
    }
    function Nc(e, t, n, r, a) {
      if (((n.flags |= 32768), typeof r == `object` && r && typeof r.then == `function`)) {
        if (((t = n.alternate), t !== null && Ca(t, n, a, !0), (n = Mo.current), n !== null)) {
          switch (n.tag) {
            case 31:
            case 13:
            case 19:
              return (
                No === null ? Kd() : n.alternate === null && od === 0 && (od = 3),
                (n.flags &= -257),
                (n.flags |= 65536),
                (n.lanes = a),
                r === $a
                  ? (n.flags |= 16384)
                  : ((t = n.updateQueue),
                    t === null ? (n.updateQueue = new Set([r])) : t.add(r),
                    mf(e, r, a)),
                !1
              );
            case 22:
              return (
                (n.flags |= 65536),
                r === $a
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
      if (z)
        return (
          (t = Mo.current),
          t === null
            ? (r !== ca && ((t = Error(i(423), { cause: r })), ha(Wi(t, n))),
              (e = e.current.alternate),
              (e.flags |= 65536),
              (a &= -a),
              (e.lanes |= a),
              (r = Wi(r, n)),
              (a = Ac(e.stateNode, r, a)),
              xo(e, a),
              od !== 4 && (od = 2))
            : (!(t.flags & 65536) && (t.flags |= 256),
              (t.flags |= 65536),
              (t.lanes = a),
              r !== ca && ((e = Error(i(422), { cause: r })), ha(Wi(e, n)))),
          !1
        );
      var o = Error(i(520), { cause: r });
      if (((o = Wi(o, n)), fd === null ? (fd = [o]) : fd.push(o), od !== 4 && (od = 2), t === null))
        return !0;
      ((r = Wi(r, n)), (n = t));
      do {
        switch (n.tag) {
          case 3:
            return (
              (n.flags |= 65536),
              (e = a & -a),
              (n.lanes |= e),
              (e = Ac(n.stateNode, r, e)),
              xo(n, e),
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
                    (yd === null || !yd.has(o)))))
            )
              return (
                (n.flags |= 65536),
                (a &= -a),
                (n.lanes |= a),
                (a = jc(a)),
                Mc(a, e, n, r),
                xo(n, a),
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
    var Pc = Error(i(461)),
      Fc = !1;
    function Ic(e, t, n, r) {
      t.child = e === null ? mo(t, null, n, r) : po(t, e.child, n, r);
    }
    function Lc(e, t, n, r, i) {
      n = n.render;
      var a = t.ref;
      if (`ref` in r) {
        var o = {};
        for (var s in r) s !== `ref` && (o[s] = r[s]);
      } else o = r;
      return (
        Ta(t),
        (r = es(e, t, n, o, a, i)),
        (s = is()),
        e !== null && !Fc
          ? (as(e, t, i), dl(e, t, i))
          : (z && s && na(t), (t.flags |= 1), Ic(e, t, r, i), t.child)
      );
    }
    function Rc(e, t, n, r, i) {
      if (e === null) {
        var a = n.type;
        return typeof a == `function` && !Fi(a) && a.defaultProps === void 0 && n.compare === null
          ? ((t.tag = 15), (t.type = a), zc(e, t, a, r, i))
          : ((e = Ri(n.type, null, r, t, t.mode, i)),
            (e.ref = t.ref),
            (e.return = t),
            (t.child = e));
      }
      if (((a = e.child), !fl(e, i))) {
        var o = a.memoizedProps;
        if (((n = n.compare), (n = n === null ? Wr : n), n(o, r) && e.ref === t.ref))
          return dl(e, t, i);
      }
      return ((t.flags |= 1), (e = Ii(a, r)), (e.ref = t.ref), (e.return = t), (t.child = e));
    }
    function zc(e, t, n, r, i) {
      if (e !== null) {
        var a = e.memoizedProps;
        if (Wr(a, r) && e.ref === t.ref) {
          if (((Fc = !1), (t.pendingProps = r = a), fl(e, i))) e.flags & 131072 && (Fc = !0);
          else return ((t.lanes = e.lanes), dl(e, t, i));
        }
      }
      return qc(e, t, n, r, i);
    }
    function Bc(e, t, n, r) {
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
          return Hc(e, t, a, n, r);
        }
        if (n & 536870912)
          ((t.memoizedState = { baseLanes: 0, cachePool: null }),
            e !== null && Ja(t, a === null ? null : a.cachePool),
            a === null ? Ao() : ko(t, a),
            Io(t));
        else return ((r = t.lanes = 536870912), Hc(e, t, a === null ? n : a.baseLanes | n, n, r));
      } else
        a === null
          ? (e !== null && Ja(t, null), Ao(), Lo())
          : (Ja(t, a.cachePool), ko(t, a), Lo(), (t.memoizedState = null));
      return (Ic(e, t, i, n), t.child);
    }
    function Vc(e, t) {
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
    function Hc(e, t, n, r, i) {
      var a = qa();
      return (
        (a = a === null ? null : { parent: Ma._currentValue, pool: a }),
        (t.memoizedState = { baseLanes: n, cachePool: a }),
        e !== null && Ja(t, null),
        Ao(),
        Io(t),
        e !== null && Ca(e, t, r, !0),
        (t.childLanes = i),
        null
      );
    }
    function Uc(e, t) {
      return (
        (t = nl({ mode: t.mode, children: t.children }, e.mode)),
        (t.ref = e.ref),
        (e.child = t),
        (t.return = e),
        t
      );
    }
    function Wc(e, t, n) {
      return (
        po(t, e.child, null, n),
        (e = Uc(t, t.pendingProps)),
        (e.flags |= 2),
        Ro(t),
        (t.memoizedState = null),
        e
      );
    }
    function Gc(e, t, n) {
      var r = t.pendingProps,
        a = !!(t.flags & 128);
      if (((t.flags &= -129), e === null)) {
        if (z) {
          if (r.mode === `hidden`)
            return (
              (e = Uc(t, r)),
              (t.lanes = 536870912),
              (e.memoizedState = { baseLanes: 0, cachePool: null }),
              Vc(null, e)
            );
          if (
            (Fo(t),
            (e = R)
              ? ((e = am(e, sa)),
                (e = e !== null && e.data === `&` ? e : null),
                e !== null &&
                  ((t.memoizedState = {
                    dehydrated: e,
                    treeContext: Zi === null ? null : { id: Qi, overflow: $i },
                    retryLane: 536870912,
                    hydrationErrors: null,
                  }),
                  (n = Vi(e)),
                  (n.return = t),
                  (t.child = n),
                  (aa = t),
                  (R = null)))
              : (e = null),
            e === null)
          )
            throw la(t);
          return ((t.lanes = 536870912), null);
        }
        return Uc(t, r);
      }
      var o = e.memoizedState;
      if (o !== null) {
        var s = o.dehydrated;
        if ((Fo(t), a)) {
          if (t.flags & 256) ((t.flags &= -257), (t = Wc(e, t, n)));
          else if (t.memoizedState !== null) ((t.child = e.child), (t.flags |= 128), (t = null));
          else throw Error(i(558));
        } else if ((Fc || Ca(e, t, n, !1), (a = (n & e.childLanes) !== 0), Fc || a)) {
          if (Do.current === null) {
            if (((r = K), r !== null && ((s = yt(r, n)), s !== 0 && s !== o.retryLane)))
              throw ((o.retryLane = s), ki(e, s), Pd(r, e, s), Pc);
            Kd();
          }
          t = Wc(e, t, n);
        } else
          ((e = o.treeContext),
            (R = lm(s.nextSibling)),
            (aa = t),
            (z = !0),
            (oa = null),
            (sa = !1),
            e !== null && ia(t, e),
            (t = Uc(t, r)),
            (t.flags |= 134221824));
        return t;
      }
      return (
        (e = Ii(e.child, { mode: r.mode, children: r.children })),
        (e.ref = t.ref),
        (t.child = e),
        (e.return = t),
        e
      );
    }
    function Kc(e, t) {
      var n = t.ref;
      if (n === null) e !== null && e.ref !== null && (t.flags |= 4194816);
      else {
        if (typeof n != `function` && typeof n != `object`) throw Error(i(284));
        (e === null || e.ref !== n) && (t.flags |= 4194816);
      }
    }
    function qc(e, t, n, r, i) {
      return (
        Ta(t),
        (n = es(e, t, n, r, void 0, i)),
        (r = is()),
        e !== null && !Fc
          ? (as(e, t, i), dl(e, t, i))
          : (z && r && na(t), (t.flags |= 1), Ic(e, t, n, i), t.child)
      );
    }
    function Jc(e, t, n, r, i, a) {
      return (
        Ta(t),
        (t.updateQueue = null),
        (n = ns(t, r, n, i)),
        ts(e),
        (r = is()),
        e !== null && !Fc
          ? (as(e, t, a), dl(e, t, a))
          : (z && r && na(t), (t.flags |= 1), Ic(e, t, n, a), t.child)
      );
    }
    function Yc(e, t, n, r, i) {
      if ((Ta(t), t.stateNode === null)) {
        var a = Mi,
          o = n.contextType;
        (typeof o == `object` && o && (a = Ea(o)),
          (a = new n(r, a)),
          (t.memoizedState = a.state !== null && a.state !== void 0 ? a.state : null),
          (a.updater = xc),
          (t.stateNode = a),
          (a._reactInternals = t),
          (a = t.stateNode),
          (a.props = r),
          (a.state = t.memoizedState),
          (a.refs = {}),
          go(t),
          (o = n.contextType),
          (a.context = typeof o == `object` && o ? Ea(o) : Mi),
          (a.state = t.memoizedState),
          (o = n.getDerivedStateFromProps),
          typeof o == `function` && (bc(t, n, o, r), (a.state = t.memoizedState)),
          typeof n.getDerivedStateFromProps == `function` ||
            typeof a.getSnapshotBeforeUpdate == `function` ||
            (typeof a.UNSAFE_componentWillMount != `function` &&
              typeof a.componentWillMount != `function`) ||
            ((o = a.state),
            typeof a.componentWillMount == `function` && a.componentWillMount(),
            typeof a.UNSAFE_componentWillMount == `function` && a.UNSAFE_componentWillMount(),
            o !== a.state && xc.enqueueReplaceState(a, a.state, null),
            wo(t, r, a, i),
            Co(),
            (a.state = t.memoizedState)),
          typeof a.componentDidMount == `function` && (t.flags |= 4194308),
          (r = !0));
      } else if (e === null) {
        a = t.stateNode;
        var s = t.memoizedProps,
          c = wc(n, s);
        a.props = c;
        var l = a.context,
          u = n.contextType;
        ((o = Mi), typeof u == `object` && u && (o = Ea(u)));
        var d = n.getDerivedStateFromProps;
        ((u = typeof d == `function` || typeof a.getSnapshotBeforeUpdate == `function`),
          (s = t.pendingProps !== s),
          u ||
            (typeof a.UNSAFE_componentWillReceiveProps != `function` &&
              typeof a.componentWillReceiveProps != `function`) ||
            ((s || l !== o) && Cc(t, a, r, o)),
          (ho = !1));
        var f = t.memoizedState;
        ((a.state = f),
          wo(t, r, a, i),
          Co(),
          (l = t.memoizedState),
          s || f !== l || ho
            ? (typeof d == `function` && (bc(t, n, d, r), (l = t.memoizedState)),
              (c = ho || Sc(t, n, c, r, f, l, o))
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
          _o(e, t),
          (o = t.memoizedProps),
          (u = wc(n, o)),
          (a.props = u),
          (d = t.pendingProps),
          (f = a.context),
          (l = n.contextType),
          (c = Mi),
          typeof l == `object` && l && (c = Ea(l)),
          (s = n.getDerivedStateFromProps),
          (l = typeof s == `function` || typeof a.getSnapshotBeforeUpdate == `function`) ||
            (typeof a.UNSAFE_componentWillReceiveProps != `function` &&
              typeof a.componentWillReceiveProps != `function`) ||
            ((o !== d || f !== c) && Cc(t, a, r, c)),
          (ho = !1),
          (f = t.memoizedState),
          (a.state = f),
          wo(t, r, a, i),
          Co());
        var p = t.memoizedState;
        o !== d || f !== p || ho || (e !== null && e.dependencies !== null && wa(e.dependencies))
          ? (typeof s == `function` && (bc(t, n, s, r), (p = t.memoizedState)),
            (u =
              ho ||
              Sc(t, n, u, r, f, p, c) ||
              (e !== null && e.dependencies !== null && wa(e.dependencies)))
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
        Kc(e, t),
        (r = !!(t.flags & 128)),
        a || r
          ? ((a = t.stateNode),
            (n = r && typeof n.getDerivedStateFromError != `function` ? null : a.render()),
            (t.flags |= 1),
            e !== null && r
              ? ((t.child = po(t, e.child, null, i)), (t.child = po(t, null, n, i)))
              : Ic(e, t, n, i),
            (t.memoizedState = a.state),
            (e = t.child))
          : (e = dl(e, t, i)),
        e
      );
    }
    function Xc(e, t, n, r) {
      return (pa(), (t.flags |= 256), Ic(e, t, n, r), t.child);
    }
    var Zc = { dehydrated: null, treeContext: null, retryLane: 0, hydrationErrors: null };
    function Qc(e) {
      return { baseLanes: e, cachePool: Ya() };
    }
    function $c(e, t, n) {
      return ((e = e === null ? 0 : e.childLanes & ~n), t && (e |= ud), e);
    }
    function el(e, t, n) {
      var r = t.pendingProps,
        i = !1,
        a = !!(t.flags & 128),
        o;
      if (
        ((o = a) || (o = e !== null && e.memoizedState === null ? !1 : !!(zo.current & 2)),
        o && ((i = !0), (t.flags &= -129)),
        (o = !!(t.flags & 32)),
        (t.flags &= -33),
        e === null)
      ) {
        if (z) {
          if (
            (i ? Po(t) : Lo(),
            (e = R)
              ? ((e = am(e, sa)),
                (e = e !== null && e.data !== `&` ? e : null),
                e !== null &&
                  ((t.memoizedState = {
                    dehydrated: e,
                    treeContext: Zi === null ? null : { id: Qi, overflow: $i },
                    retryLane: 536870912,
                    hydrationErrors: null,
                  }),
                  (n = Vi(e)),
                  (n.return = t),
                  (t.child = n),
                  (aa = t),
                  (R = null)))
              : (e = null),
            e === null)
          )
            throw la(t);
          return ((t.lanes = sm(e) ? 32 : 536870912), null);
        }
        return (
          (a = r.children),
          (r = r.fallback),
          i
            ? (Lo(),
              (i = t.mode),
              (a = nl({ mode: `hidden`, children: a }, i)),
              (r = zi(r, i, n, null)),
              (a.return = t),
              (r.return = t),
              (a.sibling = r),
              (t.child = a),
              (r = t.child),
              (r.memoizedState = Qc(n)),
              (r.childLanes = $c(e, o, n)),
              (t.memoizedState = Zc),
              Vc(null, r))
            : (Po(t), tl(t, a))
        );
      }
      var s = e.memoizedState;
      if (s !== null) {
        var c = s.dehydrated;
        if (c !== null) return il(e, t, a, o, r, c, s, n);
      }
      return i
        ? (Lo(),
          (i = r.fallback),
          (a = t.mode),
          (s = e.child),
          (c = s.sibling),
          (r = Ii(s, { mode: `hidden`, children: r.children })),
          (r.subtreeFlags = s.subtreeFlags & 1206910976),
          c === null ? ((i = zi(i, a, n, null)), (i.flags |= 2)) : (i = Ii(c, i)),
          (i.return = t),
          (r.return = t),
          (r.sibling = i),
          (t.child = r),
          Vc(null, r),
          (r = t.child),
          (i = e.child.memoizedState),
          i === null
            ? (i = Qc(n))
            : ((a = i.cachePool),
              a === null
                ? (a = Ya())
                : ((s = Ma._currentValue), (a = a.parent === s ? a : { parent: s, pool: s })),
              (i = { baseLanes: i.baseLanes | n, cachePool: a })),
          (r.memoizedState = i),
          (r.childLanes = $c(e, o, n)),
          (t.memoizedState = Zc),
          Vc(e.child, r))
        : (Po(t),
          (n = e.child),
          (e = n.sibling),
          (n = Ii(n, { mode: `visible`, children: r.children })),
          (n.return = t),
          (n.sibling = null),
          e !== null &&
            ((o = t.deletions), o === null ? ((t.deletions = [e]), (t.flags |= 16)) : o.push(e)),
          (t.child = n),
          (t.memoizedState = null),
          n);
    }
    function tl(e, t) {
      return ((t = nl({ mode: `visible`, children: t }, e.mode)), (t.return = e), (e.child = t));
    }
    function nl(e, t) {
      return ((e = Pi(22, e, null, t)), (e.lanes = 0), e);
    }
    function rl(e, t, n) {
      return (
        po(t, e.child, null, n),
        (e = tl(t, t.pendingProps.children)),
        (e.flags |= 2),
        (t.memoizedState = null),
        e
      );
    }
    function il(e, t, n, r, a, o, s, c) {
      if (n)
        return t.flags & 256
          ? (Po(t), (t.flags &= -257), rl(e, t, c))
          : t.memoizedState === null
            ? (Lo(),
              (o = a.fallback),
              (s = t.mode),
              (a = nl({ mode: `visible`, children: a.children }, s)),
              (o = zi(o, s, c, null)),
              (o.flags |= 2),
              (a.return = t),
              (o.return = t),
              (a.sibling = o),
              (t.child = a),
              po(t, e.child, null, c),
              (a = t.child),
              (a.memoizedState = Qc(c)),
              (a.childLanes = $c(e, r, c)),
              (t.memoizedState = Zc),
              Vc(null, a))
            : (Lo(), (t.child = e.child), (t.flags |= 128), null);
      if ((Po(t), sm(o))) {
        if (((r = o.nextSibling && o.nextSibling.dataset), r)) var l = r.dgst;
        return (
          (r = l),
          r !== `` &&
            ((a = Error(i(419))),
            (a.stack = ``),
            (a.digest = r),
            ha({ value: a, source: null, stack: null })),
          rl(e, t, c)
        );
      }
      if ((Fc || Ca(e, t, c, !1), (r = (c & e.childLanes) !== 0), Fc || r)) {
        if (Do.current !== null) return rl(e, t, c);
        if (((r = K), r !== null && ((a = yt(r, c)), a !== 0 && a !== s.retryLane)))
          throw ((s.retryLane = a), ki(e, a), Pd(r, e, a), Pc);
        return (om(o) || Kd(), rl(e, t, c));
      }
      return om(o)
        ? ((t.flags |= 192), (t.child = e.child), null)
        : ((e = s.treeContext),
          (R = lm(o.nextSibling)),
          (aa = t),
          (z = !0),
          (oa = null),
          (sa = !1),
          e !== null && ia(t, e),
          (t = tl(t, a.children)),
          (t.flags |= 134221824),
          t);
    }
    function al(e, t, n) {
      e.lanes |= t;
      var r = e.alternate;
      (r !== null && (r.lanes |= t), xa(e.return, t, n));
    }
    function ol(e) {
      for (var t = null; e !== null;) {
        var n = e.alternate;
        (n !== null && Ho(n) === null && (t = e), (e = e.sibling));
      }
      return t;
    }
    function sl(e, t, n, r, i, a) {
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
    function cl(e) {
      var t = e.child;
      for (e.child = null; t !== null;) {
        var n = t.sibling;
        ((t.sibling = e.child), (e.child = t), (t = n));
      }
    }
    function ll(e, t, n) {
      var r = t.pendingProps,
        i = r.revealOrder,
        a = r.tail;
      r = r.children;
      var o = zo.current;
      if (t.flags & 128) return (Bo(t, o), null);
      var s = !!(o & 2);
      if (
        (s ? ((o = (o & 1) | 2), (t.flags |= 128)) : (o &= 1),
        Bo(t, o),
        i === `backwards` && e !== null ? (cl(e), Ic(e, t, r, n), cl(e)) : Ic(e, t, r, n),
        (r = z ? Ji : 0),
        !s && e !== null && e.flags & 128)
      )
        a: for (e = t.child; e !== null;) {
          if (e.tag === 13) e.memoizedState !== null && al(e, n, t);
          else if (e.tag === 19) al(e, n, t);
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
          ((n = ol(t.child)),
            n === null
              ? ((i = t.child), (t.child = null))
              : ((i = n.sibling), (n.sibling = null), cl(t)),
            sl(t, !0, i, null, a, r));
          break;
        case `unstable_legacy-backwards`:
          for (n = null, i = t.child, t.child = null; i !== null;) {
            if (((e = i.alternate), e !== null && Ho(e) === null)) {
              t.child = i;
              break;
            }
            ((e = i.sibling), (i.sibling = n), (n = i), (i = e));
          }
          sl(t, !0, n, null, a, r);
          break;
        case `together`:
          sl(t, !1, null, null, void 0, r);
          break;
        case `independent`:
          t.memoizedState = null;
          break;
        default:
          ((n = ol(t.child)),
            n === null ? ((i = t.child), (t.child = null)) : ((i = n.sibling), (n.sibling = null)),
            sl(t, !1, i, n, a, r));
      }
      return t.child;
    }
    function ul(e, t, n) {
      var r = t.pendingProps;
      return (ya(t, t.type, r.value), Ic(e, t, r.children, n), t.child);
    }
    function dl(e, t, n) {
      if (
        (e !== null && (t.dependencies = e.dependencies), (sd |= t.lanes), (n & t.childLanes) === 0)
      ) {
        if (e !== null) {
          if ((Ca(e, t, n, !1), (n & t.childLanes) === 0)) return null;
        } else return null;
      }
      if (e !== null && t.child !== e.child) throw Error(i(153));
      if (t.child !== null) {
        for (e = t.child, n = Ii(e, e.pendingProps), t.child = n, n.return = t; e.sibling !== null;)
          ((e = e.sibling), (n = n.sibling = Ii(e, e.pendingProps)), (n.return = t));
        n.sibling = null;
      }
      return t.child;
    }
    function fl(e, t) {
      return (e.lanes & t) !== 0 || ((e = e.dependencies), !!(e !== null && wa(e)));
    }
    function pl(e, t, n) {
      switch (t.tag) {
        case 3:
          (Oe(t, t.stateNode.containerInfo), ya(t, Ma, e.memoizedState.cache), pa());
          break;
        case 27:
        case 5:
          Ae(t);
          break;
        case 4:
          Oe(t, t.stateNode.containerInfo);
          break;
        case 10:
          ya(t, t.type, t.memoizedProps.value);
          break;
        case 31:
          if (t.memoizedState !== null) return ((t.flags |= 128), Fo(t), null);
          break;
        case 13:
          var r = t.memoizedState;
          if (r !== null) {
            if (r.dehydrated !== null) return (Po(t), (t.flags |= 128), null);
            r = Ca(e, t, n, !1);
            var i = t.child.childLanes;
            return r || (n & i) !== 0
              ? el(e, t, n)
              : (Po(t), (e = dl(e, t, n)), e === null ? null : e.sibling);
          }
          Po(t);
          break;
        case 19:
          if (t.flags & 128) return ll(e, t, n);
          if (
            ((i = !!(e.flags & 128)),
            (r = (n & t.childLanes) !== 0),
            (r ||= (Ca(e, t, n, !1), (n & t.childLanes) !== 0)),
            i)
          ) {
            if (r) return ll(e, t, n);
            t.flags |= 128;
          }
          if (
            ((i = t.memoizedState),
            i !== null && ((i.rendering = null), (i.tail = null), (i.lastEffect = null)),
            Bo(t, zo.current),
            r)
          )
            break;
          return null;
        case 22:
          return ((t.lanes = 0), Bc(e, t, n, t.pendingProps));
        case 24:
          ya(t, Ma, e.memoizedState.cache);
      }
      return dl(e, t, n);
    }
    function ml(e, t, n) {
      if (e !== null) {
        if (e.memoizedProps !== t.pendingProps) Fc = !0;
        else {
          if (!fl(e, n) && !(t.flags & 128)) return ((Fc = !1), pl(e, t, n));
          Fc = !!(e.flags & 131072);
        }
      } else ((Fc = !1), z && t.flags & 1048576 && ta(t, Ji, t.index));
      switch (((t.lanes = 0), t.tag)) {
        case 16:
          a: {
            var r = t.pendingProps;
            if (((e = no(t.elementType)), (t.type = e), typeof e == `function`))
              Fi(e)
                ? ((r = wc(e, r)), (t.tag = 1), (t = Yc(null, t, e, r, n)))
                : ((t.tag = 0), (t = qc(null, t, e, r, n)));
            else {
              if (e != null) {
                var a = e.$$typeof;
                if (a === O) {
                  ((t.tag = 11), (t = Lc(null, t, e, r, n)));
                  break a;
                }
                if (a === A) {
                  ((t.tag = 14), (t = Rc(null, t, e, r, n)));
                  break a;
                }
                if (a === D) {
                  ((t.tag = 10), (t.type = e), (t = ul(null, t, n)));
                  break a;
                }
              }
              throw ((t = _e(e) || e), Error(i(306, t, ``)));
            }
          }
          return t;
        case 0:
          return qc(e, t, t.type, t.pendingProps, n);
        case 1:
          return ((r = t.type), (a = wc(r, t.pendingProps)), Yc(e, t, r, a, n));
        case 3:
          a: {
            if ((Oe(t, t.stateNode.containerInfo), e === null)) throw Error(i(387));
            r = t.pendingProps;
            var o = t.memoizedState;
            ((a = o.element), _o(e, t), wo(t, r, null, n));
            var s = t.memoizedState;
            if (
              ((r = s.cache),
              ya(t, Ma, r),
              r !== o.cache && Sa(t, [Ma], n, !0),
              Co(),
              (r = s.element),
              o.isDehydrated)
            ) {
              if (
                ((o = { element: r, isDehydrated: !1, cache: s.cache }),
                (t.updateQueue.baseState = o),
                (t.memoizedState = o),
                t.flags & 256)
              ) {
                t = Xc(e, t, r, n);
                break a;
              }
              if (r !== a) {
                ((a = Wi(Error(i(424)), t)), ha(a), (t = Xc(e, t, r, n)));
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
                R = lm(e.firstChild),
                  aa = t,
                  z = !0,
                  oa = null,
                  sa = !0,
                  n = mo(t, null, r, n),
                  t.child = n;
                n;
              )
                ((n.flags = (n.flags & -3) | 134221824), (n = n.sibling));
            } else {
              if ((pa(), r === a)) {
                t = dl(e, t, n);
                break a;
              }
              Ic(e, t, r, n);
            }
            t = t.child;
          }
          return t;
        case 26:
          return (
            Kc(e, t),
            e === null
              ? (n = Nm(t.type, null, t.pendingProps, null))
                ? (t.memoizedState = n)
                : z || (t.stateNode = fp(t.type, t.pendingProps, Ee.current, t))
              : (t.memoizedState = Nm(t.type, e.memoizedProps, t.pendingProps, e.memoizedState)),
            null
          );
        case 27:
          return (
            Ae(t),
            e === null &&
              z &&
              ((r = t.stateNode = hm(t.type, t.pendingProps, Ee.current)),
              (aa = t),
              (sa = !0),
              (a = R),
              Sp(t.type) ? ((um = a), (R = lm(r.firstChild))) : (R = a)),
            Ic(e, t, t.pendingProps.children, n),
            Kc(e, t),
            e === null && (t.flags |= 4194304),
            t.child
          );
        case 5:
          return (
            e === null &&
              z &&
              ((a = r = R) &&
                ((r = rm(r, t.type, t.pendingProps, sa)),
                r === null
                  ? (a = !1)
                  : ((t.stateNode = r), (aa = t), (R = lm(r.firstChild)), (sa = !1), (a = !0))),
              a || la(t)),
            Ae(t),
            (a = t.type),
            (o = t.pendingProps),
            (s = e === null ? null : e.memoizedProps),
            (r = o.children),
            pp(a, o) ? (r = null) : s !== null && pp(a, s) && (t.flags |= 32),
            t.memoizedState !== null && ((a = es(e, t, rs, null, null, n)), (sh._currentValue = a)),
            Kc(e, t),
            Ic(e, t, r, n),
            t.child
          );
        case 6:
          return (
            e === null &&
              z &&
              ((e = n = R) &&
                ((n = im(n, t.pendingProps, sa)),
                n === null ? (e = !1) : ((t.stateNode = n), (aa = t), (R = null), (e = !0))),
              e || la(t)),
            null
          );
        case 13:
          return el(e, t, n);
        case 4:
          return (
            Oe(t, t.stateNode.containerInfo),
            (r = t.pendingProps),
            e === null ? (t.child = po(t, null, r, n)) : Ic(e, t, r, n),
            t.child
          );
        case 11:
          return Lc(e, t, t.type, t.pendingProps, n);
        case 7:
          return ((r = t.pendingProps), Kc(e, t), Ic(e, t, r, n), t.child);
        case 8:
          return (Ic(e, t, t.pendingProps.children, n), t.child);
        case 12:
          return (Ic(e, t, t.pendingProps.children, n), t.child);
        case 10:
          return ul(e, t, n);
        case 9:
          return (
            (a = t.type._context),
            (r = t.pendingProps.children),
            Ta(t),
            (a = Ea(a)),
            (r = r(a)),
            (t.flags |= 1),
            Ic(e, t, r, n),
            t.child
          );
        case 14:
          return Rc(e, t, t.type, t.pendingProps, n);
        case 15:
          return zc(e, t, t.type, t.pendingProps, n);
        case 19:
          return ll(e, t, n);
        case 31:
          return Gc(e, t, n);
        case 22:
          return Bc(e, t, n, t.pendingProps);
        case 24:
          return (
            Ta(t),
            (r = Ea(Ma)),
            e === null
              ? ((a = qa()),
                a === null &&
                  ((a = K),
                  (o = Na()),
                  (a.pooledCache = o),
                  o.refCount++,
                  o !== null && (a.pooledCacheLanes |= n),
                  (a = o)),
                (t.memoizedState = { parent: r, cache: a }),
                go(t),
                ya(t, Ma, a))
              : ((e.lanes & n) !== 0 && (_o(e, t), wo(t, null, null, n), Co()),
                (a = e.memoizedState),
                (o = t.memoizedState),
                a.parent === r
                  ? ((r = o.cache), ya(t, Ma, r), r !== a.cache && Sa(t, [Ma], n, !0))
                  : ((a = { parent: r, cache: r }),
                    (t.memoizedState = a),
                    t.lanes === 0 && (t.memoizedState = t.updateQueue.baseState = a),
                    ya(t, Ma, r))),
            Ic(e, t, t.pendingProps.children, n),
            t.child
          );
        case 30:
          return (
            t.stateNode === null &&
              (t.stateNode = { autoName: null, paired: null, clones: null, ref: null }),
            (r = t.pendingProps),
            r.name != null && r.name !== `auto`
              ? (t.flags |= e === null ? 18882560 : 18874368)
              : z && na(t),
            e !== null && e.memoizedProps.name !== r.name ? (t.flags |= 4194816) : Kc(e, t),
            Ic(e, t, r.children, n),
            t.child
          );
        case 29:
          throw t.pendingProps;
      }
      throw Error(i(156, t.tag));
    }
    function hl(e) {
      e.flags |= 4;
    }
    function gl(e, t, n, r, i) {
      var a;
      if (
        ((a = !!(e.mode & 32)) &&
          (a = n === null ? Jm(t, r) : Jm(t, r) && (r.src !== n.src || r.srcSet !== n.srcSet)),
        a)
      ) {
        if (((e.flags |= 16777216), (i & 335544128) === i)) {
          if (e.stateNode.complete) e.flags |= 8192;
          else if (Ud()) e.flags |= 8192;
          else throw ((ro = $a), Za);
        }
      } else e.flags &= -16777217;
    }
    function _l(e, t) {
      if (t.type !== `stylesheet` || t.state.loading & 4) e.flags &= -16777217;
      else if (((e.flags |= 16777216), !Ym(t))) {
        if (Ud()) e.flags |= 8192;
        else throw ((ro = $a), Za);
      }
    }
    function vl(e, t) {
      (t !== null && (e.flags |= 4),
        e.flags & 16384 && ((t = e.tag === 22 ? 536870912 : pt()), (e.lanes |= t), (dd |= t)));
    }
    function yl(e, t) {
      if (!z)
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
    function H(e) {
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
    function bl(e, t, n) {
      var r = t.pendingProps;
      switch ((ra(t), t.tag)) {
        case 16:
        case 15:
        case 0:
        case 11:
        case 7:
        case 8:
        case 12:
        case 9:
        case 14:
          return (H(t), null);
        case 1:
          return (H(t), null);
        case 3:
          return (
            (n = t.stateNode),
            (r = null),
            e !== null && (r = e.memoizedState.cache),
            t.memoizedState.cache !== r && (t.flags |= 2048),
            ba(Ma),
            ke(),
            n.pendingContext && ((n.context = n.pendingContext), (n.pendingContext = null)),
            (e === null || e.child === null) &&
              (fa(t)
                ? hl(t)
                : e === null ||
                  (e.memoizedState.isDehydrated && !(t.flags & 256)) ||
                  ((t.flags |= 1024), ma())),
            H(t),
            null
          );
        case 26:
          var a = t.type,
            o = t.memoizedState;
          return (
            e === null
              ? (hl(t), o === null ? (H(t), gl(t, a, null, r, n)) : (H(t), _l(t, o)))
              : o
                ? o === e.memoizedState
                  ? (H(t), (t.flags &= -16777217))
                  : (hl(t), H(t), _l(t, o))
                : ((e = e.memoizedProps), e !== r && hl(t), H(t), gl(t, a, e, r, n)),
            null
          );
        case 27:
          if ((je(t), (n = Ee.current), (a = t.type), e !== null && t.stateNode != null))
            e.memoizedProps !== r && hl(t);
          else {
            if (!r) {
              if (t.stateNode === null) throw Error(i(166));
              return (H(t), (t.subtreeFlags &= -33554433), null);
            }
            ((e = we.current), fa(t) ? ua(t, e) : ((e = hm(a, r, n)), (t.stateNode = e), hl(t)));
          }
          return (H(t), (t.subtreeFlags &= -33554433), null);
        case 5:
          if ((je(t), (a = t.type), e !== null && t.stateNode != null))
            e.memoizedProps !== r && hl(t);
          else {
            if (!r) {
              if (t.stateNode === null) throw Error(i(166));
              return (H(t), (t.subtreeFlags &= -33554433), null);
            }
            if (((o = we.current), fa(t))) ua(t, o);
            else {
              var s = lp(Ee.current);
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
              ((o[Tt] = t), (o[Et] = r));
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
              r && hl(t);
            }
          }
          return (
            H(t),
            (t.subtreeFlags &= -33554433),
            gl(t, t.type, e === null ? null : e.memoizedProps, t.pendingProps, n),
            null
          );
        case 6:
          if (e && t.stateNode != null) e.memoizedProps !== r && hl(t);
          else {
            if (typeof r != `string` && t.stateNode === null) throw Error(i(166));
            if (((e = Ee.current), fa(t))) {
              if (((e = t.stateNode), (n = t.memoizedProps), (r = null), (a = aa), a !== null))
                switch (a.tag) {
                  case 27:
                  case 5:
                    r = a.memoizedProps;
                }
              ((e[Tt] = t),
                (e = !!(
                  e.nodeValue === n ||
                  (r !== null && !0 === r.suppressHydrationWarning) ||
                  ep(e.nodeValue, n)
                )),
                e || la(t, !0));
            } else ((e = lp(e).createTextNode(r)), (e[Tt] = t), (t.stateNode = e));
          }
          return (H(t), null);
        case 31:
          if (((n = t.memoizedState), e === null || e.memoizedState !== null)) {
            if (((r = fa(t)), n !== null)) {
              if (e === null) {
                if (!r) throw Error(i(318));
                if (((e = t.memoizedState), (e = e === null ? null : e.dehydrated), !e))
                  throw Error(i(557));
                e[Tt] = t;
              } else (pa(), !(t.flags & 128) && (t.memoizedState = null), (t.flags |= 4));
              (H(t), (e = !1));
            } else
              ((n = ma()),
                e !== null && e.memoizedState !== null && (e.memoizedState.hydrationErrors = n),
                (e = !0));
            if (!e) return t.flags & 256 ? (Ro(t), t) : (Ro(t), null);
            if (t.flags & 128) throw Error(i(558));
          }
          return (H(t), null);
        case 13:
          if (
            ((r = t.memoizedState),
            e === null || (e.memoizedState !== null && e.memoizedState.dehydrated !== null))
          ) {
            if (((a = fa(t)), r !== null && r.dehydrated !== null)) {
              if (e === null) {
                if (!a) throw Error(i(318));
                if (((a = t.memoizedState), (a = a === null ? null : a.dehydrated), !a))
                  throw Error(i(317));
                a[Tt] = t;
              } else (pa(), !(t.flags & 128) && (t.memoizedState = null), (t.flags |= 4));
              (H(t), (a = !1));
            } else
              ((a = ma()),
                e !== null && e.memoizedState !== null && (e.memoizedState.hydrationErrors = a),
                (a = !0));
            if (!a) return t.flags & 256 ? (Ro(t), t) : (Ro(t), null);
          }
          return (
            Ro(t),
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
                vl(t, t.updateQueue),
                H(t),
                null)
          );
        case 4:
          return (
            ke(), e === null && Wf(t.stateNode.containerInfo), (t.flags |= 67108864), H(t), null
          );
        case 10:
          return (ba(t.type), H(t), null);
        case 19:
          if ((Vo(t), (r = t.memoizedState), r === null)) return (H(t), null);
          if (((a = !!(t.flags & 128)), (o = r.rendering), o === null)) {
            if (a) yl(r, !1);
            else {
              if (od !== 0 || (e !== null && e.flags & 128))
                for (e = t.child; e !== null;) {
                  if (((o = Ho(e)), o !== null)) {
                    for (
                      t.flags |= 128,
                        yl(r, !1),
                        e = o.updateQueue,
                        t.updateQueue = e,
                        vl(t, e),
                        t.subtreeFlags = 0,
                        e = n,
                        n = t.child;
                      n !== null;
                    )
                      (Li(n, e), (n = n.sibling));
                    return (Bo(t, (zo.current & 1) | 2), z && ea(t, r.treeForkCount), t.child);
                  }
                  e = e.sibling;
                }
              r.tail !== null &&
                We() > _d &&
                ((t.flags |= 128), (a = !0), yl(r, !1), (t.lanes = 4194304));
            }
          } else {
            if (!a) {
              if (((e = Ho(o)), e !== null)) {
                if (
                  ((t.flags |= 128),
                  (a = !0),
                  (e = e.updateQueue),
                  (t.updateQueue = e),
                  vl(t, e),
                  yl(r, !0),
                  r.tail === null &&
                    r.tailMode !== `collapsed` &&
                    r.tailMode !== `visible` &&
                    !o.alternate &&
                    !z)
                )
                  return (H(t), null);
              } else
                2 * We() - r.renderingStartTime > _d &&
                  n !== 536870912 &&
                  ((t.flags |= 128), (a = !0), yl(r, !1), (t.lanes = 4194304));
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
              (r.renderingStartTime = We()),
              (e.sibling = null),
              (o = zo.current),
              (o = a ? (o & 1) | 2 : o & 1),
              r.tailMode === `visible` || r.tailMode === `collapsed` || !n || z
                ? Bo(t, o)
                : ((n = o), P(Mo, t), P(zo, n), No === null && (No = t)),
              z && ea(t, r.treeForkCount),
              e
            );
          }
          return (H(t), null);
        case 22:
        case 23:
          return (
            Ro(t),
            jo(),
            (r = t.memoizedState !== null),
            e === null
              ? r && (t.flags |= 8192)
              : (e.memoizedState !== null) !== r && (t.flags |= 8192),
            r
              ? n & 536870912 && !(t.flags & 128) && (H(t), t.subtreeFlags & 6 && (t.flags |= 8192))
              : H(t),
            (n = t.updateQueue),
            n !== null && vl(t, n.retryQueue),
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
            e !== null && Ce(Ka),
            null
          );
        case 24:
          return (
            (n = null),
            e !== null && (n = e.memoizedState.cache),
            t.memoizedState.cache !== n && (t.flags |= 2048),
            ba(Ma),
            H(t),
            null
          );
        case 25:
          return null;
        case 30:
          return ((t.flags |= 33554432), H(t), null);
      }
      throw Error(i(156, t.tag));
    }
    function xl(e, t) {
      switch ((ra(t), t.tag)) {
        case 1:
          return ((e = t.flags), e & 65536 ? ((t.flags = (e & -65537) | 128), t) : null);
        case 3:
          return (
            ba(Ma),
            ke(),
            (e = t.flags),
            e & 65536 && !(e & 128) ? ((t.flags = (e & -65537) | 128), t) : null
          );
        case 26:
        case 27:
        case 5:
          return (je(t), null);
        case 31:
          if (t.memoizedState !== null) {
            if ((Ro(t), t.alternate === null)) throw Error(i(340));
            pa();
          }
          return ((e = t.flags), e & 65536 ? ((t.flags = (e & -65537) | 128), t) : null);
        case 13:
          if ((Ro(t), (e = t.memoizedState), e !== null && e.dehydrated !== null)) {
            if (t.alternate === null) throw Error(i(340));
            pa();
          }
          return ((e = t.flags), e & 65536 ? ((t.flags = (e & -65537) | 128), t) : null);
        case 19:
          return (
            Vo(t),
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
          return (ke(), null);
        case 10:
          return (ba(t.type), null);
        case 22:
        case 23:
          return (
            Ro(t),
            jo(),
            e !== null && Ce(Ka),
            (e = t.flags),
            e & 65536 ? ((t.flags = (e & -65537) | 128), t) : null
          );
        case 24:
          return (ba(Ma), null);
        case 25:
          return null;
        default:
          return null;
      }
    }
    function Sl(e, t) {
      switch ((ra(t), t.tag)) {
        case 3:
          (ba(Ma), ke());
          break;
        case 26:
        case 27:
        case 5:
          je(t);
          break;
        case 4:
          ke();
          break;
        case 31:
          t.memoizedState !== null && Ro(t);
          break;
        case 13:
          Ro(t);
          break;
        case 19:
          Vo(t);
          break;
        case 10:
          ba(t.type);
          break;
        case 22:
        case 23:
          (Ro(t), jo(), e !== null && Ce(Ka));
          break;
        case 24:
          ba(Ma);
      }
    }
    function Cl(e, t) {
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
    function wl(e, t, n) {
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
    function Tl(e) {
      var t = e.updateQueue;
      if (t !== null) {
        var n = e.stateNode;
        try {
          Eo(t, n);
        } catch (t) {
          Z(e, e.return, t);
        }
      }
    }
    function El(e, t, n) {
      ((n.props = wc(e.type, e.memoizedProps)), (n.state = e.memoizedState));
      try {
        n.componentWillUnmount();
      } catch (n) {
        Z(e, t, n);
      }
    }
    function Dl(e, t) {
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
                a = yi(e.memoizedProps, i);
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
    function Ol(e, t) {
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
    function kl(e, t) {
      if ((e.tag === 5 || e.tag === 27 || e.tag === 6) && e.alternate === null && t !== null)
        for (var n = 0; n < t.length; n++) em(e.stateNode, t[n]);
    }
    function Al(e) {
      for (var t = e.return; t !== null && (Nl(t) && em(e.stateNode, t.stateNode), !Ml(t));)
        t = t.return;
    }
    function jl(e) {
      for (var t = e.return; t !== null && (Nl(t) && tm(e.stateNode, t.stateNode), !Ml(t));)
        t = t.return;
    }
    function Ml(e) {
      return e.tag === 5 || e.tag === 3 || e.tag === 27;
    }
    function Nl(e) {
      return e && e.tag === 7 && e.stateNode !== null;
    }
    function Pl(e) {
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
    function Fl(e, t, n) {
      try {
        var r = e.stateNode;
        (ip(r, e.type, n, t), (r[Et] = t));
      } catch (t) {
        Z(e, e.return, t);
      }
    }
    function Il(e) {
      return (
        e.tag === 5 || e.tag === 3 || e.tag === 26 || (e.tag === 27 && Sp(e.type)) || e.tag === 4
      );
    }
    function Ll(e) {
      a: for (;;) {
        for (; e.sibling === null;) {
          if (e.return === null || Il(e.return)) return null;
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
    function Rl(e, t, n, r) {
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
              n != null || t.onclick !== null || (t.onclick = bn)),
          kl(e, r),
          (L = !0));
      else if (
        i !== 4 &&
        (i === 27 && (kl(e, r), (r = null), Sp(e.type) && ((n = e.stateNode), (t = null))),
        (e = e.child),
        e !== null)
      )
        for (Rl(e, t, n, r), e = e.sibling; e !== null;) (Rl(e, t, n, r), (e = e.sibling));
    }
    function zl(e, t, n, r) {
      var i = e.tag;
      if (i === 5 || i === 6)
        ((i = e.stateNode), t ? n.insertBefore(i, t) : n.appendChild(i), kl(e, r), (L = !0));
      else if (
        i !== 4 &&
        (i === 27 && (kl(e, r), (r = null), Sp(e.type) && (n = e.stateNode)),
        (e = e.child),
        e !== null)
      )
        for (zl(e, t, n, r), e = e.sibling; e !== null;) (zl(e, t, n, r), (e = e.sibling));
    }
    function Bl(e) {
      var t = e.stateNode,
        n = e.memoizedProps;
      try {
        for (var r = e.type, i = t.attributes; i.length;) t.removeAttributeNode(i[0]);
        (np(t, r, n), (t[Tt] = e), (t[Et] = n));
      } catch (t) {
        Z(e, e.return, t);
      }
    }
    var Vl = !1,
      Hl = null;
    function Ul(e) {
      (e.tag === 30 || e.subtreeFlags & 33554432) && (Vl = !0);
    }
    var Wl = null;
    function Gl() {
      var e = Wl;
      return ((Wl = null), e);
    }
    var Kl = 0;
    function ql(e, t, n, r, i) {
      return ((Kl = 0), Jl(e.child, t, n, r, i));
    }
    function Jl(e, t, n, r, i) {
      for (var a = !1; e !== null;) {
        if (e.tag === 5) {
          var o = e.stateNode;
          if (r !== null) {
            var s = Op(o);
            (r.push(s), s.view && (a = !0));
          } else a || (Op(o).view && (a = !0));
          ((Vl = !0), Tp(o, Kl === 0 ? t : t + `_` + Kl, n), Kl++);
        } else
          (e.tag !== 22 || e.memoizedState === null) &&
            ((e.tag === 30 && i) || (Jl(e.child, t, n, r, i) && (a = !0)));
        e = e.sibling;
      }
      return a;
    }
    function Yl(e, t) {
      for (; e !== null;)
        (e.tag === 5
          ? Ep(e.stateNode, e.memoizedProps)
          : (e.tag !== 22 || e.memoizedState === null) && ((e.tag === 30 && t) || Yl(e.child, t)),
          (e = e.sibling));
    }
    function Xl(e) {
      if (e.subtreeFlags & 18874368)
        for (e = e.child; e !== null;) {
          if (
            (e.tag !== 22 || e.memoizedState === null) &&
            (Xl(e), e.tag === 30 && e.flags & 18874368 && e.stateNode.paired)
          ) {
            var t = e.memoizedProps;
            if (t.name == null || t.name === `auto`) throw Error(i(544));
            var n = t.name;
            ((t = xi(t.default, t.share)),
              t !== `none` && (ql(e, n, t, null, !1) || Yl(e.child, !1)));
          }
          e = e.sibling;
        }
    }
    function Zl(e, t) {
      if (e.tag === 30) {
        var n = e.stateNode,
          r = e.memoizedProps,
          i = yi(r, n),
          a = xi(r.default, n.paired ? r.share : r.enter);
        a === `none`
          ? Xl(e)
          : ql(e, i, a, null, !1)
            ? (Xl(e), n.paired || t || Nd(e, r.onEnter))
            : Yl(e.child, !1);
      } else if (e.subtreeFlags & 33554432)
        for (e = e.child; e !== null;) (Zl(e, t), (e = e.sibling));
      else Xl(e);
    }
    function Ql(e) {
      if (Hl !== null && Hl.size !== 0) {
        var t = Hl;
        if (e.subtreeFlags & 18874368)
          for (e = e.child; e !== null;) {
            if (e.tag !== 22 || e.memoizedState === null) {
              if (e.tag === 30 && e.flags & 18874368) {
                var n = e.memoizedProps,
                  r = n.name;
                if (r != null && r !== `auto`) {
                  var i = t.get(r);
                  if (i !== void 0) {
                    var a = xi(n.default, n.share);
                    if (
                      (a !== `none` &&
                        (ql(e, r, a, null, !1)
                          ? ((a = e.stateNode), (i.paired = a), (a.paired = i), Nd(e, n.onShare))
                          : Yl(e.child, !1)),
                      t.delete(r),
                      t.size === 0)
                    )
                      break;
                  }
                }
              }
              Ql(e);
            }
            e = e.sibling;
          }
      }
    }
    function $l(e) {
      if (e.tag === 30) {
        var t = e.memoizedProps,
          n = yi(t, e.stateNode),
          r = Hl === null ? void 0 : Hl.get(n),
          i = xi(t.default, r === void 0 ? t.exit : t.share);
        (i !== `none` &&
          (ql(e, n, i, null, !1)
            ? r === void 0
              ? Nd(e, t.onExit)
              : ((i = e.stateNode), (r.paired = i), (i.paired = r), Hl.delete(n), Nd(e, t.onShare))
            : Yl(e.child, !1)),
          Hl !== null && Ql(e));
      } else if (e.subtreeFlags & 33554432) for (e = e.child; e !== null;) ($l(e), (e = e.sibling));
      else Hl !== null && Ql(e);
    }
    function eu(e) {
      for (e = e.child; e !== null;) {
        if (e.tag === 30) {
          var t = e.memoizedProps,
            n = yi(t, e.stateNode);
          ((t = xi(t.default, t.update)),
            (e.flags &= -5),
            t !== `none` && ql(e, n, t, (e.memoizedState = []), !1));
        } else e.subtreeFlags & 33554432 && eu(e);
        e = e.sibling;
      }
    }
    function tu(e) {
      if (e.subtreeFlags & 18874368)
        for (e = e.child; e !== null;) {
          if (e.tag !== 22 || e.memoizedState === null) {
            if (e.tag === 30 && e.flags & 18874368) {
              var t = e.stateNode;
              t.paired !== null && ((t.paired = null), Yl(e.child, !1));
            }
            tu(e);
          }
          e = e.sibling;
        }
    }
    function nu(e) {
      if (e.tag === 30) ((e.stateNode.paired = null), Yl(e.child, !1), tu(e));
      else if (e.subtreeFlags & 33554432) for (e = e.child; e !== null;) (nu(e), (e = e.sibling));
      else tu(e);
    }
    function ru(e) {
      for (e = e.child; e !== null;)
        (e.tag === 30 ? Yl(e.child, !1) : e.subtreeFlags & 33554432 && ru(e), (e = e.sibling));
    }
    function iu(e, t, n, r, i, a, o) {
      for (var s = !1; t !== null;) {
        if (t.tag === 5) {
          var c = t.stateNode;
          if (a !== null && Kl < a.length) {
            var l = a[Kl],
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
          (e.flags & 4 && Tp(c, Kl === 0 ? n : n + `_` + Kl, i),
            (s && e.flags & 4) ||
              (Wl === null && (Wl = []), Wl.push(c, Kl === 0 ? r : r + `_` + Kl, t.memoizedProps)),
            Kl++);
        } else
          (t.tag !== 22 || t.memoizedState === null) &&
            (t.tag === 30 && o
              ? (e.flags |= t.flags & 32)
              : iu(e, t.child, n, r, i, a, o) && (s = !0));
        t = t.sibling;
      }
      return s;
    }
    function au(e, t) {
      for (e = e.child; e !== null;) {
        if (e.tag === 30) {
          var n = e.memoizedProps,
            r = e.stateNode,
            i = yi(n, r),
            a = xi(n.default, n.update);
          if (t) {
            r = r.clones;
            var o = r === null ? null : r.map(kp);
          } else ((o = e.memoizedState), (e.memoizedState = null));
          r = e;
          var s = e.child;
          ((Kl = 0), (i = iu(r, s, i, i, a, o, !1)), e.flags & 4 && i && (t || Nd(e, n.onUpdate)));
        } else e.subtreeFlags & 33554432 && au(e, t);
        e = e.sibling;
      }
    }
    var ou = !1,
      U = !1,
      su = !1,
      cu = !1,
      lu = typeof WeakSet == `function` ? WeakSet : Set,
      uu = null,
      du = !1,
      fu = !1,
      pu = !1,
      mu = !1;
    function hu(e, t, n) {
      if (((e = e.containerInfo), (sp = gh), (e = Yr(e)), Xr(e))) {
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
          uu = t,
          t = n ? 9270 : 1024;
        uu !== null;
      ) {
        if (((e = uu), n && ((r = e.deletions), r !== null)))
          for (a = 0; a < r.length; a++) n && $l(r[a]);
        if (e.alternate === null && e.flags & 2) (n && Ul(e), gu(n));
        else {
          if (e.tag === 22) {
            if (((r = e.alternate), e.memoizedState !== null)) {
              (r !== null && r.memoizedState === null && n && $l(r), gu(n));
              continue;
            }
            if (r !== null && r.memoizedState !== null) {
              (n && Ul(e), gu(n));
              continue;
            }
          }
          ((r = e.child),
            (e.subtreeFlags & t) !== 0 && r !== null
              ? ((r.return = e), (uu = r))
              : (n && eu(e), gu(n)));
        }
      }
      Hl = null;
    }
    function gu(e) {
      for (; uu !== null;) {
        var t = uu,
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
                var s = wc(t.type, a);
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
              ((n = yi(r.memoizedProps, r.stateNode)),
              (a = t.memoizedProps),
              (a = xi(a.default, a.update)),
              a !== `none` && ql(r, n, a, (r.memoizedState = []), !0));
            break;
          default:
            if (a & 1024) throw Error(i(163));
        }
        if (((r = t.sibling), r !== null)) {
          ((r.return = t.return), (uu = r));
          break;
        }
        uu = t.return;
      }
    }
    function _u(e, t, n) {
      var r = n.flags;
      switch (n.tag) {
        case 0:
        case 11:
        case 15:
          (Iu(e, n), r & 4 && Cl(5, n));
          break;
        case 1:
          if ((Iu(e, n), r & 4)) {
            if (((e = n.stateNode), t === null))
              try {
                e.componentDidMount();
              } catch (e) {
                Z(n, n.return, e);
              }
            else {
              var i = wc(n.type, t.memoizedProps);
              t = t.memoizedState;
              try {
                e.componentDidUpdate(i, t, e.__reactInternalSnapshotBeforeUpdate);
              } catch (e) {
                Z(n, n.return, e);
              }
            }
          }
          (r & 64 && Tl(n), r & 512 && Dl(n, n.return));
          break;
        case 3:
          if ((Iu(e, n), r & 64 && ((e = n.updateQueue), e !== null))) {
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
              Eo(e, t);
            } catch (e) {
              Z(n, n.return, e);
            }
          }
          break;
        case 27:
          t === null && r & 4 && Bl(n);
        case 26:
        case 5:
          (Iu(e, n), t === null && r & 4 && Pl(n), r & 512 && Dl(n, n.return));
          break;
        case 12:
          Iu(e, n);
          break;
        case 31:
          (Iu(e, n), r & 4 && Tu(e, n));
          break;
        case 13:
          (Iu(e, n),
            r & 4 && Eu(e, n),
            r & 64 &&
              ((e = n.memoizedState),
              e !== null &&
                ((e = e.dehydrated), e !== null && ((n = _f.bind(null, n)), cm(e, n)))));
          break;
        case 22:
          if (((r = n.memoizedState !== null || ou), !r)) {
            var a = (t !== null && t.memoizedState !== null) || U;
            ((t = ou),
              (i = U),
              (ou = r),
              (U = a) && !i ? ((r = 2), n.subtreeFlags & 8772 && (r |= 1), Ru(e, n, r)) : Iu(e, n),
              (ou = t),
              (U = i));
          }
          break;
        case 30:
          (Iu(e, n), r & 512 && Dl(n, n.return));
          break;
        case 7:
          r & 512 && Dl(n, n.return);
        default:
          Iu(e, n);
      }
    }
    function vu(e, t) {
      for (e = e.child; e !== null;) (yu(e, t), (e = e.sibling));
    }
    function yu(e, t) {
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
          bu(e, t);
          break;
        case 6:
          try {
            ((e.stateNode.nodeValue = t ? `` : e.memoizedProps), (L = !0));
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
          e.memoizedState === null && vu(e, t);
          break;
        default:
          vu(e, t);
      }
    }
    function bu(e, t) {
      if (e.subtreeFlags & 67108864)
        for (e = e.child; e !== null;) {
          a: {
            var n = e,
              r = t;
            switch (n.tag) {
              case 4:
                yu(n, r);
                break a;
              case 22:
                n.memoizedState === null && bu(n, r);
                break a;
              default:
                bu(n, r);
            }
          }
          e = e.sibling;
        }
    }
    function xu(e) {
      var t = e.alternate;
      (t !== null && ((e.alternate = null), xu(t)),
        (e.child = null),
        (e.deletions = null),
        (e.sibling = null),
        e.tag === 5 && ((t = e.stateNode), t !== null && Nt(t)),
        (e.stateNode = null),
        (e.return = null),
        (e.dependencies = null),
        (e.memoizedProps = null),
        (e.memoizedState = null),
        (e.pendingProps = null),
        (e.stateNode = null),
        (e.updateQueue = null));
    }
    var W = null,
      Su = !1;
    function Cu(e, t, n) {
      for (n = n.child; n !== null;) (wu(e, t, n), (n = n.sibling));
    }
    function wu(e, t, n) {
      if (et && typeof et.onCommitFiberUnmount == `function`)
        try {
          et.onCommitFiberUnmount($e, n);
        } catch {}
      switch (n.tag) {
        case 26:
          (U || Ol(n, t),
            Cu(e, t, n),
            n.memoizedState
              ? n.memoizedState.count--
              : n.stateNode && !U && ((n = n.stateNode), n.parentNode.removeChild(n)));
          break;
        case 27:
          (U || Ol(n, t), jl(n));
          var r = W,
            i = Su;
          (Sp(n.type) && ((W = n.stateNode), (Su = !1)),
            Cu(e, t, n),
            gm(n.stateNode, n.type, n.memoizedProps),
            (W = r),
            (Su = i));
          break;
        case 5:
          (U || Ol(n, t), jl(n));
        case 6:
          if (
            (n.tag === 6 && jl(n),
            (r = W),
            (i = Su),
            (W = null),
            Cu(e, t, n),
            (W = r),
            (Su = i),
            W !== null)
          ) {
            if (Su)
              try {
                ((W.nodeType === 9
                  ? W.body
                  : W.nodeName === `HTML`
                    ? W.ownerDocument.body
                    : W
                ).removeChild(n.stateNode),
                  (L = !0));
              } catch (e) {
                Z(n, t, e);
              }
            else
              try {
                (W.removeChild(n.stateNode), (L = !0));
              } catch (e) {
                Z(n, t, e);
              }
          }
          break;
        case 18:
          W !== null &&
            (Su
              ? ((e = W),
                Cp(
                  e.nodeType === 9 ? e.body : e.nodeName === `HTML` ? e.ownerDocument.body : e,
                  n.stateNode,
                ),
                Hh(e))
              : Cp(W, n.stateNode));
          break;
        case 4:
          ((r = W),
            (i = Su),
            (W = n.stateNode.containerInfo),
            (Su = !0),
            Cu(e, t, n),
            (W = r),
            (Su = i));
          break;
        case 0:
        case 11:
        case 14:
        case 15:
          (wl(2, n, t), U || wl(4, n, t), Cu(e, t, n));
          break;
        case 1:
          (U ||
            (Ol(n, t),
            (r = n.stateNode),
            typeof r.componentWillUnmount == `function` && El(n, t, r)),
            Cu(e, t, n));
          break;
        case 21:
          Cu(e, t, n);
          break;
        case 22:
          ((U = (r = U) || n.memoizedState !== null), Cu(e, t, n), (U = r));
          break;
        case 30:
          (Ol(n, t), Cu(e, t, n));
          break;
        case 7:
          (U || Ol(n, t), Cu(e, t, n));
          break;
        default:
          Cu(e, t, n);
      }
    }
    function Tu(e, t) {
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
    function Eu(e, t) {
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
    function Du(e) {
      switch (e.tag) {
        case 31:
        case 13:
        case 19:
          var t = e.stateNode;
          return (t === null && (t = e.stateNode = new lu()), t);
        case 22:
          return (
            (e = e.stateNode), (t = e._retryCache), t === null && (t = e._retryCache = new lu()), t
          );
        default:
          throw Error(i(435, e.tag));
      }
    }
    function Ou(e, t) {
      var n = Du(e);
      t.forEach(function (t) {
        if (!n.has(t)) {
          n.add(t);
          var r = vf.bind(null, e, t);
          t.then(r, r);
        }
      });
    }
    function ku(e, t, n) {
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
                  ((W = l.stateNode), (Su = !1));
                  break a;
                }
                break;
              case 5:
                ((W = l.stateNode), (Su = !1));
                break a;
              case 3:
              case 4:
                ((W = l.stateNode.containerInfo), (Su = !0));
                break a;
            }
            l = l.return;
          }
          if (W === null) throw Error(i(160));
          (wu(s, c, o),
            (W = null),
            (Su = !1),
            (s = o.alternate),
            s !== null && (s.return = null),
            (o.return = null));
        }
      if (t.subtreeFlags & 13886) for (t = t.child; t !== null;) (ju(t, e, n), (t = t.sibling));
    }
    var Au = null;
    function ju(e, t, n) {
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
          (ku(t, e, n), Mu(e), a & 4 && (wl(3, e, e.return), Cl(3, e), wl(5, e, e.return)));
          break;
        case 1:
          (ku(t, e, n),
            Mu(e),
            a & 512 && (U || r === null || Ol(r, r.return)),
            a & 64 &&
              ou &&
              ((e = e.updateQueue),
              e !== null &&
                ((t = e.callbacks),
                t !== null &&
                  ((n = e.shared.hiddenCallbacks),
                  (e.shared.hiddenCallbacks = n === null ? t : n.concat(t))))));
          break;
        case 26:
          if (
            ((o = Au), ku(t, e, n), Mu(e), a & 512 && (U || r === null || Ol(r, r.return)), a & 4)
          ) {
            if (((a = r === null ? null : r.memoizedState), (n = e.memoizedState), r === null)) {
              if (n === null) {
                if (e.stateNode === null) {
                  if (ou) e.stateNode = fp(e.type, e.memoizedProps, t.containerInfo, e);
                  else {
                    a: {
                      ((t = e.type), (n = e.memoizedProps), (a = o.ownerDocument || o));
                      b: switch (t) {
                        case `title`:
                          ((r = a.getElementsByTagName(`title`)[0]),
                            (!r ||
                              r[jt] ||
                              r[Tt] ||
                              r.namespaceURI === `http://www.w3.org/2000/svg` ||
                              r.hasAttribute(`itemprop`)) &&
                              ((r = a.createElement(t)),
                              a.head.insertBefore(r, a.querySelector(`head > title`))),
                            np(r, t, n),
                            (r[Tt] = e),
                            Rt(r),
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
                      ((r[Tt] = e), Rt(r), (t = r));
                    }
                    e.stateNode = t;
                  }
                } else ou || Km(o, e.type, e.stateNode);
              } else e.stateNode = Bm(o, n, e.memoizedProps);
            } else
              a === n
                ? n === null && e.stateNode !== null && Fl(e, e.memoizedProps, r.memoizedProps)
                : (a === null
                    ? ((t = r.stateNode), t === null || U || t.parentNode.removeChild(t))
                    : a.count--,
                  n === null ? ou || Km(o, e.type, e.stateNode) : Bm(o, n, e.memoizedProps));
          }
          break;
        case 27:
          (ku(t, e, n),
            Mu(e),
            a & 512 && (U || r === null || Ol(r, r.return)),
            r !== null && a & 4 && Fl(e, e.memoizedProps, r.memoizedProps));
          break;
        case 5:
          if (
            ((o = su),
            (su = !1),
            ku(t, e, n),
            (su = o),
            Mu(e),
            a & 512 && (U || r === null || Ol(r, r.return)),
            e.flags & 32)
          ) {
            t = e.stateNode;
            try {
              (fn(t, ``), (L = !0));
            } catch (t) {
              Z(e, e.return, t);
            }
          }
          (a & 4 &&
            e.stateNode != null &&
            ((t = e.memoizedProps), Fl(e, t, r === null ? t : r.memoizedProps)),
            a & 1024 && (cu = !0));
          break;
        case 6:
          if ((ku(t, e, n), Mu(e), a & 4)) {
            if (e.stateNode === null) throw Error(i(162));
            ((t = e.memoizedProps), (n = e.stateNode));
            try {
              ((n.nodeValue = t), (L = !0));
            } catch (t) {
              Z(e, e.return, t);
            }
          }
          break;
        case 3:
          if (
            ((L = !1),
            (Wm = null),
            (o = Au),
            (Au = bm(t.containerInfo)),
            ku(t, e, n),
            (Au = o),
            Mu(e),
            a & 4 && r !== null && r.memoizedState.isDehydrated)
          )
            try {
              Hh(t.containerInfo);
            } catch (t) {
              Z(e, e.return, t);
            }
          (cu && ((cu = !1), Nu(e)), (L = !1));
          break;
        case 4:
          ((a = su),
            (su = ou),
            (r = Jt()),
            (o = Au),
            (Au = bm(e.stateNode.containerInfo)),
            ku(t, e, n),
            Mu(e),
            (Au = o),
            L && fu && (pu = !0),
            (L = r),
            (su = a));
          break;
        case 12:
          (ku(t, e, n), Mu(e));
          break;
        case 31:
          (ku(t, e, n),
            Mu(e),
            a & 4 && ((t = e.updateQueue), t !== null && ((e.updateQueue = null), Ou(e, t))));
          break;
        case 13:
          (ku(t, e, n),
            Mu(e),
            e.child.flags & 8192 &&
              (e.memoizedState !== null) != (r !== null && r.memoizedState !== null) &&
              (hd = We()),
            a & 4 && ((t = e.updateQueue), t !== null && ((e.updateQueue = null), Ou(e, t))));
          break;
        case 22:
          ((o = e.memoizedState !== null), (s = r !== null && r.memoizedState !== null));
          var c = ou,
            l = U,
            u = su;
          ((ou = c || o),
            (su = u || o),
            (U = l || s),
            ku(t, e, n),
            (U = l),
            (su = u),
            (ou = c),
            Mu(e),
            a & 8192 &&
              ((t = e.stateNode),
              (t._visibility = o ? t._visibility & -2 : t._visibility | 1),
              !o ||
                r === null ||
                s ||
                ou ||
                U ||
                ((t = s || U),
                (n = ou),
                (r = U),
                (ou = o || ou),
                (U = t),
                Lu(e, 2),
                (ou = n),
                (U = r)),
              (!o && su) || vu(e, o)),
            a & 4 &&
              ((t = e.updateQueue),
              t !== null && ((n = t.retryQueue), n !== null && ((t.retryQueue = null), Ou(e, n)))));
          break;
        case 19:
          (ku(t, e, n),
            Mu(e),
            a & 4 && ((t = e.updateQueue), t !== null && ((e.updateQueue = null), Ou(e, t))));
          break;
        case 30:
          (a & 512 && (U || r === null || Ol(r, r.return)),
            (a = Jt()),
            (o = fu),
            (s = (n & 335544064) === n),
            (c = e.memoizedProps),
            (fu = s && xi(c.default, c.update) !== `none`),
            ku(t, e, n),
            Mu(e),
            s && r !== null && L && (e.flags |= 4),
            (fu = o),
            (L = a));
          break;
        case 21:
          break;
        case 7:
          (a & 512 && (U || r === null || Ol(r, r.return)),
            r && r.stateNode !== null && (r.stateNode._fragmentFiber = e));
        default:
          (ku(t, e, n), Mu(e));
      }
    }
    function Mu(e) {
      var t = e.flags;
      if (t & 2) {
        try {
          for (var n, r = e.return; r !== null;) {
            if (Il(r)) {
              n = r;
              break;
            }
            r = r.return;
          }
          r = null;
          for (var a = e.return; a !== null;) {
            if (Nl(a)) {
              var o = a.stateNode;
              r === null ? (r = [o]) : r.push(o);
            }
            if (Ml(a)) break;
            a = a.return;
          }
          var s = r;
          if (n == null) throw Error(i(160));
          switch (n.tag) {
            case 27:
              var c = n.stateNode;
              zl(e, Ll(e), c, s);
              break;
            case 5:
              var l = n.stateNode;
              (n.flags & 32 && (fn(l, ``), (n.flags &= -33)), zl(e, Ll(e), l, s));
              break;
            case 3:
            case 4:
              var u = n.stateNode.containerInfo;
              Rl(e, Ll(e), u, s);
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
    function Nu(e) {
      if (e.subtreeFlags & 1024)
        for (e = e.child; e !== null;) {
          var t = e;
          (Nu(t),
            t.tag === 5 && t.flags & 1024 && ((t = t.stateNode), (gh = !0), t.reset(), (gh = !1)),
            (e = e.sibling));
        }
    }
    function Pu(e, t) {
      if (t.subtreeFlags & 9270) for (t = t.child; t !== null;) (Fu(t, e), (t = t.sibling));
      else au(t, !1);
    }
    function Fu(e, t) {
      var n = e.alternate;
      if (n === null) Zl(e, !1);
      else
        switch (e.tag) {
          case 3:
            if (((mu = du = !1), Gl(), Pu(t, e), !du && !pu)) {
              if (((e = Wl), e !== null))
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
                (mu = !0));
            }
            Wl = null;
            break;
          case 5:
            Pu(t, e);
            break;
          case 4:
            ((r = du), (du = !1), Pu(t, e), du && (pu = !0), (du = r));
            break;
          case 22:
            e.memoizedState === null && (n.memoizedState === null ? Pu(t, e) : Zl(e, !1));
            break;
          case 30:
            ((r = du), (i = Gl()), (du = !1), Pu(t, e), du && (e.flags |= 4));
            var a = e.memoizedProps,
              o = e.stateNode;
            ((t = yi(a, o)), (o = yi(n.memoizedProps, o)));
            var s = xi(a.default, a.update);
            (s === `none`
              ? (t = !1)
              : ((a = n.memoizedState),
                (n.memoizedState = null),
                (n = e.child),
                (Kl = 0),
                (t = iu(e, n, t, o, s, a, !0)),
                Kl !== (a === null ? 0 : a.length) && (e.flags |= 32)),
              e.flags & 4 && t
                ? (Nd(e, e.memoizedProps.onUpdate), (Wl = i))
                : i !== null && (i.push.apply(i, Wl), (Wl = i)),
              (du = e.flags & 32 ? !0 : r));
            break;
          default:
            Pu(t, e);
        }
    }
    function Iu(e, t) {
      if (t.subtreeFlags & 8772)
        for (t = t.child; t !== null;) (_u(e, t.alternate, t), (t = t.sibling));
    }
    function Lu(e, t) {
      for (e = e.child; e !== null;) {
        var n = e,
          r = t;
        switch (n.tag) {
          case 0:
          case 11:
          case 14:
          case 15:
            (wl(4, n, n.return), Lu(n, r));
            break;
          case 1:
            Ol(n, n.return);
            var i = n.stateNode;
            (typeof i.componentWillUnmount == `function` && El(n, n.return, i), Lu(n, r));
            break;
          case 27:
            r & 2 && gm(n.stateNode, n.type, n.memoizedProps);
          case 5:
            (Ol(n, n.return), (n.tag !== 5 && n.tag !== 27) || jl(n), Lu(n, r));
            break;
          case 6:
            jl(n);
            break;
          case 26:
            (Ol(n, n.return),
              (i = n.stateNode),
              n.memoizedState !== null || i === null || U || i.parentNode.removeChild(i),
              Lu(n, r));
            break;
          case 22:
            n.memoizedState === null && Lu(n, r);
            break;
          case 30:
            (Ol(n, n.return), Lu(n, r));
            break;
          case 7:
            Ol(n, n.return);
          default:
            Lu(n, r);
        }
        e = e.sibling;
      }
    }
    function Ru(e, t, n) {
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
            (Ru(i, a, n), Cl(4, a));
            break;
          case 1:
            if ((Ru(i, a, n), (r = a), (i = r.stateNode), typeof i.componentDidMount == `function`))
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
                  for (i.shared.hiddenCallbacks = null, i = 0; i < l.length; i++) To(l[i], c);
              } catch (e) {
                Z(r, r.return, e);
              }
            }
            (s && o & 64 && Tl(a), Dl(a, a.return));
            break;
          case 27:
            n & 2 && Bl(a);
          case 5:
            ((a.tag !== 5 && a.tag !== 27) || Al(a),
              Ru(i, a, n),
              s && r === null && o & 4 && Pl(a),
              Dl(a, a.return));
            break;
          case 6:
            Al(a);
            break;
          case 26:
            ((c = a.stateNode),
              a.memoizedState !== null || c === null || ou || Km(bm(c.ownerDocument), a.type, c),
              Ru(i, a, n),
              s && r === null && o & 4 && Pl(a),
              Dl(a, a.return));
            break;
          case 12:
            Ru(i, a, n);
            break;
          case 31:
            (Ru(i, a, n), s && o & 4 && Tu(i, a));
            break;
          case 13:
            (Ru(i, a, n), s && o & 4 && Eu(i, a));
            break;
          case 22:
            (a.memoizedState === null && Ru(i, a, n), Dl(a, a.return));
            break;
          case 30:
            (Ru(i, a, n), Dl(a, a.return));
            break;
          case 7:
            Dl(a, a.return);
          default:
            Ru(i, a, n);
        }
        t = t.sibling;
      }
    }
    function zu(e, t) {
      var n = null;
      (e !== null &&
        e.memoizedState !== null &&
        e.memoizedState.cachePool !== null &&
        (n = e.memoizedState.cachePool.pool),
        (e = null),
        t.memoizedState !== null &&
          t.memoizedState.cachePool !== null &&
          (e = t.memoizedState.cachePool.pool),
        e !== n && (e != null && e.refCount++, n != null && Pa(n)));
    }
    function Bu(e, t) {
      ((e = null),
        t.alternate !== null && (e = t.alternate.memoizedState.cache),
        (t = t.memoizedState.cache),
        t !== e && (t.refCount++, e != null && Pa(e)));
    }
    function Vu(e, t, n, r) {
      var i = (n & 335544064) === n;
      if (t.subtreeFlags & (i ? 10262 : 10256))
        for (t = t.child; t !== null;) (Hu(e, t, n, r), (t = t.sibling));
      else i && ru(t);
    }
    function Hu(e, t, n, r) {
      var i = (n & 335544064) === n;
      i && t.alternate === null && t.return !== null && t.return.alternate !== null && nu(t);
      var a = t.flags;
      switch (t.tag) {
        case 0:
        case 11:
        case 15:
          (Vu(e, t, n, r), a & 2048 && Cl(9, t));
          break;
        case 1:
          Vu(e, t, n, r);
          break;
        case 3:
          (Vu(e, t, n, r),
            i &&
              mu &&
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
              t !== a && (t.refCount++, a != null && Pa(a))));
          break;
        case 12:
          if (a & 2048) {
            (Vu(e, t, n, r), (a = t.stateNode));
            try {
              var o = t.memoizedProps,
                s = o.id,
                c = o.onPostCommit;
              typeof c == `function` &&
                c(s, t.alternate === null ? `mount` : `update`, a.passiveEffectDuration, -0);
            } catch (e) {
              Z(t, t.return, e);
            }
          } else Vu(e, t, n, r);
          break;
        case 31:
          Vu(e, t, n, r);
          break;
        case 13:
          Vu(e, t, n, r);
          break;
        case 23:
          break;
        case 22:
          ((o = t.stateNode),
            (s = t.alternate),
            t.memoizedState === null
              ? (i && s !== null && s.memoizedState !== null && nu(t),
                o._visibility & 2
                  ? Vu(e, t, n, r)
                  : ((o._visibility |= 2), Uu(e, t, n, r, !!(t.subtreeFlags & 10256) || !1)))
              : (i && s !== null && s.memoizedState === null && nu(s),
                o._visibility & 2 ? Vu(e, t, n, r) : Wu(e, t)),
            a & 2048 && zu(s, t));
          break;
        case 24:
          (Vu(e, t, n, r), a & 2048 && Bu(t.alternate, t));
          break;
        case 30:
          (i && ((a = t.alternate), a !== null && (Yl(a.child, !0), Yl(t.child, !0))),
            Vu(e, t, n, r));
          break;
        default:
          Vu(e, t, n, r);
      }
    }
    function Uu(e, t, n, r, i) {
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
            (Uu(a, o, s, c, i), Cl(8, o));
            break;
          case 23:
            break;
          case 22:
            var u = o.stateNode;
            (o.memoizedState === null
              ? ((u._visibility |= 2), Uu(a, o, s, c, i))
              : u._visibility & 2
                ? Uu(a, o, s, c, i)
                : Wu(a, o),
              i && l & 2048 && zu(o.alternate, o));
            break;
          case 24:
            (Uu(a, o, s, c, i), i && l & 2048 && Bu(o.alternate, o));
            break;
          default:
            Uu(a, o, s, c, i);
        }
        t = t.sibling;
      }
    }
    function Wu(e, t) {
      if (t.subtreeFlags & 10256)
        for (t = t.child; t !== null;) {
          var n = e,
            r = t,
            i = r.flags;
          switch (r.tag) {
            case 22:
              (Wu(n, r), i & 2048 && zu(r.alternate, r));
              break;
            case 24:
              (Wu(n, r), i & 2048 && Bu(r.alternate, r));
              break;
            default:
              Wu(n, r);
          }
          t = t.sibling;
        }
    }
    var Gu = 8192;
    function Ku(e, t, n) {
      if (e.subtreeFlags & Gu) for (e = e.child; e !== null;) (qu(e, t, n), (e = e.sibling));
    }
    function qu(e, t, n) {
      switch (e.tag) {
        case 26:
          (Ku(e, t, n),
            e.flags & Gu &&
              (e.memoizedState === null
                ? ((e = e.stateNode), (t & 335544128) === t && Zm(n, e))
                : Qm(n, Au, e.memoizedState, e.memoizedProps)));
          break;
        case 5:
          (Ku(e, t, n), e.flags & Gu && ((e = e.stateNode), (t & 335544128) === t && Zm(n, e)));
          break;
        case 3:
        case 4:
          var r = Au;
          ((Au = bm(e.stateNode.containerInfo)), Ku(e, t, n), (Au = r));
          break;
        case 22:
          e.memoizedState === null &&
            ((r = e.alternate),
            r !== null && r.memoizedState !== null
              ? ((r = Gu), (Gu = 16777216), Ku(e, t, n), (Gu = r))
              : Ku(e, t, n));
          break;
        case 30:
          if ((e.flags & Gu) !== 0 && ((r = e.memoizedProps.name), r != null && r !== `auto`)) {
            var i = e.stateNode;
            ((i.paired = null), Hl === null && (Hl = new Map()), Hl.set(r, i));
          }
          Ku(e, t, n);
          break;
        default:
          Ku(e, t, n);
      }
    }
    function Ju(e) {
      var t = e.alternate;
      if (t !== null && ((e = t.child), e !== null)) {
        t.child = null;
        do ((t = e.sibling), (e.sibling = null), (e = t));
        while (e !== null);
      }
    }
    function Yu(e) {
      var t = e.deletions;
      if (e.flags & 16) {
        if (t !== null)
          for (var n = 0; n < t.length; n++) {
            var r = t[n];
            ((uu = r), Qu(r, e));
          }
        Ju(e);
      }
      if (e.subtreeFlags & 10256) for (e = e.child; e !== null;) (Xu(e), (e = e.sibling));
    }
    function Xu(e) {
      switch (e.tag) {
        case 0:
        case 11:
        case 15:
          (Yu(e), e.flags & 2048 && wl(9, e, e.return));
          break;
        case 3:
          Yu(e);
          break;
        case 12:
          Yu(e);
          break;
        case 22:
          var t = e.stateNode;
          e.memoizedState !== null &&
          t._visibility & 2 &&
          (e.return === null || e.return.tag !== 13)
            ? ((t._visibility &= -3), Zu(e))
            : Yu(e);
          break;
        default:
          Yu(e);
      }
    }
    function Zu(e) {
      var t = e.deletions;
      if (e.flags & 16) {
        if (t !== null)
          for (var n = 0; n < t.length; n++) {
            var r = t[n];
            ((uu = r), Qu(r, e));
          }
        Ju(e);
      }
      for (e = e.child; e !== null;) {
        switch (((t = e), t.tag)) {
          case 0:
          case 11:
          case 15:
            (wl(8, t, t.return), Zu(t));
            break;
          case 22:
            ((n = t.stateNode), n._visibility & 2 && ((n._visibility &= -3), Zu(t)));
            break;
          default:
            Zu(t);
        }
        e = e.sibling;
      }
    }
    function Qu(e, t) {
      for (; uu !== null;) {
        var n = uu;
        switch (n.tag) {
          case 0:
          case 11:
          case 15:
            wl(8, n, t);
            break;
          case 23:
          case 22:
            if (n.memoizedState !== null && n.memoizedState.cachePool !== null) {
              var r = n.memoizedState.cachePool.pool;
              r != null && r.refCount++;
            }
            break;
          case 24:
            Pa(n.memoizedState.cache);
        }
        if (((r = n.child), r !== null)) ((r.return = n), (uu = r));
        else
          a: for (n = e; uu !== null;) {
            r = uu;
            var i = r.sibling,
              a = r.return;
            if ((xu(r), r === n)) {
              uu = null;
              break a;
            }
            if (i !== null) {
              ((i.return = a), (uu = i));
              break a;
            }
            uu = a;
          }
      }
    }
    var $u = {
        getCacheForType: function (e) {
          var t = Ea(Ma),
            n = t.data.get(e);
          return (n === void 0 && ((n = e()), t.data.set(e, n)), n);
        },
        cacheSignal: function () {
          return Ea(Ma).controller.signal;
        },
      },
      ed = typeof WeakMap == `function` ? WeakMap : Map,
      G = 0,
      K = null,
      q = null,
      J = 0,
      Y = 0,
      td = null,
      nd = !1,
      rd = !1,
      id = !1,
      ad = 0,
      od = 0,
      sd = 0,
      cd = 0,
      ld = 0,
      ud = 0,
      dd = 0,
      fd = null,
      pd = null,
      md = !1,
      hd = 0,
      gd = 0,
      _d = 1 / 0,
      vd = null,
      yd = null,
      X = 0,
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
      return G & 2 && J !== 0 ? J & -J : M.T === null ? St() : Pf();
    }
    function Md() {
      if (ud === 0) {
        if (!(J & 536870912) || z) {
          var e = st;
          ((st <<= 1), !(st & 3932160) && (st = 262144), (ud = e));
        } else ud = 536870912;
      }
      return ((e = Mo.current), e !== null && (e.flags |= 32), ud);
    }
    function Nd(e, t) {
      if (t != null) {
        var n = e.stateNode,
          r = n.ref;
        (r === null && (r = n.ref = Pp(yi(e.memoizedProps, n))),
          Dd === null && (Dd = []),
          Dd.push(t.bind(null, r)));
      }
    }
    function Pd(e, t, n) {
      (((e === K && (Y === 2 || Y === 9)) || e.cancelPendingCommit !== null) &&
        (Vd(e, 0), Rd(e, J, ud, !1)),
        ht(e, n),
        (!(G & 2) || e !== K) &&
          (e === K && (!(G & 2) && (cd |= n), od === 4 && Rd(e, J, ud, !1)), Ef(e)));
    }
    function Fd(e, t, n) {
      if (G & 6) throw Error(i(327));
      var r = (!n && !(t & 127) && (t & e.expiredLanes) === 0) || ut(e, t),
        a = r ? Yd(e, t) : qd(e, t, !0),
        o = r;
      do {
        if (a === 0) {
          rd && !r && Rd(e, t, 0, !1);
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
                a = fd;
                var l = c.current.memoizedState.isDehydrated;
                if ((l && (Vd(c, s).flags |= 256), (s = qd(c, s, !1)), s !== 2 && s !== 6)) {
                  if (id && !l) {
                    ((c.errorRecoveryDisabledLanes |= o), (cd |= o), (a = 4));
                    break a;
                  }
                  ((o = pd),
                    (pd = a),
                    o !== null && (pd === null ? (pd = o) : pd.push.apply(pd, o)));
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
                Rd(r, t, ud, !nd);
                break a;
              case 2:
                pd = null;
                break;
              case 3:
              case 5:
                break;
              default:
                throw Error(i(329));
            }
            if ((t & 62914560) === t && ((a = hd + 300 - We()), 10 < a)) {
              if ((Rd(r, t, ud, !nd), lt(r, 0, !0) !== 0)) break a;
              ((Sd = t),
                (r.timeoutHandle = gp(
                  Id.bind(null, r, n, pd, vd, md, t, ud, cd, dd, nd, o, `Throttled`, -0, 0),
                  a,
                )));
              break a;
            }
            Id(r, n, pd, vd, md, t, ud, cd, dd, nd, o, null, -0, 0);
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
          unsuspend: bn,
        }),
        (Hl = null),
        qu(t, a, d),
        h &&
          ((m = d),
          (h = e.containerInfo),
          (h = (h.nodeType === 9 ? h : h.ownerDocument).__reactViewTransition),
          h != null &&
            (m.count++,
            (m.waitingForViewTransition = !0),
            (m = nh.bind(m)),
            h.finished.then(m, m))),
        (m = (a & 62914560) === a ? hd - We() : (a & 4194048) === a ? gd - We() : 0),
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
              if (!Ur(a(), i)) return !1;
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
      ((t = dt(e, t)),
        (t &= ~ld),
        (t &= ~cd),
        (e.suspendedLanes |= t),
        (e.pingedLanes &= ~t),
        r && (e.warmLanes |= t),
        (r = e.expirationTimes));
      for (var i = t; 0 < i;) {
        var a = 31 - nt(i),
          o = 1 << a;
        ((r[a] = -1), (i &= ~o));
      }
      n !== 0 && _t(e, n, t);
    }
    function zd() {
      return G & 6 ? !0 : (Df(0, !1), !1);
    }
    function Bd() {
      if (q !== null) {
        if (Y === 0) var e = q.return;
        else ((e = q), (va = _a = null), os(e), (oo = null), (so = 0), (e = q));
        for (; e !== null;) (Sl(e.alternate, e), (e = e.return));
        q = null;
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
        (K = e),
        (q = n = Ii(e.current, null)),
        (J = t),
        (Y = 0),
        (td = null),
        (nd = !1),
        (rd = ut(e, t)),
        (id = !1),
        (dd = ud = ld = cd = sd = od = 0),
        (pd = fd = null),
        (md = !1),
        (ad = dt(e, t)),
        Ei(),
        n
      );
    }
    function Hd(e, t) {
      ((B = null),
        (M.H = gc),
        t === Xa || t === Qa
          ? ((t = io()), (Y = 3))
          : t === Za
            ? ((t = io()), (Y = 4))
            : (Y = t === Pc ? 8 : typeof t == `object` && t && typeof t.then == `function` ? 6 : 1),
        (td = t),
        q === null && ((od = 1), Oc(e, Wi(t, e.current))));
    }
    function Ud() {
      var e = Mo.current;
      return e === null
        ? !0
        : (J & 4194048) === J
          ? No === null
          : (J & 62914560) === J || J & 536870912
            ? e === No
            : !1;
    }
    function Wd() {
      var e = M.H;
      return ((M.H = gc), e === null ? gc : e);
    }
    function Gd() {
      var e = M.A;
      return ((M.A = $u), e);
    }
    function Kd() {
      ((od = 4),
        nd || ((J & 4194048) !== J && Mo.current !== null) || (rd = !0),
        (!(sd & 134217727) && !(cd & 134217727)) || K === null || Rd(K, J, ud, !1));
    }
    function qd(e, t, n) {
      var r = G;
      G |= 2;
      var i = Wd(),
        a = Gd();
      ((K !== e || J !== t) && ((vd = null), Vd(e, t)), (t = !1));
      var o = od;
      a: do
        try {
          if (Y !== 0 && q !== null) {
            var s = q,
              c = td;
            switch (Y) {
              case 8:
                (Bd(), (o = 6));
                break a;
              case 3:
              case 2:
              case 9:
              case 6:
                Mo.current === null && (t = !0);
                var l = Y;
                if (((Y = 0), (td = null), $d(e, s, c, l), n && rd)) {
                  o = 0;
                  break a;
                }
                break;
              default:
                ((l = Y), (Y = 0), (td = null), $d(e, s, c, l));
            }
          }
          (Jd(), (o = od));
          break;
        } catch (t) {
          Hd(e, t);
        }
      while (1);
      return (
        t && e.shellSuspendCounter++,
        (va = _a = null),
        (G = r),
        (M.H = i),
        (M.A = a),
        q === null && ((K = null), (J = 0), Ei()),
        o
      );
    }
    function Jd() {
      for (; q !== null;) Zd(q);
    }
    function Yd(e, t) {
      var n = G;
      G |= 2;
      var r = Wd(),
        a = Gd();
      K !== e || J !== t ? ((vd = null), (_d = We() + 500), Vd(e, t)) : (rd = ut(e, t));
      a: do
        try {
          if (Y !== 0 && q !== null) {
            t = q;
            var o = td;
            b: switch (Y) {
              case 1:
                ((Y = 0), (td = null), $d(e, t, o, 1));
                break;
              case 2:
              case 9:
                if (eo(o)) {
                  ((Y = 0), (td = null), Qd(t));
                  break;
                }
                ((t = function () {
                  ((Y !== 2 && Y !== 9) || K !== e || (Y = 7), Ef(e));
                }),
                  o.then(t, t));
                break a;
              case 3:
                Y = 7;
                break a;
              case 4:
                Y = 5;
                break a;
              case 7:
                eo(o) ? ((Y = 0), (td = null), Qd(t)) : ((Y = 0), (td = null), $d(e, t, o, 7));
                break;
              case 5:
                var s = null;
                switch (q.tag) {
                  case 26:
                    s = q.memoizedState;
                  case 5:
                  case 27:
                    var c = q;
                    if (s ? Ym(s) : c.stateNode.complete) {
                      ((Y = 0), (td = null));
                      var l = c.sibling;
                      if (l !== null) q = l;
                      else {
                        var u = c.return;
                        u === null ? (q = null) : ((q = u), ef(u));
                      }
                      break b;
                    }
                }
                ((Y = 0), (td = null), $d(e, t, o, 5));
                break;
              case 6:
                ((Y = 0), (td = null), $d(e, t, o, 6));
                break;
              case 8:
                (Bd(), (od = 6));
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
        (va = _a = null),
        (M.H = r),
        (M.A = a),
        (G = n),
        q === null ? ((K = null), (J = 0), Ei(), od) : 0
      );
    }
    function Xd() {
      for (; q !== null && !He();) Zd(q);
    }
    function Zd(e) {
      var t = ml(e.alternate, e, ad);
      ((e.memoizedProps = e.pendingProps), t === null ? ef(e) : (q = t));
    }
    function Qd(e) {
      var t = e,
        n = t.alternate;
      switch (t.tag) {
        case 15:
        case 0:
          t = Jc(n, t, t.pendingProps, t.type, void 0, J);
          break;
        case 11:
          t = Jc(n, t, t.pendingProps, t.type.render, t.ref, J);
          break;
        case 5:
          os(t);
          var r = t;
          r === aa &&
            (z
              ? (da(r), r.tag === 5 && r.stateNode != null && (R = r.stateNode))
              : (da(r), (z = !0)));
        default:
          (Sl(n, t), (t = q = Li(t, ad)), (t = ml(n, t, ad)));
      }
      ((e.memoizedProps = e.pendingProps), t === null ? ef(e) : (q = t));
    }
    function $d(e, t, n, r) {
      ((va = _a = null), os(t), (oo = null), (so = 0));
      var i = t.return;
      try {
        if (Nc(e, i, t, n, J)) {
          ((od = 1), Oc(e, Wi(n, e.current)), (q = null));
          return;
        }
      } catch (t) {
        if (i !== null) throw ((q = i), t);
        ((od = 1), Oc(e, Wi(n, e.current)), (q = null));
        return;
      }
      t.flags & 32768
        ? (z || r === 1
            ? (e = !0)
            : rd || J & 536870912
              ? (e = !1)
              : ((nd = e = !0),
                (r === 2 || r === 9 || r === 3 || r === 6) &&
                  ((r = Mo.current), r !== null && r.tag === 13 && (r.flags |= 16384))),
          tf(t, e))
        : ef(t);
    }
    function ef(e) {
      var t = e;
      do {
        if (t.flags & 32768) {
          tf(t, nd);
          return;
        }
        e = t.return;
        var n = bl(t.alternate, t, ad);
        if (n !== null) {
          q = n;
          return;
        }
        if (((t = t.sibling), t !== null)) {
          q = t;
          return;
        }
        q = t = e;
      } while (t !== null);
      od === 0 && (od = 5);
    }
    function tf(e, t) {
      do {
        var n = xl(e.alternate, e);
        if (n !== null) {
          ((n.flags &= 32767), (q = n));
          return;
        }
        if (
          ((n = e.return),
          n !== null && ((n.flags |= 32768), (n.subtreeFlags = 0), (n.deletions = null)),
          !t && ((e = e.sibling), e !== null))
        ) {
          q = e;
          return;
        }
        q = e = n;
      } while (e !== null);
      ((od = 6), (q = null));
    }
    function nf(e, t, n, r, a, o, s, c, l, u, d, f) {
      e.cancelPendingCommit = null;
      do df();
      while (X !== 0);
      if (G & 6) throw Error(i(327));
      if (t !== null) {
        if (t === e.current) throw Error(i(177));
        (e === K && ((q = K = null), (J = 0)),
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
        (s |= Ti),
        gt(e, n, s, r, i, a),
        (Dd = null),
        (n & 335544064) === n ? ((Od = La(e)), (r = 10262)) : ((Od = null), (r = 10256)),
        (t.subtreeFlags & r) !== 0 || (t.flags & r) !== 0
          ? ((e.callbackNode = null),
            (e.callbackPriority = 0),
            yf(Je, function () {
              return (ff(), null);
            }))
          : ((e.callbackNode = null), (e.callbackPriority = 0)),
        (Vl = !1),
        (r = !!(t.flags & 13878)),
        t.subtreeFlags & 13878 || r)
      ) {
        ((r = M.T), (M.T = null), (i = N.p), (N.p = 2), (a = G), (G |= 4));
        try {
          hu(e, t, n);
        } finally {
          ((G = a), (N.p = i), (M.T = r));
        }
      }
      ((X = 1),
        Vl
          ? (Ed = Mp(o, e.containerInfo, Od, sf, cf, of, lf, ff, af, null, null))
          : (sf(), cf(), lf()));
    }
    function af(e) {
      if (X !== 0) {
        var t = bd.onRecoverableError;
        t(e, { componentStack: null });
      }
    }
    function of() {
      X === 3 && ((X = 0), Fu(xd, bd), (X = 4));
    }
    function sf() {
      if (X === 1) {
        X = 0;
        var e = bd,
          t = xd,
          n = Sd,
          r = !!(t.flags & 13878);
        if (t.subtreeFlags & 13878 || r) {
          ((r = M.T), (M.T = null));
          var i = N.p;
          N.p = 2;
          var a = G;
          G |= 4;
          try {
            ((fu = pu = !1), ju(t, e, n), (n = cp));
            var o = Yr(e.containerInfo),
              s = n.focusedElem,
              c = n.selectionRange;
            if (o !== s && s && s.ownerDocument && Jr(s.ownerDocument.documentElement, s)) {
              if (c !== null && Xr(s)) {
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
                    var _ = qr(s, h),
                      v = qr(s, g);
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
            ((G = a), (N.p = i), (M.T = r));
          }
        }
        ((e.current = t), (X = 2));
      }
    }
    function cf() {
      if (X === 2) {
        X = 0;
        var e = bd,
          t = xd,
          n = !!(t.flags & 8772);
        if (t.subtreeFlags & 8772 || n) {
          ((n = M.T), (M.T = null));
          var r = N.p;
          N.p = 2;
          var i = G;
          G |= 4;
          try {
            _u(e, t.alternate, t);
          } finally {
            ((G = i), (N.p = r), (M.T = n));
          }
        }
        X = 3;
      }
    }
    function lf() {
      if (X === 4 || X === 3) {
        X = 0;
        var e = Ed;
        ((Ed = null), Ue());
        var t = bd,
          n = xd,
          r = Sd,
          i = Td,
          a = (r & 335544064) === r ? 10262 : 10256;
        if (
          ((n.subtreeFlags & a) !== 0 || (n.flags & a) !== 0
            ? (X = 5)
            : ((X = 0), (xd = bd = null), uf(t, t.pendingLanes)),
          (a = t.pendingLanes),
          a === 0 && (yd = null),
          xt(r),
          (n = n.stateNode),
          et && typeof et.onCommitFiberRoot == `function`)
        )
          try {
            et.onCommitFiberRoot($e, n, void 0, (n.current.flags & 128) == 128);
          } catch {}
        if (i !== null) {
          ((n = M.T), (a = N.p), (N.p = 2), (M.T = null));
          try {
            for (var o = t.onRecoverableError, s = 0; s < i.length; s++) {
              var c = i[s];
              o(c.value, { componentStack: c.stack });
            }
          } finally {
            ((M.T = n), (N.p = a));
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
        ((t = e.pooledCache), t != null && ((e.pooledCache = null), Pa(t)));
    }
    function df() {
      return (Ed !== null && (Ed.skipTransition(), (Ed = null)), sf(), cf(), lf(), ff());
    }
    function ff() {
      if (X !== 5) return !1;
      var e = bd,
        t = Cd;
      Cd = 0;
      var n = xt(Sd),
        r = M.T,
        a = N.p;
      try {
        ((N.p = 32 > n ? 32 : n), (M.T = null), (n = wd), (wd = null));
        var o = bd,
          s = Sd;
        if (((X = 0), (xd = bd = null), (Sd = 0), G & 6)) throw Error(i(331));
        var c = G;
        if (
          ((G |= 4),
          Xu(o.current),
          Hu(o, o.current, s, n),
          (G = c),
          Df(0, !1),
          et && typeof et.onPostCommitFiberRoot == `function`)
        )
          try {
            et.onPostCommitFiberRoot($e, o);
          } catch {}
        return !0;
      } finally {
        ((N.p = a), (M.T = r), uf(e, t));
      }
    }
    function pf(e, t, n) {
      ((t = Wi(n, t)),
        (t = Ac(e.stateNode, t, 2)),
        (e = yo(e, t, 2)),
        e !== null && (ht(e, 2), Ef(e)));
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
              (typeof r.componentDidCatch == `function` && (yd === null || !yd.has(r)))
            ) {
              ((e = Wi(n, e)),
                (n = jc(2)),
                (r = yo(t, n, 2)),
                r !== null && (Mc(n, r, t, e), ht(r, 2), Ef(r)));
              break;
            }
          }
          t = t.return;
        }
    }
    function mf(e, t, n) {
      var r = e.pingCache;
      if (r === null) {
        r = e.pingCache = new ed();
        var i = new Set();
        r.set(t, i);
      } else ((i = r.get(t)), i === void 0 && ((i = new Set()), r.set(t, i)));
      i.has(n) || ((id = !0), i.add(n), (e = hf.bind(null, e, t, n)), t.then(e, e));
    }
    function hf(e, t, n) {
      var r = e.pingCache;
      (r !== null && r.delete(t),
        (e.pingedLanes |= e.suspendedLanes & n),
        (e.warmLanes &= ~n),
        K === e &&
          (J & n) === n &&
          (od === 4 || (od === 3 && (J & 62914560) === J && 300 > We() - hd)
            ? G & 2
              ? (ld |= n)
              : Vd(e, 0)
            : (ld |= n),
          dd === J && (dd = 0)),
        Ef(e));
    }
    function gf(e, t) {
      (t === 0 && (t = pt()), (e = ki(e, t)), e !== null && (ht(e, t), Ef(e)));
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
      return Be(e, t);
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
                  ((a = (1 << (31 - nt(42 | e) + 1)) - 1),
                    (a &= i & ~(o & ~s)),
                    (a = a & 201326741 ? (a & 201326741) | 1 : a ? a | 2 : 0));
                }
                a !== 0 && ((n = !0), Mf(r, a));
              } else
                ((a = J),
                  (a = lt(
                    r,
                    r === K ? a : 0,
                    r.cancelPendingCommit !== null || r.timeoutHandle !== -1,
                  )),
                  !(a & 3) || ut(r, a) || ((n = !0), Mf(r, a)));
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
      for (var t = We(), n = null, r = bf; r !== null;) {
        var i = r.next,
          a = Af(r, t);
        (a === 0
          ? ((r.next = null), n === null ? (bf = i) : (n.next = i), i === null && (xf = n))
          : ((n = r), (e !== 0 || a & 3) && (Cf = !0)),
          (r = i));
      }
      ((X !== 0 && X !== 5) || Df(e, !1), Tf !== 0 && (Tf = 0));
    }
    function Af(e, t) {
      for (
        var n = e.suspendedLanes,
          r = e.pingedLanes,
          i = e.expirationTimes,
          a = e.pendingLanes & -62914561;
        0 < a;
      ) {
        var o = 31 - nt(a),
          s = 1 << o,
          c = i[o];
        (c === -1
          ? ((s & n) === 0 || (s & r) !== 0) && (i[o] = ft(s, t))
          : c <= t && (e.expiredLanes |= s),
          (a &= ~s));
      }
      if (
        ((t = K),
        (n = J),
        (n = lt(e, e === t ? n : 0, e.cancelPendingCommit !== null || e.timeoutHandle !== -1)),
        (r = e.callbackNode),
        n === 0 || (e === t && (Y === 2 || Y === 9)) || e.cancelPendingCommit !== null)
      )
        return (
          r !== null && r !== null && Ve(r), (e.callbackNode = null), (e.callbackPriority = 0)
        );
      if (!(n & 3) || ut(e, n)) {
        if (((t = n & -n), t === e.callbackPriority)) return t;
        switch ((r !== null && Ve(r), xt(n))) {
          case 2:
          case 8:
            n = qe;
            break;
          case 32:
            n = Je;
            break;
          case 268435456:
            n = Xe;
            break;
          default:
            n = Je;
        }
        return (
          (r = jf.bind(null, e)), (n = Be(n, r)), (e.callbackPriority = t), (e.callbackNode = n), t
        );
      }
      return (
        r !== null && r !== null && Ve(r), (e.callbackPriority = 2), (e.callbackNode = null), 2
      );
    }
    function jf(e, t) {
      if (X !== 0 && X !== 5) return ((e.callbackNode = null), (e.callbackPriority = 0), null);
      var n = e.callbackNode;
      if (df() && e.callbackNode !== n) return null;
      var r = J;
      return (
        (r = lt(e, e === K ? r : 0, e.cancelPendingCommit !== null || e.timeoutHandle !== -1)),
        r === 0
          ? null
          : (Fd(e, r, t),
            Af(e, We()),
            e.callbackNode != null && e.callbackNode === n ? jf.bind(null, e) : null)
      );
    }
    function Mf(e, t) {
      if (df()) return null;
      Fd(e, t, !0);
    }
    function Nf() {
      bp(function () {
        G & 6 ? Be(Ke, Of) : kf();
      });
    }
    function Pf() {
      if (Tf === 0) {
        var e = Ba;
        (e === 0 && ((e = ot), (ot <<= 1), !(ot & 261888) && (ot = 256)), (Tf = e));
      }
      return Tf;
    }
    function Ff(e) {
      return e == null || typeof e == `symbol` || typeof e == `boolean`
        ? null
        : typeof e == `function`
          ? e
          : yn(e);
    }
    function If(e, t, n, r, i) {
      if (t === `submit` && n && n.stateNode === i) {
        var a = Ff((i[Et] || null).action),
          o = r.submitter;
        o &&
          ((t = (t = o[Et] || null) ? Ff(t.formAction) : o.getAttribute(`formAction`)),
          t !== null && ((a = t), (o = null)));
        var s = new Vn(`action`, `action`, null, r, i);
        e.push({
          event: s,
          listeners: [
            {
              instance: null,
              listener: function () {
                if (r.defaultPrevented) {
                  if (Tf !== 0) {
                    var e = new FormData(i, o);
                    nc(n, { pending: !0, data: e, method: i.method, action: a }, null, e);
                  }
                } else
                  typeof a == `function` &&
                    (s.preventDefault(),
                    (e = new FormData(i, o)),
                    nc(n, { pending: !0, data: e, method: i.method, action: a }, a, e));
              },
              currentTarget: i,
            },
          ],
        });
      }
    }
    for (var Lf = 0; Lf < gi.length; Lf++) {
      var Rf = gi[Lf];
      _i(Rf.toLowerCase(), `on` + (Rf[0].toUpperCase() + Rf.slice(1)));
    }
    (_i(ci, `onAnimationEnd`),
      _i(li, `onAnimationIteration`),
      _i(ui, `onAnimationStart`),
      _i(`dblclick`, `onDoubleClick`),
      _i(`focusin`, `onFocus`),
      _i(`focusout`, `onBlur`),
      _i(di, `onTransitionRun`),
      _i(fi, `onTransitionStart`),
      _i(pi, `onTransitionCancel`),
      _i(mi, `onTransitionEnd`),
      Ut(`onMouseEnter`, [`mouseout`, `mouseover`]),
      Ut(`onMouseLeave`, [`mouseout`, `mouseover`]),
      Ut(`onPointerEnter`, [`pointerout`, `pointerover`]),
      Ut(`onPointerLeave`, [`pointerout`, `pointerover`]),
      Ht(
        `onChange`,
        `change click focusin focusout input keydown keyup selectionchange`.split(` `),
      ),
      Ht(
        `onSelect`,
        `focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange`.split(
          ` `,
        ),
      ),
      Ht(`onBeforeInput`, [`compositionend`, `keypress`, `textInput`, `paste`]),
      Ht(`onCompositionEnd`, `compositionend focusout keydown keypress keyup mousedown`.split(` `)),
      Ht(
        `onCompositionStart`,
        `compositionstart focusout keydown keypress keyup mousedown`.split(` `),
      ),
      Ht(
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
                Si(e);
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
                Si(e);
              }
              ((i.currentTarget = null), (a = c));
            }
        }
      }
    }
    function Q(e, t) {
      var n = t[Ot];
      n === void 0 && (n = t[Ot] = new Set());
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
          Bt.forEach(function (t) {
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
        !An || (t !== `touchstart` && t !== `touchmove` && t !== `wheel`) || (i = !0),
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
              if (((s = Pt(c)), s === null)) return;
              if (((l = s.tag), l === 5 || l === 6 || l === 26 || l === 27)) {
                r = a = s;
                continue a;
              }
              c = c.parentNode;
            }
          }
          r = r.return;
        }
      Dn(function () {
        var r = a,
          i = Sn(n),
          s = [];
        a: {
          var c = hi.get(e);
          if (c !== void 0) {
            var l = Vn,
              u = e;
            switch (e) {
              case `keypress`:
                if (In(n) === 0) break a;
              case `keydown`:
              case `keyup`:
                l = ar;
                break;
              case `focusin`:
                ((u = `focus`), (l = Xn));
                break;
              case `focusout`:
                ((u = `blur`), (l = Xn));
                break;
              case `beforeblur`:
              case `afterblur`:
                l = Xn;
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
                l = Jn;
                break;
              case `drag`:
              case `dragend`:
              case `dragenter`:
              case `dragexit`:
              case `dragleave`:
              case `dragover`:
              case `dragstart`:
              case `drop`:
                l = Yn;
                break;
              case `touchcancel`:
              case `touchend`:
              case `touchmove`:
              case `touchstart`:
                l = cr;
                break;
              case ci:
              case li:
              case ui:
                l = Zn;
                break;
              case mi:
                l = lr;
                break;
              case `scroll`:
              case `scrollend`:
                l = Un;
                break;
              case `wheel`:
                l = ur;
                break;
              case `copy`:
              case `cut`:
              case `paste`:
                l = Qn;
                break;
              case `gotpointercapture`:
              case `lostpointercapture`:
              case `pointercancel`:
              case `pointerdown`:
              case `pointermove`:
              case `pointerout`:
              case `pointerover`:
              case `pointerup`:
                l = or;
                break;
              case `submit`:
                l = sr;
                break;
              case `toggle`:
              case `beforetoggle`:
                l = dr;
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
                  ((g = On(m, p)), g != null && d.push(qf(m, g, h))),
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
              l && n !== xn && (u = n.relatedTarget || n.fromElement) && (Pt(u) || u[Dt]))
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
                  (l = l ? Pt(l) : null),
                  l !== null &&
                    ((f = o(l)), (d = l.tag), l !== f || (d !== 5 && d !== 27 && d !== 6)) &&
                    (l = null))
                : ((c = null), (l = r)),
              c !== l &&
                ((d = Jn),
                (g = `onMouseLeave`),
                (p = `onMouseEnter`),
                (m = `mouse`),
                (e === `pointerout` || e === `pointerover`) &&
                  ((d = or), (g = `onPointerLeave`), (p = `onPointerEnter`), (m = `pointer`)),
                (f = c == null ? u : It(c)),
                (h = l == null ? u : It(l)),
                (u = new d(g, m + `leave`, c, n, i)),
                (u.target = f),
                (u.relatedTarget = h),
                (g = null),
                Pt(i) === r &&
                  ((d = new d(p, m + `enter`, l, n, i)),
                  (d.target = h),
                  (d.relatedTarget = f),
                  (g = d)),
                (f = g),
                (d = c && l ? ne(c, l, Yf) : null),
                c !== null && Xf(s, u, c, d, !1),
                l !== null && f !== null && Xf(s, f, l, d, !0)));
          }
          a: {
            if (
              ((c = r ? It(r) : window),
              (l = c.nodeName && c.nodeName.toLowerCase()),
              l === `select` || (l === `input` && c.type === `file`))
            )
              var _ = jr;
            else if (Tr(c)) {
              if (Mr) _ = Vr;
              else {
                _ = zr;
                var v = Rr;
              }
            } else
              ((l = c.nodeName),
                !l || l.toLowerCase() !== `input` || (c.type !== `checkbox` && c.type !== `radio`)
                  ? r && gn(r.elementType) && (_ = jr)
                  : (_ = Br));
            if ((_ &&= _(e, r))) {
              Er(s, _, n, i);
              break a;
            }
            v && v(e, c, r);
          }
          switch (((v = r ? It(r) : window), e)) {
            case `focusin`:
              (Tr(v) || v.contentEditable === `true`) && ((Qr = v), ($r = r), (ei = null));
              break;
            case `focusout`:
              ei = $r = Qr = null;
              break;
            case `mousedown`:
              ti = !0;
              break;
            case `contextmenu`:
            case `mouseup`:
            case `dragend`:
              ((ti = !1), ni(s, n, i));
              break;
            case `selectionchange`:
              if (Zr) break;
            case `keydown`:
            case `keyup`:
              ni(s, n, i);
          }
          var y;
          if (pr)
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
            xr
              ? yr(e, n) && (b = `onCompositionEnd`)
              : e === `keydown` && n.keyCode === 229 && (b = `onCompositionStart`);
          (b &&
            (gr &&
              n.locale !== `ko` &&
              (xr || b !== `onCompositionStart`
                ? b === `onCompositionEnd` && xr && (y = Fn())
                : ((Mn = i), (Nn = `value` in Mn ? Mn.value : Mn.textContent), (xr = !0))),
            (v = Jf(r, b)),
            0 < v.length &&
              ((b = new $n(b, e, null, n, i)),
              s.push({ event: b, listeners: v }),
              y ? (b.data = y) : ((y = br(n)), y !== null && (b.data = y)))),
            (y = hr ? Sr(e, n) : Cr(e, n)) &&
              ((b = Jf(r, `onBeforeInput`)),
              0 < b.length &&
                ((v = new $n(`onBeforeInput`, `beforeinput`, null, n, i)),
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
            ((i = On(e, n)),
            i != null && r.unshift(qf(e, i, a)),
            (i = On(e, t)),
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
            ? ((l = On(n, a)), l != null && o.unshift(qf(n, l, c)))
            : i || ((l = On(n, a)), l != null && o.push(qf(n, l, c)))),
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
          if (typeof r == `string`) t === `body` || (t === `textarea` && r === ``) || fn(e, r);
          else if (typeof r == `number` || typeof r == `bigint`) t !== `body` && fn(e, `` + r);
          else return;
          break;
        case `className`:
          Xt(e, `class`, r);
          break;
        case `tabIndex`:
          Xt(e, `tabindex`, r);
          break;
        case `dir`:
        case `role`:
        case `viewBox`:
        case `width`:
        case `height`:
          Xt(e, n, r);
          break;
        case `style`:
          hn(e, r, o);
          return;
        case `data`:
          if (t !== `object`) {
            Xt(e, `data`, r);
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
          ((r = yn(r)), e.setAttribute(n, r));
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
          ((r = yn(r)), e.setAttribute(n, r));
          break;
        case `onClick`:
          r != null && (e.onclick = bn);
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
          ((n = yn(r)), e.setAttributeNS(`http://www.w3.org/1999/xlink`, `xlink:href`, n));
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
          (Q(`beforetoggle`, e), Q(`toggle`, e), Yt(e, `popover`, r));
          break;
        case `xlinkActuate`:
          Zt(e, `http://www.w3.org/1999/xlink`, `xlink:actuate`, r);
          break;
        case `xlinkArcrole`:
          Zt(e, `http://www.w3.org/1999/xlink`, `xlink:arcrole`, r);
          break;
        case `xlinkRole`:
          Zt(e, `http://www.w3.org/1999/xlink`, `xlink:role`, r);
          break;
        case `xlinkShow`:
          Zt(e, `http://www.w3.org/1999/xlink`, `xlink:show`, r);
          break;
        case `xlinkTitle`:
          Zt(e, `http://www.w3.org/1999/xlink`, `xlink:title`, r);
          break;
        case `xlinkType`:
          Zt(e, `http://www.w3.org/1999/xlink`, `xlink:type`, r);
          break;
        case `xmlBase`:
          Zt(e, `http://www.w3.org/XML/1998/namespace`, `xml:base`, r);
          break;
        case `xmlLang`:
          Zt(e, `http://www.w3.org/XML/1998/namespace`, `xml:lang`, r);
          break;
        case `xmlSpace`:
          Zt(e, `http://www.w3.org/XML/1998/namespace`, `xml:space`, r);
          break;
        case `is`:
          Yt(e, `is`, r);
          break;
        case `innerText`:
        case `textContent`:
          return;
        default:
          if (!(2 < n.length) || (n[0] !== `o` && n[0] !== `O`) || (n[1] !== `n` && n[1] !== `N`))
            ((n = _n.get(n) || n), Yt(e, n, r));
          else return;
      }
      L = !0;
    }
    function tp(e, t, n, r, a, o) {
      switch (n) {
        case `style`:
          hn(e, r, o);
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
          if (typeof r == `string`) fn(e, r);
          else if (typeof r == `number` || typeof r == `bigint`) fn(e, `` + r);
          else return;
          break;
        case `onScroll`:
          r != null && Q(`scroll`, e);
          return;
        case `onScrollEnd`:
          r != null && Q(`scrollend`, e);
          return;
        case `onClick`:
          r != null && (e.onclick = bn);
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
          if (!Vt.hasOwnProperty(n))
            a: {
              if (
                n[0] === `o` &&
                n[1] === `n` &&
                ((a = n.endsWith(`Capture`)),
                (o = n.slice(2, a ? n.length - 7 : void 0)),
                (t = e[Et] || null),
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
              ((L = !0), n in e ? (e[n] = r) : !0 === r ? e.setAttribute(n, ``) : Yt(e, n, r));
            }
          return;
      }
      L = !0;
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
          sn(e, o, c, l, u, s, a, !1);
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
            t == null ? n != null && ln(e, !!r, n, !0) : ln(e, !!r, t, !1));
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
          dn(e, r, a, o);
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
          if (gn(t)) {
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
                  (m !== f && (L = !0), (o = m));
                  break;
                case `name`:
                  (m !== f && (L = !0), (a = m));
                  break;
                case `checked`:
                  (m !== f && (L = !0), (u = m));
                  break;
                case `defaultChecked`:
                  (m !== f && (L = !0), (d = m));
                  break;
                case `value`:
                  (m !== f && (L = !0), (s = m));
                  break;
                case `defaultValue`:
                  (m !== f && (L = !0), (c = m));
                  break;
                case `children`:
                case `dangerouslySetInnerHTML`:
                  if (m != null) throw Error(i(137, t));
                  break;
                default:
                  m !== f && $(e, t, p, m, r, f);
              }
          }
          on(e, s, c, l, u, d, o, a);
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
                  (o !== l && (L = !0), (p = o));
                  break;
                case `defaultValue`:
                  (o !== l && (L = !0), (c = o));
                  break;
                case `multiple`:
                  (o !== l && (L = !0), (s = o));
                default:
                  o !== l && $(e, t, a, o, r, l);
              }
          ((t = c),
            (n = s),
            (r = m),
            p == null
              ? !!r != !!n && (t == null ? ln(e, !!n, n ? [] : ``, !1) : ln(e, !!n, t, !0))
              : ln(e, !!n, p, !1));
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
                  (a !== o && (L = !0), (p = a));
                  break;
                case `defaultValue`:
                  (a !== o && (L = !0), (m = a));
                  break;
                case `children`:
                  break;
                case `dangerouslySetInnerHTML`:
                  if (a != null) throw Error(i(91));
                  break;
                default:
                  a !== o && $(e, t, s, a, r, o);
              }
          un(e, p, m);
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
                  (p !== m && (L = !0),
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
          if (gn(t)) {
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
      return ((n = lp(n).createElement(e)), (n[Tt] = r), (n[Et] = t), np(n, e, t), Rt(n), n);
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
              (a[jt] ||
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
        (t = typeof t == `number` ? { duration: t } : w({}, t)),
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
      var a = Pt(i);
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
            ((t = ne(n, a, C)),
            t === null ? (t = !1) : (h(t, !0, S, a, n), (a = x), (x = null), (t = a !== null))),
          t)
        : e & Node.DOCUMENT_POSITION_FOLLOWING
          ? ((t = !!a) &&
              !(t = a === r) &&
              ((t = ne(r, a, C)),
              t === null
                ? (t = !1)
                : (h(t, !0, te, a, r), (a = x), (ee = x = null), (t = a !== null))),
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
            (nm(n), Nt(n));
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
        } else if (!e[jt])
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
        e.onclick === bn && (e.onclick = null),
        Nt(e));
    }
    function _m(e) {
      for (var t = e.attributes; t.length;) e.removeAttributeNode(t[0]);
      Nt(e);
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
    var xm = N.d;
    N.d = { f: Sm, r: Cm, D: Em, C: Dm, L: Om, m: km, X: jm, S: Am, M: Mm };
    function Sm() {
      var e = xm.f(),
        t = zd();
      return e || t;
    }
    function Cm(e) {
      var t = Ft(e);
      t !== null && t.tag === 5 && t.type === `form` ? ic(t) : xm.r(e);
    }
    var wm = typeof document > `u` ? null : document;
    function Tm(e, t, n) {
      var r = wm;
      if (r && typeof t == `string` && t) {
        var i = an(t);
        ((i = `link[rel="` + e + `"][href="` + i + `"]`),
          typeof n == `string` && (i += `[crossorigin="` + n + `"]`),
          ym.has(i) ||
            (ym.add(i),
            (e = { rel: e, crossOrigin: n, href: t }),
            r.querySelector(i) === null &&
              ((t = r.createElement(`link`)), np(t, `link`, e), Rt(t), r.head.appendChild(t))));
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
        var i = `link[rel="preload"][as="` + an(t) + `"]`;
        t === `image` && n && n.imageSrcSet
          ? ((i += `[imagesrcset="` + an(n.imageSrcSet) + `"]`),
            typeof n.imageSizes == `string` && (i += `[imagesizes="` + an(n.imageSizes) + `"]`))
          : (i += `[href="` + an(e) + `"]`);
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
            ((e = w(
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
              ((o[Mt] = !0),
              (o.onload = o.onerror =
                function () {
                  zt(o);
                })),
            Rt(o),
            r.head.appendChild(o));
        }
      }
    }
    function km(e, t) {
      xm.m(e, t);
      var n = wm;
      if (n && e) {
        var r = t && typeof t.as == `string` ? t.as : `script`,
          i = `link[rel="modulepreload"][as="` + an(r) + `"][href="` + an(e) + `"]`,
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
          ((e = w({ rel: `modulepreload`, href: e }, t)), vm.set(a, e), n.querySelector(i) === null)
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
          ((r = n.createElement(`link`)), np(r, `link`, e), Rt(r), n.head.appendChild(r));
        }
      }
    }
    function Am(e, t, n) {
      xm.S(e, t, n);
      var r = wm;
      if (r && e) {
        var i = Lt(r).hoistableStyles,
          a = Pm(e);
        t ||= `default`;
        var o = i.get(a);
        if (!o) {
          var s = { loading: 0, preload: null };
          if ((o = r.querySelector(Fm(a)))) s.loading = 5;
          else {
            ((e = w({ rel: `stylesheet`, href: e, "data-precedence": t }, n)),
              (n = vm.get(a)) && Hm(e, n));
            var c = (o = r.createElement(`link`));
            (Rt(c),
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
        var r = Lt(n).hoistableScripts,
          i = Rm(e),
          a = r.get(i);
        a ||
          ((a = n.querySelector(zm(i))),
          a ||
            ((e = w({ src: e, async: !0 }, t)),
            (t = vm.get(i)) && Um(e, t),
            (a = n.createElement(`script`)),
            Rt(a),
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
        var r = Lt(n).hoistableScripts,
          i = Rm(e),
          a = r.get(i);
        a ||
          ((a = n.querySelector(zm(i))),
          a ||
            ((e = w({ src: e, async: !0, type: `module` }, t)),
            (t = vm.get(i)) && Um(e, t),
            (a = n.createElement(`script`)),
            Rt(a),
            np(a, `link`, e),
            n.head.appendChild(a)),
          (a = { type: `script`, instance: a, count: 1, state: null }),
          r.set(i, a));
      }
    }
    function Nm(e, t, n, r) {
      var a = (a = Ee.current) ? bm(a) : null;
      if (!a) throw Error(i(446));
      switch (e) {
        case `meta`:
        case `title`:
          return null;
        case `style`:
          return typeof n.precedence == `string` && typeof n.href == `string`
            ? ((n = Pm(n.href)),
              (t = Lt(a).hoistableStyles),
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
            var o = Lt(a).hoistableStyles,
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
                (t = Lt(a).hoistableScripts),
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
      return `href="` + an(e) + `"`;
    }
    function Fm(e) {
      return `link[rel="stylesheet"][` + e + `]`;
    }
    function Im(e) {
      return w({}, e, { "data-precedence": e.precedence, precedence: null });
    }
    function Lm(e, t, n, r) {
      if ((t = e.querySelector(`link[rel="preload"][as="style"][` + t + `]`))) {
        if (!0 !== t[Mt]) {
          r.loading = 1;
          return;
        }
      } else
        ((t = e.createElement(`link`)),
          (t[Mt] = !0),
          (t.onload = t.onerror = zt.bind(null, t)),
          np(t, `link`, n),
          Rt(t),
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
      return `[src="` + an(e) + `"]`;
    }
    function zm(e) {
      return `script[async]` + e;
    }
    function Bm(e, t, n) {
      if ((t.count++, t.instance === null))
        switch (t.type) {
          case `style`:
            var r = e.querySelector(`style[data-href~="` + an(n.href) + `"]`);
            if (r) return ((t.instance = r), Rt(r), r);
            var a = w({}, n, {
              "data-href": n.href,
              "data-precedence": n.precedence,
              href: null,
              precedence: null,
            });
            return (
              (r = (e.ownerDocument || e).createElement(`style`)),
              Rt(r),
              np(r, `style`, a),
              Vm(r, n.precedence, e),
              (t.instance = r)
            );
          case `stylesheet`:
            a = Pm(n.href);
            var o = e.querySelector(Fm(a));
            if (o) return ((t.state.loading |= 4), (t.instance = o), Rt(o), o);
            ((r = Im(n)),
              (a = vm.get(a)) && Hm(r, a),
              (o = (e.ownerDocument || e).createElement(`link`)),
              Rt(o));
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
                ? ((t.instance = a), Rt(a), a)
                : ((r = n),
                  (a = vm.get(o)) && ((r = w({}, n)), Um(r, a)),
                  (e = e.ownerDocument || e),
                  (a = e.createElement(`script`)),
                  Rt(a),
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
          !(a[jt] || a[Tt] || (e === `link` && a.getAttribute(`rel`) === `stylesheet`)) &&
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
              Rt(a));
            return;
          }
          ((a = t.ownerDocument || t),
            (r = Im(r)),
            (i = vm.get(i)) && Hm(r, i),
            (a = a.createElement(`link`)),
            Rt(a));
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
      $$typeof: D,
      Provider: null,
      Consumer: null,
      _currentValue: ye,
      _currentValue2: ye,
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
        (this.expirationTimes = mt(-1)),
        (this.entangledLanes =
          this.shellSuspendCounter =
          this.errorRecoveryDisabledLanes =
          this.expiredLanes =
          this.warmLanes =
          this.pingedLanes =
          this.suspendedLanes =
          this.pendingLanes =
            0),
        (this.entanglements = mt(0)),
        (this.hiddenUpdates = mt(null)),
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
        (a = Pi(3, null, null, t)),
        (e.current = a),
        (a.stateNode = e),
        (t = Na()),
        t.refCount++,
        (e.pooledCache = t),
        t.refCount++,
        (a.memoizedState = { element: r, isDehydrated: n, cache: t }),
        go(a),
        e
      );
    }
    function uh(e) {
      return e ? ((e = Mi), e) : Mi;
    }
    function dh(e, t, n, r, i, a) {
      ((i = uh(i)),
        r.context === null ? (r.context = i) : (r.pendingContext = i),
        (r = vo(t)),
        (r.payload = { element: n }),
        (a = a === void 0 ? null : a),
        a !== null && (r.callback = a),
        (n = yo(e, r, t)),
        n !== null && (Pd(n, e, t), bo(n, e, t)));
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
        var t = ki(e, 67108864);
        (t !== null && Pd(t, e, 67108864), ph(e, 67108864));
      }
    }
    function hh(e) {
      if (e.tag === 13 || e.tag === 31) {
        var t = jd();
        t = bt(t);
        var n = ki(e, t);
        (n !== null && Pd(n, e, t), ph(e, t));
      }
    }
    var gh = !0;
    function _h(e, t, n, r) {
      var i = M.T;
      M.T = null;
      var a = N.p;
      try {
        ((N.p = 2), yh(e, t, n, r));
      } finally {
        ((N.p = a), (M.T = i));
      }
    }
    function vh(e, t, n, r) {
      var i = M.T;
      M.T = null;
      var a = N.p;
      try {
        ((N.p = 8), yh(e, t, n, r));
      } finally {
        ((N.p = a), (M.T = i));
      }
    }
    function yh(e, t, n, r) {
      if (gh) {
        var i = bh(r);
        if (i === null) (Kf(e, t, r, xh, n), Mh(e, r));
        else if (Ph(i, e, t, n, r)) r.stopPropagation();
        else if ((Mh(e, r), t & 4 && -1 < jh.indexOf(e))) {
          for (; i !== null;) {
            var a = Ft(i);
            if (a !== null)
              switch (a.tag) {
                case 3:
                  if (((a = a.stateNode), a.current.memoizedState.isDehydrated)) {
                    var o = F(a.pendingLanes);
                    if (o !== 0) {
                      var s = a;
                      for (s.pendingLanes |= 2, s.entangledLanes |= 2; o;) {
                        var c = 1 << (31 - nt(o));
                        ((s.entanglements[1] |= c), (o &= ~c));
                      }
                      (Ef(a), !(G & 6) && ((_d = We() + 500), Df(0, !1)));
                    }
                  }
                  break;
                case 31:
                case 13:
                  ((s = ki(a, 2)), s !== null && Pd(s, a, 2), zd(), ph(a, 2));
              }
            if (((a = bh(r)), a === null && Kf(e, t, r, xh, n), a === i)) break;
            i = a;
          }
          i !== null && r.stopPropagation();
        } else Kf(e, t, r, null, n);
      }
    }
    function bh(e) {
      return ((e = Sn(e)), Sh(e));
    }
    var xh = null;
    function Sh(e) {
      if (((xh = null), (e = Pt(e)), e !== null)) {
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
          switch (Ge()) {
            case Ke:
              return 2;
            case qe:
              return 8;
            case Je:
            case Ye:
              return 32;
            case Xe:
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
          t !== null && ((t = Ft(t)), t !== null && mh(t)),
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
      var t = Pt(e.target);
      if (t !== null) {
        var n = o(t);
        if (n !== null) {
          if (((t = n.tag), t === 13)) {
            if (((t = s(n)), t !== null)) {
              ((e.blockedOn = t),
                Ct(e.priority, function () {
                  hh(n);
                }));
              return;
            }
          } else if (t === 31) {
            if (((t = c(n)), t !== null)) {
              ((e.blockedOn = t),
                Ct(e.priority, function () {
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
          ((xn = r), n.target.dispatchEvent(r), (xn = null));
        } else return ((t = Ft(n)), t !== null && mh(t), (e.blockedOn = n), !1);
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
            var a = Ft(n);
            a !== null &&
              (e.splice(t, 3),
              (t -= 3),
              nc(a, { pending: !0, data: i, method: n.method, action: r }, r, i));
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
            o = i[Et] || null;
          if (typeof a == `function`) o || Vh(n);
          else if (o) {
            var s = null;
            if (a && a.hasAttribute(`formAction`)) {
              if (((i = a), (o = a[Et] || null))) s = o.formAction;
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
            (dh(e.current, 2, null, e, null, null), zd(), (t[Dt] = null));
          }
        }));
    function Gh(e) {
      this._internalRoot = e;
    }
    Gh.prototype.unstable_scheduleHydration = function (e) {
      if (e) {
        var t = St();
        e = { blockedOn: null, target: e, priority: t };
        for (var n = 0; n < Ah.length && t !== 0 && t < Ah[n].priority; n++);
        (Ah.splice(n, 0, e), n === 0 && Fh(e));
      }
    };
    var Kh = n.version;
    if (Kh !== `19.3.1`) throw Error(i(527, Kh, `19.3.1`));
    N.findDOMNode = function (e) {
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
      currentDispatcherRef: M,
      reconcilerVersion: `19.3.1`,
    };
    if (typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ < `u`) {
      var Jh = __REACT_DEVTOOLS_GLOBAL_HOOK__;
      if (!Jh.isDisabled && Jh.supportsFiber)
        try {
          (($e = Jh.inject(qh)), (et = Jh));
        } catch {}
    }
    e.createRoot = function (e, t) {
      if (!a(e)) throw Error(i(299));
      var n = !1,
        r = ``,
        o = Tc,
        s = Ec,
        c = Dc;
      return (
        t != null &&
          (!0 === t.unstable_strictMode && (n = !0),
          t.identifierPrefix !== void 0 && (r = t.identifierPrefix),
          t.onUncaughtError !== void 0 && (o = t.onUncaughtError),
          t.onCaughtError !== void 0 && (s = t.onCaughtError),
          t.onRecoverableError !== void 0 && (c = t.onRecoverableError)),
        (t = lh(e, 1, !1, null, null, n, r, null, o, s, c, Uh)),
        (e[Dt] = t.current),
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
  ee = (e) => {
    let t = y.useContext(x);
    if (e) return e;
    if (!t) throw Error(`No QueryClient set, use QueryClientProvider to set one`);
    return t;
  },
  S = ({ client: e, children: t }) => (
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
  te = {
    setTimeout: (e, t) => setTimeout(e, t),
    clearTimeout: (e) => clearTimeout(e),
    setInterval: (e, t) => setInterval(e, t),
    clearInterval: (e) => clearInterval(e),
  },
  C = new (class {
    #e = te;
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
function ne(e) {
  setTimeout(e, 0);
}
var w = typeof window > `u` || `Deno` in globalThis;
function T() {}
function re(e, t) {
  return typeof e == `function` ? e(t) : e;
}
function ie(e) {
  return typeof e == `number` && e >= 0 && e !== 1 / 0;
}
function ae(e, t) {
  return Math.max(e + (t || 0) - Date.now(), 0);
}
function E(e, t) {
  return typeof e == `function` ? e(t) : e;
}
function oe(e, t) {
  let { type: n = `all`, exact: r, fetchStatus: i, predicate: a, queryKey: o, stale: s } = e;
  if (o) {
    if (r) {
      if (t.queryHash !== D(o, t.options)) return !1;
    } else if (!k(t.queryKey, o)) return !1;
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
function se(e, t) {
  let { exact: n, status: r, predicate: i, mutationKey: a } = e;
  if (a) {
    if (!t.options.mutationKey) return !1;
    if (n) {
      if (O(t.options.mutationKey) !== O(a)) return !1;
    } else if (!k(t.options.mutationKey, a)) return !1;
  }
  return !((r && t.state.status !== r) || (i && !i(t)));
}
function D(e, t) {
  return (t?.queryKeyHashFn || O)(e);
}
function O(e) {
  return JSON.stringify(e, (e, t) =>
    ue(t)
      ? Object.keys(t)
          .sort()
          .reduce((e, n) => ((e[n] = t[n]), e), {})
      : t,
  );
}
function k(e, t) {
  if (e === t) return !0;
  if (typeof e != typeof t) return !1;
  if (e && t && typeof e == `object` && typeof t == `object`) {
    if (Array.isArray(e) && Array.isArray(t)) {
      if (t.length > e.length) return !1;
      for (let n = 0; n < t.length; n++) if (!k(e[n], t[n])) return !1;
      return !0;
    }
    let n = Object.keys(t);
    for (let r of n) if (!k(e[r], t[r])) return !1;
    return !0;
  }
  return !1;
}
var ce = Object.prototype.hasOwnProperty;
function A(e, t, n = 0) {
  if (e === t) return e;
  if (n > 500) return t;
  let r = le(e) && le(t);
  if (!r && !(ue(e) && ue(t))) return t;
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
      ((s[o] = u), (r ? l < i : ce.call(e, o)) && c++);
      continue;
    }
    if (u === null || d === null || typeof u != `object` || typeof d != `object`) {
      s[o] = d;
      continue;
    }
    let f = A(u, d, n + 1);
    ((s[o] = f), f === u && c++);
  }
  return i === o && c === i ? e : s;
}
function j(e, t) {
  if (!t || Object.keys(e).length !== Object.keys(t).length) return !1;
  for (let n in e) if (e[n] !== t[n]) return !1;
  return !0;
}
function le(e) {
  return Array.isArray(e) && e.length === Object.keys(e).length;
}
function ue(e) {
  if (!de(e)) return !1;
  let t = Object.getPrototypeOf(e),
    n = t?.constructor;
  if (n === void 0) return !0;
  if (typeof n != `function`) return !1;
  let r = n.prototype;
  return !(!de(r) || !r.hasOwnProperty(`isPrototypeOf`) || t !== Object.prototype);
}
function de(e) {
  return Object.prototype.toString.call(e) === `[object Object]`;
}
function fe(e) {
  return new Promise((t) => {
    C.setTimeout(t, e);
  });
}
function pe(e, t, n) {
  return typeof n.structuralSharing == `function`
    ? n.structuralSharing(e, t)
    : n.structuralSharing === !1
      ? t
      : A(e, t);
}
function me(e, t, n = 0) {
  let r = [...e, t];
  return n && r.length > n ? r.slice(1) : r;
}
function he(e, t, n = 0) {
  let r = [t, ...e];
  return n && r.length > n ? r.slice(0, -1) : r;
}
var ge = Symbol();
function _e(e, t) {
  return !e.queryFn && t?.initialPromise
    ? () => t.initialPromise
    : !e.queryFn || e.queryFn === ge
      ? () => Promise.reject(Error(`Missing queryFn: '${e.queryHash}'`))
      : e.queryFn;
}
function ve(e, t) {
  return typeof e == `function` ? e(...t) : !!e;
}
function M(e, t, n) {
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
var N = () => w,
  ye = () => N(),
  be = class {
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
  xe = new (class extends be {
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
  Se = ne;
function Ce() {
  let e = [],
    t = 0,
    n = (e) => {
      e();
    },
    r = (e) => {
      e();
    },
    i = Se,
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
var P = Ce(),
  we = new (class extends be {
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
function Te(e) {
  return Math.min(1e3 * 2 ** e, 3e4);
}
function Ee(e) {
  return (e ?? `online`) !== `online` || we.isOnline();
}
var De = class extends Error {
  constructor(e) {
    (super(`CancelledError`), (this.revert = e?.revert), (this.silent = e?.silent));
  }
};
function Oe(e) {
  let t = !1,
    n = 0,
    r,
    i = `pending`,
    a,
    o,
    s = new Promise((e, t) => {
      ((a = e), (o = t));
    });
  s.catch(T);
  let c = () => i !== `pending`,
    l = (t) => {
      if (!c()) {
        let n = new De(t);
        (h(n), e.onCancel?.(n));
      }
    },
    u = () => {
      t = !0;
    },
    d = () => {
      t = !1;
    },
    f = () => xe.isFocused() && (e.networkMode === `always` || we.isOnline()) && e.canRun(),
    p = () => Ee(e.networkMode) && e.canRun(),
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
          let i = e.retry ?? (ye() ? 0 : 3),
            a = e.retryDelay ?? Te,
            o = typeof a == `function` ? a(n, r) : a,
            s = i === !0 || (typeof i == `number` && n < i) || (typeof i == `function` && i(n, r));
          t || !s
            ? h(r)
            : (n++,
              e.onFail?.(n, r),
              fe(o)
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
var ke = class {
  #e;
  destroy() {
    this.clearGcTimeout();
  }
  scheduleGc() {
    (this.clearGcTimeout(),
      ie(this.gcTime) &&
        (this.#e = C.setTimeout(() => {
          this.optionalRemove();
        }, this.gcTime)));
  }
  updateGcTime(e) {
    this.gcTime = Math.max(this.gcTime || 0, e ?? (ye() ? 1 / 0 : 3e5));
  }
  clearGcTimeout() {
    this.#e !== void 0 && (C.clearTimeout(this.#e), (this.#e = void 0));
  }
};
function Ae(e) {
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
              M(
                e,
                () => t.signal,
                () => (n = !0),
              );
            },
            u = _e(t.options, t.fetchOptions),
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
                c = i ? he : me;
              return { pages: c(e.pages, o, s), pageParams: c(e.pageParams, r, s) };
            };
          if (i && a.length) {
            let e = i === `backward`,
              t = e ? Me : je,
              n = { pages: a, pageParams: o };
            s = await d(n, t(r, n), e);
          } else {
            let t = e ?? a.length;
            do {
              let e = c === 0 ? (o[0] === void 0 ? r.initialPageParam : o[0]) : je(r, s);
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
function je(e, { pages: t, pageParams: n }) {
  let r = t.length - 1;
  return t.length > 0 ? e.getNextPageParam(t[r], t, n[r], n) : void 0;
}
function Me(e, { pages: t, pageParams: n }) {
  return t.length > 0 ? e.getPreviousPageParam?.(t[0], t, n[0], n) : void 0;
}
var Ne = class extends ke {
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
      (this.#t = Ie(this.options)),
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
      let e = Ie(this.options);
      e.data !== void 0 && (this.setState(Fe(e.data, e.dataUpdatedAt)), (this.#t = e));
    }
  }
  optionalRemove() {
    !this.observers.length && this.state.fetchStatus === `idle` && this.#r.remove(this);
  }
  setData(e, t) {
    let n = pe(this.state.data, e, this.options);
    return (
      this.#c({ data: n, type: `success`, dataUpdatedAt: t?.updatedAt, manual: t?.manual }), n
    );
  }
  setState(e) {
    this.#c({ type: `setState`, state: e });
  }
  cancel(e) {
    let t = this.#a?.promise;
    return (this.#a?.cancel(e), t ? t.then(T).catch(T) : Promise.resolve());
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
    return this.observers.some((e) => E(e.options.enabled, this) !== !1);
  }
  isDisabled() {
    return this.getObserversCount() > 0
      ? !this.isActive()
      : this.options.queryFn === ge || !this.isFetched();
  }
  isFetched() {
    return this.state.dataUpdateCount + this.state.errorUpdateCount > 0;
  }
  isStatic() {
    return (
      this.getObserversCount() > 0 &&
      this.observers.some((e) => E(e.options.staleTime, this) === `static`)
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
          : !ae(this.state.dataUpdatedAt, e);
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
        let e = _e(this.options, t),
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
    ((this.#e === `infinite` ? Ae(this.options.pages) : this.options.behavior)?.onFetch(a, this),
      (this.#n = this.state),
      (this.state.fetchStatus === `idle` || this.state.fetchMeta !== a.fetchOptions?.meta) &&
        this.#c({ type: `fetch`, meta: a.fetchOptions?.meta }));
    let o = (this.#a = Oe({
      initialPromise: t?.initialPromise,
      fn: a.fetchFn,
      onCancel: (e) => {
        (e instanceof De && e.revert && this.setState({ ...this.#n, fetchStatus: `idle` }),
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
      if (e instanceof De) {
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
          return { ...t, ...Pe(t.data, this.options), fetchMeta: e.meta ?? null };
        case `success`:
          let n = {
            ...t,
            ...Fe(e.data, e.dataUpdatedAt),
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
      P.batch(() => {
        (this.observers.slice().forEach((e) => {
          e.onQueryUpdate();
        }),
          this.#r.notify({ query: this, type: `updated`, action: e }));
      }));
  }
};
function Pe(e, t) {
  return {
    fetchFailureCount: 0,
    fetchFailureReason: null,
    fetchStatus: Ee(t.networkMode) ? `fetching` : `paused`,
    ...(e === void 0 && { error: null, status: `pending` }),
  };
}
function Fe(e, t) {
  return {
    data: e,
    dataUpdatedAt: t ?? Date.now(),
    error: null,
    isInvalidated: !1,
    status: `success`,
  };
}
function Ie(e) {
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
var Le = class extends be {
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
      ze(this.#t, this.options) ? this.#m() : this.updateResult(),
      this.#y());
  }
  onUnsubscribe() {
    this.hasListeners() || this.destroy();
  }
  shouldFetchOnReconnect() {
    return Be(this.#t, this.options, this.options.refetchOnReconnect);
  }
  shouldFetchOnWindowFocus() {
    return Be(this.#t, this.options, this.options.refetchOnWindowFocus);
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
        typeof E(this.options.enabled, this.#t) != `boolean`)
    )
      throw Error(`Expected enabled to be a boolean or a callback that returns a boolean`);
    (this.#S(),
      this.#t.setOptions(this.options),
      t._defaulted &&
        !j(this.options, t) &&
        this.#e
          .getQueryCache()
          .notify({ type: `observerOptionsUpdated`, query: this.#t, observer: this }));
    let r = this.hasListeners();
    (r && Ve(this.#t, n, this.options, t) && this.#m(),
      this.updateResult(),
      r &&
        (this.#t !== n ||
          E(this.options.enabled, this.#t) !== E(t.enabled, this.#t) ||
          E(this.options.staleTime, this.#t) !== E(t.staleTime, this.#t)) &&
        this.#g());
    let i = this.#_();
    r &&
      (this.#t !== n ||
        E(this.options.enabled, this.#t) !== E(t.enabled, this.#t) ||
        i !== this.#f) &&
      this.#v(i);
  }
  getOptimisticResult(e) {
    let t = this.#e.getQueryCache().build(this.#e, e),
      n = this.createResult(t, e);
    return (
      j(this.getCurrentResult(), n) ||
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
    return (e?.throwOnError || (t = t.catch(T)), t);
  }
  #h(e) {
    return !ye() && E(this.options.enabled, this.#t) !== !1 && ie(e);
  }
  #g() {
    this.#b();
    let e = E(this.options.staleTime, this.#t);
    if (this.#r.isStale || !this.#h(e)) return;
    let t = ae(this.#r.dataUpdatedAt, e) + 1;
    this.#u = C.setTimeout(() => {
      this.#r.isStale || this.updateResult();
    }, t);
  }
  #_() {
    return E(this.options.refetchInterval, this.#t) ?? !1;
  }
  #v(e) {
    (this.#x(),
      (this.#f = e),
      this.#f !== 0 &&
        this.#h(this.#f) &&
        (this.#d = C.setInterval(() => {
          (this.options.refetchIntervalInBackground || xe.isFocused()) && this.#m();
        }, this.#f)));
  }
  #y() {
    (this.#g(), this.#v(this.#_()));
  }
  #b() {
    this.#u !== void 0 && (C.clearTimeout(this.#u), (this.#u = void 0));
  }
  #x() {
    this.#d !== void 0 && (C.clearInterval(this.#d), (this.#d = void 0));
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
        a = !i && ze(e, t),
        o = i && Ve(e, n, t, r);
      ((a || o) && (l = { ...l, ...Pe(c.data, e.options) }),
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
        e !== void 0 && ((m = `success`), (d = pe(i?.data, e, t)), (u = !0)));
    }
    if (t.select && d !== void 0 && !h) {
      if (i && d === a?.data && t.select === this.#s) d = this.#c;
      else
        try {
          ((this.#s = t.select),
            (d = t.select(d)),
            (d = pe(i?.data, d, t)),
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
      isStale: He(e, t),
      refetch: this.refetch,
      isEnabled: E(t.enabled, e) !== !1,
    };
  }
  updateResult() {
    let e = this.#r,
      t = this.createResult(this.#t, this.options);
    if (
      ((this.#i = this.#t.state),
      (this.#a = this.options),
      this.#i.data !== void 0 && (this.#l = this.#t),
      j(t, e))
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
    P.batch(() => {
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
function Re(e, t) {
  return (
    E(t.enabled, e) !== !1 &&
    e.state.data === void 0 &&
    (e.state.status !== `error` || E(t.retryOnMount, e) !== !1)
  );
}
function ze(e, t) {
  return Re(e, t) || (e.state.data !== void 0 && Be(e, t, t.refetchOnMount));
}
function Be(e, t, n) {
  if (E(t.enabled, e) !== !1 && E(t.staleTime, e) !== `static`) {
    let r = E(n, e);
    return r === `always` || (r !== !1 && He(e, t));
  }
  return !1;
}
function Ve(e, t, n, r) {
  return (
    (e !== t || E(r.enabled, e) === !1) && (!n.suspense || e.state.status !== `error`) && He(e, n)
  );
}
function He(e, t) {
  return E(t.enabled, e) !== !1 && e.isStaleByTime(E(t.staleTime, e));
}
var Ue = class extends ke {
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
      (this.state = e.state || We()),
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
      r = (this.#r = Oe({
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
      P.batch(() => {
        (this.#t.forEach((t) => {
          t.onMutationUpdate(e);
        }),
          this.#n.notify({ mutation: this, type: `updated`, action: e }));
      }));
  }
};
function We() {
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
var Ge = class extends be {
  #e;
  #t;
  #n;
  constructor(e = {}) {
    (super(), (this.config = e), (this.#e = new Set()), (this.#t = new Map()), (this.#n = 0));
  }
  build(e, t, n) {
    let r = new Ue({
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
    let t = Ke(e);
    if (typeof t == `string`) {
      let n = this.#t.get(t);
      n ? n.push(e) : this.#t.set(t, [e]);
    }
    this.notify({ type: `added`, mutation: e });
  }
  remove(e) {
    if (this.#e.delete(e)) {
      let t = Ke(e);
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
    let t = Ke(e);
    if (typeof t == `string`) {
      let n = this.#t.get(t)?.find((e) => e.state.status === `pending`);
      return !n || n === e;
    }
    return !0;
  }
  runNext(e) {
    let t = Ke(e);
    return typeof t == `string`
      ? (this.#t
          .get(t)
          ?.find((t) => t !== e && t.state.isPaused)
          ?.continue() ?? Promise.resolve())
      : Promise.resolve();
  }
  clear() {
    P.batch(() => {
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
    return this.getAll().find((e) => se(t, e));
  }
  findAll(e = {}) {
    return this.getAll().filter((t) => se(e, t));
  }
  notify(e) {
    P.batch(() => {
      this.listeners.forEach((t) => {
        t(e);
      });
    });
  }
  resumePausedMutations() {
    let e = this.getAll().filter((e) => e.state.isPaused);
    return P.batch(() => Promise.all(e.map((e) => e.continue().catch(T))));
  }
};
function Ke(e) {
  return e.options.scope?.id;
}
var qe = class extends be {
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
        j(this.options, t) ||
          this.#e
            .getMutationCache()
            .notify({ type: `observerOptionsUpdated`, mutation: this.#n, observer: this }),
        t?.mutationKey &&
        this.options.mutationKey &&
        O(t.mutationKey) !== O(this.options.mutationKey)
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
      let e = this.#n?.state ?? We();
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
      P.batch(() => {
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
  Je = class extends be {
    #e;
    constructor(e = {}) {
      (super(), (this.config = e), (this.#e = new Map()));
    }
    build(e, t, n) {
      let r = t.queryKey,
        i = t.queryHash ?? D(r, t),
        a = this.get(i);
      return (
        a ||
          ((a = new Ne({
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
      P.batch(() => {
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
      return this.getAll().find((e) => oe(t, e));
    }
    findAll(e = {}) {
      let t = this.getAll();
      return Object.keys(e).length > 0 ? t.filter((t) => oe(e, t)) : t;
    }
    notify(e) {
      P.batch(() => {
        this.listeners.forEach((t) => {
          t(e);
        });
      });
    }
    onFocus() {
      P.batch(() => {
        this.getAll().forEach((e) => {
          e.onFocus();
        });
      });
    }
    onOnline() {
      P.batch(() => {
        this.getAll().forEach((e) => {
          e.onOnline();
        });
      });
    }
  },
  Ye = class {
    #e;
    #t;
    #n;
    #r;
    #i;
    #a;
    #o;
    #s;
    constructor(e = {}) {
      ((this.#e = e.queryCache || new Je()),
        (this.#t = e.mutationCache || new Ge()),
        (this.#n = e.defaultOptions || {}),
        (this.#r = new Map()),
        (this.#i = new Map()),
        (this.#a = 0));
    }
    mount() {
      (this.#a++,
        this.#a === 1 &&
          ((this.#o = xe.subscribe(async (e) => {
            e && (await this.resumePausedMutations(), this.#e.onFocus());
          })),
          (this.#s = we.subscribe(async (e) => {
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
        : (e.revalidateIfStale && n.isStaleByTime(E(t.staleTime, n)) && this.prefetchQuery(t),
          Promise.resolve(r));
    }
    getQueriesData(e) {
      return this.#e.findAll(e).map(({ queryKey: e, state: t }) => [e, t.data]);
    }
    setQueryData(e, t, n) {
      let r = this.defaultQueryOptions({ queryKey: e }),
        i = this.#e.get(r.queryHash)?.state.data,
        a = re(t, i);
      if (a !== void 0) return this.#e.build(this, r).setData(a, { ...n, manual: !0 });
    }
    setQueriesData(e, t, n) {
      return P.batch(() =>
        this.#e.findAll(e).map(({ queryKey: e }) => [e, this.setQueryData(e, t, n)]),
      );
    }
    getQueryState(e) {
      let t = this.defaultQueryOptions({ queryKey: e });
      return this.#e.get(t.queryHash)?.state;
    }
    removeQueries(e) {
      let t = this.#e;
      P.batch(() => {
        t.findAll(e).forEach((e) => {
          t.remove(e);
        });
      });
    }
    resetQueries(e, t) {
      let n = this.#e;
      return P.batch(() => {
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
        r = P.batch(() => this.#e.findAll(e).map((e) => e.cancel(n)));
      return Promise.all(r).then(T).catch(T);
    }
    invalidateQueries(e, t = {}) {
      return P.batch(
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
        r = P.batch(() =>
          this.#e
            .findAll(e)
            .filter((e) => !e.isDisabled() && !e.isStatic())
            .map((e) => {
              let t = e.fetch(void 0, n);
              return (
                n.throwOnError || (t = t.catch(T)),
                e.state.fetchStatus === `paused` ? Promise.resolve() : t
              );
            }),
        );
      return Promise.all(r).then(T);
    }
    async query(e) {
      let t = this.defaultQueryOptions(e);
      t.retry === void 0 && (t.retry = !1);
      let n = this.#e.build(this, t),
        r = n.isStaleByTime(E(t.staleTime, n)) ? await n.fetch(t) : n.state.data,
        i = t.select;
      return i ? i(r) : r;
    }
    fetchQuery(e) {
      let t = this.defaultQueryOptions(e);
      t.retry === void 0 && (t.retry = !1);
      let n = this.#e.build(this, t);
      return n.isStaleByTime(E(t.staleTime, n)) ? n.fetch(t) : Promise.resolve(n.state.data);
    }
    prefetchQuery(e) {
      return this.fetchQuery(e).then(T).catch(T);
    }
    infiniteQuery(e) {
      return ((e._type = `infinite`), this.query(e));
    }
    fetchInfiniteQuery(e) {
      return ((e._type = `infinite`), this.fetchQuery(e));
    }
    prefetchInfiniteQuery(e) {
      return this.fetchInfiniteQuery(e).then(T).catch(T);
    }
    ensureInfiniteQueryData(e) {
      return ((e._type = `infinite`), this.ensureQueryData(e));
    }
    resumePausedMutations() {
      return we.isOnline() ? this.#t.resumePausedMutations() : Promise.resolve();
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
      this.#r.set(O(e), { queryKey: e, defaultOptions: t });
    }
    getQueryDefaults(e) {
      let t = [...this.#r.values()],
        n = {};
      return (
        t.forEach((t) => {
          k(e, t.queryKey) && Object.assign(n, t.defaultOptions);
        }),
        n
      );
    }
    setMutationDefaults(e, t) {
      this.#i.set(O(e), { mutationKey: e, defaultOptions: t });
    }
    getMutationDefaults(e) {
      let t = [...this.#i.values()],
        n = {};
      return (
        t.forEach((t) => {
          k(e, t.mutationKey) && Object.assign(n, t.defaultOptions);
        }),
        n
      );
    }
    defaultQueryOptions(e) {
      if (e._defaulted) return e;
      let t = { ...this.#n.queries, ...this.getQueryDefaults(e.queryKey), ...e, _defaulted: !0 };
      return (
        (t.queryHash ||= D(t.queryKey, t)),
        t.refetchOnReconnect === void 0 && (t.refetchOnReconnect = t.networkMode !== `always`),
        t.throwOnError === void 0 && (t.throwOnError = !!t.suspense),
        !t.networkMode && t.persister && (t.networkMode = `offlineFirst`),
        t.queryFn === ge && (t.enabled = !1),
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
  Xe = y.createContext(!1),
  Ze = () => y.useContext(Xe);
Xe.Provider;
function Qe() {
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
var $e = y.createContext(Qe()),
  et = () => y.useContext($e),
  tt = (e, t, n) => {
    let r =
      n?.state.status === `error` && typeof e.throwOnError == `function`
        ? ve(e.throwOnError, [n.state.error, n])
        : e.throwOnError;
    (e.suspense || r) && (t.isReset() || (e.retryOnMount = !1));
  },
  nt = (e) => {
    y.useEffect(() => {
      e.clearReset();
    }, [e]);
  },
  rt = ({ result: e, errorResetBoundary: t, throwOnError: n, query: r, suspense: i }) =>
    e.isError &&
    !t.isReset() &&
    !e.isFetching &&
    r &&
    ((i && e.data === void 0) || ve(n, [e.error, r])),
  it = (e) => {
    if (e.suspense) {
      let t = 1e3,
        n = (e) => (e === `static` ? e : Math.max(e ?? t, t)),
        r = e.staleTime;
      ((e.staleTime = typeof r == `function` ? (...e) => n(r(...e)) : n(r)),
        typeof e.gcTime == `number` && (e.gcTime = Math.max(e.gcTime, t)));
    }
  },
  at = (e, t) => e?.suspense && t.isPending,
  ot = (e, t, n) =>
    t.fetchOptimistic(e).catch(() => {
      n.clearReset();
    });
function st(e, t, n) {
  let r = Ze(),
    i = et(),
    a = ee(n),
    o = a.defaultQueryOptions(e),
    s = a.getQueryCache().get(o.queryHash),
    c = e.subscribed !== !1;
  ((o._optimisticResults = r ? `isRestoring` : c ? `optimistic` : void 0),
    it(o),
    tt(o, i, s),
    nt(i));
  let [l] = y.useState(() => new t(a, o)),
    u = l.getOptimisticResult(o),
    d = !r && c;
  if (
    (y.useSyncExternalStore(
      y.useCallback(
        (e) => {
          let t = d ? l.subscribe(P.batchCalls(e)) : T;
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
    at(o, u))
  )
    throw ot(o, l, i);
  if (
    rt({
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
function ct(e, t) {
  return st(e, Le, t);
}
function F(e, t) {
  let n = ee(t),
    [r] = y.useState(() => new qe(n, e));
  y.useEffect(() => {
    r.setOptions(e);
  }, [r, e]);
  let i = y.useSyncExternalStore(
      y.useCallback((e) => r.subscribe(P.batchCalls(e)), [r]),
      () => r.getCurrentResult(),
      () => r.getCurrentResult(),
    ),
    a = y.useCallback(
      (...e) => {
        r.mutate(e[0], e[1]).catch(T);
      },
      [r],
    );
  if (i.error && ve(r.options.throwOnError, [i.error])) throw i.error;
  return { ...i, mutate: a, mutateAsync: i.mutate };
}
var lt = g();
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
function ut(e) {
  return typeof e == `object` && !!e && !Array.isArray(e);
}
var dt = (e) => typeof e == `string`,
  ft = (e) => e === void 0 || dt(e),
  pt = (e) => typeof e == `number` && Number.isFinite(e),
  mt = (e) => pt(e) && Number.isSafeInteger(e) && e >= 0,
  ht = (e) => e === void 0 || pt(e),
  gt = (e) => e === `valid` || e === `missing` || e === `invalid`;
function _t(e) {
  return ut(e) && Object.values(e).every(dt);
}
function vt(e) {
  return ut(e) && dt(e.snippet) && ht(e.startSeconds) && ht(e.endSeconds);
}
function yt(e) {
  return (
    ut(e) &&
    dt(e.sourceFile) &&
    mt(e.segmentCount) &&
    pt(e.totalDuration) &&
    Array.isArray(e.spans) &&
    e.spans.every(vt)
  );
}
function bt(e) {
  return (
    ut(e) &&
    (e.kind === `enrolled` || e.kind === `anonymous`) &&
    ft(e.profileId) &&
    ft(e.label) &&
    ft(e.anonymousLabel) &&
    Array.isArray(e.files) &&
    e.files.every(yt)
  );
}
function xt(e) {
  return ut(e) && dt(e.id) && dt(e.label) && _t(e.metadata);
}
function St(e) {
  return ut(e) && dt(e.sourceFile) && dt(e.message);
}
function Ct(e) {
  return !ut(e) || !ut(e.library) || !ut(e.trace)
    ? !1
    : (e.scope === `local` || e.scope === `global` || e.scope === `explicit`) &&
        dt(e.path) &&
        dt(e.library.path) &&
        gt(e.library.status) &&
        mt(e.library.profileCount) &&
        ft(e.library.message) &&
        Array.isArray(e.profiles) &&
        e.profiles.every(xt) &&
        dt(e.trace.path) &&
        gt(e.trace.status) &&
        ft(e.trace.scanRoot) &&
        ft(e.trace.message) &&
        Array.isArray(e.trace.speakers) &&
        e.trace.speakers.every(bt) &&
        Array.isArray(e.trace.errors) &&
        e.trace.errors.every(St);
}
function wt(e) {
  if (!Ct(e)) throw Error(`Invalid Speaker Directory response.`);
  return e;
}
var Tt = `X-Native-Whisperx-Session-Token`;
async function Et(e, t) {
  let n = await fetch(e, {
    ...t,
    signal: t?.signal ?? AbortSignal.timeout(3e4),
    headers: {
      ...(t?.body ? { "Content-Type": `application/json` } : {}),
      ...(window.nativeWhisperxSessionToken ? { [Tt]: window.nativeWhisperxSessionToken } : {}),
      ...t?.headers,
    },
  });
  if (!n.ok) throw Error(await n.text());
  return await n.json();
}
var Dt = {
    getState: async () => wt(await Et(`/api/state`)),
    updateProfile: async (e, t) => (
      await Et(`/api/profiles/${encodeURIComponent(e)}`, {
        method: `PUT`,
        body: JSON.stringify(t),
      }),
      Dt.getState()
    ),
    deleteProfile: async (e) => (
      await Et(`/api/profiles/${encodeURIComponent(e)}`, { method: `DELETE` }),
      Dt.getState()
    ),
    createProfile: () => Et(`/api/profiles`, { method: `POST`, body: JSON.stringify({}) }),
    rebuildTrace: (e) =>
      Et(`/api/trace/rebuild`, { method: `POST`, body: JSON.stringify(e.scanRoot ? e : {}) }),
  },
  Ot = Dt;
function kt() {
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
function At(e) {
  if (!e || typeof e != `object` || Array.isArray(e))
    throw Error(`Invalid local workbench response.`);
  return Object.fromEntries(Object.entries(e));
}
function I(e) {
  if (typeof e != `string`) throw Error(`Invalid workbench text.`);
  return e;
}
function jt(e) {
  if (typeof e != `boolean`) throw Error(`Invalid workbench capability.`);
  return e;
}
function Mt(e) {
  return e == null ? null : I(e);
}
function Nt(e) {
  switch (e) {
    case `auto`:
    case `cpu`:
    case `cuda`:
      return e;
    default:
      throw Error(`Invalid saved device choice.`);
  }
}
function Pt(e) {
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
function Ft(e) {
  let t = At(e),
    n = t.translation == null ? null : At(t.translation),
    r = t.speakerDirectory == null ? null : At(t.speakerDirectory),
    i = null;
  if (r) {
    let e = r.scope;
    if (e !== `auto` && e !== `local` && e !== `global`)
      throw Error(`Invalid saved speaker index scope.`);
    i = { scope: e, explicitPath: Mt(r.explicitPath) };
  }
  if (!Array.isArray(t.formats)) throw Error(`Invalid saved output formats.`);
  return {
    align: jt(t.align),
    identifySpeakers: jt(t.identifySpeakers),
    autoUpdateSpeakers: jt(t.autoUpdateSpeakers),
    translation: n
      ? {
          modelId: I(n.modelId),
          sourceLanguage: I(n.sourceLanguage),
          targetLanguage: I(n.targetLanguage),
        }
      : null,
    model: I(t.model),
    language: Mt(t.language),
    device: Nt(t.device),
    modelDir: Mt(t.modelDir),
    vadBundle: Mt(t.vadBundle),
    diarizationBundle: Mt(t.diarizationBundle),
    outputDir: Mt(t.outputDir),
    formats: t.formats.map(Pt),
    speakerDirectory: i,
  };
}
function It(e) {
  let t = At(e),
    n = At(t.preset);
  return { id: I(t.id), preset: { name: I(n.name), settings: Ft(n.settings) } };
}
function Lt(e) {
  let t = At(e);
  if (!Array.isArray(t.presets) || !Array.isArray(t.builtins))
    throw Error(`Invalid saved presets response.`);
  return {
    presets: t.presets.map(It),
    activeId: Mt(t.activeId),
    problem: Mt(t.problem),
    canReset: jt(t.canReset),
    backup: Mt(t.backup),
    builtins: t.builtins.map((e) => {
      let t = At(e);
      return { ...It(e), supported: jt(t.supported), note: I(t.note) };
    }),
  };
}
function Rt(e) {
  let t = At(e);
  if (!Array.isArray(t.checks)) throw Error(`Invalid preflight checks.`);
  return {
    ready: jt(t.ready),
    checks: t.checks.map((e) => {
      let t = At(e);
      return { resource: I(t.resource), available: jt(t.available), message: I(t.message) };
    }),
  };
}
async function zt(e, t) {
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
function Bt(e) {
  let t = At(e),
    n = I(t.status);
  if (![`idle`, `running`, `cancelling`, `completed`, `failed`, `cancelled`].includes(n))
    throw Error(`Unknown workbench job state.`);
  let r = Object.fromEntries(
      Object.entries(At(t.tasks ?? {})).map(([e, t]) => [e, { status: I(At(t).status) }]),
    ),
    i = t.outputs ?? [];
  if (!Array.isArray(i)) throw Error(`Invalid workbench outputs.`);
  return {
    status: n,
    reviewId: t.reviewId == null ? null : I(t.reviewId),
    tasks: r,
    outputs: i.map((e) => {
      let t = At(e);
      return { path: I(t.path), format: I(t.format) };
    }),
    input: t.input == null ? null : I(At(t.input).path),
    currentTask: t.currentTask == null ? null : I(t.currentTask),
    activity: t.activity == null ? `` : I(t.activity),
    error: t.error == null ? `` : I(t.error),
  };
}
var Vt = {
  async presets() {
    return Lt(await zt(`presets`));
  },
  async savePreset(e) {
    return Lt(await zt(`preset-save`, e));
  },
  async selectPreset(e) {
    return Lt(await zt(`preset-select`, { id: e }));
  },
  async deletePreset(e) {
    return Lt(await zt(`preset-delete`, { id: e }));
  },
  async resetPresets() {
    return Lt(await zt(`preset-reset`, {}));
  },
  async preflight(e) {
    return Rt(await zt(`preflight`, e));
  },
  async capabilities() {
    let e = At(await zt(`capabilities`));
    return {
      native: jt(e.native),
      align: jt(e.align),
      identifySpeakers: jt(e.identifySpeakers),
      translate: jt(e.translate),
      cuda: jt(e.cuda),
      root: I(e.root),
      speakerDirectory: I(e.speakerDirectory),
    };
  },
  async state() {
    return Bt(await zt(`state`));
  },
  async run(e) {
    return Bt(await zt(`run`, e));
  },
  async cancel() {
    return Bt(await zt(`cancel`, {}));
  },
  async result(e) {
    let t = At(await zt(`result`, { index: e }));
    return { text: I(t.text), content: I(t.content), path: I(t.path) };
  },
  async files(e) {
    let t = At(await zt(`files`, { directory: e }));
    if (!Array.isArray(t.entries)) throw Error(`Invalid workbench file list.`);
    return {
      root: I(t.root),
      directory: I(t.directory),
      truncated: jt(t.truncated),
      entries: t.entries.map((e) => {
        let t = At(e);
        return { path: I(t.path), name: I(t.name), directory: jt(t.directory) };
      }),
    };
  },
};
function Ht(e) {
  if (!ut(e)) throw Error(`Invalid Speaker Library response.`);
  return e;
}
function Ut(e) {
  if (typeof e != `string`) throw Error(`Invalid Speaker Library path or identifier.`);
  return e;
}
function Wt(e) {
  if (typeof e != `number` || !Number.isSafeInteger(e) || e < 0)
    throw Error(`Invalid Speaker Library profile count.`);
  return e;
}
function Gt(e) {
  if (typeof e != `boolean`) throw Error(`Invalid Speaker Library operation state.`);
  return e;
}
function Kt(e) {
  return e === null ? null : Ut(e);
}
function qt(e) {
  let t = Ht(e);
  return {
    path: Ut(t.path),
    profileCount: Wt(t.profileCount),
    changed: Gt(t.changed),
    backupPath: Kt(t.backupPath),
  };
}
async function L(e, t) {
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
function Jt(e) {
  let t = Ht(e),
    n = Ht(t.summary),
    r = Ut(t.id);
  switch (t.kind) {
    case `new`:
      return {
        id: r,
        kind: `new`,
        summary: {
          directory: Ut(n.directory),
          destination: Ut(n.destination),
          profileCount: Wt(n.profileCount),
        },
      };
    case `export`:
      return {
        id: r,
        kind: `export`,
        summary: {
          source: Ut(n.source),
          destination: Ut(n.destination),
          profileCount: Wt(n.profileCount),
          confirmedProfiles: Wt(n.confirmedProfiles),
          draftProfiles: Wt(n.draftProfiles),
          unclassifiedProfiles: Wt(n.unclassifiedProfiles),
        },
      };
    case `import`:
      if (n.mode !== `merge` && n.mode !== `replace`)
        throw Error(`Unknown Speaker Library import mode.`);
      return {
        id: r,
        kind: `import`,
        summary: {
          source: Ut(n.source),
          destination: Ut(n.destination),
          mode: n.mode,
          existingProfiles: Wt(n.existingProfiles),
          incomingProfiles: Wt(n.incomingProfiles),
          confirmedProfiles: Wt(n.confirmedProfiles),
          draftProfiles: Wt(n.draftProfiles),
          unclassifiedProfiles: Wt(n.unclassifiedProfiles),
          addedProfiles: Wt(n.addedProfiles),
          unchangedProfiles: Wt(n.unchangedProfiles),
          replacedProfiles: Wt(n.replacedProfiles),
          removedProfiles: Wt(n.removedProfiles),
          resultingProfiles: Wt(n.resultingProfiles),
          changed: Gt(n.changed),
          backupPath: Kt(n.backupPath),
        },
      };
    default:
      throw Error(`Unknown Speaker Library preview.`);
  }
}
var Yt = {
    async preview(e) {
      let t = Jt(await L(`preview`, e));
      if (t.kind !== e.kind)
        throw Error(`Speaker Library preview does not match the requested operation.`);
      return t;
    },
    async confirm(e) {
      let t = Ht(await L(`confirm`, { id: e, confirmed: !0 }));
      switch (t.kind) {
        case `export`:
          return { kind: `export`, receipt: qt(t.receipt) };
        case `new`:
        case `import`:
          return { kind: t.kind, receipt: qt(t.receipt), state: wt(t.state) };
        default:
          throw Error(`Unknown Speaker Library confirmation result.`);
      }
    },
    async load(e) {
      let t = Ht(await L(`load`, { selection: e }));
      if (t.kind !== `load`) throw Error(`Invalid selected Speaker Library.`);
      return { kind: `load`, state: wt(t.state) };
    },
    async cancel(e) {
      Gt(Ht(await L(`cancel`, { id: e })).cancelled);
    },
  },
  Xt = [`workbench`, `presets`];
function Zt(e, t, n, r) {
  let i = ee(),
    a = ct({ queryKey: Xt, queryFn: Vt.presets, retry: !1 }),
    o = ct({ queryKey: [`speaker-directory-state`], queryFn: Ot.getState, enabled: !1 }),
    [s, c] = (0, y.useState)(kt),
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
    x = F({
      mutationFn: async (e) => {
        (u(e?.id ?? null), f(e?.preset.name ?? ``), e && c(e.preset.settings));
        let t = await Vt.selectPreset(e?.id ?? null);
        i.setQueryData(Xt, t);
        let n = e?.preset.settings.speakerDirectory;
        if (n && e?.preset.settings.identifySpeakers) {
          let e = await Yt.load(
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
  let S = F({
      mutationFn: (e) =>
        Vt.savePreset({
          ...(e && v && l ? { id: l } : {}),
          preset: { name: d.trim(), settings: s },
        }),
      onSuccess: (e) => {
        (i.setQueryData(Xt, e), u(e.activeId));
        let t = e.presets.find((t) => t.id === e.activeId);
        t && (c(t.preset.settings), f(t.preset.name));
      },
    }),
    te = F({
      mutationFn: Vt.deletePreset,
      onSuccess: (e) => {
        (i.setQueryData(Xt, e), u(null), f(``));
      },
    }),
    C = F({
      mutationFn: Vt.resetPresets,
      onSuccess: (e) => {
        (i.setQueryData(Xt, e), u(null), f(``));
      },
    }),
    ne = x.isPending || S.isPending || te.isPending || C.isPending,
    w = t ? `` : e,
    T = ct({
      queryKey: [`workbench`, `preflight`, w, s],
      queryFn: () => Vt.preflight({ ...s, input: w }),
      enabled: p && r && !n && !ne,
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
    preflight: T,
    pending: ne,
    directory: o.data,
    select: (e) => x.mutate(g.find((t) => t.id === e) ?? null),
    save: (e) => S.mutate(e),
    remove: () => {
      v && l && te.mutate(l);
    },
    recover: () => C.mutate(),
    error: a.error ?? x.error ?? S.error ?? te.error ?? C.error ?? T.error,
    backup: C.data?.backup,
  };
}
function Qt({ configuration: e, disabled: t }) {
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
function $t(e) {
  if (!e || typeof e != `object` || Array.isArray(e)) throw Error(`Invalid Jobs response.`);
  return Object.fromEntries(Object.entries(e));
}
function en(e) {
  if (typeof e != `string`) throw Error(`Invalid Jobs text.`);
  return e;
}
function tn(e) {
  if (typeof e != `boolean`) throw Error(`Invalid Jobs setting.`);
  return e;
}
function nn(e) {
  return e == null ? null : en(e);
}
function rn(e) {
  if (!Array.isArray(e)) throw Error(`Invalid Jobs list.`);
  return e;
}
function an(e) {
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
function on(e) {
  return {
    reviews: rn($t(e).reviews ?? []).map((e) => {
      let t = $t(e);
      return {
        resultId: en(t.resultId),
        originalResultId: en(t.originalResultId),
        input: en(t.input),
        outputs: rn(t.outputs).map((e) => {
          let t = $t(e);
          return { path: en(t.path), format: en(t.format) };
        }),
      };
    }),
    jobs: rn($t(e).jobs).map((e) => {
      let t = $t(e),
        n = $t(t.settings);
      return {
        id: en(t.id),
        status: an(t.status),
        error: nn(t.error),
        speakerDirectory: en(t.speakerDirectory),
        settings: {
          model: en(n.model),
          device: en(n.device),
          align: tn(n.align),
          identifySpeakers: tn(n.identifySpeakers),
          translate: n.translation != null,
        },
        files: rn(t.files).map((e) => {
          let t = $t(e);
          if (
            t.seconds != null &&
            (typeof t.seconds != `number` || !Number.isFinite(t.seconds) || t.seconds < 0)
          )
            throw Error(`Invalid job duration.`);
          return {
            input: en(t.input),
            status: an(t.status),
            currentTask: nn(t.currentTask),
            seconds: t.seconds == null ? null : t.seconds,
            error: nn(t.error),
            tasks: Object.fromEntries(
              Object.entries($t(t.tasks)).map(([e, t]) => [e, { status: an($t(t).status) }]),
            ),
            outputs: rn(t.outputs).map((e) => {
              let t = $t(e);
              return { path: en(t.path), format: en(t.format) };
            }),
          };
        }),
      };
    }),
  };
}
async function sn(e, t) {
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
var cn = {
    async history() {
      return on(await sn(`jobs`));
    },
    async queue(e) {
      return on(await sn(`queue`, e));
    },
    async retry(e) {
      return on(await sn(`retry`, e));
    },
    async result(e) {
      let t = $t(await sn(`job-result`, e));
      return { path: en(t.path), text: en(t.text), content: en(t.content) };
    },
  },
  ln = [`workbench`, `jobs`],
  un = {
    queued: `Queued`,
    running: `Running`,
    completed: `Completed`,
    failed: `Failed`,
    cancelled: `Cancelled`,
    notRun: `Not run`,
    interrupted: `Interrupted`,
  },
  dn = {
    decode: `Read media`,
    vad: `Detect speech`,
    transcribe: `Transcribe`,
    align: `Align timestamps`,
    identifySpeakers: `Identify speakers`,
    translate: `Translate`,
    writeOutputs: `Write outputs`,
  };
function fn(e) {
  return e.split(/[\\/]/).at(-1) ?? e;
}
function pn(e) {
  return e.length <= 180
    ? e
    : /decode|WAV|RIFF/i.test(e)
      ? `Could not read this media file. Check its format before retrying.`
      : /model/i.test(e)
        ? `A required model could not be loaded. Check local model resources before retrying.`
        : `Native processing failed. Open error details before retrying.`;
}
function mn({ message: e }) {
  let t = pn(e);
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
function hn() {
  let e = ee(),
    [t, n] = (0, y.useState)(``),
    [r, i] = (0, y.useState)(20),
    a = ct({
      queryKey: ln,
      queryFn: cn.history,
      retry: !1,
      refetchInterval: (e) =>
        e.state.data?.jobs.some((e) => e.status === `running` || e.status === `queued`) ? 750 : !1,
    }),
    o = F({ mutationFn: cn.retry, onSuccess: (t) => e.setQueryData(ln, t) }),
    s = F({
      mutationFn: Vt.cancel,
      onSuccess: () => {
        (n(`Cancellation requested; waiting for a native safe boundary.`),
          e.invalidateQueries({ queryKey: ln }));
      },
    }),
    c = F({ mutationFn: cn.result }),
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
              (0, b.jsx)(`h2`, { children: un[e.status] }),
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
                ? (0, b.jsx)(mn, { message: e.error })
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
                                      un[t.status],
                                      t.currentTask
                                        ? ` · ${dn[t.currentTask] ?? t.currentTask}…`
                                        : ``,
                                    ],
                                  }),
                                  t.error ? (0, b.jsx)(mn, { message: t.error }) : null,
                                  Object.keys(t.tasks).length
                                    ? (0, b.jsxs)(`details`, {
                                        children: [
                                          (0, b.jsx)(`summary`, { children: `Task outcomes` }),
                                          (0, b.jsx)(`ul`, {
                                            children: Object.entries(t.tasks).map(([e, t]) =>
                                              (0, b.jsxs)(
                                                `li`,
                                                { children: [dn[e] ?? e, `: `, un[t.status]] },
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
                                            children: [`Open `, fn(t.path)],
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
                                          (n(`Retry requested for ${fn(t.input)}.`),
                                            o.mutate({ batchId: e.id, fileIndex: r }));
                                        },
                                        children: [
                                          t.status === `notRun` ? `Run` : `Retry`,
                                          ` `,
                                          fn(t.input),
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
              (0, b.jsx)(`p`, { children: c.data.text }),
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
var gn = [`json`, `srt`, `vtt`, `txt`, `tsv`, `native-json`, `aud`, `all`],
  _n = [`workbench`, `current-job`],
  vn = {
    pending: `Waiting`,
    running: `Running`,
    completed: `Completed`,
    failed: `Failed`,
    cancelled: `Cancelled`,
    notRun: `Not run`,
  },
  yn = {
    decode: `Read media`,
    vad: `Detect speech`,
    transcribe: `Transcribe`,
    align: `Align timestamps`,
    identifySpeakers: `Identify speakers`,
    translate: `Translate`,
    writeOutputs: `Write outputs`,
  };
function bn({ speakerIndex: e }) {
  let t = ee(),
    n = ct({ queryKey: [`workbench`, `capabilities`], queryFn: Vt.capabilities, retry: !1 }),
    r = ct({
      queryKey: _n,
      queryFn: Vt.state,
      retry: !1,
      refetchInterval: (e) =>
        [`running`, `cancelling`].includes(e.state.data?.status ?? ``) ? 750 : !1,
    }),
    [i, a] = (0, y.useState)(``),
    [o, s] = (0, y.useState)(!1),
    [c, l] = (0, y.useState)(``),
    [u, d] = (0, y.useState)(!1),
    f = F({ mutationFn: Vt.files }),
    p = F({
      mutationFn: Vt.run,
      onSuccess: (e) => {
        (t.setQueryData(_n, e), g.reset());
      },
    }),
    m = F({
      mutationFn: cn.queue,
      onSuccess: (e) => {
        (t.setQueryData(ln, e), t.invalidateQueries({ queryKey: _n }), g.reset());
      },
    }),
    h = F({ mutationFn: Vt.cancel, onSuccess: (e) => t.setQueryData(_n, e) }),
    g = F({ mutationFn: Vt.result });
  (0, y.useEffect)(() => {
    [`completed`, `failed`, `cancelled`].includes(r.data?.status ?? ``) &&
      t.invalidateQueries({ queryKey: [`speaker-directory-state`] });
  }, [t, r.data?.status]);
  let _ = p.isPending || m.isPending || [`running`, `cancelling`].includes(r.data?.status ?? ``),
    v = n.data,
    x = Zt(i, o, _, v?.native ?? !1),
    { settings: S, setSettings: te } = x,
    {
      align: C,
      identifySpeakers: ne,
      autoUpdateSpeakers: w,
      model: T,
      device: re,
      formats: ie,
    } = S,
    ae = S.language ?? ``,
    E = S.modelDir ?? ``,
    oe = S.outputDir ?? ``,
    se = S.vadBundle ?? ``,
    D = S.diarizationBundle ?? ``,
    O = S.translation !== null,
    k = S.translation?.targetLanguage ?? `de`,
    ce = S.translation?.modelId ?? ``,
    A = _ || x.pending || !x.initialized;
  function j(e, t) {
    te((n) => ({ ...n, [e]: t }));
  }
  function le(e) {
    te((t) => ({ ...t, identifySpeakers: e, autoUpdateSpeakers: e && t.autoUpdateSpeakers }));
  }
  function ue(e) {
    te((t) => {
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
  function de(e) {
    te((t) => ({
      ...t,
      language: e || null,
      translation: t.translation ? { ...t.translation, sourceLanguage: e } : null,
    }));
  }
  function fe(e, t) {
    te((n) => ({ ...n, translation: n.translation ? { ...n.translation, [e]: t } : null }));
  }
  function pe(e, t) {
    te((n) => ({ ...n, formats: t ? [...n.formats, e] : n.formats.filter((t) => t !== e) }));
  }
  let me = x.error ?? n.error ?? r.error ?? p.error ?? m.error ?? h.error ?? f.error ?? g.error;
  return (0, b.jsxs)(`main`, {
    className: `page workbench`,
    children: [
      (0, b.jsx)(`h1`, { children: `Transcribe` }),
      me ? (0, b.jsx)(`p`, { role: `alert`, children: me.message }) : null,
      m.isSuccess
        ? (0, b.jsxs)(`p`, {
            role: `status`,
            children: [
              `Batch added. `,
              (0, b.jsx)(`a`, { href: `?view=jobs`, children: `View jobs` }),
            ],
          })
        : null,
      (0, b.jsx)(Qt, { configuration: x, disabled: A }),
      (0, b.jsxs)(`form`, {
        className: `workbenchForm`,
        onSubmit: (e) => {
          e.preventDefault();
          let t = { ...S, input: i };
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
            disabled: A || !v,
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
            disabled: A || !v,
            children: [
              (0, b.jsx)(`legend`, { children: `Tasks · in order` }),
              (0, b.jsx)(`p`, { children: `1. Transcribe` }),
              (0, b.jsxs)(`label`, {
                children: [
                  (0, b.jsx)(`input`, {
                    type: `checkbox`,
                    checked: C,
                    disabled: !v?.align && !C,
                    onChange: (e) => j(`align`, e.currentTarget.checked),
                  }),
                  `Align timestamps`,
                ],
              }),
              (0, b.jsxs)(`label`, {
                children: [
                  (0, b.jsx)(`input`, {
                    type: `checkbox`,
                    checked: ne,
                    disabled: !v?.identifySpeakers && !ne,
                    onChange: (e) => le(e.currentTarget.checked),
                  }),
                  `Identify speakers`,
                ],
              }),
              ne
                ? (0, b.jsxs)(`label`, {
                    children: [
                      (0, b.jsx)(`input`, {
                        type: `checkbox`,
                        checked: w,
                        onChange: (e) => j(`autoUpdateSpeakers`, e.currentTarget.checked),
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
                    checked: O,
                    disabled: !v?.translate && !O,
                    onChange: (e) => ue(e.currentTarget.checked),
                  }),
                  `Translate`,
                ],
              }),
              O
                ? (0, b.jsxs)(`div`, {
                    className: `settingsRow taskLanguages`,
                    children: [
                      (0, b.jsxs)(`label`, {
                        children: [
                          `Source language`,
                          (0, b.jsx)(`input`, {
                            required: !0,
                            value: ae,
                            onChange: (e) => de(e.currentTarget.value),
                          }),
                        ],
                      }),
                      (0, b.jsxs)(`label`, {
                        children: [
                          `Target language`,
                          (0, b.jsx)(`input`, {
                            required: !0,
                            value: k,
                            onChange: (e) => fe(`targetLanguage`, e.currentTarget.value),
                          }),
                        ],
                      }),
                    ],
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
            disabled: A || !v,
            children: [
              (0, b.jsx)(`legend`, { children: `Output` }),
              (0, b.jsx)(`p`, {
                className: `muted`,
                children: `A canonical Native JSON result is also saved for Review & Edit.`,
              }),
              (0, b.jsx)(`div`, {
                className: `formatChoices`,
                children: gn.map((e) =>
                  (0, b.jsxs)(
                    `label`,
                    {
                      children: [
                        (0, b.jsx)(`input`, {
                          type: `checkbox`,
                          checked: ie.includes(e),
                          onChange: (t) => pe(e, t.currentTarget.checked),
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
                        value: oe,
                        onChange: (e) => j(`outputDir`, e.currentTarget.value || null),
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
                  O
                    ? (0, b.jsxs)(`label`, {
                        children: [
                          `Translation model`,
                          (0, b.jsx)(`input`, {
                            disabled: A,
                            required: !0,
                            value: ce,
                            onChange: (e) => fe(`modelId`, e.currentTarget.value),
                          }),
                        ],
                      })
                    : null,
                  (0, b.jsxs)(`label`, {
                    children: [
                      `Model cache folder`,
                      (0, b.jsx)(`input`, {
                        disabled: A,
                        value: E,
                        onChange: (e) => j(`modelDir`, e.currentTarget.value || null),
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
                            disabled: A,
                            value: se,
                            onChange: (e) => j(`vadBundle`, e.currentTarget.value || null),
                            placeholder: `Auto-select cached bundle`,
                          }),
                        ],
                      }),
                      (0, b.jsxs)(`label`, {
                        children: [
                          `Verified diarization bundle`,
                          (0, b.jsx)(`input`, {
                            disabled: A,
                            value: D,
                            onChange: (e) => j(`diarizationBundle`, e.currentTarget.value || null),
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
                        disabled: A,
                        required: !0,
                        value: T,
                        onChange: (e) => j(`model`, e.currentTarget.value),
                      }),
                    ],
                  }),
                  O
                    ? null
                    : (0, b.jsxs)(`label`, {
                        children: [
                          `Source language`,
                          (0, b.jsx)(`input`, {
                            disabled: A,
                            required: O,
                            value: ae,
                            onChange: (e) => de(e.currentTarget.value),
                            placeholder: `Auto-detect`,
                          }),
                        ],
                      }),
                  (0, b.jsxs)(`label`, {
                    children: [
                      `Device`,
                      (0, b.jsxs)(`select`, {
                        disabled: A,
                        value: re,
                        onChange: (e) => j(`device`, Nt(e.currentTarget.value)),
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
                        disabled: x.preflight.isFetching || A,
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
              A ||
              !v?.native ||
              !i.trim() ||
              ie.length === 0 ||
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
                    children: yn[r.data.currentTask] ?? r.data.currentTask,
                  })
                : null,
              r.data.activity && _ ? (0, b.jsx)(`p`, { children: r.data.activity }) : null,
              (0, b.jsx)(`ol`, {
                children: Object.keys(yn)
                  .filter((e) => r.data?.tasks[e])
                  .map((e) =>
                    (0, b.jsxs)(
                      `li`,
                      {
                        children: [
                          yn[e] ?? e,
                          `:`,
                          ` `,
                          vn[r.data?.tasks[e]?.status ?? ``] ?? `Unknown`,
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
              (0, b.jsx)(`p`, { children: g.data.text }),
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
function xn(e) {
  if (!e || typeof e != `object` || Array.isArray(e)) throw Error(`Invalid Review response.`);
  return Object.fromEntries(Object.entries(e));
}
function Sn(e) {
  if (typeof e != `string`) throw Error(`Invalid Review text.`);
  return e;
}
function Cn(e) {
  return e == null ? null : Sn(e);
}
function wn(e) {
  if (typeof e != `boolean`) throw Error(`Invalid Review state.`);
  return e;
}
function Tn(e) {
  if (!Array.isArray(e)) throw Error(`Invalid Review list.`);
  return e;
}
function En(e) {
  if (e == null) return null;
  if (typeof e != `number` || !Number.isFinite(e) || e < 0) throw Error(`Invalid Review timing.`);
  return e;
}
function Dn(e) {
  let t = xn(e);
  return {
    language: Cn(t.language),
    segments: Tn(t.segments).map((e) => {
      let t = xn(e),
        n = t.wordCount;
      if (typeof n != `number` || !Number.isSafeInteger(n) || n < 0)
        throw Error(`Invalid alignment count.`);
      return {
        id: Sn(t.id),
        text: Sn(t.text),
        startSeconds: En(t.startSeconds),
        endSeconds: En(t.endSeconds),
        speaker: Cn(t.speaker),
        wordCount: n,
        overlapsPrevious: wn(t.overlapsPrevious),
      };
    }),
  };
}
function On(e) {
  let t = xn(e),
    n = Cn(t.mediaUrl);
  if (n && !/^\/api\/review\/media\/[a-f0-9]{64}$/.test(n))
    throw Error(`Invalid local recording capability.`);
  let r = t.translationProvenance == null ? null : xn(t.translationProvenance);
  return {
    resultId: Sn(t.resultId),
    snapshotId: Sn(t.snapshotId),
    originalName: Cn(t.originalName),
    sourceName: Cn(t.sourceName),
    mediaUrl: n,
    mediaType: Sn(t.mediaType),
    mediaError: Cn(t.mediaError),
    root: Sn(t.root),
    transcript: Dn(t.transcript),
    sourceTranscript: t.sourceTranscript == null ? null : Dn(t.sourceTranscript),
    isTranslation: wn(t.isTranslation),
    translationProvenance: r
      ? {
          modelId: Cn(r.modelId),
          sourceLanguage: Cn(r.sourceLanguage),
          targetLanguage: Cn(r.targetLanguage),
        }
      : null,
  };
}
async function kn(e, t) {
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
var An = {
  async open(e) {
    return On(await kn(`open`, { resultId: e }));
  },
  async reset(e) {
    return On(await kn(`reset`, { resultId: e }));
  },
  async recording(e) {
    return On(await kn(`recording`, e));
  },
  async preview(e) {
    let t = xn(await kn(`preview`, e));
    return {
      previewId: Sn(t.previewId),
      transcript: Dn(t.transcript),
      sourceTranscript: t.sourceTranscript == null ? null : Dn(t.sourceTranscript),
      clearedAlignment: Tn(t.clearedAlignment).map(Sn),
      clearedSourceAlignment: Tn(t.clearedSourceAlignment).map(Sn),
    };
  },
  async save(e) {
    let t = xn(await kn(`save`, e));
    return {
      resultId: Sn(t.resultId),
      outputs: Tn(t.outputs).map((e) => {
        let t = xn(e);
        return { path: Sn(t.path), format: Sn(t.format) };
      }),
    };
  },
};
function jn(e) {
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
function Mn(e) {
  if (!Number.isFinite(e)) return `—`;
  let t = Math.floor(e / 3600),
    n = Math.floor((e % 3600) / 60),
    r = (e % 60).toFixed(3).padStart(6, `0`);
  return `${String(t).padStart(2, `0`)}:${String(n).padStart(2, `0`)}:${r}`;
}
var Nn = JSON.stringify({ transcript: {}, source: {} });
function Pn(e) {
  return {
    text: e.text,
    start: e.startSeconds == null ? `` : String(e.startSeconds),
    end: e.endSeconds == null ? `` : String(e.endSeconds),
    speaker: e.speaker ?? ``,
  };
}
function Fn(e, t) {
  return Object.entries(e).map(([e, n]) => {
    if (!t?.segments.some((t) => t.id === e))
      throw Error(`Selected segment no longer exists. Reopen the result.`);
    return {
      segmentId: e,
      text: n.text,
      startSeconds: jn(n.start),
      endSeconds: jn(n.end),
      speaker: n.speaker || null,
    };
  });
}
function In(e) {
  return `?view=review&result=${encodeURIComponent(e)}`;
}
function Ln({ api: e = Ot }) {
  let t = new URLSearchParams(window.location.search).get(`result`),
    n = ct({
      queryKey: [`review`, t],
      queryFn: () => An.open(t ?? ``),
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
        : (0, b.jsx)(zn, { original: n.data, api: e }, n.data.snapshotId)
    : (0, b.jsx)(Rn, {});
}
function Rn() {
  let e = ct({ queryKey: ln, queryFn: cn.history, retry: !1 }),
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
                        href: In(e.resultId),
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
                  href: In(`${e.id}:${n}`),
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
function zn({ original: e, api: t }) {
  let n = ee(),
    r = ct({ queryKey: [`speaker-directory-state`], queryFn: () => t.getState(), retry: !1 }),
    i = new URLSearchParams(window.location.search),
    [a, o] = (0, y.useState)(
      i.get(`layer`) === `source` && e.sourceTranscript ? `source` : `transcript`,
    ),
    [s, c] = (0, y.useState)(i.get(`segment`) ?? e.transcript.segments[0]?.id ?? ``),
    [l, u] = (0, y.useState)({ transcript: {}, source: {} }),
    d = JSON.stringify(l),
    [f, p] = (0, y.useState)(Nn),
    m = d !== f,
    [h, g] = (0, y.useState)(`.`),
    [_, v] = (0, y.useState)(
      `${(e.originalName ?? `result`).replace(/(?:\.native)?\.json$/i, ``)}.corrected`,
    ),
    [x, S] = (0, y.useState)([`json`, `srt`]),
    [te, C] = (0, y.useState)(`transcript`),
    [ne, w] = (0, y.useState)(``),
    [T, re] = (0, y.useState)(``),
    [ie, ae] = (0, y.useState)(0),
    E = (0, y.useRef)(null),
    oe = (0, y.useRef)(new Map()),
    se = (0, y.useRef)(null),
    D = F({
      mutationFn: async (t) => ({
        fingerprint: t.fingerprint,
        preview: await An.preview({
          resultId: e.resultId,
          corrections: Fn(t.drafts.transcript, e.transcript),
          sourceCorrections: Fn(t.drafts.source, e.sourceTranscript),
        }),
      }),
    }),
    O = D.data?.fingerprint === d ? D.data.preview : null,
    k = F({
      mutationFn: async (t) =>
        An.save({
          resultId: e.resultId,
          previewId: t.previewId,
          directory: h,
          basename: _,
          formats: x,
          exportLayer: te,
        }),
      onSuccess: (e, t) => {
        (p(t.fingerprint), n.invalidateQueries({ queryKey: ln }));
      },
    }),
    ce = F({
      mutationFn: () => An.reset(e.resultId),
      onSuccess: () => {
        (u({ transcript: {}, source: {} }), p(Nn), D.reset(), k.reset());
      },
    }),
    A = F({
      mutationFn: () => An.recording({ resultId: e.resultId, path: ne }),
      onSuccess: (t) => {
        (re(``), n.setQueryData([`review`, e.resultId], t));
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
  let j =
      (a === `source`
        ? (O?.sourceTranscript ?? e.sourceTranscript)
        : (O?.transcript ?? e.transcript)
      )?.segments ?? [],
    le = a === `source` ? e.sourceTranscript : e.transcript,
    ue = new Set(
      j
        .filter(
          (e) =>
            e.startSeconds != null &&
            e.endSeconds != null &&
            ie >= e.startSeconds &&
            ie < e.endSeconds,
        )
        .map((e) => e.id),
    ),
    de = r.data?.profiles ?? [],
    fe = new Map();
  for (let t of [...e.transcript.segments, ...(e.sourceTranscript?.segments ?? [])])
    t.speaker && fe.set(t.speaker, t.speaker);
  for (let e of de)
    (fe.set(e.label, `${e.label} (${e.metadata.status ?? `unclassified`})`),
      fe.has(e.id) &&
        fe.set(e.id, `${e.label} (${e.metadata.status ?? `unclassified`}; stored ID)`));
  for (let e of Object.values(l))
    for (let t of Object.values(e)) t.speaker && !fe.has(t.speaker) && fe.set(t.speaker, t.speaker);
  let pe = D.error ?? k.error ?? ce.error ?? A.error ?? r.error;
  function me(e) {
    (c(e.id),
      E.current &&
        e.startSeconds != null &&
        ((E.current.currentTime = e.startSeconds), ae(E.current.currentTime)));
    let t = new URL(window.location.href);
    (t.searchParams.set(`segment`, e.id),
      t.searchParams.set(`layer`, a),
      window.history.replaceState(null, ``, t));
  }
  function he(e, t, n) {
    let r = le?.segments.find((t) => t.id === e.id);
    r &&
      (u((i) => {
        let o = { ...(i[a][e.id] ?? Pn(r)), [t]: n },
          s = { ...i[a] };
        return (
          JSON.stringify(o) === JSON.stringify(Pn(r)) ? delete s[e.id] : (s[e.id] = o),
          { ...i, [a]: s }
        );
      }),
      D.reset(),
      k.reset());
  }
  function ge() {
    let e = j.find((e) => e.id === s);
    E.current &&
      e?.startSeconds != null &&
      ((E.current.currentTime = e.startSeconds), ae(E.current.currentTime));
  }
  let _e = {
    "aria-label": `Source recording`,
    controls: !0,
    src: e.mediaUrl ?? void 0,
    onLoadedMetadata: ge,
    onTimeUpdate: () => {
      if (E.current) {
        let e = E.current.currentTime;
        ae(e);
        let t = j.find(
          (t) =>
            t.startSeconds != null &&
            t.endSeconds != null &&
            e >= t.startSeconds &&
            e < t.endSeconds,
        );
        t &&
          se.current !== t.id &&
          ((se.current = t.id),
          [`TEXTAREA`, `INPUT`, `SELECT`].includes(document.activeElement?.tagName ?? ``) ||
            oe.current.get(t.id)?.scrollIntoView?.({ block: `nearest` }));
      }
    },
    onError: () =>
      re(
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
      pe ? (0, b.jsx)(`p`, { role: `alert`, children: pe.message }) : null,
      (0, b.jsxs)(`div`, {
        className: `reviewActions`,
        children: [
          (0, b.jsx)(`button`, {
            type: `button`,
            disabled: ce.isPending || k.isPending || (!m && !O),
            onClick: () => ce.mutate(),
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
                      ..._e,
                      ref: (e) => {
                        E.current = e;
                      },
                    })
                  : (0, b.jsx)(`audio`, {
                      ..._e,
                      ref: (e) => {
                        E.current = e;
                      },
                    })
                : null,
              (0, b.jsxs)(`p`, { children: [`Playback `, Mn(ie)] }),
              e.mediaError || T
                ? (0, b.jsx)(`p`, { role: `alert`, children: e.mediaError ?? T })
                : null,
              (0, b.jsxs)(`details`, {
                open: !!e.mediaError || !!T,
                children: [
                  (0, b.jsx)(`summary`, { children: `Locate source recording` }),
                  (0, b.jsxs)(`label`, {
                    children: [
                      `Matching local recording path`,
                      (0, b.jsx)(`input`, { value: ne, onChange: (e) => w(e.currentTarget.value) }),
                    ],
                  }),
                  (0, b.jsx)(`button`, {
                    type: `button`,
                    disabled: !ne.trim() || A.isPending,
                    onClick: () => A.mutate(),
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
              j.length > 0 && s && !j.some((e) => e.id === s)
                ? (0, b.jsx)(`p`, {
                    children: `The selected segment was not found. Choose another segment below.`,
                  })
                : null,
              j.length
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
              j.map((e, t) => {
                let n = le?.segments.find((t) => t.id === e.id) ?? e,
                  r = l[a][e.id] ?? Pn(n),
                  i = s === e.id;
                return (0, b.jsxs)(
                  `article`,
                  {
                    className: `reviewSegment${i ? ` isSelected` : ``}${ue.has(e.id) ? ` isPlaying` : ``}`,
                    children: [
                      (0, b.jsxs)(`button`, {
                        type: `button`,
                        ref: (t) => {
                          t ? oe.current.set(e.id, t) : oe.current.delete(e.id);
                        },
                        tabIndex: i || (t === 0 && !j.some((e) => e.id === s)) ? 0 : -1,
                        "aria-label": `Seek to segment ${t + 1}`,
                        "aria-current": ue.has(e.id) ? `time` : void 0,
                        onClick: () => me(e),
                        onKeyDown: (e) => {
                          if (e.altKey || e.ctrlKey || e.metaKey) return;
                          let n =
                            e.key === `ArrowDown`
                              ? j[t + 1]
                              : e.key === `ArrowUp`
                                ? j[t - 1]
                                : e.key === `Home`
                                  ? j[0]
                                  : e.key === `End`
                                    ? j.at(-1)
                                    : void 0;
                          n && (e.preventDefault(), me(n), oe.current.get(n.id)?.focus());
                        },
                        children: [
                          `Segment `,
                          t + 1,
                          ` ·`,
                          ` `,
                          e.startSeconds == null ? `Untimed` : Mn(e.startSeconds),
                          ` →`,
                          ` `,
                          e.endSeconds == null ? `Untimed` : Mn(e.endSeconds),
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
                                    onChange: (t) => he(e, `text`, t.currentTarget.value),
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
                                        onChange: (t) => he(e, `start`, t.currentTarget.value),
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
                                        onChange: (t) => he(e, `end`, t.currentTarget.value),
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
                                    onChange: (t) => he(e, `speaker`, t.currentTarget.value),
                                    children: [
                                      (0, b.jsx)(`option`, { value: ``, children: `Unassigned` }),
                                      [...fe].map(([e, t]) =>
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
            disabled: D.isPending || !j.length,
            onClick: () => {
              (k.reset(), D.mutate({ fingerprint: d, drafts: l }));
            },
            children: `Preview changes`,
          }),
          O
            ? (0, b.jsxs)(b.Fragment, {
                children: [
                  (0, b.jsx)(`p`, {
                    role: `status`,
                    children: `Preview validated. Segment order is unchanged.`,
                  }),
                  O.clearedAlignment.length || O.clearedSourceAlignment.length
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
              te === `source` ? `Source` : e.isTranslation ? `Translation` : `Transcript`,
              `. Existing files and the original recording/result are never overwritten.`,
            ],
          }),
          e.sourceTranscript
            ? (0, b.jsxs)(`label`, {
                children: [
                  `Text/subtitle export layer`,
                  (0, b.jsxs)(`select`, {
                    "aria-label": `Text/subtitle export layer`,
                    value: te,
                    onChange: (e) => {
                      let t = e.currentTarget.value;
                      (t === `source` || t === `transcript`) && C(t);
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
                              S((n) =>
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
            disabled: !O || k.isPending || k.isSuccess,
            onClick: () => {
              O && k.mutate({ fingerprint: d, previewId: O.previewId });
            },
            children: `Save corrected copy`,
          }),
          k.data
            ? (0, b.jsxs)(`div`, {
                role: `status`,
                children: [
                  (0, b.jsxs)(`p`, {
                    children: [
                      `Corrected copy saved. `,
                      (0, b.jsx)(`a`, {
                        href: In(k.data.resultId),
                        children: `Open corrected copy`,
                      }),
                    ],
                  }),
                  (0, b.jsx)(`ul`, {
                    children: k.data.outputs.map((e) =>
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
function Bn({ kind: e, onChoose: t }) {
  let n = F({ mutationFn: Vt.files });
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
var Vn = [`speaker-directory-state`],
  Hn = {
    load: `Load speaker index`,
    new: `New speaker index`,
    export: `Save speaker index`,
    import: `Import speaker index`,
  },
  Un = { new: `Create empty library`, export: `Save copy`, import: `Import library` };
function Wn(e) {
  throw Error(`Unsupported Speaker Library operation.`);
}
function Gn({ api: e = Ot, manageHref: t, onIndexReplaced: n }) {
  let r = ee(),
    i = ct({ queryKey: Vn, queryFn: () => e.getState(), refetchOnMount: !1 }),
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
        ? (0, b.jsx)(qn, {
            action: a,
            onApplied: async (e) => {
              if (`state` in e) {
                let t = e.kind !== `import` || e.receipt.changed;
                (t && n?.(),
                  r.setQueryData(Vn, e.state),
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
                  Wn(e);
              }
              o(null);
            },
            onDismiss: () => o(null),
          })
        : null,
    ],
  });
}
function Kn({ title: e, onCancel: t, children: n }) {
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
function qn({ action: e, onApplied: t, onDismiss: n }) {
  let [r, i] = (0, y.useState)(`explicit`),
    [a, o] = (0, y.useState)(``),
    [s, c] = (0, y.useState)(`merge`),
    [l, u] = (0, y.useState)(!1),
    [d, f] = (0, y.useState)(!1),
    p = F({ mutationFn: Yt.preview }),
    m = F({ mutationFn: Yt.confirm, onSuccess: t }),
    h = F({ mutationFn: Yt.load, onSuccess: t }),
    g = F({ mutationFn: Yt.cancel }),
    _ = m.isPending || h.isPending,
    v = _ || p.isPending,
    x = () => {
      (p.data && g.mutate(p.data.id), p.reset(), m.reset(), u(!1));
    },
    ee = () => {
      _ || (p.data && g.mutate(p.data.id), n());
    },
    S = r === `explicit` ? { scope: r, path: a } : { scope: r },
    te = () => {
      if (e === `load`) {
        h.mutate(S);
        return;
      }
      let t;
      switch (e) {
        case `new`:
          t = { kind: `new`, selection: S };
          break;
        case `export`:
          t = { kind: `export`, destination: a };
          break;
        case `import`:
          t = { kind: `import`, source: a, mode: s };
      }
      p.mutate(t);
    },
    C = p.error ?? m.error ?? h.error ?? g.error,
    ne = p.data?.kind === `import` && !p.data.summary.changed;
  return (0, b.jsxs)(Kn, {
    title: Hn[e],
    onCancel: ee,
    children: [
      (0, b.jsxs)(`form`, {
        onSubmit: (e) => {
          (e.preventDefault(), e.stopPropagation(), te());
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
                        ? (0, b.jsx)(Bn, {
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
          C ? (0, b.jsx)(`p`, { role: `alert`, children: C.message }) : null,
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
              (0, b.jsx)(Yn, { preview: p.data }),
              ne
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
                          Jn(p.data),
                        ],
                      }),
                      (0, b.jsx)(`button`, {
                        type: `button`,
                        disabled: !l || _,
                        onClick: () => {
                          p.data && m.mutate(p.data.id);
                        },
                        children: Un[p.data.kind],
                      }),
                    ],
                  }),
            ],
          })
        : null,
      (0, b.jsx)(`button`, {
        type: `button`,
        disabled: _,
        onClick: ee,
        children: ne ? `Close` : `Cancel`,
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
function Jn(e) {
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
      return Wn(e);
  }
}
function Yn({ preview: e }) {
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
      return Wn(e);
  }
}
function Xn({ api: e = Ot }) {
  let [t] = (0, y.useState)(() => new Ye()),
    n = new URLSearchParams(window.location.search).get(`view`) ?? `speakers`;
  return (0, b.jsxs)(S, {
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
        ? (0, b.jsx)(bn, { speakerIndex: (0, b.jsx)(Gn, { api: e, manageHref: `?view=speakers` }) })
        : n === `jobs`
          ? (0, b.jsx)(hn, {})
          : n === `review`
            ? (0, b.jsx)(Ln, { api: e })
            : (0, b.jsx)(Zn, { api: e }),
    ],
  });
}
function Zn({ api: e }) {
  let [t, n] = (0, y.useState)(0),
    r = ct({ queryKey: [`speaker-directory-state`], queryFn: () => e.getState() });
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
      : (0, b.jsx)(Qn, {
          api: e,
          state: r.data,
          editorGeneration: t,
          onIndexReplaced: () => n((e) => e + 1),
        });
}
function Qn({ api: e, state: t, editorGeneration: n, onIndexReplaced: r }) {
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
      (0, b.jsx)(Gn, { api: e, onIndexReplaced: r }),
      (0, b.jsxs)(`section`, {
        className: `summaryGrid`,
        "aria-label": `Speaker Directory summary`,
        children: [
          (0, b.jsx)($n, {
            title: `Speaker Library`,
            status: t.library.status,
            detail: `${t.library.profileCount} profile${t.library.profileCount === 1 ? `` : `s`}`,
          }),
          (0, b.jsx)($n, {
            title: `Speaker Trace`,
            status: t.trace.status,
            detail: t.trace.scanRoot ?? `No scan root`,
          }),
          (0, b.jsx)($n, { title: `Scope`, status: t.scope, detail: `Speaker Directory` }),
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
              (0, b.jsx)(er, { api: e, profile: r }, JSON.stringify([t.path, n, r.id])),
            ),
          }),
        ],
      }),
      (0, b.jsx)(tr, { api: e, trace: t.trace }),
    ],
  });
}
function $n({ title: e, status: t, detail: n }) {
  return (0, b.jsxs)(`article`, {
    className: `statusPanel`,
    children: [
      (0, b.jsx)(`h2`, { children: e }),
      (0, b.jsx)(`p`, { className: `status`, children: t }),
      (0, b.jsx)(`p`, { children: n }),
    ],
  });
}
function er({ api: e, profile: t }) {
  let n = ee(),
    [r, i] = (0, y.useState)(t.label),
    [a, o] = (0, y.useState)(or(t.metadata)),
    [s, c] = (0, y.useState)(null),
    l = F({
      mutationFn: () => {
        let n = sr(a);
        return (c(null), e.updateProfile(t.id, { id: t.id, label: r, metadata: n }));
      },
      onSuccess: (e) => {
        n.setQueryData([`speaker-directory-state`], e);
      },
      onError: (e) => {
        c(e instanceof Error ? e.message : `Failed to save Speaker Library profile.`);
      },
    }),
    u = F({
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
                    sr(a);
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
function tr({ api: e, trace: t }) {
  let n = ee(),
    [r, i] = (0, y.useState)(``),
    [a, o] = (0, y.useState)(null),
    [s, c] = (0, y.useState)(null),
    l = t.speakers.filter((e) => e.kind === `enrolled`),
    u = t.speakers.filter((e) => e.kind === `anonymous`),
    d = F({
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
      s ? (0, b.jsx)(nr, { report: s }) : null,
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
                (0, b.jsx)(rr, { files: e.files }),
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
                (0, b.jsx)(rr, { files: e.files }),
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
function nr({ report: e }) {
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
function rr({ files: e }) {
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
                (0, b.jsx)(ir, { spans: e.spans }),
              ],
            },
            e.sourceFile,
          ),
        ),
      });
}
function ir({ spans: e }) {
  return e.length === 0
    ? (0, b.jsx)(`p`, { className: `muted`, children: `No spans recorded.` })
    : (0, b.jsx)(`ol`, {
        className: `traceSpanList`,
        children: e.map((e, t) =>
          (0, b.jsxs)(
            `li`,
            {
              children: [
                (0, b.jsx)(`span`, { className: `mono`, children: ar(e) }),
                (0, b.jsx)(`p`, { children: e.snippet }),
              ],
            },
            `${e.startSeconds ?? `unknown`}:${e.endSeconds ?? `unknown`}:${t}`,
          ),
        ),
      });
}
function ar(e) {
  return e.startSeconds === void 0 || e.endSeconds === void 0
    ? `Timing unavailable`
    : `${e.startSeconds.toFixed(2)}s - ${e.endSeconds.toFixed(2)}s`;
}
function or(e) {
  return Object.entries(e).map(([e, t]) => `${e}=${t}`).join(`
`);
}
function sr(e) {
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
(0, lt.createRoot)(document.getElementById(`root`)).render(
  (0, b.jsx)(y.StrictMode, { children: (0, b.jsx)(Xn, {}) }),
);
