(function () {
  const f = document.createElement("link").relList;
  if (f && f.supports && f.supports("modulepreload")) return;
  for (const g of document.querySelectorAll('link[rel="modulepreload"]')) r(g);
  new MutationObserver((g) => {
    for (const M of g)
      if (M.type === "childList")
        for (const U of M.addedNodes) U.tagName === "LINK" && U.rel === "modulepreload" && r(U);
  }).observe(document, { childList: !0, subtree: !0 });
  function o(g) {
    const M = {};
    return (
      g.integrity && (M.integrity = g.integrity),
      g.referrerPolicy && (M.referrerPolicy = g.referrerPolicy),
      g.crossOrigin === "use-credentials"
        ? (M.credentials = "include")
        : g.crossOrigin === "anonymous"
          ? (M.credentials = "omit")
          : (M.credentials = "same-origin"),
      M
    );
  }
  function r(g) {
    if (g.ep) return;
    g.ep = !0;
    const M = o(g);
    fetch(g.href, M);
  }
})();
var Sf = { exports: {} },
  Mu = {};
var Xh;
function Lv() {
  if (Xh) return Mu;
  Xh = 1;
  var i = Symbol.for("react.transitional.element"),
    f = Symbol.for("react.fragment");
  function o(r, g, M) {
    var U = null;
    if ((M !== void 0 && (U = "" + M), g.key !== void 0 && (U = "" + g.key), "key" in g)) {
      M = {};
      for (var Q in g) Q !== "key" && (M[Q] = g[Q]);
    } else M = g;
    return ((g = M.ref), { $$typeof: i, type: r, key: U, ref: g !== void 0 ? g : null, props: M });
  }
  return ((Mu.Fragment = f), (Mu.jsx = o), (Mu.jsxs = o), Mu);
}
var Lh;
function Zv() {
  return (Lh || ((Lh = 1), (Sf.exports = Lv())), Sf.exports);
}
var q = Zv(),
  pf = { exports: {} },
  V = {};
var Zh;
function Kv() {
  if (Zh) return V;
  Zh = 1;
  var i = Symbol.for("react.transitional.element"),
    f = Symbol.for("react.portal"),
    o = Symbol.for("react.fragment"),
    r = Symbol.for("react.strict_mode"),
    g = Symbol.for("react.profiler"),
    M = Symbol.for("react.consumer"),
    U = Symbol.for("react.context"),
    Q = Symbol.for("react.forward_ref"),
    z = Symbol.for("react.suspense"),
    T = Symbol.for("react.memo"),
    C = Symbol.for("react.lazy"),
    N = Symbol.for("react.activity"),
    R = Symbol.iterator;
  function lt(y) {
    return y === null || typeof y != "object"
      ? null
      : ((y = (R && y[R]) || y["@@iterator"]), typeof y == "function" ? y : null);
  }
  var W = {
      isMounted: function () {
        return !1;
      },
      enqueueForceUpdate: function () {},
      enqueueReplaceState: function () {},
      enqueueSetState: function () {},
    },
    Z = Object.assign,
    St = {};
  function st(y, D, j) {
    ((this.props = y), (this.context = D), (this.refs = St), (this.updater = j || W));
  }
  ((st.prototype.isReactComponent = {}),
    (st.prototype.setState = function (y, D) {
      if (typeof y != "object" && typeof y != "function" && y != null)
        throw Error(
          "takes an object of state variables to update or a function which returns an object of state variables.",
        );
      this.updater.enqueueSetState(this, y, D, "setState");
    }),
    (st.prototype.forceUpdate = function (y) {
      this.updater.enqueueForceUpdate(this, y, "forceUpdate");
    }));
  function Dt() {}
  Dt.prototype = st.prototype;
  function gt(y, D, j) {
    ((this.props = y), (this.context = D), (this.refs = St), (this.updater = j || W));
  }
  var Ut = (gt.prototype = new Dt());
  ((Ut.constructor = gt), Z(Ut, st.prototype), (Ut.isPureReactComponent = !0));
  var xt = Array.isArray;
  function Bt() {}
  var K = { H: null, A: null, T: null, S: null },
    yt = Object.prototype.hasOwnProperty;
  function Jt(y, D, j) {
    var B = j.ref;
    return { $$typeof: i, type: y, key: D, ref: B !== void 0 ? B : null, props: j };
  }
  function _e(y, D) {
    return Jt(y.type, D, y.props);
  }
  function ue(y) {
    return typeof y == "object" && y !== null && y.$$typeof === i;
  }
  function jt(y) {
    var D = { "=": "=0", ":": "=2" };
    return (
      "$" +
      y.replace(/[=:]/g, function (j) {
        return D[j];
      })
    );
  }
  var me = /\/+/g;
  function Qe(y, D) {
    return typeof y == "object" && y !== null && y.key != null ? jt("" + y.key) : D.toString(36);
  }
  function De(y) {
    switch (y.status) {
      case "fulfilled":
        return y.value;
      case "rejected":
        throw y.reason;
      default:
        switch (
          (typeof y.status == "string"
            ? y.then(Bt, Bt)
            : ((y.status = "pending"),
              y.then(
                function (D) {
                  y.status === "pending" && ((y.status = "fulfilled"), (y.value = D));
                },
                function (D) {
                  y.status === "pending" && ((y.status = "rejected"), (y.reason = D));
                },
              )),
          y.status)
        ) {
          case "fulfilled":
            return y.value;
          case "rejected":
            throw y.reason;
        }
    }
    throw y;
  }
  function O(y, D, j, B, J) {
    var $ = typeof y;
    ($ === "undefined" || $ === "boolean") && (y = null);
    var it = !1;
    if (y === null) it = !0;
    else
      switch ($) {
        case "bigint":
        case "string":
        case "number":
          it = !0;
          break;
        case "object":
          switch (y.$$typeof) {
            case i:
            case f:
              it = !0;
              break;
            case C:
              return ((it = y._init), O(it(y._payload), D, j, B, J));
          }
      }
    if (it)
      return (
        (J = J(y)),
        (it = B === "" ? "." + Qe(y, 0) : B),
        xt(J)
          ? ((j = ""),
            it != null && (j = it.replace(me, "$&/") + "/"),
            O(J, D, j, "", function (Ha) {
              return Ha;
            }))
          : J != null &&
            (ue(J) &&
              (J = _e(
                J,
                j +
                  (J.key == null || (y && y.key === J.key)
                    ? ""
                    : ("" + J.key).replace(me, "$&/") + "/") +
                  it,
              )),
            D.push(J)),
        1
      );
    it = 0;
    var Ft = B === "" ? "." : B + ":";
    if (xt(y))
      for (var At = 0; At < y.length; At++)
        ((B = y[At]), ($ = Ft + Qe(B, At)), (it += O(B, D, j, $, J)));
    else if (((At = lt(y)), typeof At == "function"))
      for (y = At.call(y), At = 0; !(B = y.next()).done; )
        ((B = B.value), ($ = Ft + Qe(B, At++)), (it += O(B, D, j, $, J)));
    else if ($ === "object") {
      if (typeof y.then == "function") return O(De(y), D, j, B, J);
      throw (
        (D = String(y)),
        Error(
          "Objects are not valid as a React child (found: " +
            (D === "[object Object]" ? "object with keys {" + Object.keys(y).join(", ") + "}" : D) +
            "). If you meant to render a collection of children, use an array instead.",
        )
      );
    }
    return it;
  }
  function H(y, D, j) {
    if (y == null) return y;
    var B = [],
      J = 0;
    return (
      O(y, B, "", "", function ($) {
        return D.call(j, $, J++);
      }),
      B
    );
  }
  function L(y) {
    if (y._status === -1) {
      var D = y._result;
      ((D = D()),
        D.then(
          function (j) {
            (y._status === 0 || y._status === -1) && ((y._status = 1), (y._result = j));
          },
          function (j) {
            (y._status === 0 || y._status === -1) && ((y._status = 2), (y._result = j));
          },
        ),
        y._status === -1 && ((y._status = 0), (y._result = D)));
    }
    if (y._status === 1) return y._result.default;
    throw y._result;
  }
  var rt =
      typeof reportError == "function"
        ? reportError
        : function (y) {
            if (typeof window == "object" && typeof window.ErrorEvent == "function") {
              var D = new window.ErrorEvent("error", {
                bubbles: !0,
                cancelable: !0,
                message:
                  typeof y == "object" && y !== null && typeof y.message == "string"
                    ? String(y.message)
                    : String(y),
                error: y,
              });
              if (!window.dispatchEvent(D)) return;
            } else if (typeof process == "object" && typeof process.emit == "function") {
              process.emit("uncaughtException", y);
              return;
            }
            console.error(y);
          },
    vt = {
      map: H,
      forEach: function (y, D, j) {
        H(
          y,
          function () {
            D.apply(this, arguments);
          },
          j,
        );
      },
      count: function (y) {
        var D = 0;
        return (
          H(y, function () {
            D++;
          }),
          D
        );
      },
      toArray: function (y) {
        return (
          H(y, function (D) {
            return D;
          }) || []
        );
      },
      only: function (y) {
        if (!ue(y))
          throw Error("React.Children.only expected to receive a single React element child.");
        return y;
      },
    };
  return (
    (V.Activity = N),
    (V.Children = vt),
    (V.Component = st),
    (V.Fragment = o),
    (V.Profiler = g),
    (V.PureComponent = gt),
    (V.StrictMode = r),
    (V.Suspense = z),
    (V.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE = K),
    (V.__COMPILER_RUNTIME = {
      __proto__: null,
      c: function (y) {
        return K.H.useMemoCache(y);
      },
    }),
    (V.cache = function (y) {
      return function () {
        return y.apply(null, arguments);
      };
    }),
    (V.cacheSignal = function () {
      return null;
    }),
    (V.cloneElement = function (y, D, j) {
      if (y == null) throw Error("The argument must be a React element, but you passed " + y + ".");
      var B = Z({}, y.props),
        J = y.key;
      if (D != null)
        for ($ in (D.key !== void 0 && (J = "" + D.key), D))
          !yt.call(D, $) ||
            $ === "key" ||
            $ === "__self" ||
            $ === "__source" ||
            ($ === "ref" && D.ref === void 0) ||
            (B[$] = D[$]);
      var $ = arguments.length - 2;
      if ($ === 1) B.children = j;
      else if (1 < $) {
        for (var it = Array($), Ft = 0; Ft < $; Ft++) it[Ft] = arguments[Ft + 2];
        B.children = it;
      }
      return Jt(y.type, J, B);
    }),
    (V.createContext = function (y) {
      return (
        (y = {
          $$typeof: U,
          _currentValue: y,
          _currentValue2: y,
          _threadCount: 0,
          Provider: null,
          Consumer: null,
        }),
        (y.Provider = y),
        (y.Consumer = { $$typeof: M, _context: y }),
        y
      );
    }),
    (V.createElement = function (y, D, j) {
      var B,
        J = {},
        $ = null;
      if (D != null)
        for (B in (D.key !== void 0 && ($ = "" + D.key), D))
          yt.call(D, B) && B !== "key" && B !== "__self" && B !== "__source" && (J[B] = D[B]);
      var it = arguments.length - 2;
      if (it === 1) J.children = j;
      else if (1 < it) {
        for (var Ft = Array(it), At = 0; At < it; At++) Ft[At] = arguments[At + 2];
        J.children = Ft;
      }
      if (y && y.defaultProps)
        for (B in ((it = y.defaultProps), it)) J[B] === void 0 && (J[B] = it[B]);
      return Jt(y, $, J);
    }),
    (V.createRef = function () {
      return { current: null };
    }),
    (V.forwardRef = function (y) {
      return { $$typeof: Q, render: y };
    }),
    (V.isValidElement = ue),
    (V.lazy = function (y) {
      return { $$typeof: C, _payload: { _status: -1, _result: y }, _init: L };
    }),
    (V.memo = function (y, D) {
      return { $$typeof: T, type: y, compare: D === void 0 ? null : D };
    }),
    (V.startTransition = function (y) {
      var D = K.T,
        j = {};
      K.T = j;
      try {
        var B = y(),
          J = K.S;
        (J !== null && J(j, B),
          typeof B == "object" && B !== null && typeof B.then == "function" && B.then(Bt, rt));
      } catch ($) {
        rt($);
      } finally {
        (D !== null && j.types !== null && (D.types = j.types), (K.T = D));
      }
    }),
    (V.unstable_useCacheRefresh = function () {
      return K.H.useCacheRefresh();
    }),
    (V.use = function (y) {
      return K.H.use(y);
    }),
    (V.useActionState = function (y, D, j) {
      return K.H.useActionState(y, D, j);
    }),
    (V.useCallback = function (y, D) {
      return K.H.useCallback(y, D);
    }),
    (V.useContext = function (y) {
      return K.H.useContext(y);
    }),
    (V.useDebugValue = function () {}),
    (V.useDeferredValue = function (y, D) {
      return K.H.useDeferredValue(y, D);
    }),
    (V.useEffect = function (y, D) {
      return K.H.useEffect(y, D);
    }),
    (V.useEffectEvent = function (y) {
      return K.H.useEffectEvent(y);
    }),
    (V.useId = function () {
      return K.H.useId();
    }),
    (V.useImperativeHandle = function (y, D, j) {
      return K.H.useImperativeHandle(y, D, j);
    }),
    (V.useInsertionEffect = function (y, D) {
      return K.H.useInsertionEffect(y, D);
    }),
    (V.useLayoutEffect = function (y, D) {
      return K.H.useLayoutEffect(y, D);
    }),
    (V.useMemo = function (y, D) {
      return K.H.useMemo(y, D);
    }),
    (V.useOptimistic = function (y, D) {
      return K.H.useOptimistic(y, D);
    }),
    (V.useReducer = function (y, D, j) {
      return K.H.useReducer(y, D, j);
    }),
    (V.useRef = function (y) {
      return K.H.useRef(y);
    }),
    (V.useState = function (y) {
      return K.H.useState(y);
    }),
    (V.useSyncExternalStore = function (y, D, j) {
      return K.H.useSyncExternalStore(y, D, j);
    }),
    (V.useTransition = function () {
      return K.H.useTransition();
    }),
    (V.version = "19.2.7"),
    V
  );
}
var Kh;
function Nf() {
  return (Kh || ((Kh = 1), (pf.exports = Kv())), pf.exports);
}
var Tt = Nf(),
  bf = { exports: {} },
  _u = {},
  Ef = { exports: {} },
  Tf = {};
var Vh;
function Vv() {
  return (
    Vh ||
      ((Vh = 1),
      (function (i) {
        function f(O, H) {
          var L = O.length;
          O.push(H);
          t: for (; 0 < L; ) {
            var rt = (L - 1) >>> 1,
              vt = O[rt];
            if (0 < g(vt, H)) ((O[rt] = H), (O[L] = vt), (L = rt));
            else break t;
          }
        }
        function o(O) {
          return O.length === 0 ? null : O[0];
        }
        function r(O) {
          if (O.length === 0) return null;
          var H = O[0],
            L = O.pop();
          if (L !== H) {
            O[0] = L;
            t: for (var rt = 0, vt = O.length, y = vt >>> 1; rt < y; ) {
              var D = 2 * (rt + 1) - 1,
                j = O[D],
                B = D + 1,
                J = O[B];
              if (0 > g(j, L))
                B < vt && 0 > g(J, j)
                  ? ((O[rt] = J), (O[B] = L), (rt = B))
                  : ((O[rt] = j), (O[D] = L), (rt = D));
              else if (B < vt && 0 > g(J, L)) ((O[rt] = J), (O[B] = L), (rt = B));
              else break t;
            }
          }
          return H;
        }
        function g(O, H) {
          var L = O.sortIndex - H.sortIndex;
          return L !== 0 ? L : O.id - H.id;
        }
        if (
          ((i.unstable_now = void 0),
          typeof performance == "object" && typeof performance.now == "function")
        ) {
          var M = performance;
          i.unstable_now = function () {
            return M.now();
          };
        } else {
          var U = Date,
            Q = U.now();
          i.unstable_now = function () {
            return U.now() - Q;
          };
        }
        var z = [],
          T = [],
          C = 1,
          N = null,
          R = 3,
          lt = !1,
          W = !1,
          Z = !1,
          St = !1,
          st = typeof setTimeout == "function" ? setTimeout : null,
          Dt = typeof clearTimeout == "function" ? clearTimeout : null,
          gt = typeof setImmediate < "u" ? setImmediate : null;
        function Ut(O) {
          for (var H = o(T); H !== null; ) {
            if (H.callback === null) r(T);
            else if (H.startTime <= O) (r(T), (H.sortIndex = H.expirationTime), f(z, H));
            else break;
            H = o(T);
          }
        }
        function xt(O) {
          if (((Z = !1), Ut(O), !W))
            if (o(z) !== null) ((W = !0), Bt || ((Bt = !0), jt()));
            else {
              var H = o(T);
              H !== null && De(xt, H.startTime - O);
            }
        }
        var Bt = !1,
          K = -1,
          yt = 5,
          Jt = -1;
        function _e() {
          return St ? !0 : !(i.unstable_now() - Jt < yt);
        }
        function ue() {
          if (((St = !1), Bt)) {
            var O = i.unstable_now();
            Jt = O;
            var H = !0;
            try {
              t: {
                ((W = !1), Z && ((Z = !1), Dt(K), (K = -1)), (lt = !0));
                var L = R;
                try {
                  e: {
                    for (Ut(O), N = o(z); N !== null && !(N.expirationTime > O && _e()); ) {
                      var rt = N.callback;
                      if (typeof rt == "function") {
                        ((N.callback = null), (R = N.priorityLevel));
                        var vt = rt(N.expirationTime <= O);
                        if (((O = i.unstable_now()), typeof vt == "function")) {
                          ((N.callback = vt), Ut(O), (H = !0));
                          break e;
                        }
                        (N === o(z) && r(z), Ut(O));
                      } else r(z);
                      N = o(z);
                    }
                    if (N !== null) H = !0;
                    else {
                      var y = o(T);
                      (y !== null && De(xt, y.startTime - O), (H = !1));
                    }
                  }
                  break t;
                } finally {
                  ((N = null), (R = L), (lt = !1));
                }
                H = void 0;
              }
            } finally {
              H ? jt() : (Bt = !1);
            }
          }
        }
        var jt;
        if (typeof gt == "function")
          jt = function () {
            gt(ue);
          };
        else if (typeof MessageChannel < "u") {
          var me = new MessageChannel(),
            Qe = me.port2;
          ((me.port1.onmessage = ue),
            (jt = function () {
              Qe.postMessage(null);
            }));
        } else
          jt = function () {
            st(ue, 0);
          };
        function De(O, H) {
          K = st(function () {
            O(i.unstable_now());
          }, H);
        }
        ((i.unstable_IdlePriority = 5),
          (i.unstable_ImmediatePriority = 1),
          (i.unstable_LowPriority = 4),
          (i.unstable_NormalPriority = 3),
          (i.unstable_Profiling = null),
          (i.unstable_UserBlockingPriority = 2),
          (i.unstable_cancelCallback = function (O) {
            O.callback = null;
          }),
          (i.unstable_forceFrameRate = function (O) {
            0 > O || 125 < O
              ? console.error(
                  "forceFrameRate takes a positive int between 0 and 125, forcing frame rates higher than 125 fps is not supported",
                )
              : (yt = 0 < O ? Math.floor(1e3 / O) : 5);
          }),
          (i.unstable_getCurrentPriorityLevel = function () {
            return R;
          }),
          (i.unstable_next = function (O) {
            switch (R) {
              case 1:
              case 2:
              case 3:
                var H = 3;
                break;
              default:
                H = R;
            }
            var L = R;
            R = H;
            try {
              return O();
            } finally {
              R = L;
            }
          }),
          (i.unstable_requestPaint = function () {
            St = !0;
          }),
          (i.unstable_runWithPriority = function (O, H) {
            switch (O) {
              case 1:
              case 2:
              case 3:
              case 4:
              case 5:
                break;
              default:
                O = 3;
            }
            var L = R;
            R = O;
            try {
              return H();
            } finally {
              R = L;
            }
          }),
          (i.unstable_scheduleCallback = function (O, H, L) {
            var rt = i.unstable_now();
            switch (
              (typeof L == "object" && L !== null
                ? ((L = L.delay), (L = typeof L == "number" && 0 < L ? rt + L : rt))
                : (L = rt),
              O)
            ) {
              case 1:
                var vt = -1;
                break;
              case 2:
                vt = 250;
                break;
              case 5:
                vt = 1073741823;
                break;
              case 4:
                vt = 1e4;
                break;
              default:
                vt = 5e3;
            }
            return (
              (vt = L + vt),
              (O = {
                id: C++,
                callback: H,
                priorityLevel: O,
                startTime: L,
                expirationTime: vt,
                sortIndex: -1,
              }),
              L > rt
                ? ((O.sortIndex = L),
                  f(T, O),
                  o(z) === null && O === o(T) && (Z ? (Dt(K), (K = -1)) : (Z = !0), De(xt, L - rt)))
                : ((O.sortIndex = vt), f(z, O), W || lt || ((W = !0), Bt || ((Bt = !0), jt()))),
              O
            );
          }),
          (i.unstable_shouldYield = _e),
          (i.unstable_wrapCallback = function (O) {
            var H = R;
            return function () {
              var L = R;
              R = H;
              try {
                return O.apply(this, arguments);
              } finally {
                R = L;
              }
            };
          }));
      })(Tf)),
    Tf
  );
}
var Jh;
function Jv() {
  return (Jh || ((Jh = 1), (Ef.exports = Vv())), Ef.exports);
}
var Of = { exports: {} },
  wt = {};
var wh;
function wv() {
  if (wh) return wt;
  wh = 1;
  var i = Nf();
  function f(z) {
    var T = "https://react.dev/errors/" + z;
    if (1 < arguments.length) {
      T += "?args[]=" + encodeURIComponent(arguments[1]);
      for (var C = 2; C < arguments.length; C++) T += "&args[]=" + encodeURIComponent(arguments[C]);
    }
    return (
      "Minified React error #" +
      z +
      "; visit " +
      T +
      " for the full message or use the non-minified dev environment for full errors and additional helpful warnings."
    );
  }
  function o() {}
  var r = {
      d: {
        f: o,
        r: function () {
          throw Error(f(522));
        },
        D: o,
        C: o,
        L: o,
        m: o,
        X: o,
        S: o,
        M: o,
      },
      p: 0,
      findDOMNode: null,
    },
    g = Symbol.for("react.portal");
  function M(z, T, C) {
    var N = 3 < arguments.length && arguments[3] !== void 0 ? arguments[3] : null;
    return {
      $$typeof: g,
      key: N == null ? null : "" + N,
      children: z,
      containerInfo: T,
      implementation: C,
    };
  }
  var U = i.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE;
  function Q(z, T) {
    if (z === "font") return "";
    if (typeof T == "string") return T === "use-credentials" ? T : "";
  }
  return (
    (wt.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE = r),
    (wt.createPortal = function (z, T) {
      var C = 2 < arguments.length && arguments[2] !== void 0 ? arguments[2] : null;
      if (!T || (T.nodeType !== 1 && T.nodeType !== 9 && T.nodeType !== 11)) throw Error(f(299));
      return M(z, T, null, C);
    }),
    (wt.flushSync = function (z) {
      var T = U.T,
        C = r.p;
      try {
        if (((U.T = null), (r.p = 2), z)) return z();
      } finally {
        ((U.T = T), (r.p = C), r.d.f());
      }
    }),
    (wt.preconnect = function (z, T) {
      typeof z == "string" &&
        (T
          ? ((T = T.crossOrigin),
            (T = typeof T == "string" ? (T === "use-credentials" ? T : "") : void 0))
          : (T = null),
        r.d.C(z, T));
    }),
    (wt.prefetchDNS = function (z) {
      typeof z == "string" && r.d.D(z);
    }),
    (wt.preinit = function (z, T) {
      if (typeof z == "string" && T && typeof T.as == "string") {
        var C = T.as,
          N = Q(C, T.crossOrigin),
          R = typeof T.integrity == "string" ? T.integrity : void 0,
          lt = typeof T.fetchPriority == "string" ? T.fetchPriority : void 0;
        C === "style"
          ? r.d.S(z, typeof T.precedence == "string" ? T.precedence : void 0, {
              crossOrigin: N,
              integrity: R,
              fetchPriority: lt,
            })
          : C === "script" &&
            r.d.X(z, {
              crossOrigin: N,
              integrity: R,
              fetchPriority: lt,
              nonce: typeof T.nonce == "string" ? T.nonce : void 0,
            });
      }
    }),
    (wt.preinitModule = function (z, T) {
      if (typeof z == "string")
        if (typeof T == "object" && T !== null) {
          if (T.as == null || T.as === "script") {
            var C = Q(T.as, T.crossOrigin);
            r.d.M(z, {
              crossOrigin: C,
              integrity: typeof T.integrity == "string" ? T.integrity : void 0,
              nonce: typeof T.nonce == "string" ? T.nonce : void 0,
            });
          }
        } else T == null && r.d.M(z);
    }),
    (wt.preload = function (z, T) {
      if (typeof z == "string" && typeof T == "object" && T !== null && typeof T.as == "string") {
        var C = T.as,
          N = Q(C, T.crossOrigin);
        r.d.L(z, C, {
          crossOrigin: N,
          integrity: typeof T.integrity == "string" ? T.integrity : void 0,
          nonce: typeof T.nonce == "string" ? T.nonce : void 0,
          type: typeof T.type == "string" ? T.type : void 0,
          fetchPriority: typeof T.fetchPriority == "string" ? T.fetchPriority : void 0,
          referrerPolicy: typeof T.referrerPolicy == "string" ? T.referrerPolicy : void 0,
          imageSrcSet: typeof T.imageSrcSet == "string" ? T.imageSrcSet : void 0,
          imageSizes: typeof T.imageSizes == "string" ? T.imageSizes : void 0,
          media: typeof T.media == "string" ? T.media : void 0,
        });
      }
    }),
    (wt.preloadModule = function (z, T) {
      if (typeof z == "string")
        if (T) {
          var C = Q(T.as, T.crossOrigin);
          r.d.m(z, {
            as: typeof T.as == "string" && T.as !== "script" ? T.as : void 0,
            crossOrigin: C,
            integrity: typeof T.integrity == "string" ? T.integrity : void 0,
          });
        } else r.d.m(z);
    }),
    (wt.requestFormReset = function (z) {
      r.d.r(z);
    }),
    (wt.unstable_batchedUpdates = function (z, T) {
      return z(T);
    }),
    (wt.useFormState = function (z, T, C) {
      return U.H.useFormState(z, T, C);
    }),
    (wt.useFormStatus = function () {
      return U.H.useHostTransitionStatus();
    }),
    (wt.version = "19.2.7"),
    wt
  );
}
var Fh;
function Fv() {
  if (Fh) return Of.exports;
  Fh = 1;
  function i() {
    if (
      !(
        typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ > "u" ||
        typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE != "function"
      )
    )
      try {
        __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(i);
      } catch (f) {
        console.error(f);
      }
  }
  return (i(), (Of.exports = wv()), Of.exports);
}
var Wh;
function Wv() {
  if (Wh) return _u;
  Wh = 1;
  var i = Jv(),
    f = Nf(),
    o = Fv();
  function r(t) {
    var e = "https://react.dev/errors/" + t;
    if (1 < arguments.length) {
      e += "?args[]=" + encodeURIComponent(arguments[1]);
      for (var l = 2; l < arguments.length; l++) e += "&args[]=" + encodeURIComponent(arguments[l]);
    }
    return (
      "Minified React error #" +
      t +
      "; visit " +
      e +
      " for the full message or use the non-minified dev environment for full errors and additional helpful warnings."
    );
  }
  function g(t) {
    return !(!t || (t.nodeType !== 1 && t.nodeType !== 9 && t.nodeType !== 11));
  }
  function M(t) {
    var e = t,
      l = t;
    if (t.alternate) for (; e.return; ) e = e.return;
    else {
      t = e;
      do ((e = t), (e.flags & 4098) !== 0 && (l = e.return), (t = e.return));
      while (t);
    }
    return e.tag === 3 ? l : null;
  }
  function U(t) {
    if (t.tag === 13) {
      var e = t.memoizedState;
      if ((e === null && ((t = t.alternate), t !== null && (e = t.memoizedState)), e !== null))
        return e.dehydrated;
    }
    return null;
  }
  function Q(t) {
    if (t.tag === 31) {
      var e = t.memoizedState;
      if ((e === null && ((t = t.alternate), t !== null && (e = t.memoizedState)), e !== null))
        return e.dehydrated;
    }
    return null;
  }
  function z(t) {
    if (M(t) !== t) throw Error(r(188));
  }
  function T(t) {
    var e = t.alternate;
    if (!e) {
      if (((e = M(t)), e === null)) throw Error(r(188));
      return e !== t ? null : t;
    }
    for (var l = t, a = e; ; ) {
      var u = l.return;
      if (u === null) break;
      var n = u.alternate;
      if (n === null) {
        if (((a = u.return), a !== null)) {
          l = a;
          continue;
        }
        break;
      }
      if (u.child === n.child) {
        for (n = u.child; n; ) {
          if (n === l) return (z(u), t);
          if (n === a) return (z(u), e);
          n = n.sibling;
        }
        throw Error(r(188));
      }
      if (l.return !== a.return) ((l = u), (a = n));
      else {
        for (var c = !1, s = u.child; s; ) {
          if (s === l) {
            ((c = !0), (l = u), (a = n));
            break;
          }
          if (s === a) {
            ((c = !0), (a = u), (l = n));
            break;
          }
          s = s.sibling;
        }
        if (!c) {
          for (s = n.child; s; ) {
            if (s === l) {
              ((c = !0), (l = n), (a = u));
              break;
            }
            if (s === a) {
              ((c = !0), (a = n), (l = u));
              break;
            }
            s = s.sibling;
          }
          if (!c) throw Error(r(189));
        }
      }
      if (l.alternate !== a) throw Error(r(190));
    }
    if (l.tag !== 3) throw Error(r(188));
    return l.stateNode.current === l ? t : e;
  }
  function C(t) {
    var e = t.tag;
    if (e === 5 || e === 26 || e === 27 || e === 6) return t;
    for (t = t.child; t !== null; ) {
      if (((e = C(t)), e !== null)) return e;
      t = t.sibling;
    }
    return null;
  }
  var N = Object.assign,
    R = Symbol.for("react.element"),
    lt = Symbol.for("react.transitional.element"),
    W = Symbol.for("react.portal"),
    Z = Symbol.for("react.fragment"),
    St = Symbol.for("react.strict_mode"),
    st = Symbol.for("react.profiler"),
    Dt = Symbol.for("react.consumer"),
    gt = Symbol.for("react.context"),
    Ut = Symbol.for("react.forward_ref"),
    xt = Symbol.for("react.suspense"),
    Bt = Symbol.for("react.suspense_list"),
    K = Symbol.for("react.memo"),
    yt = Symbol.for("react.lazy"),
    Jt = Symbol.for("react.activity"),
    _e = Symbol.for("react.memo_cache_sentinel"),
    ue = Symbol.iterator;
  function jt(t) {
    return t === null || typeof t != "object"
      ? null
      : ((t = (ue && t[ue]) || t["@@iterator"]), typeof t == "function" ? t : null);
  }
  var me = Symbol.for("react.client.reference");
  function Qe(t) {
    if (t == null) return null;
    if (typeof t == "function") return t.$$typeof === me ? null : t.displayName || t.name || null;
    if (typeof t == "string") return t;
    switch (t) {
      case Z:
        return "Fragment";
      case st:
        return "Profiler";
      case St:
        return "StrictMode";
      case xt:
        return "Suspense";
      case Bt:
        return "SuspenseList";
      case Jt:
        return "Activity";
    }
    if (typeof t == "object")
      switch (t.$$typeof) {
        case W:
          return "Portal";
        case gt:
          return t.displayName || "Context";
        case Dt:
          return (t._context.displayName || "Context") + ".Consumer";
        case Ut:
          var e = t.render;
          return (
            (t = t.displayName),
            t ||
              ((t = e.displayName || e.name || ""),
              (t = t !== "" ? "ForwardRef(" + t + ")" : "ForwardRef")),
            t
          );
        case K:
          return ((e = t.displayName || null), e !== null ? e : Qe(t.type) || "Memo");
        case yt:
          ((e = t._payload), (t = t._init));
          try {
            return Qe(t(e));
          } catch {}
      }
    return null;
  }
  var De = Array.isArray,
    O = f.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,
    H = o.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,
    L = { pending: !1, data: null, method: null, action: null },
    rt = [],
    vt = -1;
  function y(t) {
    return { current: t };
  }
  function D(t) {
    0 > vt || ((t.current = rt[vt]), (rt[vt] = null), vt--);
  }
  function j(t, e) {
    (vt++, (rt[vt] = t.current), (t.current = e));
  }
  var B = y(null),
    J = y(null),
    $ = y(null),
    it = y(null);
  function Ft(t, e) {
    switch ((j($, e), j(J, t), j(B, null), e.nodeType)) {
      case 9:
      case 11:
        t = (t = e.documentElement) && (t = t.namespaceURI) ? rh(t) : 0;
        break;
      default:
        if (((t = e.tagName), (e = e.namespaceURI))) ((e = rh(e)), (t = oh(e, t)));
        else
          switch (t) {
            case "svg":
              t = 1;
              break;
            case "math":
              t = 2;
              break;
            default:
              t = 0;
          }
    }
    (D(B), j(B, t));
  }
  function At() {
    (D(B), D(J), D($));
  }
  function Ha(t) {
    t.memoizedState !== null && j(it, t);
    var e = B.current,
      l = oh(e, t.type);
    e !== l && (j(J, t), j(B, l));
  }
  function Cu(t) {
    (J.current === t && (D(B), D(J)), it.current === t && (D(it), (Tu._currentValue = L)));
  }
  var Pn, Yf;
  function _l(t) {
    if (Pn === void 0)
      try {
        throw Error();
      } catch (l) {
        var e = l.stack.trim().match(/\n( *(at )?)/);
        ((Pn = (e && e[1]) || ""),
          (Yf =
            -1 <
            l.stack.indexOf(`
    at`)
              ? " (<anonymous>)"
              : -1 < l.stack.indexOf("@")
                ? "@unknown:0:0"
                : ""));
      }
    return (
      `
` +
      Pn +
      t +
      Yf
    );
  }
  var ti = !1;
  function ei(t, e) {
    if (!t || ti) return "";
    ti = !0;
    var l = Error.prepareStackTrace;
    Error.prepareStackTrace = void 0;
    try {
      var a = {
        DetermineComponentFrameRoot: function () {
          try {
            if (e) {
              var _ = function () {
                throw Error();
              };
              if (
                (Object.defineProperty(_.prototype, "props", {
                  set: function () {
                    throw Error();
                  },
                }),
                typeof Reflect == "object" && Reflect.construct)
              ) {
                try {
                  Reflect.construct(_, []);
                } catch (b) {
                  var p = b;
                }
                Reflect.construct(t, [], _);
              } else {
                try {
                  _.call();
                } catch (b) {
                  p = b;
                }
                t.call(_.prototype);
              }
            } else {
              try {
                throw Error();
              } catch (b) {
                p = b;
              }
              (_ = t()) && typeof _.catch == "function" && _.catch(function () {});
            }
          } catch (b) {
            if (b && p && typeof b.stack == "string") return [b.stack, p.stack];
          }
          return [null, null];
        },
      };
      a.DetermineComponentFrameRoot.displayName = "DetermineComponentFrameRoot";
      var u = Object.getOwnPropertyDescriptor(a.DetermineComponentFrameRoot, "name");
      u &&
        u.configurable &&
        Object.defineProperty(a.DetermineComponentFrameRoot, "name", {
          value: "DetermineComponentFrameRoot",
        });
      var n = a.DetermineComponentFrameRoot(),
        c = n[0],
        s = n[1];
      if (c && s) {
        var h = c.split(`
`),
          S = s.split(`
`);
        for (u = a = 0; a < h.length && !h[a].includes("DetermineComponentFrameRoot"); ) a++;
        for (; u < S.length && !S[u].includes("DetermineComponentFrameRoot"); ) u++;
        if (a === h.length || u === S.length)
          for (a = h.length - 1, u = S.length - 1; 1 <= a && 0 <= u && h[a] !== S[u]; ) u--;
        for (; 1 <= a && 0 <= u; a--, u--)
          if (h[a] !== S[u]) {
            if (a !== 1 || u !== 1)
              do
                if ((a--, u--, 0 > u || h[a] !== S[u])) {
                  var E =
                    `
` + h[a].replace(" at new ", " at ");
                  return (
                    t.displayName &&
                      E.includes("<anonymous>") &&
                      (E = E.replace("<anonymous>", t.displayName)),
                    E
                  );
                }
              while (1 <= a && 0 <= u);
            break;
          }
      }
    } finally {
      ((ti = !1), (Error.prepareStackTrace = l));
    }
    return (l = t ? t.displayName || t.name : "") ? _l(l) : "";
  }
  function pd(t, e) {
    switch (t.tag) {
      case 26:
      case 27:
      case 5:
        return _l(t.type);
      case 16:
        return _l("Lazy");
      case 13:
        return t.child !== e && e !== null ? _l("Suspense Fallback") : _l("Suspense");
      case 19:
        return _l("SuspenseList");
      case 0:
      case 15:
        return ei(t.type, !1);
      case 11:
        return ei(t.type.render, !1);
      case 1:
        return ei(t.type, !0);
      case 31:
        return _l("Activity");
      default:
        return "";
    }
  }
  function Gf(t) {
    try {
      var e = "",
        l = null;
      do ((e += pd(t, l)), (l = t), (t = t.return));
      while (t);
      return e;
    } catch (a) {
      return (
        `
Error generating stack: ` +
        a.message +
        `
` +
        a.stack
      );
    }
  }
  var li = Object.prototype.hasOwnProperty,
    ai = i.unstable_scheduleCallback,
    ui = i.unstable_cancelCallback,
    bd = i.unstable_shouldYield,
    Ed = i.unstable_requestPaint,
    ne = i.unstable_now,
    Td = i.unstable_getCurrentPriorityLevel,
    Xf = i.unstable_ImmediatePriority,
    Lf = i.unstable_UserBlockingPriority,
    Nu = i.unstable_NormalPriority,
    Od = i.unstable_LowPriority,
    Zf = i.unstable_IdlePriority,
    zd = i.log,
    Ad = i.unstable_setDisableYieldValue,
    ja = null,
    ie = null;
  function el(t) {
    if ((typeof zd == "function" && Ad(t), ie && typeof ie.setStrictMode == "function"))
      try {
        ie.setStrictMode(ja, t);
      } catch {}
  }
  var ce = Math.clz32 ? Math.clz32 : Dd,
    Md = Math.log,
    _d = Math.LN2;
  function Dd(t) {
    return ((t >>>= 0), t === 0 ? 32 : (31 - ((Md(t) / _d) | 0)) | 0);
  }
  var Hu = 256,
    ju = 262144,
    qu = 4194304;
  function Dl(t) {
    var e = t & 42;
    if (e !== 0) return e;
    switch (t & -t) {
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
        return t & 261888;
      case 262144:
      case 524288:
      case 1048576:
      case 2097152:
        return t & 3932160;
      case 4194304:
      case 8388608:
      case 16777216:
      case 33554432:
        return t & 62914560;
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
        return t;
    }
  }
  function Qu(t, e, l) {
    var a = t.pendingLanes;
    if (a === 0) return 0;
    var u = 0,
      n = t.suspendedLanes,
      c = t.pingedLanes;
    t = t.warmLanes;
    var s = a & 134217727;
    return (
      s !== 0
        ? ((a = s & ~n),
          a !== 0
            ? (u = Dl(a))
            : ((c &= s), c !== 0 ? (u = Dl(c)) : l || ((l = s & ~t), l !== 0 && (u = Dl(l)))))
        : ((s = a & ~n),
          s !== 0
            ? (u = Dl(s))
            : c !== 0
              ? (u = Dl(c))
              : l || ((l = a & ~t), l !== 0 && (u = Dl(l)))),
      u === 0
        ? 0
        : e !== 0 &&
            e !== u &&
            (e & n) === 0 &&
            ((n = u & -u), (l = e & -e), n >= l || (n === 32 && (l & 4194048) !== 0))
          ? e
          : u
    );
  }
  function qa(t, e) {
    return (t.pendingLanes & ~(t.suspendedLanes & ~t.pingedLanes) & e) === 0;
  }
  function Ud(t, e) {
    switch (t) {
      case 1:
      case 2:
      case 4:
      case 8:
      case 64:
        return e + 250;
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
        return e + 5e3;
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
  function Kf() {
    var t = qu;
    return ((qu <<= 1), (qu & 62914560) === 0 && (qu = 4194304), t);
  }
  function ni(t) {
    for (var e = [], l = 0; 31 > l; l++) e.push(t);
    return e;
  }
  function Qa(t, e) {
    ((t.pendingLanes |= e),
      e !== 268435456 && ((t.suspendedLanes = 0), (t.pingedLanes = 0), (t.warmLanes = 0)));
  }
  function Rd(t, e, l, a, u, n) {
    var c = t.pendingLanes;
    ((t.pendingLanes = l),
      (t.suspendedLanes = 0),
      (t.pingedLanes = 0),
      (t.warmLanes = 0),
      (t.expiredLanes &= l),
      (t.entangledLanes &= l),
      (t.errorRecoveryDisabledLanes &= l),
      (t.shellSuspendCounter = 0));
    var s = t.entanglements,
      h = t.expirationTimes,
      S = t.hiddenUpdates;
    for (l = c & ~l; 0 < l; ) {
      var E = 31 - ce(l),
        _ = 1 << E;
      ((s[E] = 0), (h[E] = -1));
      var p = S[E];
      if (p !== null)
        for (S[E] = null, E = 0; E < p.length; E++) {
          var b = p[E];
          b !== null && (b.lane &= -536870913);
        }
      l &= ~_;
    }
    (a !== 0 && Vf(t, a, 0),
      n !== 0 && u === 0 && t.tag !== 0 && (t.suspendedLanes |= n & ~(c & ~e)));
  }
  function Vf(t, e, l) {
    ((t.pendingLanes |= e), (t.suspendedLanes &= ~e));
    var a = 31 - ce(e);
    ((t.entangledLanes |= e),
      (t.entanglements[a] = t.entanglements[a] | 1073741824 | (l & 261930)));
  }
  function Jf(t, e) {
    var l = (t.entangledLanes |= e);
    for (t = t.entanglements; l; ) {
      var a = 31 - ce(l),
        u = 1 << a;
      ((u & e) | (t[a] & e) && (t[a] |= e), (l &= ~u));
    }
  }
  function wf(t, e) {
    var l = e & -e;
    return ((l = (l & 42) !== 0 ? 1 : ii(l)), (l & (t.suspendedLanes | e)) !== 0 ? 0 : l);
  }
  function ii(t) {
    switch (t) {
      case 2:
        t = 1;
        break;
      case 8:
        t = 4;
        break;
      case 32:
        t = 16;
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
        t = 128;
        break;
      case 268435456:
        t = 134217728;
        break;
      default:
        t = 0;
    }
    return t;
  }
  function ci(t) {
    return ((t &= -t), 2 < t ? (8 < t ? ((t & 134217727) !== 0 ? 32 : 268435456) : 8) : 2);
  }
  function Ff() {
    var t = H.p;
    return t !== 0 ? t : ((t = window.event), t === void 0 ? 32 : jh(t.type));
  }
  function Wf(t, e) {
    var l = H.p;
    try {
      return ((H.p = t), e());
    } finally {
      H.p = l;
    }
  }
  var ll = Math.random().toString(36).slice(2),
    Xt = "__reactFiber$" + ll,
    kt = "__reactProps$" + ll,
    Fl = "__reactContainer$" + ll,
    fi = "__reactEvents$" + ll,
    Cd = "__reactListeners$" + ll,
    Nd = "__reactHandles$" + ll,
    $f = "__reactResources$" + ll,
    xa = "__reactMarker$" + ll;
  function si(t) {
    (delete t[Xt], delete t[kt], delete t[fi], delete t[Cd], delete t[Nd]);
  }
  function Wl(t) {
    var e = t[Xt];
    if (e) return e;
    for (var l = t.parentNode; l; ) {
      if ((e = l[Fl] || l[Xt])) {
        if (((l = e.alternate), e.child !== null || (l !== null && l.child !== null)))
          for (t = Sh(t); t !== null; ) {
            if ((l = t[Xt])) return l;
            t = Sh(t);
          }
        return e;
      }
      ((t = l), (l = t.parentNode));
    }
    return null;
  }
  function $l(t) {
    if ((t = t[Xt] || t[Fl])) {
      var e = t.tag;
      if (e === 5 || e === 6 || e === 13 || e === 31 || e === 26 || e === 27 || e === 3) return t;
    }
    return null;
  }
  function Ba(t) {
    var e = t.tag;
    if (e === 5 || e === 26 || e === 27 || e === 6) return t.stateNode;
    throw Error(r(33));
  }
  function kl(t) {
    var e = t[$f];
    return (e || (e = t[$f] = { hoistableStyles: new Map(), hoistableScripts: new Map() }), e);
  }
  function Yt(t) {
    t[xa] = !0;
  }
  var kf = new Set(),
    If = {};
  function Ul(t, e) {
    (Il(t, e), Il(t + "Capture", e));
  }
  function Il(t, e) {
    for (If[t] = e, t = 0; t < e.length; t++) kf.add(e[t]);
  }
  var Hd = RegExp(
      "^[:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD][:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040]*$",
    ),
    Pf = {},
    ts = {};
  function jd(t) {
    return li.call(ts, t)
      ? !0
      : li.call(Pf, t)
        ? !1
        : Hd.test(t)
          ? (ts[t] = !0)
          : ((Pf[t] = !0), !1);
  }
  function xu(t, e, l) {
    if (jd(e))
      if (l === null) t.removeAttribute(e);
      else {
        switch (typeof l) {
          case "undefined":
          case "function":
          case "symbol":
            t.removeAttribute(e);
            return;
          case "boolean":
            var a = e.toLowerCase().slice(0, 5);
            if (a !== "data-" && a !== "aria-") {
              t.removeAttribute(e);
              return;
            }
        }
        t.setAttribute(e, "" + l);
      }
  }
  function Bu(t, e, l) {
    if (l === null) t.removeAttribute(e);
    else {
      switch (typeof l) {
        case "undefined":
        case "function":
        case "symbol":
        case "boolean":
          t.removeAttribute(e);
          return;
      }
      t.setAttribute(e, "" + l);
    }
  }
  function xe(t, e, l, a) {
    if (a === null) t.removeAttribute(l);
    else {
      switch (typeof a) {
        case "undefined":
        case "function":
        case "symbol":
        case "boolean":
          t.removeAttribute(l);
          return;
      }
      t.setAttributeNS(e, l, "" + a);
    }
  }
  function ge(t) {
    switch (typeof t) {
      case "bigint":
      case "boolean":
      case "number":
      case "string":
      case "undefined":
        return t;
      case "object":
        return t;
      default:
        return "";
    }
  }
  function es(t) {
    var e = t.type;
    return (t = t.nodeName) && t.toLowerCase() === "input" && (e === "checkbox" || e === "radio");
  }
  function qd(t, e, l) {
    var a = Object.getOwnPropertyDescriptor(t.constructor.prototype, e);
    if (
      !t.hasOwnProperty(e) &&
      typeof a < "u" &&
      typeof a.get == "function" &&
      typeof a.set == "function"
    ) {
      var u = a.get,
        n = a.set;
      return (
        Object.defineProperty(t, e, {
          configurable: !0,
          get: function () {
            return u.call(this);
          },
          set: function (c) {
            ((l = "" + c), n.call(this, c));
          },
        }),
        Object.defineProperty(t, e, { enumerable: a.enumerable }),
        {
          getValue: function () {
            return l;
          },
          setValue: function (c) {
            l = "" + c;
          },
          stopTracking: function () {
            ((t._valueTracker = null), delete t[e]);
          },
        }
      );
    }
  }
  function ri(t) {
    if (!t._valueTracker) {
      var e = es(t) ? "checked" : "value";
      t._valueTracker = qd(t, e, "" + t[e]);
    }
  }
  function ls(t) {
    if (!t) return !1;
    var e = t._valueTracker;
    if (!e) return !0;
    var l = e.getValue(),
      a = "";
    return (
      t && (a = es(t) ? (t.checked ? "true" : "false") : t.value),
      (t = a),
      t !== l ? (e.setValue(t), !0) : !1
    );
  }
  function Yu(t) {
    if (((t = t || (typeof document < "u" ? document : void 0)), typeof t > "u")) return null;
    try {
      return t.activeElement || t.body;
    } catch {
      return t.body;
    }
  }
  var Qd = /[\n"\\]/g;
  function Se(t) {
    return t.replace(Qd, function (e) {
      return "\\" + e.charCodeAt(0).toString(16) + " ";
    });
  }
  function oi(t, e, l, a, u, n, c, s) {
    ((t.name = ""),
      c != null && typeof c != "function" && typeof c != "symbol" && typeof c != "boolean"
        ? (t.type = c)
        : t.removeAttribute("type"),
      e != null
        ? c === "number"
          ? ((e === 0 && t.value === "") || t.value != e) && (t.value = "" + ge(e))
          : t.value !== "" + ge(e) && (t.value = "" + ge(e))
        : (c !== "submit" && c !== "reset") || t.removeAttribute("value"),
      e != null
        ? hi(t, c, ge(e))
        : l != null
          ? hi(t, c, ge(l))
          : a != null && t.removeAttribute("value"),
      u == null && n != null && (t.defaultChecked = !!n),
      u != null && (t.checked = u && typeof u != "function" && typeof u != "symbol"),
      s != null && typeof s != "function" && typeof s != "symbol" && typeof s != "boolean"
        ? (t.name = "" + ge(s))
        : t.removeAttribute("name"));
  }
  function as(t, e, l, a, u, n, c, s) {
    if (
      (n != null &&
        typeof n != "function" &&
        typeof n != "symbol" &&
        typeof n != "boolean" &&
        (t.type = n),
      e != null || l != null)
    ) {
      if (!((n !== "submit" && n !== "reset") || e != null)) {
        ri(t);
        return;
      }
      ((l = l != null ? "" + ge(l) : ""),
        (e = e != null ? "" + ge(e) : l),
        s || e === t.value || (t.value = e),
        (t.defaultValue = e));
    }
    ((a = a ?? u),
      (a = typeof a != "function" && typeof a != "symbol" && !!a),
      (t.checked = s ? t.checked : !!a),
      (t.defaultChecked = !!a),
      c != null &&
        typeof c != "function" &&
        typeof c != "symbol" &&
        typeof c != "boolean" &&
        (t.name = c),
      ri(t));
  }
  function hi(t, e, l) {
    (e === "number" && Yu(t.ownerDocument) === t) ||
      t.defaultValue === "" + l ||
      (t.defaultValue = "" + l);
  }
  function Pl(t, e, l, a) {
    if (((t = t.options), e)) {
      e = {};
      for (var u = 0; u < l.length; u++) e["$" + l[u]] = !0;
      for (l = 0; l < t.length; l++)
        ((u = e.hasOwnProperty("$" + t[l].value)),
          t[l].selected !== u && (t[l].selected = u),
          u && a && (t[l].defaultSelected = !0));
    } else {
      for (l = "" + ge(l), e = null, u = 0; u < t.length; u++) {
        if (t[u].value === l) {
          ((t[u].selected = !0), a && (t[u].defaultSelected = !0));
          return;
        }
        e !== null || t[u].disabled || (e = t[u]);
      }
      e !== null && (e.selected = !0);
    }
  }
  function us(t, e, l) {
    if (e != null && ((e = "" + ge(e)), e !== t.value && (t.value = e), l == null)) {
      t.defaultValue !== e && (t.defaultValue = e);
      return;
    }
    t.defaultValue = l != null ? "" + ge(l) : "";
  }
  function ns(t, e, l, a) {
    if (e == null) {
      if (a != null) {
        if (l != null) throw Error(r(92));
        if (De(a)) {
          if (1 < a.length) throw Error(r(93));
          a = a[0];
        }
        l = a;
      }
      (l == null && (l = ""), (e = l));
    }
    ((l = ge(e)),
      (t.defaultValue = l),
      (a = t.textContent),
      a === l && a !== "" && a !== null && (t.value = a),
      ri(t));
  }
  function ta(t, e) {
    if (e) {
      var l = t.firstChild;
      if (l && l === t.lastChild && l.nodeType === 3) {
        l.nodeValue = e;
        return;
      }
    }
    t.textContent = e;
  }
  var xd = new Set(
    "animationIterationCount aspectRatio borderImageOutset borderImageSlice borderImageWidth boxFlex boxFlexGroup boxOrdinalGroup columnCount columns flex flexGrow flexPositive flexShrink flexNegative flexOrder gridArea gridRow gridRowEnd gridRowSpan gridRowStart gridColumn gridColumnEnd gridColumnSpan gridColumnStart fontWeight lineClamp lineHeight opacity order orphans scale tabSize widows zIndex zoom fillOpacity floodOpacity stopOpacity strokeDasharray strokeDashoffset strokeMiterlimit strokeOpacity strokeWidth MozAnimationIterationCount MozBoxFlex MozBoxFlexGroup MozLineClamp msAnimationIterationCount msFlex msZoom msFlexGrow msFlexNegative msFlexOrder msFlexPositive msFlexShrink msGridColumn msGridColumnSpan msGridRow msGridRowSpan WebkitAnimationIterationCount WebkitBoxFlex WebKitBoxFlexGroup WebkitBoxOrdinalGroup WebkitColumnCount WebkitColumns WebkitFlex WebkitFlexGrow WebkitFlexPositive WebkitFlexShrink WebkitLineClamp".split(
      " ",
    ),
  );
  function is(t, e, l) {
    var a = e.indexOf("--") === 0;
    l == null || typeof l == "boolean" || l === ""
      ? a
        ? t.setProperty(e, "")
        : e === "float"
          ? (t.cssFloat = "")
          : (t[e] = "")
      : a
        ? t.setProperty(e, l)
        : typeof l != "number" || l === 0 || xd.has(e)
          ? e === "float"
            ? (t.cssFloat = l)
            : (t[e] = ("" + l).trim())
          : (t[e] = l + "px");
  }
  function cs(t, e, l) {
    if (e != null && typeof e != "object") throw Error(r(62));
    if (((t = t.style), l != null)) {
      for (var a in l)
        !l.hasOwnProperty(a) ||
          (e != null && e.hasOwnProperty(a)) ||
          (a.indexOf("--") === 0
            ? t.setProperty(a, "")
            : a === "float"
              ? (t.cssFloat = "")
              : (t[a] = ""));
      for (var u in e) ((a = e[u]), e.hasOwnProperty(u) && l[u] !== a && is(t, u, a));
    } else for (var n in e) e.hasOwnProperty(n) && is(t, n, e[n]);
  }
  function di(t) {
    if (t.indexOf("-") === -1) return !1;
    switch (t) {
      case "annotation-xml":
      case "color-profile":
      case "font-face":
      case "font-face-src":
      case "font-face-uri":
      case "font-face-format":
      case "font-face-name":
      case "missing-glyph":
        return !1;
      default:
        return !0;
    }
  }
  var Bd = new Map([
      ["acceptCharset", "accept-charset"],
      ["htmlFor", "for"],
      ["httpEquiv", "http-equiv"],
      ["crossOrigin", "crossorigin"],
      ["accentHeight", "accent-height"],
      ["alignmentBaseline", "alignment-baseline"],
      ["arabicForm", "arabic-form"],
      ["baselineShift", "baseline-shift"],
      ["capHeight", "cap-height"],
      ["clipPath", "clip-path"],
      ["clipRule", "clip-rule"],
      ["colorInterpolation", "color-interpolation"],
      ["colorInterpolationFilters", "color-interpolation-filters"],
      ["colorProfile", "color-profile"],
      ["colorRendering", "color-rendering"],
      ["dominantBaseline", "dominant-baseline"],
      ["enableBackground", "enable-background"],
      ["fillOpacity", "fill-opacity"],
      ["fillRule", "fill-rule"],
      ["floodColor", "flood-color"],
      ["floodOpacity", "flood-opacity"],
      ["fontFamily", "font-family"],
      ["fontSize", "font-size"],
      ["fontSizeAdjust", "font-size-adjust"],
      ["fontStretch", "font-stretch"],
      ["fontStyle", "font-style"],
      ["fontVariant", "font-variant"],
      ["fontWeight", "font-weight"],
      ["glyphName", "glyph-name"],
      ["glyphOrientationHorizontal", "glyph-orientation-horizontal"],
      ["glyphOrientationVertical", "glyph-orientation-vertical"],
      ["horizAdvX", "horiz-adv-x"],
      ["horizOriginX", "horiz-origin-x"],
      ["imageRendering", "image-rendering"],
      ["letterSpacing", "letter-spacing"],
      ["lightingColor", "lighting-color"],
      ["markerEnd", "marker-end"],
      ["markerMid", "marker-mid"],
      ["markerStart", "marker-start"],
      ["overlinePosition", "overline-position"],
      ["overlineThickness", "overline-thickness"],
      ["paintOrder", "paint-order"],
      ["panose-1", "panose-1"],
      ["pointerEvents", "pointer-events"],
      ["renderingIntent", "rendering-intent"],
      ["shapeRendering", "shape-rendering"],
      ["stopColor", "stop-color"],
      ["stopOpacity", "stop-opacity"],
      ["strikethroughPosition", "strikethrough-position"],
      ["strikethroughThickness", "strikethrough-thickness"],
      ["strokeDasharray", "stroke-dasharray"],
      ["strokeDashoffset", "stroke-dashoffset"],
      ["strokeLinecap", "stroke-linecap"],
      ["strokeLinejoin", "stroke-linejoin"],
      ["strokeMiterlimit", "stroke-miterlimit"],
      ["strokeOpacity", "stroke-opacity"],
      ["strokeWidth", "stroke-width"],
      ["textAnchor", "text-anchor"],
      ["textDecoration", "text-decoration"],
      ["textRendering", "text-rendering"],
      ["transformOrigin", "transform-origin"],
      ["underlinePosition", "underline-position"],
      ["underlineThickness", "underline-thickness"],
      ["unicodeBidi", "unicode-bidi"],
      ["unicodeRange", "unicode-range"],
      ["unitsPerEm", "units-per-em"],
      ["vAlphabetic", "v-alphabetic"],
      ["vHanging", "v-hanging"],
      ["vIdeographic", "v-ideographic"],
      ["vMathematical", "v-mathematical"],
      ["vectorEffect", "vector-effect"],
      ["vertAdvY", "vert-adv-y"],
      ["vertOriginX", "vert-origin-x"],
      ["vertOriginY", "vert-origin-y"],
      ["wordSpacing", "word-spacing"],
      ["writingMode", "writing-mode"],
      ["xmlnsXlink", "xmlns:xlink"],
      ["xHeight", "x-height"],
    ]),
    Yd =
      /^[\u0000-\u001F ]*j[\r\n\t]*a[\r\n\t]*v[\r\n\t]*a[\r\n\t]*s[\r\n\t]*c[\r\n\t]*r[\r\n\t]*i[\r\n\t]*p[\r\n\t]*t[\r\n\t]*:/i;
  function Gu(t) {
    return Yd.test("" + t)
      ? "javascript:throw new Error('React has blocked a javascript: URL as a security precaution.')"
      : t;
  }
  function Be() {}
  var yi = null;
  function vi(t) {
    return (
      (t = t.target || t.srcElement || window),
      t.correspondingUseElement && (t = t.correspondingUseElement),
      t.nodeType === 3 ? t.parentNode : t
    );
  }
  var ea = null,
    la = null;
  function fs(t) {
    var e = $l(t);
    if (e && (t = e.stateNode)) {
      var l = t[kt] || null;
      t: switch (((t = e.stateNode), e.type)) {
        case "input":
          if (
            (oi(
              t,
              l.value,
              l.defaultValue,
              l.defaultValue,
              l.checked,
              l.defaultChecked,
              l.type,
              l.name,
            ),
            (e = l.name),
            l.type === "radio" && e != null)
          ) {
            for (l = t; l.parentNode; ) l = l.parentNode;
            for (
              l = l.querySelectorAll('input[name="' + Se("" + e) + '"][type="radio"]'), e = 0;
              e < l.length;
              e++
            ) {
              var a = l[e];
              if (a !== t && a.form === t.form) {
                var u = a[kt] || null;
                if (!u) throw Error(r(90));
                oi(
                  a,
                  u.value,
                  u.defaultValue,
                  u.defaultValue,
                  u.checked,
                  u.defaultChecked,
                  u.type,
                  u.name,
                );
              }
            }
            for (e = 0; e < l.length; e++) ((a = l[e]), a.form === t.form && ls(a));
          }
          break t;
        case "textarea":
          us(t, l.value, l.defaultValue);
          break t;
        case "select":
          ((e = l.value), e != null && Pl(t, !!l.multiple, e, !1));
      }
    }
  }
  var mi = !1;
  function ss(t, e, l) {
    if (mi) return t(e, l);
    mi = !0;
    try {
      var a = t(e);
      return a;
    } finally {
      if (
        ((mi = !1),
        (ea !== null || la !== null) &&
          (Dn(), ea && ((e = ea), (t = la), (la = ea = null), fs(e), t)))
      )
        for (e = 0; e < t.length; e++) fs(t[e]);
    }
  }
  function Ya(t, e) {
    var l = t.stateNode;
    if (l === null) return null;
    var a = l[kt] || null;
    if (a === null) return null;
    l = a[e];
    t: switch (e) {
      case "onClick":
      case "onClickCapture":
      case "onDoubleClick":
      case "onDoubleClickCapture":
      case "onMouseDown":
      case "onMouseDownCapture":
      case "onMouseMove":
      case "onMouseMoveCapture":
      case "onMouseUp":
      case "onMouseUpCapture":
      case "onMouseEnter":
        ((a = !a.disabled) ||
          ((t = t.type),
          (a = !(t === "button" || t === "input" || t === "select" || t === "textarea"))),
          (t = !a));
        break t;
      default:
        t = !1;
    }
    if (t) return null;
    if (l && typeof l != "function") throw Error(r(231, e, typeof l));
    return l;
  }
  var Ye = !(
      typeof window > "u" ||
      typeof window.document > "u" ||
      typeof window.document.createElement > "u"
    ),
    gi = !1;
  if (Ye)
    try {
      var Ga = {};
      (Object.defineProperty(Ga, "passive", {
        get: function () {
          gi = !0;
        },
      }),
        window.addEventListener("test", Ga, Ga),
        window.removeEventListener("test", Ga, Ga));
    } catch {
      gi = !1;
    }
  var al = null,
    Si = null,
    Xu = null;
  function rs() {
    if (Xu) return Xu;
    var t,
      e = Si,
      l = e.length,
      a,
      u = "value" in al ? al.value : al.textContent,
      n = u.length;
    for (t = 0; t < l && e[t] === u[t]; t++);
    var c = l - t;
    for (a = 1; a <= c && e[l - a] === u[n - a]; a++);
    return (Xu = u.slice(t, 1 < a ? 1 - a : void 0));
  }
  function Lu(t) {
    var e = t.keyCode;
    return (
      "charCode" in t ? ((t = t.charCode), t === 0 && e === 13 && (t = 13)) : (t = e),
      t === 10 && (t = 13),
      32 <= t || t === 13 ? t : 0
    );
  }
  function Zu() {
    return !0;
  }
  function os() {
    return !1;
  }
  function It(t) {
    function e(l, a, u, n, c) {
      ((this._reactName = l),
        (this._targetInst = u),
        (this.type = a),
        (this.nativeEvent = n),
        (this.target = c),
        (this.currentTarget = null));
      for (var s in t) t.hasOwnProperty(s) && ((l = t[s]), (this[s] = l ? l(n) : n[s]));
      return (
        (this.isDefaultPrevented = (
          n.defaultPrevented != null ? n.defaultPrevented : n.returnValue === !1
        )
          ? Zu
          : os),
        (this.isPropagationStopped = os),
        this
      );
    }
    return (
      N(e.prototype, {
        preventDefault: function () {
          this.defaultPrevented = !0;
          var l = this.nativeEvent;
          l &&
            (l.preventDefault
              ? l.preventDefault()
              : typeof l.returnValue != "unknown" && (l.returnValue = !1),
            (this.isDefaultPrevented = Zu));
        },
        stopPropagation: function () {
          var l = this.nativeEvent;
          l &&
            (l.stopPropagation
              ? l.stopPropagation()
              : typeof l.cancelBubble != "unknown" && (l.cancelBubble = !0),
            (this.isPropagationStopped = Zu));
        },
        persist: function () {},
        isPersistent: Zu,
      }),
      e
    );
  }
  var Rl = {
      eventPhase: 0,
      bubbles: 0,
      cancelable: 0,
      timeStamp: function (t) {
        return t.timeStamp || Date.now();
      },
      defaultPrevented: 0,
      isTrusted: 0,
    },
    Ku = It(Rl),
    Xa = N({}, Rl, { view: 0, detail: 0 }),
    Gd = It(Xa),
    pi,
    bi,
    La,
    Vu = N({}, Xa, {
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
      getModifierState: Ti,
      button: 0,
      buttons: 0,
      relatedTarget: function (t) {
        return t.relatedTarget === void 0
          ? t.fromElement === t.srcElement
            ? t.toElement
            : t.fromElement
          : t.relatedTarget;
      },
      movementX: function (t) {
        return "movementX" in t
          ? t.movementX
          : (t !== La &&
              (La && t.type === "mousemove"
                ? ((pi = t.screenX - La.screenX), (bi = t.screenY - La.screenY))
                : (bi = pi = 0),
              (La = t)),
            pi);
      },
      movementY: function (t) {
        return "movementY" in t ? t.movementY : bi;
      },
    }),
    hs = It(Vu),
    Xd = N({}, Vu, { dataTransfer: 0 }),
    Ld = It(Xd),
    Zd = N({}, Xa, { relatedTarget: 0 }),
    Ei = It(Zd),
    Kd = N({}, Rl, { animationName: 0, elapsedTime: 0, pseudoElement: 0 }),
    Vd = It(Kd),
    Jd = N({}, Rl, {
      clipboardData: function (t) {
        return "clipboardData" in t ? t.clipboardData : window.clipboardData;
      },
    }),
    wd = It(Jd),
    Fd = N({}, Rl, { data: 0 }),
    ds = It(Fd),
    Wd = {
      Esc: "Escape",
      Spacebar: " ",
      Left: "ArrowLeft",
      Up: "ArrowUp",
      Right: "ArrowRight",
      Down: "ArrowDown",
      Del: "Delete",
      Win: "OS",
      Menu: "ContextMenu",
      Apps: "ContextMenu",
      Scroll: "ScrollLock",
      MozPrintableKey: "Unidentified",
    },
    $d = {
      8: "Backspace",
      9: "Tab",
      12: "Clear",
      13: "Enter",
      16: "Shift",
      17: "Control",
      18: "Alt",
      19: "Pause",
      20: "CapsLock",
      27: "Escape",
      32: " ",
      33: "PageUp",
      34: "PageDown",
      35: "End",
      36: "Home",
      37: "ArrowLeft",
      38: "ArrowUp",
      39: "ArrowRight",
      40: "ArrowDown",
      45: "Insert",
      46: "Delete",
      112: "F1",
      113: "F2",
      114: "F3",
      115: "F4",
      116: "F5",
      117: "F6",
      118: "F7",
      119: "F8",
      120: "F9",
      121: "F10",
      122: "F11",
      123: "F12",
      144: "NumLock",
      145: "ScrollLock",
      224: "Meta",
    },
    kd = { Alt: "altKey", Control: "ctrlKey", Meta: "metaKey", Shift: "shiftKey" };
  function Id(t) {
    var e = this.nativeEvent;
    return e.getModifierState ? e.getModifierState(t) : (t = kd[t]) ? !!e[t] : !1;
  }
  function Ti() {
    return Id;
  }
  var Pd = N({}, Xa, {
      key: function (t) {
        if (t.key) {
          var e = Wd[t.key] || t.key;
          if (e !== "Unidentified") return e;
        }
        return t.type === "keypress"
          ? ((t = Lu(t)), t === 13 ? "Enter" : String.fromCharCode(t))
          : t.type === "keydown" || t.type === "keyup"
            ? $d[t.keyCode] || "Unidentified"
            : "";
      },
      code: 0,
      location: 0,
      ctrlKey: 0,
      shiftKey: 0,
      altKey: 0,
      metaKey: 0,
      repeat: 0,
      locale: 0,
      getModifierState: Ti,
      charCode: function (t) {
        return t.type === "keypress" ? Lu(t) : 0;
      },
      keyCode: function (t) {
        return t.type === "keydown" || t.type === "keyup" ? t.keyCode : 0;
      },
      which: function (t) {
        return t.type === "keypress"
          ? Lu(t)
          : t.type === "keydown" || t.type === "keyup"
            ? t.keyCode
            : 0;
      },
    }),
    ty = It(Pd),
    ey = N({}, Vu, {
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
    ys = It(ey),
    ly = N({}, Xa, {
      touches: 0,
      targetTouches: 0,
      changedTouches: 0,
      altKey: 0,
      metaKey: 0,
      ctrlKey: 0,
      shiftKey: 0,
      getModifierState: Ti,
    }),
    ay = It(ly),
    uy = N({}, Rl, { propertyName: 0, elapsedTime: 0, pseudoElement: 0 }),
    ny = It(uy),
    iy = N({}, Vu, {
      deltaX: function (t) {
        return "deltaX" in t ? t.deltaX : "wheelDeltaX" in t ? -t.wheelDeltaX : 0;
      },
      deltaY: function (t) {
        return "deltaY" in t
          ? t.deltaY
          : "wheelDeltaY" in t
            ? -t.wheelDeltaY
            : "wheelDelta" in t
              ? -t.wheelDelta
              : 0;
      },
      deltaZ: 0,
      deltaMode: 0,
    }),
    cy = It(iy),
    fy = N({}, Rl, { newState: 0, oldState: 0 }),
    sy = It(fy),
    ry = [9, 13, 27, 32],
    Oi = Ye && "CompositionEvent" in window,
    Za = null;
  Ye && "documentMode" in document && (Za = document.documentMode);
  var oy = Ye && "TextEvent" in window && !Za,
    vs = Ye && (!Oi || (Za && 8 < Za && 11 >= Za)),
    ms = " ",
    gs = !1;
  function Ss(t, e) {
    switch (t) {
      case "keyup":
        return ry.indexOf(e.keyCode) !== -1;
      case "keydown":
        return e.keyCode !== 229;
      case "keypress":
      case "mousedown":
      case "focusout":
        return !0;
      default:
        return !1;
    }
  }
  function ps(t) {
    return ((t = t.detail), typeof t == "object" && "data" in t ? t.data : null);
  }
  var aa = !1;
  function hy(t, e) {
    switch (t) {
      case "compositionend":
        return ps(e);
      case "keypress":
        return e.which !== 32 ? null : ((gs = !0), ms);
      case "textInput":
        return ((t = e.data), t === ms && gs ? null : t);
      default:
        return null;
    }
  }
  function dy(t, e) {
    if (aa)
      return t === "compositionend" || (!Oi && Ss(t, e))
        ? ((t = rs()), (Xu = Si = al = null), (aa = !1), t)
        : null;
    switch (t) {
      case "paste":
        return null;
      case "keypress":
        if (!(e.ctrlKey || e.altKey || e.metaKey) || (e.ctrlKey && e.altKey)) {
          if (e.char && 1 < e.char.length) return e.char;
          if (e.which) return String.fromCharCode(e.which);
        }
        return null;
      case "compositionend":
        return vs && e.locale !== "ko" ? null : e.data;
      default:
        return null;
    }
  }
  var yy = {
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
  function bs(t) {
    var e = t && t.nodeName && t.nodeName.toLowerCase();
    return e === "input" ? !!yy[t.type] : e === "textarea";
  }
  function Es(t, e, l, a) {
    (ea ? (la ? la.push(a) : (la = [a])) : (ea = a),
      (e = qn(e, "onChange")),
      0 < e.length &&
        ((l = new Ku("onChange", "change", null, l, a)), t.push({ event: l, listeners: e })));
  }
  var Ka = null,
    Va = null;
  function vy(t) {
    uh(t, 0);
  }
  function Ju(t) {
    var e = Ba(t);
    if (ls(e)) return t;
  }
  function Ts(t, e) {
    if (t === "change") return e;
  }
  var Os = !1;
  if (Ye) {
    var zi;
    if (Ye) {
      var Ai = "oninput" in document;
      if (!Ai) {
        var zs = document.createElement("div");
        (zs.setAttribute("oninput", "return;"), (Ai = typeof zs.oninput == "function"));
      }
      zi = Ai;
    } else zi = !1;
    Os = zi && (!document.documentMode || 9 < document.documentMode);
  }
  function As() {
    Ka && (Ka.detachEvent("onpropertychange", Ms), (Va = Ka = null));
  }
  function Ms(t) {
    if (t.propertyName === "value" && Ju(Va)) {
      var e = [];
      (Es(e, Va, t, vi(t)), ss(vy, e));
    }
  }
  function my(t, e, l) {
    t === "focusin"
      ? (As(), (Ka = e), (Va = l), Ka.attachEvent("onpropertychange", Ms))
      : t === "focusout" && As();
  }
  function gy(t) {
    if (t === "selectionchange" || t === "keyup" || t === "keydown") return Ju(Va);
  }
  function Sy(t, e) {
    if (t === "click") return Ju(e);
  }
  function py(t, e) {
    if (t === "input" || t === "change") return Ju(e);
  }
  function by(t, e) {
    return (t === e && (t !== 0 || 1 / t === 1 / e)) || (t !== t && e !== e);
  }
  var fe = typeof Object.is == "function" ? Object.is : by;
  function Ja(t, e) {
    if (fe(t, e)) return !0;
    if (typeof t != "object" || t === null || typeof e != "object" || e === null) return !1;
    var l = Object.keys(t),
      a = Object.keys(e);
    if (l.length !== a.length) return !1;
    for (a = 0; a < l.length; a++) {
      var u = l[a];
      if (!li.call(e, u) || !fe(t[u], e[u])) return !1;
    }
    return !0;
  }
  function _s(t) {
    for (; t && t.firstChild; ) t = t.firstChild;
    return t;
  }
  function Ds(t, e) {
    var l = _s(t);
    t = 0;
    for (var a; l; ) {
      if (l.nodeType === 3) {
        if (((a = t + l.textContent.length), t <= e && a >= e)) return { node: l, offset: e - t };
        t = a;
      }
      t: {
        for (; l; ) {
          if (l.nextSibling) {
            l = l.nextSibling;
            break t;
          }
          l = l.parentNode;
        }
        l = void 0;
      }
      l = _s(l);
    }
  }
  function Us(t, e) {
    return t && e
      ? t === e
        ? !0
        : t && t.nodeType === 3
          ? !1
          : e && e.nodeType === 3
            ? Us(t, e.parentNode)
            : "contains" in t
              ? t.contains(e)
              : t.compareDocumentPosition
                ? !!(t.compareDocumentPosition(e) & 16)
                : !1
      : !1;
  }
  function Rs(t) {
    t =
      t != null && t.ownerDocument != null && t.ownerDocument.defaultView != null
        ? t.ownerDocument.defaultView
        : window;
    for (var e = Yu(t.document); e instanceof t.HTMLIFrameElement; ) {
      try {
        var l = typeof e.contentWindow.location.href == "string";
      } catch {
        l = !1;
      }
      if (l) t = e.contentWindow;
      else break;
      e = Yu(t.document);
    }
    return e;
  }
  function Mi(t) {
    var e = t && t.nodeName && t.nodeName.toLowerCase();
    return (
      e &&
      ((e === "input" &&
        (t.type === "text" ||
          t.type === "search" ||
          t.type === "tel" ||
          t.type === "url" ||
          t.type === "password")) ||
        e === "textarea" ||
        t.contentEditable === "true")
    );
  }
  var Ey = Ye && "documentMode" in document && 11 >= document.documentMode,
    ua = null,
    _i = null,
    wa = null,
    Di = !1;
  function Cs(t, e, l) {
    var a = l.window === l ? l.document : l.nodeType === 9 ? l : l.ownerDocument;
    Di ||
      ua == null ||
      ua !== Yu(a) ||
      ((a = ua),
      "selectionStart" in a && Mi(a)
        ? (a = { start: a.selectionStart, end: a.selectionEnd })
        : ((a = ((a.ownerDocument && a.ownerDocument.defaultView) || window).getSelection()),
          (a = {
            anchorNode: a.anchorNode,
            anchorOffset: a.anchorOffset,
            focusNode: a.focusNode,
            focusOffset: a.focusOffset,
          })),
      (wa && Ja(wa, a)) ||
        ((wa = a),
        (a = qn(_i, "onSelect")),
        0 < a.length &&
          ((e = new Ku("onSelect", "select", null, e, l)),
          t.push({ event: e, listeners: a }),
          (e.target = ua))));
  }
  function Cl(t, e) {
    var l = {};
    return (
      (l[t.toLowerCase()] = e.toLowerCase()),
      (l["Webkit" + t] = "webkit" + e),
      (l["Moz" + t] = "moz" + e),
      l
    );
  }
  var na = {
      animationend: Cl("Animation", "AnimationEnd"),
      animationiteration: Cl("Animation", "AnimationIteration"),
      animationstart: Cl("Animation", "AnimationStart"),
      transitionrun: Cl("Transition", "TransitionRun"),
      transitionstart: Cl("Transition", "TransitionStart"),
      transitioncancel: Cl("Transition", "TransitionCancel"),
      transitionend: Cl("Transition", "TransitionEnd"),
    },
    Ui = {},
    Ns = {};
  Ye &&
    ((Ns = document.createElement("div").style),
    "AnimationEvent" in window ||
      (delete na.animationend.animation,
      delete na.animationiteration.animation,
      delete na.animationstart.animation),
    "TransitionEvent" in window || delete na.transitionend.transition);
  function Nl(t) {
    if (Ui[t]) return Ui[t];
    if (!na[t]) return t;
    var e = na[t],
      l;
    for (l in e) if (e.hasOwnProperty(l) && l in Ns) return (Ui[t] = e[l]);
    return t;
  }
  var Hs = Nl("animationend"),
    js = Nl("animationiteration"),
    qs = Nl("animationstart"),
    Ty = Nl("transitionrun"),
    Oy = Nl("transitionstart"),
    zy = Nl("transitioncancel"),
    Qs = Nl("transitionend"),
    xs = new Map(),
    Ri =
      "abort auxClick beforeToggle cancel canPlay canPlayThrough click close contextMenu copy cut drag dragEnd dragEnter dragExit dragLeave dragOver dragStart drop durationChange emptied encrypted ended error gotPointerCapture input invalid keyDown keyPress keyUp load loadedData loadedMetadata loadStart lostPointerCapture mouseDown mouseMove mouseOut mouseOver mouseUp paste pause play playing pointerCancel pointerDown pointerMove pointerOut pointerOver pointerUp progress rateChange reset resize seeked seeking stalled submit suspend timeUpdate touchCancel touchEnd touchStart volumeChange scroll toggle touchMove waiting wheel".split(
        " ",
      );
  Ri.push("scrollEnd");
  function Ue(t, e) {
    (xs.set(t, e), Ul(e, [t]));
  }
  var wu =
      typeof reportError == "function"
        ? reportError
        : function (t) {
            if (typeof window == "object" && typeof window.ErrorEvent == "function") {
              var e = new window.ErrorEvent("error", {
                bubbles: !0,
                cancelable: !0,
                message:
                  typeof t == "object" && t !== null && typeof t.message == "string"
                    ? String(t.message)
                    : String(t),
                error: t,
              });
              if (!window.dispatchEvent(e)) return;
            } else if (typeof process == "object" && typeof process.emit == "function") {
              process.emit("uncaughtException", t);
              return;
            }
            console.error(t);
          },
    pe = [],
    ia = 0,
    Ci = 0;
  function Fu() {
    for (var t = ia, e = (Ci = ia = 0); e < t; ) {
      var l = pe[e];
      pe[e++] = null;
      var a = pe[e];
      pe[e++] = null;
      var u = pe[e];
      pe[e++] = null;
      var n = pe[e];
      if (((pe[e++] = null), a !== null && u !== null)) {
        var c = a.pending;
        (c === null ? (u.next = u) : ((u.next = c.next), (c.next = u)), (a.pending = u));
      }
      n !== 0 && Bs(l, u, n);
    }
  }
  function Wu(t, e, l, a) {
    ((pe[ia++] = t),
      (pe[ia++] = e),
      (pe[ia++] = l),
      (pe[ia++] = a),
      (Ci |= a),
      (t.lanes |= a),
      (t = t.alternate),
      t !== null && (t.lanes |= a));
  }
  function Ni(t, e, l, a) {
    return (Wu(t, e, l, a), $u(t));
  }
  function Hl(t, e) {
    return (Wu(t, null, null, e), $u(t));
  }
  function Bs(t, e, l) {
    t.lanes |= l;
    var a = t.alternate;
    a !== null && (a.lanes |= l);
    for (var u = !1, n = t.return; n !== null; )
      ((n.childLanes |= l),
        (a = n.alternate),
        a !== null && (a.childLanes |= l),
        n.tag === 22 && ((t = n.stateNode), t === null || t._visibility & 1 || (u = !0)),
        (t = n),
        (n = n.return));
    return t.tag === 3
      ? ((n = t.stateNode),
        u &&
          e !== null &&
          ((u = 31 - ce(l)),
          (t = n.hiddenUpdates),
          (a = t[u]),
          a === null ? (t[u] = [e]) : a.push(e),
          (e.lane = l | 536870912)),
        n)
      : null;
  }
  function $u(t) {
    if (50 < vu) throw ((vu = 0), (Xc = null), Error(r(185)));
    for (var e = t.return; e !== null; ) ((t = e), (e = t.return));
    return t.tag === 3 ? t.stateNode : null;
  }
  var ca = {};
  function Ay(t, e, l, a) {
    ((this.tag = t),
      (this.key = l),
      (this.sibling =
        this.child =
        this.return =
        this.stateNode =
        this.type =
        this.elementType =
          null),
      (this.index = 0),
      (this.refCleanup = this.ref = null),
      (this.pendingProps = e),
      (this.dependencies = this.memoizedState = this.updateQueue = this.memoizedProps = null),
      (this.mode = a),
      (this.subtreeFlags = this.flags = 0),
      (this.deletions = null),
      (this.childLanes = this.lanes = 0),
      (this.alternate = null));
  }
  function se(t, e, l, a) {
    return new Ay(t, e, l, a);
  }
  function Hi(t) {
    return ((t = t.prototype), !(!t || !t.isReactComponent));
  }
  function Ge(t, e) {
    var l = t.alternate;
    return (
      l === null
        ? ((l = se(t.tag, e, t.key, t.mode)),
          (l.elementType = t.elementType),
          (l.type = t.type),
          (l.stateNode = t.stateNode),
          (l.alternate = t),
          (t.alternate = l))
        : ((l.pendingProps = e),
          (l.type = t.type),
          (l.flags = 0),
          (l.subtreeFlags = 0),
          (l.deletions = null)),
      (l.flags = t.flags & 65011712),
      (l.childLanes = t.childLanes),
      (l.lanes = t.lanes),
      (l.child = t.child),
      (l.memoizedProps = t.memoizedProps),
      (l.memoizedState = t.memoizedState),
      (l.updateQueue = t.updateQueue),
      (e = t.dependencies),
      (l.dependencies = e === null ? null : { lanes: e.lanes, firstContext: e.firstContext }),
      (l.sibling = t.sibling),
      (l.index = t.index),
      (l.ref = t.ref),
      (l.refCleanup = t.refCleanup),
      l
    );
  }
  function Ys(t, e) {
    t.flags &= 65011714;
    var l = t.alternate;
    return (
      l === null
        ? ((t.childLanes = 0),
          (t.lanes = e),
          (t.child = null),
          (t.subtreeFlags = 0),
          (t.memoizedProps = null),
          (t.memoizedState = null),
          (t.updateQueue = null),
          (t.dependencies = null),
          (t.stateNode = null))
        : ((t.childLanes = l.childLanes),
          (t.lanes = l.lanes),
          (t.child = l.child),
          (t.subtreeFlags = 0),
          (t.deletions = null),
          (t.memoizedProps = l.memoizedProps),
          (t.memoizedState = l.memoizedState),
          (t.updateQueue = l.updateQueue),
          (t.type = l.type),
          (e = l.dependencies),
          (t.dependencies = e === null ? null : { lanes: e.lanes, firstContext: e.firstContext })),
      t
    );
  }
  function ku(t, e, l, a, u, n) {
    var c = 0;
    if (((a = t), typeof t == "function")) Hi(t) && (c = 1);
    else if (typeof t == "string")
      c = Rv(t, l, B.current) ? 26 : t === "html" || t === "head" || t === "body" ? 27 : 5;
    else
      t: switch (t) {
        case Jt:
          return ((t = se(31, l, e, u)), (t.elementType = Jt), (t.lanes = n), t);
        case Z:
          return jl(l.children, u, n, e);
        case St:
          ((c = 8), (u |= 24));
          break;
        case st:
          return ((t = se(12, l, e, u | 2)), (t.elementType = st), (t.lanes = n), t);
        case xt:
          return ((t = se(13, l, e, u)), (t.elementType = xt), (t.lanes = n), t);
        case Bt:
          return ((t = se(19, l, e, u)), (t.elementType = Bt), (t.lanes = n), t);
        default:
          if (typeof t == "object" && t !== null)
            switch (t.$$typeof) {
              case gt:
                c = 10;
                break t;
              case Dt:
                c = 9;
                break t;
              case Ut:
                c = 11;
                break t;
              case K:
                c = 14;
                break t;
              case yt:
                ((c = 16), (a = null));
                break t;
            }
          ((c = 29), (l = Error(r(130, t === null ? "null" : typeof t, ""))), (a = null));
      }
    return ((e = se(c, l, e, u)), (e.elementType = t), (e.type = a), (e.lanes = n), e);
  }
  function jl(t, e, l, a) {
    return ((t = se(7, t, a, e)), (t.lanes = l), t);
  }
  function ji(t, e, l) {
    return ((t = se(6, t, null, e)), (t.lanes = l), t);
  }
  function Gs(t) {
    var e = se(18, null, null, 0);
    return ((e.stateNode = t), e);
  }
  function qi(t, e, l) {
    return (
      (e = se(4, t.children !== null ? t.children : [], t.key, e)),
      (e.lanes = l),
      (e.stateNode = {
        containerInfo: t.containerInfo,
        pendingChildren: null,
        implementation: t.implementation,
      }),
      e
    );
  }
  var Xs = new WeakMap();
  function be(t, e) {
    if (typeof t == "object" && t !== null) {
      var l = Xs.get(t);
      return l !== void 0 ? l : ((e = { value: t, source: e, stack: Gf(e) }), Xs.set(t, e), e);
    }
    return { value: t, source: e, stack: Gf(e) };
  }
  var fa = [],
    sa = 0,
    Iu = null,
    Fa = 0,
    Ee = [],
    Te = 0,
    ul = null,
    Ne = 1,
    He = "";
  function Xe(t, e) {
    ((fa[sa++] = Fa), (fa[sa++] = Iu), (Iu = t), (Fa = e));
  }
  function Ls(t, e, l) {
    ((Ee[Te++] = Ne), (Ee[Te++] = He), (Ee[Te++] = ul), (ul = t));
    var a = Ne;
    t = He;
    var u = 32 - ce(a) - 1;
    ((a &= ~(1 << u)), (l += 1));
    var n = 32 - ce(e) + u;
    if (30 < n) {
      var c = u - (u % 5);
      ((n = (a & ((1 << c) - 1)).toString(32)),
        (a >>= c),
        (u -= c),
        (Ne = (1 << (32 - ce(e) + u)) | (l << u) | a),
        (He = n + t));
    } else ((Ne = (1 << n) | (l << u) | a), (He = t));
  }
  function Qi(t) {
    t.return !== null && (Xe(t, 1), Ls(t, 1, 0));
  }
  function xi(t) {
    for (; t === Iu; ) ((Iu = fa[--sa]), (fa[sa] = null), (Fa = fa[--sa]), (fa[sa] = null));
    for (; t === ul; )
      ((ul = Ee[--Te]),
        (Ee[Te] = null),
        (He = Ee[--Te]),
        (Ee[Te] = null),
        (Ne = Ee[--Te]),
        (Ee[Te] = null));
  }
  function Zs(t, e) {
    ((Ee[Te++] = Ne), (Ee[Te++] = He), (Ee[Te++] = ul), (Ne = e.id), (He = e.overflow), (ul = t));
  }
  var Lt = null,
    pt = null,
    et = !1,
    nl = null,
    Oe = !1,
    Bi = Error(r(519));
  function il(t) {
    var e = Error(
      r(418, 1 < arguments.length && arguments[1] !== void 0 && arguments[1] ? "text" : "HTML", ""),
    );
    throw (Wa(be(e, t)), Bi);
  }
  function Ks(t) {
    var e = t.stateNode,
      l = t.type,
      a = t.memoizedProps;
    switch (((e[Xt] = t), (e[kt] = a), l)) {
      case "dialog":
        (I("cancel", e), I("close", e));
        break;
      case "iframe":
      case "object":
      case "embed":
        I("load", e);
        break;
      case "video":
      case "audio":
        for (l = 0; l < gu.length; l++) I(gu[l], e);
        break;
      case "source":
        I("error", e);
        break;
      case "img":
      case "image":
      case "link":
        (I("error", e), I("load", e));
        break;
      case "details":
        I("toggle", e);
        break;
      case "input":
        (I("invalid", e),
          as(e, a.value, a.defaultValue, a.checked, a.defaultChecked, a.type, a.name, !0));
        break;
      case "select":
        I("invalid", e);
        break;
      case "textarea":
        (I("invalid", e), ns(e, a.value, a.defaultValue, a.children));
    }
    ((l = a.children),
      (typeof l != "string" && typeof l != "number" && typeof l != "bigint") ||
      e.textContent === "" + l ||
      a.suppressHydrationWarning === !0 ||
      fh(e.textContent, l)
        ? (a.popover != null && (I("beforetoggle", e), I("toggle", e)),
          a.onScroll != null && I("scroll", e),
          a.onScrollEnd != null && I("scrollend", e),
          a.onClick != null && (e.onclick = Be),
          (e = !0))
        : (e = !1),
      e || il(t, !0));
  }
  function Vs(t) {
    for (Lt = t.return; Lt; )
      switch (Lt.tag) {
        case 5:
        case 31:
        case 13:
          Oe = !1;
          return;
        case 27:
        case 3:
          Oe = !0;
          return;
        default:
          Lt = Lt.return;
      }
  }
  function ra(t) {
    if (t !== Lt) return !1;
    if (!et) return (Vs(t), (et = !0), !1);
    var e = t.tag,
      l;
    if (
      ((l = e !== 3 && e !== 27) &&
        ((l = e === 5) &&
          ((l = t.type), (l = !(l !== "form" && l !== "button") || lf(t.type, t.memoizedProps))),
        (l = !l)),
      l && pt && il(t),
      Vs(t),
      e === 13)
    ) {
      if (((t = t.memoizedState), (t = t !== null ? t.dehydrated : null), !t)) throw Error(r(317));
      pt = gh(t);
    } else if (e === 31) {
      if (((t = t.memoizedState), (t = t !== null ? t.dehydrated : null), !t)) throw Error(r(317));
      pt = gh(t);
    } else
      e === 27
        ? ((e = pt), bl(t.type) ? ((t = ff), (ff = null), (pt = t)) : (pt = e))
        : (pt = Lt ? Ae(t.stateNode.nextSibling) : null);
    return !0;
  }
  function ql() {
    ((pt = Lt = null), (et = !1));
  }
  function Yi() {
    var t = nl;
    return (t !== null && (le === null ? (le = t) : le.push.apply(le, t), (nl = null)), t);
  }
  function Wa(t) {
    nl === null ? (nl = [t]) : nl.push(t);
  }
  var Gi = y(null),
    Ql = null,
    Le = null;
  function cl(t, e, l) {
    (j(Gi, e._currentValue), (e._currentValue = l));
  }
  function Ze(t) {
    ((t._currentValue = Gi.current), D(Gi));
  }
  function Xi(t, e, l) {
    for (; t !== null; ) {
      var a = t.alternate;
      if (
        ((t.childLanes & e) !== e
          ? ((t.childLanes |= e), a !== null && (a.childLanes |= e))
          : a !== null && (a.childLanes & e) !== e && (a.childLanes |= e),
        t === l)
      )
        break;
      t = t.return;
    }
  }
  function Li(t, e, l, a) {
    var u = t.child;
    for (u !== null && (u.return = t); u !== null; ) {
      var n = u.dependencies;
      if (n !== null) {
        var c = u.child;
        n = n.firstContext;
        t: for (; n !== null; ) {
          var s = n;
          n = u;
          for (var h = 0; h < e.length; h++)
            if (s.context === e[h]) {
              ((n.lanes |= l),
                (s = n.alternate),
                s !== null && (s.lanes |= l),
                Xi(n.return, l, t),
                a || (c = null));
              break t;
            }
          n = s.next;
        }
      } else if (u.tag === 18) {
        if (((c = u.return), c === null)) throw Error(r(341));
        ((c.lanes |= l), (n = c.alternate), n !== null && (n.lanes |= l), Xi(c, l, t), (c = null));
      } else c = u.child;
      if (c !== null) c.return = u;
      else
        for (c = u; c !== null; ) {
          if (c === t) {
            c = null;
            break;
          }
          if (((u = c.sibling), u !== null)) {
            ((u.return = c.return), (c = u));
            break;
          }
          c = c.return;
        }
      u = c;
    }
  }
  function oa(t, e, l, a) {
    t = null;
    for (var u = e, n = !1; u !== null; ) {
      if (!n) {
        if ((u.flags & 524288) !== 0) n = !0;
        else if ((u.flags & 262144) !== 0) break;
      }
      if (u.tag === 10) {
        var c = u.alternate;
        if (c === null) throw Error(r(387));
        if (((c = c.memoizedProps), c !== null)) {
          var s = u.type;
          fe(u.pendingProps.value, c.value) || (t !== null ? t.push(s) : (t = [s]));
        }
      } else if (u === it.current) {
        if (((c = u.alternate), c === null)) throw Error(r(387));
        c.memoizedState.memoizedState !== u.memoizedState.memoizedState &&
          (t !== null ? t.push(Tu) : (t = [Tu]));
      }
      u = u.return;
    }
    (t !== null && Li(e, t, l, a), (e.flags |= 262144));
  }
  function Pu(t) {
    for (t = t.firstContext; t !== null; ) {
      if (!fe(t.context._currentValue, t.memoizedValue)) return !0;
      t = t.next;
    }
    return !1;
  }
  function xl(t) {
    ((Ql = t), (Le = null), (t = t.dependencies), t !== null && (t.firstContext = null));
  }
  function Zt(t) {
    return Js(Ql, t);
  }
  function tn(t, e) {
    return (Ql === null && xl(t), Js(t, e));
  }
  function Js(t, e) {
    var l = e._currentValue;
    if (((e = { context: e, memoizedValue: l, next: null }), Le === null)) {
      if (t === null) throw Error(r(308));
      ((Le = e), (t.dependencies = { lanes: 0, firstContext: e }), (t.flags |= 524288));
    } else Le = Le.next = e;
    return l;
  }
  var My =
      typeof AbortController < "u"
        ? AbortController
        : function () {
            var t = [],
              e = (this.signal = {
                aborted: !1,
                addEventListener: function (l, a) {
                  t.push(a);
                },
              });
            this.abort = function () {
              ((e.aborted = !0),
                t.forEach(function (l) {
                  return l();
                }));
            };
          },
    _y = i.unstable_scheduleCallback,
    Dy = i.unstable_NormalPriority,
    Rt = {
      $$typeof: gt,
      Consumer: null,
      Provider: null,
      _currentValue: null,
      _currentValue2: null,
      _threadCount: 0,
    };
  function Zi() {
    return { controller: new My(), data: new Map(), refCount: 0 };
  }
  function $a(t) {
    (t.refCount--,
      t.refCount === 0 &&
        _y(Dy, function () {
          t.controller.abort();
        }));
  }
  var ka = null,
    Ki = 0,
    ha = 0,
    da = null;
  function Uy(t, e) {
    if (ka === null) {
      var l = (ka = []);
      ((Ki = 0),
        (ha = wc()),
        (da = {
          status: "pending",
          value: void 0,
          then: function (a) {
            l.push(a);
          },
        }));
    }
    return (Ki++, e.then(ws, ws), e);
  }
  function ws() {
    if (--Ki === 0 && ka !== null) {
      da !== null && (da.status = "fulfilled");
      var t = ka;
      ((ka = null), (ha = 0), (da = null));
      for (var e = 0; e < t.length; e++) (0, t[e])();
    }
  }
  function Ry(t, e) {
    var l = [],
      a = {
        status: "pending",
        value: null,
        reason: null,
        then: function (u) {
          l.push(u);
        },
      };
    return (
      t.then(
        function () {
          ((a.status = "fulfilled"), (a.value = e));
          for (var u = 0; u < l.length; u++) (0, l[u])(e);
        },
        function (u) {
          for (a.status = "rejected", a.reason = u, u = 0; u < l.length; u++) (0, l[u])(void 0);
        },
      ),
      a
    );
  }
  var Fs = O.S;
  O.S = function (t, e) {
    ((No = ne()),
      typeof e == "object" && e !== null && typeof e.then == "function" && Uy(t, e),
      Fs !== null && Fs(t, e));
  };
  var Bl = y(null);
  function Vi() {
    var t = Bl.current;
    return t !== null ? t : mt.pooledCache;
  }
  function en(t, e) {
    e === null ? j(Bl, Bl.current) : j(Bl, e.pool);
  }
  function Ws() {
    var t = Vi();
    return t === null ? null : { parent: Rt._currentValue, pool: t };
  }
  var ya = Error(r(460)),
    Ji = Error(r(474)),
    ln = Error(r(542)),
    an = { then: function () {} };
  function $s(t) {
    return ((t = t.status), t === "fulfilled" || t === "rejected");
  }
  function ks(t, e, l) {
    switch (
      ((l = t[l]), l === void 0 ? t.push(e) : l !== e && (e.then(Be, Be), (e = l)), e.status)
    ) {
      case "fulfilled":
        return e.value;
      case "rejected":
        throw ((t = e.reason), Ps(t), t);
      default:
        if (typeof e.status == "string") e.then(Be, Be);
        else {
          if (((t = mt), t !== null && 100 < t.shellSuspendCounter)) throw Error(r(482));
          ((t = e),
            (t.status = "pending"),
            t.then(
              function (a) {
                if (e.status === "pending") {
                  var u = e;
                  ((u.status = "fulfilled"), (u.value = a));
                }
              },
              function (a) {
                if (e.status === "pending") {
                  var u = e;
                  ((u.status = "rejected"), (u.reason = a));
                }
              },
            ));
        }
        switch (e.status) {
          case "fulfilled":
            return e.value;
          case "rejected":
            throw ((t = e.reason), Ps(t), t);
        }
        throw ((Gl = e), ya);
    }
  }
  function Yl(t) {
    try {
      var e = t._init;
      return e(t._payload);
    } catch (l) {
      throw l !== null && typeof l == "object" && typeof l.then == "function" ? ((Gl = l), ya) : l;
    }
  }
  var Gl = null;
  function Is() {
    if (Gl === null) throw Error(r(459));
    var t = Gl;
    return ((Gl = null), t);
  }
  function Ps(t) {
    if (t === ya || t === ln) throw Error(r(483));
  }
  var va = null,
    Ia = 0;
  function un(t) {
    var e = Ia;
    return ((Ia += 1), va === null && (va = []), ks(va, t, e));
  }
  function Pa(t, e) {
    ((e = e.props.ref), (t.ref = e !== void 0 ? e : null));
  }
  function nn(t, e) {
    throw e.$$typeof === R
      ? Error(r(525))
      : ((t = Object.prototype.toString.call(e)),
        Error(
          r(
            31,
            t === "[object Object]" ? "object with keys {" + Object.keys(e).join(", ") + "}" : t,
          ),
        ));
  }
  function tr(t) {
    function e(v, d) {
      if (t) {
        var m = v.deletions;
        m === null ? ((v.deletions = [d]), (v.flags |= 16)) : m.push(d);
      }
    }
    function l(v, d) {
      if (!t) return null;
      for (; d !== null; ) (e(v, d), (d = d.sibling));
      return null;
    }
    function a(v) {
      for (var d = new Map(); v !== null; )
        (v.key !== null ? d.set(v.key, v) : d.set(v.index, v), (v = v.sibling));
      return d;
    }
    function u(v, d) {
      return ((v = Ge(v, d)), (v.index = 0), (v.sibling = null), v);
    }
    function n(v, d, m) {
      return (
        (v.index = m),
        t
          ? ((m = v.alternate),
            m !== null
              ? ((m = m.index), m < d ? ((v.flags |= 67108866), d) : m)
              : ((v.flags |= 67108866), d))
          : ((v.flags |= 1048576), d)
      );
    }
    function c(v) {
      return (t && v.alternate === null && (v.flags |= 67108866), v);
    }
    function s(v, d, m, A) {
      return d === null || d.tag !== 6
        ? ((d = ji(m, v.mode, A)), (d.return = v), d)
        : ((d = u(d, m)), (d.return = v), d);
    }
    function h(v, d, m, A) {
      var G = m.type;
      return G === Z
        ? E(v, d, m.props.children, A, m.key)
        : d !== null &&
            (d.elementType === G ||
              (typeof G == "object" && G !== null && G.$$typeof === yt && Yl(G) === d.type))
          ? ((d = u(d, m.props)), Pa(d, m), (d.return = v), d)
          : ((d = ku(m.type, m.key, m.props, null, v.mode, A)), Pa(d, m), (d.return = v), d);
    }
    function S(v, d, m, A) {
      return d === null ||
        d.tag !== 4 ||
        d.stateNode.containerInfo !== m.containerInfo ||
        d.stateNode.implementation !== m.implementation
        ? ((d = qi(m, v.mode, A)), (d.return = v), d)
        : ((d = u(d, m.children || [])), (d.return = v), d);
    }
    function E(v, d, m, A, G) {
      return d === null || d.tag !== 7
        ? ((d = jl(m, v.mode, A, G)), (d.return = v), d)
        : ((d = u(d, m)), (d.return = v), d);
    }
    function _(v, d, m) {
      if ((typeof d == "string" && d !== "") || typeof d == "number" || typeof d == "bigint")
        return ((d = ji("" + d, v.mode, m)), (d.return = v), d);
      if (typeof d == "object" && d !== null) {
        switch (d.$$typeof) {
          case lt:
            return ((m = ku(d.type, d.key, d.props, null, v.mode, m)), Pa(m, d), (m.return = v), m);
          case W:
            return ((d = qi(d, v.mode, m)), (d.return = v), d);
          case yt:
            return ((d = Yl(d)), _(v, d, m));
        }
        if (De(d) || jt(d)) return ((d = jl(d, v.mode, m, null)), (d.return = v), d);
        if (typeof d.then == "function") return _(v, un(d), m);
        if (d.$$typeof === gt) return _(v, tn(v, d), m);
        nn(v, d);
      }
      return null;
    }
    function p(v, d, m, A) {
      var G = d !== null ? d.key : null;
      if ((typeof m == "string" && m !== "") || typeof m == "number" || typeof m == "bigint")
        return G !== null ? null : s(v, d, "" + m, A);
      if (typeof m == "object" && m !== null) {
        switch (m.$$typeof) {
          case lt:
            return m.key === G ? h(v, d, m, A) : null;
          case W:
            return m.key === G ? S(v, d, m, A) : null;
          case yt:
            return ((m = Yl(m)), p(v, d, m, A));
        }
        if (De(m) || jt(m)) return G !== null ? null : E(v, d, m, A, null);
        if (typeof m.then == "function") return p(v, d, un(m), A);
        if (m.$$typeof === gt) return p(v, d, tn(v, m), A);
        nn(v, m);
      }
      return null;
    }
    function b(v, d, m, A, G) {
      if ((typeof A == "string" && A !== "") || typeof A == "number" || typeof A == "bigint")
        return ((v = v.get(m) || null), s(d, v, "" + A, G));
      if (typeof A == "object" && A !== null) {
        switch (A.$$typeof) {
          case lt:
            return ((v = v.get(A.key === null ? m : A.key) || null), h(d, v, A, G));
          case W:
            return ((v = v.get(A.key === null ? m : A.key) || null), S(d, v, A, G));
          case yt:
            return ((A = Yl(A)), b(v, d, m, A, G));
        }
        if (De(A) || jt(A)) return ((v = v.get(m) || null), E(d, v, A, G, null));
        if (typeof A.then == "function") return b(v, d, m, un(A), G);
        if (A.$$typeof === gt) return b(v, d, m, tn(d, A), G);
        nn(d, A);
      }
      return null;
    }
    function x(v, d, m, A) {
      for (
        var G = null, at = null, Y = d, F = (d = 0), tt = null;
        Y !== null && F < m.length;
        F++
      ) {
        Y.index > F ? ((tt = Y), (Y = null)) : (tt = Y.sibling);
        var ut = p(v, Y, m[F], A);
        if (ut === null) {
          Y === null && (Y = tt);
          break;
        }
        (t && Y && ut.alternate === null && e(v, Y),
          (d = n(ut, d, F)),
          at === null ? (G = ut) : (at.sibling = ut),
          (at = ut),
          (Y = tt));
      }
      if (F === m.length) return (l(v, Y), et && Xe(v, F), G);
      if (Y === null) {
        for (; F < m.length; F++)
          ((Y = _(v, m[F], A)),
            Y !== null && ((d = n(Y, d, F)), at === null ? (G = Y) : (at.sibling = Y), (at = Y)));
        return (et && Xe(v, F), G);
      }
      for (Y = a(Y); F < m.length; F++)
        ((tt = b(Y, v, F, m[F], A)),
          tt !== null &&
            (t && tt.alternate !== null && Y.delete(tt.key === null ? F : tt.key),
            (d = n(tt, d, F)),
            at === null ? (G = tt) : (at.sibling = tt),
            (at = tt)));
      return (
        t &&
          Y.forEach(function (Al) {
            return e(v, Al);
          }),
        et && Xe(v, F),
        G
      );
    }
    function X(v, d, m, A) {
      if (m == null) throw Error(r(151));
      for (
        var G = null, at = null, Y = d, F = (d = 0), tt = null, ut = m.next();
        Y !== null && !ut.done;
        F++, ut = m.next()
      ) {
        Y.index > F ? ((tt = Y), (Y = null)) : (tt = Y.sibling);
        var Al = p(v, Y, ut.value, A);
        if (Al === null) {
          Y === null && (Y = tt);
          break;
        }
        (t && Y && Al.alternate === null && e(v, Y),
          (d = n(Al, d, F)),
          at === null ? (G = Al) : (at.sibling = Al),
          (at = Al),
          (Y = tt));
      }
      if (ut.done) return (l(v, Y), et && Xe(v, F), G);
      if (Y === null) {
        for (; !ut.done; F++, ut = m.next())
          ((ut = _(v, ut.value, A)),
            ut !== null &&
              ((d = n(ut, d, F)), at === null ? (G = ut) : (at.sibling = ut), (at = ut)));
        return (et && Xe(v, F), G);
      }
      for (Y = a(Y); !ut.done; F++, ut = m.next())
        ((ut = b(Y, v, F, ut.value, A)),
          ut !== null &&
            (t && ut.alternate !== null && Y.delete(ut.key === null ? F : ut.key),
            (d = n(ut, d, F)),
            at === null ? (G = ut) : (at.sibling = ut),
            (at = ut)));
      return (
        t &&
          Y.forEach(function (Xv) {
            return e(v, Xv);
          }),
        et && Xe(v, F),
        G
      );
    }
    function dt(v, d, m, A) {
      if (
        (typeof m == "object" &&
          m !== null &&
          m.type === Z &&
          m.key === null &&
          (m = m.props.children),
        typeof m == "object" && m !== null)
      ) {
        switch (m.$$typeof) {
          case lt:
            t: {
              for (var G = m.key; d !== null; ) {
                if (d.key === G) {
                  if (((G = m.type), G === Z)) {
                    if (d.tag === 7) {
                      (l(v, d.sibling), (A = u(d, m.props.children)), (A.return = v), (v = A));
                      break t;
                    }
                  } else if (
                    d.elementType === G ||
                    (typeof G == "object" && G !== null && G.$$typeof === yt && Yl(G) === d.type)
                  ) {
                    (l(v, d.sibling), (A = u(d, m.props)), Pa(A, m), (A.return = v), (v = A));
                    break t;
                  }
                  l(v, d);
                  break;
                } else e(v, d);
                d = d.sibling;
              }
              m.type === Z
                ? ((A = jl(m.props.children, v.mode, A, m.key)), (A.return = v), (v = A))
                : ((A = ku(m.type, m.key, m.props, null, v.mode, A)),
                  Pa(A, m),
                  (A.return = v),
                  (v = A));
            }
            return c(v);
          case W:
            t: {
              for (G = m.key; d !== null; ) {
                if (d.key === G)
                  if (
                    d.tag === 4 &&
                    d.stateNode.containerInfo === m.containerInfo &&
                    d.stateNode.implementation === m.implementation
                  ) {
                    (l(v, d.sibling), (A = u(d, m.children || [])), (A.return = v), (v = A));
                    break t;
                  } else {
                    l(v, d);
                    break;
                  }
                else e(v, d);
                d = d.sibling;
              }
              ((A = qi(m, v.mode, A)), (A.return = v), (v = A));
            }
            return c(v);
          case yt:
            return ((m = Yl(m)), dt(v, d, m, A));
        }
        if (De(m)) return x(v, d, m, A);
        if (jt(m)) {
          if (((G = jt(m)), typeof G != "function")) throw Error(r(150));
          return ((m = G.call(m)), X(v, d, m, A));
        }
        if (typeof m.then == "function") return dt(v, d, un(m), A);
        if (m.$$typeof === gt) return dt(v, d, tn(v, m), A);
        nn(v, m);
      }
      return (typeof m == "string" && m !== "") || typeof m == "number" || typeof m == "bigint"
        ? ((m = "" + m),
          d !== null && d.tag === 6
            ? (l(v, d.sibling), (A = u(d, m)), (A.return = v), (v = A))
            : (l(v, d), (A = ji(m, v.mode, A)), (A.return = v), (v = A)),
          c(v))
        : l(v, d);
    }
    return function (v, d, m, A) {
      try {
        Ia = 0;
        var G = dt(v, d, m, A);
        return ((va = null), G);
      } catch (Y) {
        if (Y === ya || Y === ln) throw Y;
        var at = se(29, Y, null, v.mode);
        return ((at.lanes = A), (at.return = v), at);
      }
    };
  }
  var Xl = tr(!0),
    er = tr(!1),
    fl = !1;
  function wi(t) {
    t.updateQueue = {
      baseState: t.memoizedState,
      firstBaseUpdate: null,
      lastBaseUpdate: null,
      shared: { pending: null, lanes: 0, hiddenCallbacks: null },
      callbacks: null,
    };
  }
  function Fi(t, e) {
    ((t = t.updateQueue),
      e.updateQueue === t &&
        (e.updateQueue = {
          baseState: t.baseState,
          firstBaseUpdate: t.firstBaseUpdate,
          lastBaseUpdate: t.lastBaseUpdate,
          shared: t.shared,
          callbacks: null,
        }));
  }
  function sl(t) {
    return { lane: t, tag: 0, payload: null, callback: null, next: null };
  }
  function rl(t, e, l) {
    var a = t.updateQueue;
    if (a === null) return null;
    if (((a = a.shared), (nt & 2) !== 0)) {
      var u = a.pending;
      return (
        u === null ? (e.next = e) : ((e.next = u.next), (u.next = e)),
        (a.pending = e),
        (e = $u(t)),
        Bs(t, null, l),
        e
      );
    }
    return (Wu(t, a, e, l), $u(t));
  }
  function tu(t, e, l) {
    if (((e = e.updateQueue), e !== null && ((e = e.shared), (l & 4194048) !== 0))) {
      var a = e.lanes;
      ((a &= t.pendingLanes), (l |= a), (e.lanes = l), Jf(t, l));
    }
  }
  function Wi(t, e) {
    var l = t.updateQueue,
      a = t.alternate;
    if (a !== null && ((a = a.updateQueue), l === a)) {
      var u = null,
        n = null;
      if (((l = l.firstBaseUpdate), l !== null)) {
        do {
          var c = { lane: l.lane, tag: l.tag, payload: l.payload, callback: null, next: null };
          (n === null ? (u = n = c) : (n = n.next = c), (l = l.next));
        } while (l !== null);
        n === null ? (u = n = e) : (n = n.next = e);
      } else u = n = e;
      ((l = {
        baseState: a.baseState,
        firstBaseUpdate: u,
        lastBaseUpdate: n,
        shared: a.shared,
        callbacks: a.callbacks,
      }),
        (t.updateQueue = l));
      return;
    }
    ((t = l.lastBaseUpdate),
      t === null ? (l.firstBaseUpdate = e) : (t.next = e),
      (l.lastBaseUpdate = e));
  }
  var $i = !1;
  function eu() {
    if ($i) {
      var t = da;
      if (t !== null) throw t;
    }
  }
  function lu(t, e, l, a) {
    $i = !1;
    var u = t.updateQueue;
    fl = !1;
    var n = u.firstBaseUpdate,
      c = u.lastBaseUpdate,
      s = u.shared.pending;
    if (s !== null) {
      u.shared.pending = null;
      var h = s,
        S = h.next;
      ((h.next = null), c === null ? (n = S) : (c.next = S), (c = h));
      var E = t.alternate;
      E !== null &&
        ((E = E.updateQueue),
        (s = E.lastBaseUpdate),
        s !== c && (s === null ? (E.firstBaseUpdate = S) : (s.next = S), (E.lastBaseUpdate = h)));
    }
    if (n !== null) {
      var _ = u.baseState;
      ((c = 0), (E = S = h = null), (s = n));
      do {
        var p = s.lane & -536870913,
          b = p !== s.lane;
        if (b ? (P & p) === p : (a & p) === p) {
          (p !== 0 && p === ha && ($i = !0),
            E !== null &&
              (E = E.next =
                { lane: 0, tag: s.tag, payload: s.payload, callback: null, next: null }));
          t: {
            var x = t,
              X = s;
            p = e;
            var dt = l;
            switch (X.tag) {
              case 1:
                if (((x = X.payload), typeof x == "function")) {
                  _ = x.call(dt, _, p);
                  break t;
                }
                _ = x;
                break t;
              case 3:
                x.flags = (x.flags & -65537) | 128;
              case 0:
                if (
                  ((x = X.payload), (p = typeof x == "function" ? x.call(dt, _, p) : x), p == null)
                )
                  break t;
                _ = N({}, _, p);
                break t;
              case 2:
                fl = !0;
            }
          }
          ((p = s.callback),
            p !== null &&
              ((t.flags |= 64),
              b && (t.flags |= 8192),
              (b = u.callbacks),
              b === null ? (u.callbacks = [p]) : b.push(p)));
        } else
          ((b = { lane: p, tag: s.tag, payload: s.payload, callback: s.callback, next: null }),
            E === null ? ((S = E = b), (h = _)) : (E = E.next = b),
            (c |= p));
        if (((s = s.next), s === null)) {
          if (((s = u.shared.pending), s === null)) break;
          ((b = s),
            (s = b.next),
            (b.next = null),
            (u.lastBaseUpdate = b),
            (u.shared.pending = null));
        }
      } while (!0);
      (E === null && (h = _),
        (u.baseState = h),
        (u.firstBaseUpdate = S),
        (u.lastBaseUpdate = E),
        n === null && (u.shared.lanes = 0),
        (vl |= c),
        (t.lanes = c),
        (t.memoizedState = _));
    }
  }
  function lr(t, e) {
    if (typeof t != "function") throw Error(r(191, t));
    t.call(e);
  }
  function ar(t, e) {
    var l = t.callbacks;
    if (l !== null) for (t.callbacks = null, t = 0; t < l.length; t++) lr(l[t], e);
  }
  var ma = y(null),
    cn = y(0);
  function ur(t, e) {
    ((t = Ie), j(cn, t), j(ma, e), (Ie = t | e.baseLanes));
  }
  function ki() {
    (j(cn, Ie), j(ma, ma.current));
  }
  function Ii() {
    ((Ie = cn.current), D(ma), D(cn));
  }
  var re = y(null),
    ze = null;
  function ol(t) {
    var e = t.alternate;
    (j(Mt, Mt.current & 1),
      j(re, t),
      ze === null && (e === null || ma.current !== null || e.memoizedState !== null) && (ze = t));
  }
  function Pi(t) {
    (j(Mt, Mt.current), j(re, t), ze === null && (ze = t));
  }
  function nr(t) {
    t.tag === 22 ? (j(Mt, Mt.current), j(re, t), ze === null && (ze = t)) : hl();
  }
  function hl() {
    (j(Mt, Mt.current), j(re, re.current));
  }
  function oe(t) {
    (D(re), ze === t && (ze = null), D(Mt));
  }
  var Mt = y(0);
  function fn(t) {
    for (var e = t; e !== null; ) {
      if (e.tag === 13) {
        var l = e.memoizedState;
        if (l !== null && ((l = l.dehydrated), l === null || nf(l) || cf(l))) return e;
      } else if (
        e.tag === 19 &&
        (e.memoizedProps.revealOrder === "forwards" ||
          e.memoizedProps.revealOrder === "backwards" ||
          e.memoizedProps.revealOrder === "unstable_legacy-backwards" ||
          e.memoizedProps.revealOrder === "together")
      ) {
        if ((e.flags & 128) !== 0) return e;
      } else if (e.child !== null) {
        ((e.child.return = e), (e = e.child));
        continue;
      }
      if (e === t) break;
      for (; e.sibling === null; ) {
        if (e.return === null || e.return === t) return null;
        e = e.return;
      }
      ((e.sibling.return = e.return), (e = e.sibling));
    }
    return null;
  }
  var Ke = 0,
    w = null,
    ot = null,
    Ct = null,
    sn = !1,
    ga = !1,
    Ll = !1,
    rn = 0,
    au = 0,
    Sa = null,
    Cy = 0;
  function Ot() {
    throw Error(r(321));
  }
  function tc(t, e) {
    if (e === null) return !1;
    for (var l = 0; l < e.length && l < t.length; l++) if (!fe(t[l], e[l])) return !1;
    return !0;
  }
  function ec(t, e, l, a, u, n) {
    return (
      (Ke = n),
      (w = e),
      (e.memoizedState = null),
      (e.updateQueue = null),
      (e.lanes = 0),
      (O.H = t === null || t.memoizedState === null ? Lr : mc),
      (Ll = !1),
      (n = l(a, u)),
      (Ll = !1),
      ga && (n = cr(e, l, a, u)),
      ir(t),
      n
    );
  }
  function ir(t) {
    O.H = iu;
    var e = ot !== null && ot.next !== null;
    if (((Ke = 0), (Ct = ot = w = null), (sn = !1), (au = 0), (Sa = null), e)) throw Error(r(300));
    t === null || Nt || ((t = t.dependencies), t !== null && Pu(t) && (Nt = !0));
  }
  function cr(t, e, l, a) {
    w = t;
    var u = 0;
    do {
      if ((ga && (Sa = null), (au = 0), (ga = !1), 25 <= u)) throw Error(r(301));
      if (((u += 1), (Ct = ot = null), t.updateQueue != null)) {
        var n = t.updateQueue;
        ((n.lastEffect = null),
          (n.events = null),
          (n.stores = null),
          n.memoCache != null && (n.memoCache.index = 0));
      }
      ((O.H = Zr), (n = e(l, a)));
    } while (ga);
    return n;
  }
  function Ny() {
    var t = O.H,
      e = t.useState()[0];
    return (
      (e = typeof e.then == "function" ? uu(e) : e),
      (t = t.useState()[0]),
      (ot !== null ? ot.memoizedState : null) !== t && (w.flags |= 1024),
      e
    );
  }
  function lc() {
    var t = rn !== 0;
    return ((rn = 0), t);
  }
  function ac(t, e, l) {
    ((e.updateQueue = t.updateQueue), (e.flags &= -2053), (t.lanes &= ~l));
  }
  function uc(t) {
    if (sn) {
      for (t = t.memoizedState; t !== null; ) {
        var e = t.queue;
        (e !== null && (e.pending = null), (t = t.next));
      }
      sn = !1;
    }
    ((Ke = 0), (Ct = ot = w = null), (ga = !1), (au = rn = 0), (Sa = null));
  }
  function Wt() {
    var t = { memoizedState: null, baseState: null, baseQueue: null, queue: null, next: null };
    return (Ct === null ? (w.memoizedState = Ct = t) : (Ct = Ct.next = t), Ct);
  }
  function _t() {
    if (ot === null) {
      var t = w.alternate;
      t = t !== null ? t.memoizedState : null;
    } else t = ot.next;
    var e = Ct === null ? w.memoizedState : Ct.next;
    if (e !== null) ((Ct = e), (ot = t));
    else {
      if (t === null) throw w.alternate === null ? Error(r(467)) : Error(r(310));
      ((ot = t),
        (t = {
          memoizedState: ot.memoizedState,
          baseState: ot.baseState,
          baseQueue: ot.baseQueue,
          queue: ot.queue,
          next: null,
        }),
        Ct === null ? (w.memoizedState = Ct = t) : (Ct = Ct.next = t));
    }
    return Ct;
  }
  function on() {
    return { lastEffect: null, events: null, stores: null, memoCache: null };
  }
  function uu(t) {
    var e = au;
    return (
      (au += 1),
      Sa === null && (Sa = []),
      (t = ks(Sa, t, e)),
      (e = w),
      (Ct === null ? e.memoizedState : Ct.next) === null &&
        ((e = e.alternate), (O.H = e === null || e.memoizedState === null ? Lr : mc)),
      t
    );
  }
  function hn(t) {
    if (t !== null && typeof t == "object") {
      if (typeof t.then == "function") return uu(t);
      if (t.$$typeof === gt) return Zt(t);
    }
    throw Error(r(438, String(t)));
  }
  function nc(t) {
    var e = null,
      l = w.updateQueue;
    if ((l !== null && (e = l.memoCache), e == null)) {
      var a = w.alternate;
      a !== null &&
        ((a = a.updateQueue),
        a !== null &&
          ((a = a.memoCache),
          a != null &&
            (e = {
              data: a.data.map(function (u) {
                return u.slice();
              }),
              index: 0,
            })));
    }
    if (
      (e == null && (e = { data: [], index: 0 }),
      l === null && ((l = on()), (w.updateQueue = l)),
      (l.memoCache = e),
      (l = e.data[e.index]),
      l === void 0)
    )
      for (l = e.data[e.index] = Array(t), a = 0; a < t; a++) l[a] = _e;
    return (e.index++, l);
  }
  function Ve(t, e) {
    return typeof e == "function" ? e(t) : e;
  }
  function dn(t) {
    var e = _t();
    return ic(e, ot, t);
  }
  function ic(t, e, l) {
    var a = t.queue;
    if (a === null) throw Error(r(311));
    a.lastRenderedReducer = l;
    var u = t.baseQueue,
      n = a.pending;
    if (n !== null) {
      if (u !== null) {
        var c = u.next;
        ((u.next = n.next), (n.next = c));
      }
      ((e.baseQueue = u = n), (a.pending = null));
    }
    if (((n = t.baseState), u === null)) t.memoizedState = n;
    else {
      e = u.next;
      var s = (c = null),
        h = null,
        S = e,
        E = !1;
      do {
        var _ = S.lane & -536870913;
        if (_ !== S.lane ? (P & _) === _ : (Ke & _) === _) {
          var p = S.revertLane;
          if (p === 0)
            (h !== null &&
              (h = h.next =
                {
                  lane: 0,
                  revertLane: 0,
                  gesture: null,
                  action: S.action,
                  hasEagerState: S.hasEagerState,
                  eagerState: S.eagerState,
                  next: null,
                }),
              _ === ha && (E = !0));
          else if ((Ke & p) === p) {
            ((S = S.next), p === ha && (E = !0));
            continue;
          } else
            ((_ = {
              lane: 0,
              revertLane: S.revertLane,
              gesture: null,
              action: S.action,
              hasEagerState: S.hasEagerState,
              eagerState: S.eagerState,
              next: null,
            }),
              h === null ? ((s = h = _), (c = n)) : (h = h.next = _),
              (w.lanes |= p),
              (vl |= p));
          ((_ = S.action), Ll && l(n, _), (n = S.hasEagerState ? S.eagerState : l(n, _)));
        } else
          ((p = {
            lane: _,
            revertLane: S.revertLane,
            gesture: S.gesture,
            action: S.action,
            hasEagerState: S.hasEagerState,
            eagerState: S.eagerState,
            next: null,
          }),
            h === null ? ((s = h = p), (c = n)) : (h = h.next = p),
            (w.lanes |= _),
            (vl |= _));
        S = S.next;
      } while (S !== null && S !== e);
      if (
        (h === null ? (c = n) : (h.next = s),
        !fe(n, t.memoizedState) && ((Nt = !0), E && ((l = da), l !== null)))
      )
        throw l;
      ((t.memoizedState = n), (t.baseState = c), (t.baseQueue = h), (a.lastRenderedState = n));
    }
    return (u === null && (a.lanes = 0), [t.memoizedState, a.dispatch]);
  }
  function cc(t) {
    var e = _t(),
      l = e.queue;
    if (l === null) throw Error(r(311));
    l.lastRenderedReducer = t;
    var a = l.dispatch,
      u = l.pending,
      n = e.memoizedState;
    if (u !== null) {
      l.pending = null;
      var c = (u = u.next);
      do ((n = t(n, c.action)), (c = c.next));
      while (c !== u);
      (fe(n, e.memoizedState) || (Nt = !0),
        (e.memoizedState = n),
        e.baseQueue === null && (e.baseState = n),
        (l.lastRenderedState = n));
    }
    return [n, a];
  }
  function fr(t, e, l) {
    var a = w,
      u = _t(),
      n = et;
    if (n) {
      if (l === void 0) throw Error(r(407));
      l = l();
    } else l = e();
    var c = !fe((ot || u).memoizedState, l);
    if (
      (c && ((u.memoizedState = l), (Nt = !0)),
      (u = u.queue),
      rc(or.bind(null, a, u, t), [t]),
      u.getSnapshot !== e || c || (Ct !== null && Ct.memoizedState.tag & 1))
    ) {
      if (
        ((a.flags |= 2048),
        pa(9, { destroy: void 0 }, rr.bind(null, a, u, l, e), null),
        mt === null)
      )
        throw Error(r(349));
      n || (Ke & 127) !== 0 || sr(a, e, l);
    }
    return l;
  }
  function sr(t, e, l) {
    ((t.flags |= 16384),
      (t = { getSnapshot: e, value: l }),
      (e = w.updateQueue),
      e === null
        ? ((e = on()), (w.updateQueue = e), (e.stores = [t]))
        : ((l = e.stores), l === null ? (e.stores = [t]) : l.push(t)));
  }
  function rr(t, e, l, a) {
    ((e.value = l), (e.getSnapshot = a), hr(e) && dr(t));
  }
  function or(t, e, l) {
    return l(function () {
      hr(e) && dr(t);
    });
  }
  function hr(t) {
    var e = t.getSnapshot;
    t = t.value;
    try {
      var l = e();
      return !fe(t, l);
    } catch {
      return !0;
    }
  }
  function dr(t) {
    var e = Hl(t, 2);
    e !== null && ae(e, t, 2);
  }
  function fc(t) {
    var e = Wt();
    if (typeof t == "function") {
      var l = t;
      if (((t = l()), Ll)) {
        el(!0);
        try {
          l();
        } finally {
          el(!1);
        }
      }
    }
    return (
      (e.memoizedState = e.baseState = t),
      (e.queue = {
        pending: null,
        lanes: 0,
        dispatch: null,
        lastRenderedReducer: Ve,
        lastRenderedState: t,
      }),
      e
    );
  }
  function yr(t, e, l, a) {
    return ((t.baseState = l), ic(t, ot, typeof a == "function" ? a : Ve));
  }
  function Hy(t, e, l, a, u) {
    if (mn(t)) throw Error(r(485));
    if (((t = e.action), t !== null)) {
      var n = {
        payload: u,
        action: t,
        next: null,
        isTransition: !0,
        status: "pending",
        value: null,
        reason: null,
        listeners: [],
        then: function (c) {
          n.listeners.push(c);
        },
      };
      (O.T !== null ? l(!0) : (n.isTransition = !1),
        a(n),
        (l = e.pending),
        l === null
          ? ((n.next = e.pending = n), vr(e, n))
          : ((n.next = l.next), (e.pending = l.next = n)));
    }
  }
  function vr(t, e) {
    var l = e.action,
      a = e.payload,
      u = t.state;
    if (e.isTransition) {
      var n = O.T,
        c = {};
      O.T = c;
      try {
        var s = l(u, a),
          h = O.S;
        (h !== null && h(c, s), mr(t, e, s));
      } catch (S) {
        sc(t, e, S);
      } finally {
        (n !== null && c.types !== null && (n.types = c.types), (O.T = n));
      }
    } else
      try {
        ((n = l(u, a)), mr(t, e, n));
      } catch (S) {
        sc(t, e, S);
      }
  }
  function mr(t, e, l) {
    l !== null && typeof l == "object" && typeof l.then == "function"
      ? l.then(
          function (a) {
            gr(t, e, a);
          },
          function (a) {
            return sc(t, e, a);
          },
        )
      : gr(t, e, l);
  }
  function gr(t, e, l) {
    ((e.status = "fulfilled"),
      (e.value = l),
      Sr(e),
      (t.state = l),
      (e = t.pending),
      e !== null &&
        ((l = e.next), l === e ? (t.pending = null) : ((l = l.next), (e.next = l), vr(t, l))));
  }
  function sc(t, e, l) {
    var a = t.pending;
    if (((t.pending = null), a !== null)) {
      a = a.next;
      do ((e.status = "rejected"), (e.reason = l), Sr(e), (e = e.next));
      while (e !== a);
    }
    t.action = null;
  }
  function Sr(t) {
    t = t.listeners;
    for (var e = 0; e < t.length; e++) (0, t[e])();
  }
  function pr(t, e) {
    return e;
  }
  function br(t, e) {
    if (et) {
      var l = mt.formState;
      if (l !== null) {
        t: {
          var a = w;
          if (et) {
            if (pt) {
              e: {
                for (var u = pt, n = Oe; u.nodeType !== 8; ) {
                  if (!n) {
                    u = null;
                    break e;
                  }
                  if (((u = Ae(u.nextSibling)), u === null)) {
                    u = null;
                    break e;
                  }
                }
                ((n = u.data), (u = n === "F!" || n === "F" ? u : null));
              }
              if (u) {
                ((pt = Ae(u.nextSibling)), (a = u.data === "F!"));
                break t;
              }
            }
            il(a);
          }
          a = !1;
        }
        a && (e = l[0]);
      }
    }
    return (
      (l = Wt()),
      (l.memoizedState = l.baseState = e),
      (a = {
        pending: null,
        lanes: 0,
        dispatch: null,
        lastRenderedReducer: pr,
        lastRenderedState: e,
      }),
      (l.queue = a),
      (l = Yr.bind(null, w, a)),
      (a.dispatch = l),
      (a = fc(!1)),
      (n = vc.bind(null, w, !1, a.queue)),
      (a = Wt()),
      (u = { state: e, dispatch: null, action: t, pending: null }),
      (a.queue = u),
      (l = Hy.bind(null, w, u, n, l)),
      (u.dispatch = l),
      (a.memoizedState = t),
      [e, l, !1]
    );
  }
  function Er(t) {
    var e = _t();
    return Tr(e, ot, t);
  }
  function Tr(t, e, l) {
    if (
      ((e = ic(t, e, pr)[0]),
      (t = dn(Ve)[0]),
      typeof e == "object" && e !== null && typeof e.then == "function")
    )
      try {
        var a = uu(e);
      } catch (c) {
        throw c === ya ? ln : c;
      }
    else a = e;
    e = _t();
    var u = e.queue,
      n = u.dispatch;
    return (
      l !== e.memoizedState &&
        ((w.flags |= 2048), pa(9, { destroy: void 0 }, jy.bind(null, u, l), null)),
      [a, n, t]
    );
  }
  function jy(t, e) {
    t.action = e;
  }
  function Or(t) {
    var e = _t(),
      l = ot;
    if (l !== null) return Tr(e, l, t);
    (_t(), (e = e.memoizedState), (l = _t()));
    var a = l.queue.dispatch;
    return ((l.memoizedState = t), [e, a, !1]);
  }
  function pa(t, e, l, a) {
    return (
      (t = { tag: t, create: l, deps: a, inst: e, next: null }),
      (e = w.updateQueue),
      e === null && ((e = on()), (w.updateQueue = e)),
      (l = e.lastEffect),
      l === null
        ? (e.lastEffect = t.next = t)
        : ((a = l.next), (l.next = t), (t.next = a), (e.lastEffect = t)),
      t
    );
  }
  function zr() {
    return _t().memoizedState;
  }
  function yn(t, e, l, a) {
    var u = Wt();
    ((w.flags |= t),
      (u.memoizedState = pa(1 | e, { destroy: void 0 }, l, a === void 0 ? null : a)));
  }
  function vn(t, e, l, a) {
    var u = _t();
    a = a === void 0 ? null : a;
    var n = u.memoizedState.inst;
    ot !== null && a !== null && tc(a, ot.memoizedState.deps)
      ? (u.memoizedState = pa(e, n, l, a))
      : ((w.flags |= t), (u.memoizedState = pa(1 | e, n, l, a)));
  }
  function Ar(t, e) {
    yn(8390656, 8, t, e);
  }
  function rc(t, e) {
    vn(2048, 8, t, e);
  }
  function qy(t) {
    w.flags |= 4;
    var e = w.updateQueue;
    if (e === null) ((e = on()), (w.updateQueue = e), (e.events = [t]));
    else {
      var l = e.events;
      l === null ? (e.events = [t]) : l.push(t);
    }
  }
  function Mr(t) {
    var e = _t().memoizedState;
    return (
      qy({ ref: e, nextImpl: t }),
      function () {
        if ((nt & 2) !== 0) throw Error(r(440));
        return e.impl.apply(void 0, arguments);
      }
    );
  }
  function _r(t, e) {
    return vn(4, 2, t, e);
  }
  function Dr(t, e) {
    return vn(4, 4, t, e);
  }
  function Ur(t, e) {
    if (typeof e == "function") {
      t = t();
      var l = e(t);
      return function () {
        typeof l == "function" ? l() : e(null);
      };
    }
    if (e != null)
      return (
        (t = t()),
        (e.current = t),
        function () {
          e.current = null;
        }
      );
  }
  function Rr(t, e, l) {
    ((l = l != null ? l.concat([t]) : null), vn(4, 4, Ur.bind(null, e, t), l));
  }
  function oc() {}
  function Cr(t, e) {
    var l = _t();
    e = e === void 0 ? null : e;
    var a = l.memoizedState;
    return e !== null && tc(e, a[1]) ? a[0] : ((l.memoizedState = [t, e]), t);
  }
  function Nr(t, e) {
    var l = _t();
    e = e === void 0 ? null : e;
    var a = l.memoizedState;
    if (e !== null && tc(e, a[1])) return a[0];
    if (((a = t()), Ll)) {
      el(!0);
      try {
        t();
      } finally {
        el(!1);
      }
    }
    return ((l.memoizedState = [a, e]), a);
  }
  function hc(t, e, l) {
    return l === void 0 || ((Ke & 1073741824) !== 0 && (P & 261930) === 0)
      ? (t.memoizedState = e)
      : ((t.memoizedState = l), (t = jo()), (w.lanes |= t), (vl |= t), l);
  }
  function Hr(t, e, l, a) {
    return fe(l, e)
      ? l
      : ma.current !== null
        ? ((t = hc(t, l, a)), fe(t, e) || (Nt = !0), t)
        : (Ke & 42) === 0 || ((Ke & 1073741824) !== 0 && (P & 261930) === 0)
          ? ((Nt = !0), (t.memoizedState = l))
          : ((t = jo()), (w.lanes |= t), (vl |= t), e);
  }
  function jr(t, e, l, a, u) {
    var n = H.p;
    H.p = n !== 0 && 8 > n ? n : 8;
    var c = O.T,
      s = {};
    ((O.T = s), vc(t, !1, e, l));
    try {
      var h = u(),
        S = O.S;
      if (
        (S !== null && S(s, h), h !== null && typeof h == "object" && typeof h.then == "function")
      ) {
        var E = Ry(h, a);
        nu(t, e, E, ye(t));
      } else nu(t, e, a, ye(t));
    } catch (_) {
      nu(t, e, { then: function () {}, status: "rejected", reason: _ }, ye());
    } finally {
      ((H.p = n), c !== null && s.types !== null && (c.types = s.types), (O.T = c));
    }
  }
  function Qy() {}
  function dc(t, e, l, a) {
    if (t.tag !== 5) throw Error(r(476));
    var u = qr(t).queue;
    jr(
      t,
      u,
      e,
      L,
      l === null
        ? Qy
        : function () {
            return (Qr(t), l(a));
          },
    );
  }
  function qr(t) {
    var e = t.memoizedState;
    if (e !== null) return e;
    e = {
      memoizedState: L,
      baseState: L,
      baseQueue: null,
      queue: {
        pending: null,
        lanes: 0,
        dispatch: null,
        lastRenderedReducer: Ve,
        lastRenderedState: L,
      },
      next: null,
    };
    var l = {};
    return (
      (e.next = {
        memoizedState: l,
        baseState: l,
        baseQueue: null,
        queue: {
          pending: null,
          lanes: 0,
          dispatch: null,
          lastRenderedReducer: Ve,
          lastRenderedState: l,
        },
        next: null,
      }),
      (t.memoizedState = e),
      (t = t.alternate),
      t !== null && (t.memoizedState = e),
      e
    );
  }
  function Qr(t) {
    var e = qr(t);
    (e.next === null && (e = t.alternate.memoizedState), nu(t, e.next.queue, {}, ye()));
  }
  function yc() {
    return Zt(Tu);
  }
  function xr() {
    return _t().memoizedState;
  }
  function Br() {
    return _t().memoizedState;
  }
  function xy(t) {
    for (var e = t.return; e !== null; ) {
      switch (e.tag) {
        case 24:
        case 3:
          var l = ye();
          t = sl(l);
          var a = rl(e, t, l);
          (a !== null && (ae(a, e, l), tu(a, e, l)), (e = { cache: Zi() }), (t.payload = e));
          return;
      }
      e = e.return;
    }
  }
  function By(t, e, l) {
    var a = ye();
    ((l = {
      lane: a,
      revertLane: 0,
      gesture: null,
      action: l,
      hasEagerState: !1,
      eagerState: null,
      next: null,
    }),
      mn(t) ? Gr(e, l) : ((l = Ni(t, e, l, a)), l !== null && (ae(l, t, a), Xr(l, e, a))));
  }
  function Yr(t, e, l) {
    var a = ye();
    nu(t, e, l, a);
  }
  function nu(t, e, l, a) {
    var u = {
      lane: a,
      revertLane: 0,
      gesture: null,
      action: l,
      hasEagerState: !1,
      eagerState: null,
      next: null,
    };
    if (mn(t)) Gr(e, u);
    else {
      var n = t.alternate;
      if (
        t.lanes === 0 &&
        (n === null || n.lanes === 0) &&
        ((n = e.lastRenderedReducer), n !== null)
      )
        try {
          var c = e.lastRenderedState,
            s = n(c, l);
          if (((u.hasEagerState = !0), (u.eagerState = s), fe(s, c)))
            return (Wu(t, e, u, 0), mt === null && Fu(), !1);
        } catch {}
      if (((l = Ni(t, e, u, a)), l !== null)) return (ae(l, t, a), Xr(l, e, a), !0);
    }
    return !1;
  }
  function vc(t, e, l, a) {
    if (
      ((a = {
        lane: 2,
        revertLane: wc(),
        gesture: null,
        action: a,
        hasEagerState: !1,
        eagerState: null,
        next: null,
      }),
      mn(t))
    ) {
      if (e) throw Error(r(479));
    } else ((e = Ni(t, l, a, 2)), e !== null && ae(e, t, 2));
  }
  function mn(t) {
    var e = t.alternate;
    return t === w || (e !== null && e === w);
  }
  function Gr(t, e) {
    ga = sn = !0;
    var l = t.pending;
    (l === null ? (e.next = e) : ((e.next = l.next), (l.next = e)), (t.pending = e));
  }
  function Xr(t, e, l) {
    if ((l & 4194048) !== 0) {
      var a = e.lanes;
      ((a &= t.pendingLanes), (l |= a), (e.lanes = l), Jf(t, l));
    }
  }
  var iu = {
    readContext: Zt,
    use: hn,
    useCallback: Ot,
    useContext: Ot,
    useEffect: Ot,
    useImperativeHandle: Ot,
    useLayoutEffect: Ot,
    useInsertionEffect: Ot,
    useMemo: Ot,
    useReducer: Ot,
    useRef: Ot,
    useState: Ot,
    useDebugValue: Ot,
    useDeferredValue: Ot,
    useTransition: Ot,
    useSyncExternalStore: Ot,
    useId: Ot,
    useHostTransitionStatus: Ot,
    useFormState: Ot,
    useActionState: Ot,
    useOptimistic: Ot,
    useMemoCache: Ot,
    useCacheRefresh: Ot,
  };
  iu.useEffectEvent = Ot;
  var Lr = {
      readContext: Zt,
      use: hn,
      useCallback: function (t, e) {
        return ((Wt().memoizedState = [t, e === void 0 ? null : e]), t);
      },
      useContext: Zt,
      useEffect: Ar,
      useImperativeHandle: function (t, e, l) {
        ((l = l != null ? l.concat([t]) : null), yn(4194308, 4, Ur.bind(null, e, t), l));
      },
      useLayoutEffect: function (t, e) {
        return yn(4194308, 4, t, e);
      },
      useInsertionEffect: function (t, e) {
        yn(4, 2, t, e);
      },
      useMemo: function (t, e) {
        var l = Wt();
        e = e === void 0 ? null : e;
        var a = t();
        if (Ll) {
          el(!0);
          try {
            t();
          } finally {
            el(!1);
          }
        }
        return ((l.memoizedState = [a, e]), a);
      },
      useReducer: function (t, e, l) {
        var a = Wt();
        if (l !== void 0) {
          var u = l(e);
          if (Ll) {
            el(!0);
            try {
              l(e);
            } finally {
              el(!1);
            }
          }
        } else u = e;
        return (
          (a.memoizedState = a.baseState = u),
          (t = {
            pending: null,
            lanes: 0,
            dispatch: null,
            lastRenderedReducer: t,
            lastRenderedState: u,
          }),
          (a.queue = t),
          (t = t.dispatch = By.bind(null, w, t)),
          [a.memoizedState, t]
        );
      },
      useRef: function (t) {
        var e = Wt();
        return ((t = { current: t }), (e.memoizedState = t));
      },
      useState: function (t) {
        t = fc(t);
        var e = t.queue,
          l = Yr.bind(null, w, e);
        return ((e.dispatch = l), [t.memoizedState, l]);
      },
      useDebugValue: oc,
      useDeferredValue: function (t, e) {
        var l = Wt();
        return hc(l, t, e);
      },
      useTransition: function () {
        var t = fc(!1);
        return ((t = jr.bind(null, w, t.queue, !0, !1)), (Wt().memoizedState = t), [!1, t]);
      },
      useSyncExternalStore: function (t, e, l) {
        var a = w,
          u = Wt();
        if (et) {
          if (l === void 0) throw Error(r(407));
          l = l();
        } else {
          if (((l = e()), mt === null)) throw Error(r(349));
          (P & 127) !== 0 || sr(a, e, l);
        }
        u.memoizedState = l;
        var n = { value: l, getSnapshot: e };
        return (
          (u.queue = n),
          Ar(or.bind(null, a, n, t), [t]),
          (a.flags |= 2048),
          pa(9, { destroy: void 0 }, rr.bind(null, a, n, l, e), null),
          l
        );
      },
      useId: function () {
        var t = Wt(),
          e = mt.identifierPrefix;
        if (et) {
          var l = He,
            a = Ne;
          ((l = (a & ~(1 << (32 - ce(a) - 1))).toString(32) + l),
            (e = "_" + e + "R_" + l),
            (l = rn++),
            0 < l && (e += "H" + l.toString(32)),
            (e += "_"));
        } else ((l = Cy++), (e = "_" + e + "r_" + l.toString(32) + "_"));
        return (t.memoizedState = e);
      },
      useHostTransitionStatus: yc,
      useFormState: br,
      useActionState: br,
      useOptimistic: function (t) {
        var e = Wt();
        e.memoizedState = e.baseState = t;
        var l = {
          pending: null,
          lanes: 0,
          dispatch: null,
          lastRenderedReducer: null,
          lastRenderedState: null,
        };
        return ((e.queue = l), (e = vc.bind(null, w, !0, l)), (l.dispatch = e), [t, e]);
      },
      useMemoCache: nc,
      useCacheRefresh: function () {
        return (Wt().memoizedState = xy.bind(null, w));
      },
      useEffectEvent: function (t) {
        var e = Wt(),
          l = { impl: t };
        return (
          (e.memoizedState = l),
          function () {
            if ((nt & 2) !== 0) throw Error(r(440));
            return l.impl.apply(void 0, arguments);
          }
        );
      },
    },
    mc = {
      readContext: Zt,
      use: hn,
      useCallback: Cr,
      useContext: Zt,
      useEffect: rc,
      useImperativeHandle: Rr,
      useInsertionEffect: _r,
      useLayoutEffect: Dr,
      useMemo: Nr,
      useReducer: dn,
      useRef: zr,
      useState: function () {
        return dn(Ve);
      },
      useDebugValue: oc,
      useDeferredValue: function (t, e) {
        var l = _t();
        return Hr(l, ot.memoizedState, t, e);
      },
      useTransition: function () {
        var t = dn(Ve)[0],
          e = _t().memoizedState;
        return [typeof t == "boolean" ? t : uu(t), e];
      },
      useSyncExternalStore: fr,
      useId: xr,
      useHostTransitionStatus: yc,
      useFormState: Er,
      useActionState: Er,
      useOptimistic: function (t, e) {
        var l = _t();
        return yr(l, ot, t, e);
      },
      useMemoCache: nc,
      useCacheRefresh: Br,
    };
  mc.useEffectEvent = Mr;
  var Zr = {
    readContext: Zt,
    use: hn,
    useCallback: Cr,
    useContext: Zt,
    useEffect: rc,
    useImperativeHandle: Rr,
    useInsertionEffect: _r,
    useLayoutEffect: Dr,
    useMemo: Nr,
    useReducer: cc,
    useRef: zr,
    useState: function () {
      return cc(Ve);
    },
    useDebugValue: oc,
    useDeferredValue: function (t, e) {
      var l = _t();
      return ot === null ? hc(l, t, e) : Hr(l, ot.memoizedState, t, e);
    },
    useTransition: function () {
      var t = cc(Ve)[0],
        e = _t().memoizedState;
      return [typeof t == "boolean" ? t : uu(t), e];
    },
    useSyncExternalStore: fr,
    useId: xr,
    useHostTransitionStatus: yc,
    useFormState: Or,
    useActionState: Or,
    useOptimistic: function (t, e) {
      var l = _t();
      return ot !== null ? yr(l, ot, t, e) : ((l.baseState = t), [t, l.queue.dispatch]);
    },
    useMemoCache: nc,
    useCacheRefresh: Br,
  };
  Zr.useEffectEvent = Mr;
  function gc(t, e, l, a) {
    ((e = t.memoizedState),
      (l = l(a, e)),
      (l = l == null ? e : N({}, e, l)),
      (t.memoizedState = l),
      t.lanes === 0 && (t.updateQueue.baseState = l));
  }
  var Sc = {
    enqueueSetState: function (t, e, l) {
      t = t._reactInternals;
      var a = ye(),
        u = sl(a);
      ((u.payload = e),
        l != null && (u.callback = l),
        (e = rl(t, u, a)),
        e !== null && (ae(e, t, a), tu(e, t, a)));
    },
    enqueueReplaceState: function (t, e, l) {
      t = t._reactInternals;
      var a = ye(),
        u = sl(a);
      ((u.tag = 1),
        (u.payload = e),
        l != null && (u.callback = l),
        (e = rl(t, u, a)),
        e !== null && (ae(e, t, a), tu(e, t, a)));
    },
    enqueueForceUpdate: function (t, e) {
      t = t._reactInternals;
      var l = ye(),
        a = sl(l);
      ((a.tag = 2),
        e != null && (a.callback = e),
        (e = rl(t, a, l)),
        e !== null && (ae(e, t, l), tu(e, t, l)));
    },
  };
  function Kr(t, e, l, a, u, n, c) {
    return (
      (t = t.stateNode),
      typeof t.shouldComponentUpdate == "function"
        ? t.shouldComponentUpdate(a, n, c)
        : e.prototype && e.prototype.isPureReactComponent
          ? !Ja(l, a) || !Ja(u, n)
          : !0
    );
  }
  function Vr(t, e, l, a) {
    ((t = e.state),
      typeof e.componentWillReceiveProps == "function" && e.componentWillReceiveProps(l, a),
      typeof e.UNSAFE_componentWillReceiveProps == "function" &&
        e.UNSAFE_componentWillReceiveProps(l, a),
      e.state !== t && Sc.enqueueReplaceState(e, e.state, null));
  }
  function Zl(t, e) {
    var l = e;
    if ("ref" in e) {
      l = {};
      for (var a in e) a !== "ref" && (l[a] = e[a]);
    }
    if ((t = t.defaultProps)) {
      l === e && (l = N({}, l));
      for (var u in t) l[u] === void 0 && (l[u] = t[u]);
    }
    return l;
  }
  function Jr(t) {
    wu(t);
  }
  function wr(t) {
    console.error(t);
  }
  function Fr(t) {
    wu(t);
  }
  function gn(t, e) {
    try {
      var l = t.onUncaughtError;
      l(e.value, { componentStack: e.stack });
    } catch (a) {
      setTimeout(function () {
        throw a;
      });
    }
  }
  function Wr(t, e, l) {
    try {
      var a = t.onCaughtError;
      a(l.value, { componentStack: l.stack, errorBoundary: e.tag === 1 ? e.stateNode : null });
    } catch (u) {
      setTimeout(function () {
        throw u;
      });
    }
  }
  function pc(t, e, l) {
    return (
      (l = sl(l)),
      (l.tag = 3),
      (l.payload = { element: null }),
      (l.callback = function () {
        gn(t, e);
      }),
      l
    );
  }
  function $r(t) {
    return ((t = sl(t)), (t.tag = 3), t);
  }
  function kr(t, e, l, a) {
    var u = l.type.getDerivedStateFromError;
    if (typeof u == "function") {
      var n = a.value;
      ((t.payload = function () {
        return u(n);
      }),
        (t.callback = function () {
          Wr(e, l, a);
        }));
    }
    var c = l.stateNode;
    c !== null &&
      typeof c.componentDidCatch == "function" &&
      (t.callback = function () {
        (Wr(e, l, a),
          typeof u != "function" && (ml === null ? (ml = new Set([this])) : ml.add(this)));
        var s = a.stack;
        this.componentDidCatch(a.value, { componentStack: s !== null ? s : "" });
      });
  }
  function Yy(t, e, l, a, u) {
    if (((l.flags |= 32768), a !== null && typeof a == "object" && typeof a.then == "function")) {
      if (((e = l.alternate), e !== null && oa(e, l, u, !0), (l = re.current), l !== null)) {
        switch (l.tag) {
          case 31:
          case 13:
            return (
              ze === null ? Un() : l.alternate === null && zt === 0 && (zt = 3),
              (l.flags &= -257),
              (l.flags |= 65536),
              (l.lanes = u),
              a === an
                ? (l.flags |= 16384)
                : ((e = l.updateQueue),
                  e === null ? (l.updateQueue = new Set([a])) : e.add(a),
                  Kc(t, a, u)),
              !1
            );
          case 22:
            return (
              (l.flags |= 65536),
              a === an
                ? (l.flags |= 16384)
                : ((e = l.updateQueue),
                  e === null
                    ? ((e = { transitions: null, markerInstances: null, retryQueue: new Set([a]) }),
                      (l.updateQueue = e))
                    : ((l = e.retryQueue), l === null ? (e.retryQueue = new Set([a])) : l.add(a)),
                  Kc(t, a, u)),
              !1
            );
        }
        throw Error(r(435, l.tag));
      }
      return (Kc(t, a, u), Un(), !1);
    }
    if (et)
      return (
        (e = re.current),
        e !== null
          ? ((e.flags & 65536) === 0 && (e.flags |= 256),
            (e.flags |= 65536),
            (e.lanes = u),
            a !== Bi && ((t = Error(r(422), { cause: a })), Wa(be(t, l))))
          : (a !== Bi && ((e = Error(r(423), { cause: a })), Wa(be(e, l))),
            (t = t.current.alternate),
            (t.flags |= 65536),
            (u &= -u),
            (t.lanes |= u),
            (a = be(a, l)),
            (u = pc(t.stateNode, a, u)),
            Wi(t, u),
            zt !== 4 && (zt = 2)),
        !1
      );
    var n = Error(r(520), { cause: a });
    if (((n = be(n, l)), yu === null ? (yu = [n]) : yu.push(n), zt !== 4 && (zt = 2), e === null))
      return !0;
    ((a = be(a, l)), (l = e));
    do {
      switch (l.tag) {
        case 3:
          return (
            (l.flags |= 65536),
            (t = u & -u),
            (l.lanes |= t),
            (t = pc(l.stateNode, a, t)),
            Wi(l, t),
            !1
          );
        case 1:
          if (
            ((e = l.type),
            (n = l.stateNode),
            (l.flags & 128) === 0 &&
              (typeof e.getDerivedStateFromError == "function" ||
                (n !== null &&
                  typeof n.componentDidCatch == "function" &&
                  (ml === null || !ml.has(n)))))
          )
            return (
              (l.flags |= 65536),
              (u &= -u),
              (l.lanes |= u),
              (u = $r(u)),
              kr(u, t, l, a),
              Wi(l, u),
              !1
            );
      }
      l = l.return;
    } while (l !== null);
    return !1;
  }
  var bc = Error(r(461)),
    Nt = !1;
  function Kt(t, e, l, a) {
    e.child = t === null ? er(e, null, l, a) : Xl(e, t.child, l, a);
  }
  function Ir(t, e, l, a, u) {
    l = l.render;
    var n = e.ref;
    if ("ref" in a) {
      var c = {};
      for (var s in a) s !== "ref" && (c[s] = a[s]);
    } else c = a;
    return (
      xl(e),
      (a = ec(t, e, l, c, n, u)),
      (s = lc()),
      t !== null && !Nt
        ? (ac(t, e, u), Je(t, e, u))
        : (et && s && Qi(e), (e.flags |= 1), Kt(t, e, a, u), e.child)
    );
  }
  function Pr(t, e, l, a, u) {
    if (t === null) {
      var n = l.type;
      return typeof n == "function" && !Hi(n) && n.defaultProps === void 0 && l.compare === null
        ? ((e.tag = 15), (e.type = n), to(t, e, n, a, u))
        : ((t = ku(l.type, null, a, e, e.mode, u)), (t.ref = e.ref), (t.return = e), (e.child = t));
    }
    if (((n = t.child), !Dc(t, u))) {
      var c = n.memoizedProps;
      if (((l = l.compare), (l = l !== null ? l : Ja), l(c, a) && t.ref === e.ref))
        return Je(t, e, u);
    }
    return ((e.flags |= 1), (t = Ge(n, a)), (t.ref = e.ref), (t.return = e), (e.child = t));
  }
  function to(t, e, l, a, u) {
    if (t !== null) {
      var n = t.memoizedProps;
      if (Ja(n, a) && t.ref === e.ref)
        if (((Nt = !1), (e.pendingProps = a = n), Dc(t, u))) (t.flags & 131072) !== 0 && (Nt = !0);
        else return ((e.lanes = t.lanes), Je(t, e, u));
    }
    return Ec(t, e, l, a, u);
  }
  function eo(t, e, l, a) {
    var u = a.children,
      n = t !== null ? t.memoizedState : null;
    if (
      (t === null &&
        e.stateNode === null &&
        (e.stateNode = {
          _visibility: 1,
          _pendingMarkers: null,
          _retryCache: null,
          _transitions: null,
        }),
      a.mode === "hidden")
    ) {
      if ((e.flags & 128) !== 0) {
        if (((n = n !== null ? n.baseLanes | l : l), t !== null)) {
          for (a = e.child = t.child, u = 0; a !== null; )
            ((u = u | a.lanes | a.childLanes), (a = a.sibling));
          a = u & ~n;
        } else ((a = 0), (e.child = null));
        return lo(t, e, n, l, a);
      }
      if ((l & 536870912) !== 0)
        ((e.memoizedState = { baseLanes: 0, cachePool: null }),
          t !== null && en(e, n !== null ? n.cachePool : null),
          n !== null ? ur(e, n) : ki(),
          nr(e));
      else return ((a = e.lanes = 536870912), lo(t, e, n !== null ? n.baseLanes | l : l, l, a));
    } else
      n !== null
        ? (en(e, n.cachePool), ur(e, n), hl(), (e.memoizedState = null))
        : (t !== null && en(e, null), ki(), hl());
    return (Kt(t, e, u, l), e.child);
  }
  function cu(t, e) {
    return (
      (t !== null && t.tag === 22) ||
        e.stateNode !== null ||
        (e.stateNode = {
          _visibility: 1,
          _pendingMarkers: null,
          _retryCache: null,
          _transitions: null,
        }),
      e.sibling
    );
  }
  function lo(t, e, l, a, u) {
    var n = Vi();
    return (
      (n = n === null ? null : { parent: Rt._currentValue, pool: n }),
      (e.memoizedState = { baseLanes: l, cachePool: n }),
      t !== null && en(e, null),
      ki(),
      nr(e),
      t !== null && oa(t, e, a, !0),
      (e.childLanes = u),
      null
    );
  }
  function Sn(t, e) {
    return (
      (e = bn({ mode: e.mode, children: e.children }, t.mode)),
      (e.ref = t.ref),
      (t.child = e),
      (e.return = t),
      e
    );
  }
  function ao(t, e, l) {
    return (
      Xl(e, t.child, null, l),
      (t = Sn(e, e.pendingProps)),
      (t.flags |= 2),
      oe(e),
      (e.memoizedState = null),
      t
    );
  }
  function Gy(t, e, l) {
    var a = e.pendingProps,
      u = (e.flags & 128) !== 0;
    if (((e.flags &= -129), t === null)) {
      if (et) {
        if (a.mode === "hidden") return ((t = Sn(e, a)), (e.lanes = 536870912), cu(null, t));
        if (
          (Pi(e),
          (t = pt)
            ? ((t = mh(t, Oe)),
              (t = t !== null && t.data === "&" ? t : null),
              t !== null &&
                ((e.memoizedState = {
                  dehydrated: t,
                  treeContext: ul !== null ? { id: Ne, overflow: He } : null,
                  retryLane: 536870912,
                  hydrationErrors: null,
                }),
                (l = Gs(t)),
                (l.return = e),
                (e.child = l),
                (Lt = e),
                (pt = null)))
            : (t = null),
          t === null)
        )
          throw il(e);
        return ((e.lanes = 536870912), null);
      }
      return Sn(e, a);
    }
    var n = t.memoizedState;
    if (n !== null) {
      var c = n.dehydrated;
      if ((Pi(e), u))
        if (e.flags & 256) ((e.flags &= -257), (e = ao(t, e, l)));
        else if (e.memoizedState !== null) ((e.child = t.child), (e.flags |= 128), (e = null));
        else throw Error(r(558));
      else if ((Nt || oa(t, e, l, !1), (u = (l & t.childLanes) !== 0), Nt || u)) {
        if (((a = mt), a !== null && ((c = wf(a, l)), c !== 0 && c !== n.retryLane)))
          throw ((n.retryLane = c), Hl(t, c), ae(a, t, c), bc);
        (Un(), (e = ao(t, e, l)));
      } else
        ((t = n.treeContext),
          (pt = Ae(c.nextSibling)),
          (Lt = e),
          (et = !0),
          (nl = null),
          (Oe = !1),
          t !== null && Zs(e, t),
          (e = Sn(e, a)),
          (e.flags |= 4096));
      return e;
    }
    return (
      (t = Ge(t.child, { mode: a.mode, children: a.children })),
      (t.ref = e.ref),
      (e.child = t),
      (t.return = e),
      t
    );
  }
  function pn(t, e) {
    var l = e.ref;
    if (l === null) t !== null && t.ref !== null && (e.flags |= 4194816);
    else {
      if (typeof l != "function" && typeof l != "object") throw Error(r(284));
      (t === null || t.ref !== l) && (e.flags |= 4194816);
    }
  }
  function Ec(t, e, l, a, u) {
    return (
      xl(e),
      (l = ec(t, e, l, a, void 0, u)),
      (a = lc()),
      t !== null && !Nt
        ? (ac(t, e, u), Je(t, e, u))
        : (et && a && Qi(e), (e.flags |= 1), Kt(t, e, l, u), e.child)
    );
  }
  function uo(t, e, l, a, u, n) {
    return (
      xl(e),
      (e.updateQueue = null),
      (l = cr(e, a, l, u)),
      ir(t),
      (a = lc()),
      t !== null && !Nt
        ? (ac(t, e, n), Je(t, e, n))
        : (et && a && Qi(e), (e.flags |= 1), Kt(t, e, l, n), e.child)
    );
  }
  function no(t, e, l, a, u) {
    if ((xl(e), e.stateNode === null)) {
      var n = ca,
        c = l.contextType;
      (typeof c == "object" && c !== null && (n = Zt(c)),
        (n = new l(a, n)),
        (e.memoizedState = n.state !== null && n.state !== void 0 ? n.state : null),
        (n.updater = Sc),
        (e.stateNode = n),
        (n._reactInternals = e),
        (n = e.stateNode),
        (n.props = a),
        (n.state = e.memoizedState),
        (n.refs = {}),
        wi(e),
        (c = l.contextType),
        (n.context = typeof c == "object" && c !== null ? Zt(c) : ca),
        (n.state = e.memoizedState),
        (c = l.getDerivedStateFromProps),
        typeof c == "function" && (gc(e, l, c, a), (n.state = e.memoizedState)),
        typeof l.getDerivedStateFromProps == "function" ||
          typeof n.getSnapshotBeforeUpdate == "function" ||
          (typeof n.UNSAFE_componentWillMount != "function" &&
            typeof n.componentWillMount != "function") ||
          ((c = n.state),
          typeof n.componentWillMount == "function" && n.componentWillMount(),
          typeof n.UNSAFE_componentWillMount == "function" && n.UNSAFE_componentWillMount(),
          c !== n.state && Sc.enqueueReplaceState(n, n.state, null),
          lu(e, a, n, u),
          eu(),
          (n.state = e.memoizedState)),
        typeof n.componentDidMount == "function" && (e.flags |= 4194308),
        (a = !0));
    } else if (t === null) {
      n = e.stateNode;
      var s = e.memoizedProps,
        h = Zl(l, s);
      n.props = h;
      var S = n.context,
        E = l.contextType;
      ((c = ca), typeof E == "object" && E !== null && (c = Zt(E)));
      var _ = l.getDerivedStateFromProps;
      ((E = typeof _ == "function" || typeof n.getSnapshotBeforeUpdate == "function"),
        (s = e.pendingProps !== s),
        E ||
          (typeof n.UNSAFE_componentWillReceiveProps != "function" &&
            typeof n.componentWillReceiveProps != "function") ||
          ((s || S !== c) && Vr(e, n, a, c)),
        (fl = !1));
      var p = e.memoizedState;
      ((n.state = p),
        lu(e, a, n, u),
        eu(),
        (S = e.memoizedState),
        s || p !== S || fl
          ? (typeof _ == "function" && (gc(e, l, _, a), (S = e.memoizedState)),
            (h = fl || Kr(e, l, h, a, p, S, c))
              ? (E ||
                  (typeof n.UNSAFE_componentWillMount != "function" &&
                    typeof n.componentWillMount != "function") ||
                  (typeof n.componentWillMount == "function" && n.componentWillMount(),
                  typeof n.UNSAFE_componentWillMount == "function" &&
                    n.UNSAFE_componentWillMount()),
                typeof n.componentDidMount == "function" && (e.flags |= 4194308))
              : (typeof n.componentDidMount == "function" && (e.flags |= 4194308),
                (e.memoizedProps = a),
                (e.memoizedState = S)),
            (n.props = a),
            (n.state = S),
            (n.context = c),
            (a = h))
          : (typeof n.componentDidMount == "function" && (e.flags |= 4194308), (a = !1)));
    } else {
      ((n = e.stateNode),
        Fi(t, e),
        (c = e.memoizedProps),
        (E = Zl(l, c)),
        (n.props = E),
        (_ = e.pendingProps),
        (p = n.context),
        (S = l.contextType),
        (h = ca),
        typeof S == "object" && S !== null && (h = Zt(S)),
        (s = l.getDerivedStateFromProps),
        (S = typeof s == "function" || typeof n.getSnapshotBeforeUpdate == "function") ||
          (typeof n.UNSAFE_componentWillReceiveProps != "function" &&
            typeof n.componentWillReceiveProps != "function") ||
          ((c !== _ || p !== h) && Vr(e, n, a, h)),
        (fl = !1),
        (p = e.memoizedState),
        (n.state = p),
        lu(e, a, n, u),
        eu());
      var b = e.memoizedState;
      c !== _ || p !== b || fl || (t !== null && t.dependencies !== null && Pu(t.dependencies))
        ? (typeof s == "function" && (gc(e, l, s, a), (b = e.memoizedState)),
          (E =
            fl ||
            Kr(e, l, E, a, p, b, h) ||
            (t !== null && t.dependencies !== null && Pu(t.dependencies)))
            ? (S ||
                (typeof n.UNSAFE_componentWillUpdate != "function" &&
                  typeof n.componentWillUpdate != "function") ||
                (typeof n.componentWillUpdate == "function" && n.componentWillUpdate(a, b, h),
                typeof n.UNSAFE_componentWillUpdate == "function" &&
                  n.UNSAFE_componentWillUpdate(a, b, h)),
              typeof n.componentDidUpdate == "function" && (e.flags |= 4),
              typeof n.getSnapshotBeforeUpdate == "function" && (e.flags |= 1024))
            : (typeof n.componentDidUpdate != "function" ||
                (c === t.memoizedProps && p === t.memoizedState) ||
                (e.flags |= 4),
              typeof n.getSnapshotBeforeUpdate != "function" ||
                (c === t.memoizedProps && p === t.memoizedState) ||
                (e.flags |= 1024),
              (e.memoizedProps = a),
              (e.memoizedState = b)),
          (n.props = a),
          (n.state = b),
          (n.context = h),
          (a = E))
        : (typeof n.componentDidUpdate != "function" ||
            (c === t.memoizedProps && p === t.memoizedState) ||
            (e.flags |= 4),
          typeof n.getSnapshotBeforeUpdate != "function" ||
            (c === t.memoizedProps && p === t.memoizedState) ||
            (e.flags |= 1024),
          (a = !1));
    }
    return (
      (n = a),
      pn(t, e),
      (a = (e.flags & 128) !== 0),
      n || a
        ? ((n = e.stateNode),
          (l = a && typeof l.getDerivedStateFromError != "function" ? null : n.render()),
          (e.flags |= 1),
          t !== null && a
            ? ((e.child = Xl(e, t.child, null, u)), (e.child = Xl(e, null, l, u)))
            : Kt(t, e, l, u),
          (e.memoizedState = n.state),
          (t = e.child))
        : (t = Je(t, e, u)),
      t
    );
  }
  function io(t, e, l, a) {
    return (ql(), (e.flags |= 256), Kt(t, e, l, a), e.child);
  }
  var Tc = { dehydrated: null, treeContext: null, retryLane: 0, hydrationErrors: null };
  function Oc(t) {
    return { baseLanes: t, cachePool: Ws() };
  }
  function zc(t, e, l) {
    return ((t = t !== null ? t.childLanes & ~l : 0), e && (t |= de), t);
  }
  function co(t, e, l) {
    var a = e.pendingProps,
      u = !1,
      n = (e.flags & 128) !== 0,
      c;
    if (
      ((c = n) || (c = t !== null && t.memoizedState === null ? !1 : (Mt.current & 2) !== 0),
      c && ((u = !0), (e.flags &= -129)),
      (c = (e.flags & 32) !== 0),
      (e.flags &= -33),
      t === null)
    ) {
      if (et) {
        if (
          (u ? ol(e) : hl(),
          (t = pt)
            ? ((t = mh(t, Oe)),
              (t = t !== null && t.data !== "&" ? t : null),
              t !== null &&
                ((e.memoizedState = {
                  dehydrated: t,
                  treeContext: ul !== null ? { id: Ne, overflow: He } : null,
                  retryLane: 536870912,
                  hydrationErrors: null,
                }),
                (l = Gs(t)),
                (l.return = e),
                (e.child = l),
                (Lt = e),
                (pt = null)))
            : (t = null),
          t === null)
        )
          throw il(e);
        return (cf(t) ? (e.lanes = 32) : (e.lanes = 536870912), null);
      }
      var s = a.children;
      return (
        (a = a.fallback),
        u
          ? (hl(),
            (u = e.mode),
            (s = bn({ mode: "hidden", children: s }, u)),
            (a = jl(a, u, l, null)),
            (s.return = e),
            (a.return = e),
            (s.sibling = a),
            (e.child = s),
            (a = e.child),
            (a.memoizedState = Oc(l)),
            (a.childLanes = zc(t, c, l)),
            (e.memoizedState = Tc),
            cu(null, a))
          : (ol(e), Ac(e, s))
      );
    }
    var h = t.memoizedState;
    if (h !== null && ((s = h.dehydrated), s !== null)) {
      if (n)
        e.flags & 256
          ? (ol(e), (e.flags &= -257), (e = Mc(t, e, l)))
          : e.memoizedState !== null
            ? (hl(), (e.child = t.child), (e.flags |= 128), (e = null))
            : (hl(),
              (s = a.fallback),
              (u = e.mode),
              (a = bn({ mode: "visible", children: a.children }, u)),
              (s = jl(s, u, l, null)),
              (s.flags |= 2),
              (a.return = e),
              (s.return = e),
              (a.sibling = s),
              (e.child = a),
              Xl(e, t.child, null, l),
              (a = e.child),
              (a.memoizedState = Oc(l)),
              (a.childLanes = zc(t, c, l)),
              (e.memoizedState = Tc),
              (e = cu(null, a)));
      else if ((ol(e), cf(s))) {
        if (((c = s.nextSibling && s.nextSibling.dataset), c)) var S = c.dgst;
        ((c = S),
          (a = Error(r(419))),
          (a.stack = ""),
          (a.digest = c),
          Wa({ value: a, source: null, stack: null }),
          (e = Mc(t, e, l)));
      } else if ((Nt || oa(t, e, l, !1), (c = (l & t.childLanes) !== 0), Nt || c)) {
        if (((c = mt), c !== null && ((a = wf(c, l)), a !== 0 && a !== h.retryLane)))
          throw ((h.retryLane = a), Hl(t, a), ae(c, t, a), bc);
        (nf(s) || Un(), (e = Mc(t, e, l)));
      } else
        nf(s)
          ? ((e.flags |= 192), (e.child = t.child), (e = null))
          : ((t = h.treeContext),
            (pt = Ae(s.nextSibling)),
            (Lt = e),
            (et = !0),
            (nl = null),
            (Oe = !1),
            t !== null && Zs(e, t),
            (e = Ac(e, a.children)),
            (e.flags |= 4096));
      return e;
    }
    return u
      ? (hl(),
        (s = a.fallback),
        (u = e.mode),
        (h = t.child),
        (S = h.sibling),
        (a = Ge(h, { mode: "hidden", children: a.children })),
        (a.subtreeFlags = h.subtreeFlags & 65011712),
        S !== null ? (s = Ge(S, s)) : ((s = jl(s, u, l, null)), (s.flags |= 2)),
        (s.return = e),
        (a.return = e),
        (a.sibling = s),
        (e.child = a),
        cu(null, a),
        (a = e.child),
        (s = t.child.memoizedState),
        s === null
          ? (s = Oc(l))
          : ((u = s.cachePool),
            u !== null
              ? ((h = Rt._currentValue), (u = u.parent !== h ? { parent: h, pool: h } : u))
              : (u = Ws()),
            (s = { baseLanes: s.baseLanes | l, cachePool: u })),
        (a.memoizedState = s),
        (a.childLanes = zc(t, c, l)),
        (e.memoizedState = Tc),
        cu(t.child, a))
      : (ol(e),
        (l = t.child),
        (t = l.sibling),
        (l = Ge(l, { mode: "visible", children: a.children })),
        (l.return = e),
        (l.sibling = null),
        t !== null &&
          ((c = e.deletions), c === null ? ((e.deletions = [t]), (e.flags |= 16)) : c.push(t)),
        (e.child = l),
        (e.memoizedState = null),
        l);
  }
  function Ac(t, e) {
    return ((e = bn({ mode: "visible", children: e }, t.mode)), (e.return = t), (t.child = e));
  }
  function bn(t, e) {
    return ((t = se(22, t, null, e)), (t.lanes = 0), t);
  }
  function Mc(t, e, l) {
    return (
      Xl(e, t.child, null, l),
      (t = Ac(e, e.pendingProps.children)),
      (t.flags |= 2),
      (e.memoizedState = null),
      t
    );
  }
  function fo(t, e, l) {
    t.lanes |= e;
    var a = t.alternate;
    (a !== null && (a.lanes |= e), Xi(t.return, e, l));
  }
  function _c(t, e, l, a, u, n) {
    var c = t.memoizedState;
    c === null
      ? (t.memoizedState = {
          isBackwards: e,
          rendering: null,
          renderingStartTime: 0,
          last: a,
          tail: l,
          tailMode: u,
          treeForkCount: n,
        })
      : ((c.isBackwards = e),
        (c.rendering = null),
        (c.renderingStartTime = 0),
        (c.last = a),
        (c.tail = l),
        (c.tailMode = u),
        (c.treeForkCount = n));
  }
  function so(t, e, l) {
    var a = e.pendingProps,
      u = a.revealOrder,
      n = a.tail;
    a = a.children;
    var c = Mt.current,
      s = (c & 2) !== 0;
    if (
      (s ? ((c = (c & 1) | 2), (e.flags |= 128)) : (c &= 1),
      j(Mt, c),
      Kt(t, e, a, l),
      (a = et ? Fa : 0),
      !s && t !== null && (t.flags & 128) !== 0)
    )
      t: for (t = e.child; t !== null; ) {
        if (t.tag === 13) t.memoizedState !== null && fo(t, l, e);
        else if (t.tag === 19) fo(t, l, e);
        else if (t.child !== null) {
          ((t.child.return = t), (t = t.child));
          continue;
        }
        if (t === e) break t;
        for (; t.sibling === null; ) {
          if (t.return === null || t.return === e) break t;
          t = t.return;
        }
        ((t.sibling.return = t.return), (t = t.sibling));
      }
    switch (u) {
      case "forwards":
        for (l = e.child, u = null; l !== null; )
          ((t = l.alternate), t !== null && fn(t) === null && (u = l), (l = l.sibling));
        ((l = u),
          l === null ? ((u = e.child), (e.child = null)) : ((u = l.sibling), (l.sibling = null)),
          _c(e, !1, u, l, n, a));
        break;
      case "backwards":
      case "unstable_legacy-backwards":
        for (l = null, u = e.child, e.child = null; u !== null; ) {
          if (((t = u.alternate), t !== null && fn(t) === null)) {
            e.child = u;
            break;
          }
          ((t = u.sibling), (u.sibling = l), (l = u), (u = t));
        }
        _c(e, !0, l, null, n, a);
        break;
      case "together":
        _c(e, !1, null, null, void 0, a);
        break;
      default:
        e.memoizedState = null;
    }
    return e.child;
  }
  function Je(t, e, l) {
    if (
      (t !== null && (e.dependencies = t.dependencies), (vl |= e.lanes), (l & e.childLanes) === 0)
    )
      if (t !== null) {
        if ((oa(t, e, l, !1), (l & e.childLanes) === 0)) return null;
      } else return null;
    if (t !== null && e.child !== t.child) throw Error(r(153));
    if (e.child !== null) {
      for (t = e.child, l = Ge(t, t.pendingProps), e.child = l, l.return = e; t.sibling !== null; )
        ((t = t.sibling), (l = l.sibling = Ge(t, t.pendingProps)), (l.return = e));
      l.sibling = null;
    }
    return e.child;
  }
  function Dc(t, e) {
    return (t.lanes & e) !== 0 ? !0 : ((t = t.dependencies), !!(t !== null && Pu(t)));
  }
  function Xy(t, e, l) {
    switch (e.tag) {
      case 3:
        (Ft(e, e.stateNode.containerInfo), cl(e, Rt, t.memoizedState.cache), ql());
        break;
      case 27:
      case 5:
        Ha(e);
        break;
      case 4:
        Ft(e, e.stateNode.containerInfo);
        break;
      case 10:
        cl(e, e.type, e.memoizedProps.value);
        break;
      case 31:
        if (e.memoizedState !== null) return ((e.flags |= 128), Pi(e), null);
        break;
      case 13:
        var a = e.memoizedState;
        if (a !== null)
          return a.dehydrated !== null
            ? (ol(e), (e.flags |= 128), null)
            : (l & e.child.childLanes) !== 0
              ? co(t, e, l)
              : (ol(e), (t = Je(t, e, l)), t !== null ? t.sibling : null);
        ol(e);
        break;
      case 19:
        var u = (t.flags & 128) !== 0;
        if (
          ((a = (l & e.childLanes) !== 0),
          a || (oa(t, e, l, !1), (a = (l & e.childLanes) !== 0)),
          u)
        ) {
          if (a) return so(t, e, l);
          e.flags |= 128;
        }
        if (
          ((u = e.memoizedState),
          u !== null && ((u.rendering = null), (u.tail = null), (u.lastEffect = null)),
          j(Mt, Mt.current),
          a)
        )
          break;
        return null;
      case 22:
        return ((e.lanes = 0), eo(t, e, l, e.pendingProps));
      case 24:
        cl(e, Rt, t.memoizedState.cache);
    }
    return Je(t, e, l);
  }
  function ro(t, e, l) {
    if (t !== null)
      if (t.memoizedProps !== e.pendingProps) Nt = !0;
      else {
        if (!Dc(t, l) && (e.flags & 128) === 0) return ((Nt = !1), Xy(t, e, l));
        Nt = (t.flags & 131072) !== 0;
      }
    else ((Nt = !1), et && (e.flags & 1048576) !== 0 && Ls(e, Fa, e.index));
    switch (((e.lanes = 0), e.tag)) {
      case 16:
        t: {
          var a = e.pendingProps;
          if (((t = Yl(e.elementType)), (e.type = t), typeof t == "function"))
            Hi(t)
              ? ((a = Zl(t, a)), (e.tag = 1), (e = no(null, e, t, a, l)))
              : ((e.tag = 0), (e = Ec(null, e, t, a, l)));
          else {
            if (t != null) {
              var u = t.$$typeof;
              if (u === Ut) {
                ((e.tag = 11), (e = Ir(null, e, t, a, l)));
                break t;
              } else if (u === K) {
                ((e.tag = 14), (e = Pr(null, e, t, a, l)));
                break t;
              }
            }
            throw ((e = Qe(t) || t), Error(r(306, e, "")));
          }
        }
        return e;
      case 0:
        return Ec(t, e, e.type, e.pendingProps, l);
      case 1:
        return ((a = e.type), (u = Zl(a, e.pendingProps)), no(t, e, a, u, l));
      case 3:
        t: {
          if ((Ft(e, e.stateNode.containerInfo), t === null)) throw Error(r(387));
          a = e.pendingProps;
          var n = e.memoizedState;
          ((u = n.element), Fi(t, e), lu(e, a, null, l));
          var c = e.memoizedState;
          if (
            ((a = c.cache),
            cl(e, Rt, a),
            a !== n.cache && Li(e, [Rt], l, !0),
            eu(),
            (a = c.element),
            n.isDehydrated)
          )
            if (
              ((n = { element: a, isDehydrated: !1, cache: c.cache }),
              (e.updateQueue.baseState = n),
              (e.memoizedState = n),
              e.flags & 256)
            ) {
              e = io(t, e, a, l);
              break t;
            } else if (a !== u) {
              ((u = be(Error(r(424)), e)), Wa(u), (e = io(t, e, a, l)));
              break t;
            } else
              for (
                t = e.stateNode.containerInfo,
                  t.nodeType === 9
                    ? (t = t.body)
                    : (t = t.nodeName === "HTML" ? t.ownerDocument.body : t),
                  pt = Ae(t.firstChild),
                  Lt = e,
                  et = !0,
                  nl = null,
                  Oe = !0,
                  l = er(e, null, a, l),
                  e.child = l;
                l;
              )
                ((l.flags = (l.flags & -3) | 4096), (l = l.sibling));
          else {
            if ((ql(), a === u)) {
              e = Je(t, e, l);
              break t;
            }
            Kt(t, e, a, l);
          }
          e = e.child;
        }
        return e;
      case 26:
        return (
          pn(t, e),
          t === null
            ? (l = Th(e.type, null, e.pendingProps, null))
              ? (e.memoizedState = l)
              : et ||
                ((l = e.type),
                (t = e.pendingProps),
                (a = Qn($.current).createElement(l)),
                (a[Xt] = e),
                (a[kt] = t),
                Vt(a, l, t),
                Yt(a),
                (e.stateNode = a))
            : (e.memoizedState = Th(e.type, t.memoizedProps, e.pendingProps, t.memoizedState)),
          null
        );
      case 27:
        return (
          Ha(e),
          t === null &&
            et &&
            ((a = e.stateNode = ph(e.type, e.pendingProps, $.current)),
            (Lt = e),
            (Oe = !0),
            (u = pt),
            bl(e.type) ? ((ff = u), (pt = Ae(a.firstChild))) : (pt = u)),
          Kt(t, e, e.pendingProps.children, l),
          pn(t, e),
          t === null && (e.flags |= 4194304),
          e.child
        );
      case 5:
        return (
          t === null &&
            et &&
            ((u = a = pt) &&
              ((a = gv(a, e.type, e.pendingProps, Oe)),
              a !== null
                ? ((e.stateNode = a), (Lt = e), (pt = Ae(a.firstChild)), (Oe = !1), (u = !0))
                : (u = !1)),
            u || il(e)),
          Ha(e),
          (u = e.type),
          (n = e.pendingProps),
          (c = t !== null ? t.memoizedProps : null),
          (a = n.children),
          lf(u, n) ? (a = null) : c !== null && lf(u, c) && (e.flags |= 32),
          e.memoizedState !== null && ((u = ec(t, e, Ny, null, null, l)), (Tu._currentValue = u)),
          pn(t, e),
          Kt(t, e, a, l),
          e.child
        );
      case 6:
        return (
          t === null &&
            et &&
            ((t = l = pt) &&
              ((l = Sv(l, e.pendingProps, Oe)),
              l !== null ? ((e.stateNode = l), (Lt = e), (pt = null), (t = !0)) : (t = !1)),
            t || il(e)),
          null
        );
      case 13:
        return co(t, e, l);
      case 4:
        return (
          Ft(e, e.stateNode.containerInfo),
          (a = e.pendingProps),
          t === null ? (e.child = Xl(e, null, a, l)) : Kt(t, e, a, l),
          e.child
        );
      case 11:
        return Ir(t, e, e.type, e.pendingProps, l);
      case 7:
        return (Kt(t, e, e.pendingProps, l), e.child);
      case 8:
        return (Kt(t, e, e.pendingProps.children, l), e.child);
      case 12:
        return (Kt(t, e, e.pendingProps.children, l), e.child);
      case 10:
        return ((a = e.pendingProps), cl(e, e.type, a.value), Kt(t, e, a.children, l), e.child);
      case 9:
        return (
          (u = e.type._context),
          (a = e.pendingProps.children),
          xl(e),
          (u = Zt(u)),
          (a = a(u)),
          (e.flags |= 1),
          Kt(t, e, a, l),
          e.child
        );
      case 14:
        return Pr(t, e, e.type, e.pendingProps, l);
      case 15:
        return to(t, e, e.type, e.pendingProps, l);
      case 19:
        return so(t, e, l);
      case 31:
        return Gy(t, e, l);
      case 22:
        return eo(t, e, l, e.pendingProps);
      case 24:
        return (
          xl(e),
          (a = Zt(Rt)),
          t === null
            ? ((u = Vi()),
              u === null &&
                ((u = mt),
                (n = Zi()),
                (u.pooledCache = n),
                n.refCount++,
                n !== null && (u.pooledCacheLanes |= l),
                (u = n)),
              (e.memoizedState = { parent: a, cache: u }),
              wi(e),
              cl(e, Rt, u))
            : ((t.lanes & l) !== 0 && (Fi(t, e), lu(e, null, null, l), eu()),
              (u = t.memoizedState),
              (n = e.memoizedState),
              u.parent !== a
                ? ((u = { parent: a, cache: a }),
                  (e.memoizedState = u),
                  e.lanes === 0 && (e.memoizedState = e.updateQueue.baseState = u),
                  cl(e, Rt, a))
                : ((a = n.cache), cl(e, Rt, a), a !== u.cache && Li(e, [Rt], l, !0))),
          Kt(t, e, e.pendingProps.children, l),
          e.child
        );
      case 29:
        throw e.pendingProps;
    }
    throw Error(r(156, e.tag));
  }
  function we(t) {
    t.flags |= 4;
  }
  function Uc(t, e, l, a, u) {
    if (((e = (t.mode & 32) !== 0) && (e = !1), e)) {
      if (((t.flags |= 16777216), (u & 335544128) === u))
        if (t.stateNode.complete) t.flags |= 8192;
        else if (Bo()) t.flags |= 8192;
        else throw ((Gl = an), Ji);
    } else t.flags &= -16777217;
  }
  function oo(t, e) {
    if (e.type !== "stylesheet" || (e.state.loading & 4) !== 0) t.flags &= -16777217;
    else if (((t.flags |= 16777216), !_h(e)))
      if (Bo()) t.flags |= 8192;
      else throw ((Gl = an), Ji);
  }
  function En(t, e) {
    (e !== null && (t.flags |= 4),
      t.flags & 16384 && ((e = t.tag !== 22 ? Kf() : 536870912), (t.lanes |= e), (Oa |= e)));
  }
  function fu(t, e) {
    if (!et)
      switch (t.tailMode) {
        case "hidden":
          e = t.tail;
          for (var l = null; e !== null; ) (e.alternate !== null && (l = e), (e = e.sibling));
          l === null ? (t.tail = null) : (l.sibling = null);
          break;
        case "collapsed":
          l = t.tail;
          for (var a = null; l !== null; ) (l.alternate !== null && (a = l), (l = l.sibling));
          a === null
            ? e || t.tail === null
              ? (t.tail = null)
              : (t.tail.sibling = null)
            : (a.sibling = null);
      }
  }
  function bt(t) {
    var e = t.alternate !== null && t.alternate.child === t.child,
      l = 0,
      a = 0;
    if (e)
      for (var u = t.child; u !== null; )
        ((l |= u.lanes | u.childLanes),
          (a |= u.subtreeFlags & 65011712),
          (a |= u.flags & 65011712),
          (u.return = t),
          (u = u.sibling));
    else
      for (u = t.child; u !== null; )
        ((l |= u.lanes | u.childLanes),
          (a |= u.subtreeFlags),
          (a |= u.flags),
          (u.return = t),
          (u = u.sibling));
    return ((t.subtreeFlags |= a), (t.childLanes = l), e);
  }
  function Ly(t, e, l) {
    var a = e.pendingProps;
    switch ((xi(e), e.tag)) {
      case 16:
      case 15:
      case 0:
      case 11:
      case 7:
      case 8:
      case 12:
      case 9:
      case 14:
        return (bt(e), null);
      case 1:
        return (bt(e), null);
      case 3:
        return (
          (l = e.stateNode),
          (a = null),
          t !== null && (a = t.memoizedState.cache),
          e.memoizedState.cache !== a && (e.flags |= 2048),
          Ze(Rt),
          At(),
          l.pendingContext && ((l.context = l.pendingContext), (l.pendingContext = null)),
          (t === null || t.child === null) &&
            (ra(e)
              ? we(e)
              : t === null ||
                (t.memoizedState.isDehydrated && (e.flags & 256) === 0) ||
                ((e.flags |= 1024), Yi())),
          bt(e),
          null
        );
      case 26:
        var u = e.type,
          n = e.memoizedState;
        return (
          t === null
            ? (we(e), n !== null ? (bt(e), oo(e, n)) : (bt(e), Uc(e, u, null, a, l)))
            : n
              ? n !== t.memoizedState
                ? (we(e), bt(e), oo(e, n))
                : (bt(e), (e.flags &= -16777217))
              : ((t = t.memoizedProps), t !== a && we(e), bt(e), Uc(e, u, t, a, l)),
          null
        );
      case 27:
        if ((Cu(e), (l = $.current), (u = e.type), t !== null && e.stateNode != null))
          t.memoizedProps !== a && we(e);
        else {
          if (!a) {
            if (e.stateNode === null) throw Error(r(166));
            return (bt(e), null);
          }
          ((t = B.current), ra(e) ? Ks(e) : ((t = ph(u, a, l)), (e.stateNode = t), we(e)));
        }
        return (bt(e), null);
      case 5:
        if ((Cu(e), (u = e.type), t !== null && e.stateNode != null))
          t.memoizedProps !== a && we(e);
        else {
          if (!a) {
            if (e.stateNode === null) throw Error(r(166));
            return (bt(e), null);
          }
          if (((n = B.current), ra(e))) Ks(e);
          else {
            var c = Qn($.current);
            switch (n) {
              case 1:
                n = c.createElementNS("http://www.w3.org/2000/svg", u);
                break;
              case 2:
                n = c.createElementNS("http://www.w3.org/1998/Math/MathML", u);
                break;
              default:
                switch (u) {
                  case "svg":
                    n = c.createElementNS("http://www.w3.org/2000/svg", u);
                    break;
                  case "math":
                    n = c.createElementNS("http://www.w3.org/1998/Math/MathML", u);
                    break;
                  case "script":
                    ((n = c.createElement("div")),
                      (n.innerHTML = "<script><\/script>"),
                      (n = n.removeChild(n.firstChild)));
                    break;
                  case "select":
                    ((n =
                      typeof a.is == "string"
                        ? c.createElement("select", { is: a.is })
                        : c.createElement("select")),
                      a.multiple ? (n.multiple = !0) : a.size && (n.size = a.size));
                    break;
                  default:
                    n =
                      typeof a.is == "string"
                        ? c.createElement(u, { is: a.is })
                        : c.createElement(u);
                }
            }
            ((n[Xt] = e), (n[kt] = a));
            t: for (c = e.child; c !== null; ) {
              if (c.tag === 5 || c.tag === 6) n.appendChild(c.stateNode);
              else if (c.tag !== 4 && c.tag !== 27 && c.child !== null) {
                ((c.child.return = c), (c = c.child));
                continue;
              }
              if (c === e) break t;
              for (; c.sibling === null; ) {
                if (c.return === null || c.return === e) break t;
                c = c.return;
              }
              ((c.sibling.return = c.return), (c = c.sibling));
            }
            e.stateNode = n;
            t: switch ((Vt(n, u, a), u)) {
              case "button":
              case "input":
              case "select":
              case "textarea":
                a = !!a.autoFocus;
                break t;
              case "img":
                a = !0;
                break t;
              default:
                a = !1;
            }
            a && we(e);
          }
        }
        return (bt(e), Uc(e, e.type, t === null ? null : t.memoizedProps, e.pendingProps, l), null);
      case 6:
        if (t && e.stateNode != null) t.memoizedProps !== a && we(e);
        else {
          if (typeof a != "string" && e.stateNode === null) throw Error(r(166));
          if (((t = $.current), ra(e))) {
            if (((t = e.stateNode), (l = e.memoizedProps), (a = null), (u = Lt), u !== null))
              switch (u.tag) {
                case 27:
                case 5:
                  a = u.memoizedProps;
              }
            ((t[Xt] = e),
              (t = !!(
                t.nodeValue === l ||
                (a !== null && a.suppressHydrationWarning === !0) ||
                fh(t.nodeValue, l)
              )),
              t || il(e, !0));
          } else ((t = Qn(t).createTextNode(a)), (t[Xt] = e), (e.stateNode = t));
        }
        return (bt(e), null);
      case 31:
        if (((l = e.memoizedState), t === null || t.memoizedState !== null)) {
          if (((a = ra(e)), l !== null)) {
            if (t === null) {
              if (!a) throw Error(r(318));
              if (((t = e.memoizedState), (t = t !== null ? t.dehydrated : null), !t))
                throw Error(r(557));
              t[Xt] = e;
            } else (ql(), (e.flags & 128) === 0 && (e.memoizedState = null), (e.flags |= 4));
            (bt(e), (t = !1));
          } else
            ((l = Yi()),
              t !== null && t.memoizedState !== null && (t.memoizedState.hydrationErrors = l),
              (t = !0));
          if (!t) return e.flags & 256 ? (oe(e), e) : (oe(e), null);
          if ((e.flags & 128) !== 0) throw Error(r(558));
        }
        return (bt(e), null);
      case 13:
        if (
          ((a = e.memoizedState),
          t === null || (t.memoizedState !== null && t.memoizedState.dehydrated !== null))
        ) {
          if (((u = ra(e)), a !== null && a.dehydrated !== null)) {
            if (t === null) {
              if (!u) throw Error(r(318));
              if (((u = e.memoizedState), (u = u !== null ? u.dehydrated : null), !u))
                throw Error(r(317));
              u[Xt] = e;
            } else (ql(), (e.flags & 128) === 0 && (e.memoizedState = null), (e.flags |= 4));
            (bt(e), (u = !1));
          } else
            ((u = Yi()),
              t !== null && t.memoizedState !== null && (t.memoizedState.hydrationErrors = u),
              (u = !0));
          if (!u) return e.flags & 256 ? (oe(e), e) : (oe(e), null);
        }
        return (
          oe(e),
          (e.flags & 128) !== 0
            ? ((e.lanes = l), e)
            : ((l = a !== null),
              (t = t !== null && t.memoizedState !== null),
              l &&
                ((a = e.child),
                (u = null),
                a.alternate !== null &&
                  a.alternate.memoizedState !== null &&
                  a.alternate.memoizedState.cachePool !== null &&
                  (u = a.alternate.memoizedState.cachePool.pool),
                (n = null),
                a.memoizedState !== null &&
                  a.memoizedState.cachePool !== null &&
                  (n = a.memoizedState.cachePool.pool),
                n !== u && (a.flags |= 2048)),
              l !== t && l && (e.child.flags |= 8192),
              En(e, e.updateQueue),
              bt(e),
              null)
        );
      case 4:
        return (At(), t === null && kc(e.stateNode.containerInfo), bt(e), null);
      case 10:
        return (Ze(e.type), bt(e), null);
      case 19:
        if ((D(Mt), (a = e.memoizedState), a === null)) return (bt(e), null);
        if (((u = (e.flags & 128) !== 0), (n = a.rendering), n === null))
          if (u) fu(a, !1);
          else {
            if (zt !== 0 || (t !== null && (t.flags & 128) !== 0))
              for (t = e.child; t !== null; ) {
                if (((n = fn(t)), n !== null)) {
                  for (
                    e.flags |= 128,
                      fu(a, !1),
                      t = n.updateQueue,
                      e.updateQueue = t,
                      En(e, t),
                      e.subtreeFlags = 0,
                      t = l,
                      l = e.child;
                    l !== null;
                  )
                    (Ys(l, t), (l = l.sibling));
                  return (j(Mt, (Mt.current & 1) | 2), et && Xe(e, a.treeForkCount), e.child);
                }
                t = t.sibling;
              }
            a.tail !== null &&
              ne() > Mn &&
              ((e.flags |= 128), (u = !0), fu(a, !1), (e.lanes = 4194304));
          }
        else {
          if (!u)
            if (((t = fn(n)), t !== null)) {
              if (
                ((e.flags |= 128),
                (u = !0),
                (t = t.updateQueue),
                (e.updateQueue = t),
                En(e, t),
                fu(a, !0),
                a.tail === null && a.tailMode === "hidden" && !n.alternate && !et)
              )
                return (bt(e), null);
            } else
              2 * ne() - a.renderingStartTime > Mn &&
                l !== 536870912 &&
                ((e.flags |= 128), (u = !0), fu(a, !1), (e.lanes = 4194304));
          a.isBackwards
            ? ((n.sibling = e.child), (e.child = n))
            : ((t = a.last), t !== null ? (t.sibling = n) : (e.child = n), (a.last = n));
        }
        return a.tail !== null
          ? ((t = a.tail),
            (a.rendering = t),
            (a.tail = t.sibling),
            (a.renderingStartTime = ne()),
            (t.sibling = null),
            (l = Mt.current),
            j(Mt, u ? (l & 1) | 2 : l & 1),
            et && Xe(e, a.treeForkCount),
            t)
          : (bt(e), null);
      case 22:
      case 23:
        return (
          oe(e),
          Ii(),
          (a = e.memoizedState !== null),
          t !== null
            ? (t.memoizedState !== null) !== a && (e.flags |= 8192)
            : a && (e.flags |= 8192),
          a
            ? (l & 536870912) !== 0 &&
              (e.flags & 128) === 0 &&
              (bt(e), e.subtreeFlags & 6 && (e.flags |= 8192))
            : bt(e),
          (l = e.updateQueue),
          l !== null && En(e, l.retryQueue),
          (l = null),
          t !== null &&
            t.memoizedState !== null &&
            t.memoizedState.cachePool !== null &&
            (l = t.memoizedState.cachePool.pool),
          (a = null),
          e.memoizedState !== null &&
            e.memoizedState.cachePool !== null &&
            (a = e.memoizedState.cachePool.pool),
          a !== l && (e.flags |= 2048),
          t !== null && D(Bl),
          null
        );
      case 24:
        return (
          (l = null),
          t !== null && (l = t.memoizedState.cache),
          e.memoizedState.cache !== l && (e.flags |= 2048),
          Ze(Rt),
          bt(e),
          null
        );
      case 25:
        return null;
      case 30:
        return null;
    }
    throw Error(r(156, e.tag));
  }
  function Zy(t, e) {
    switch ((xi(e), e.tag)) {
      case 1:
        return ((t = e.flags), t & 65536 ? ((e.flags = (t & -65537) | 128), e) : null);
      case 3:
        return (
          Ze(Rt),
          At(),
          (t = e.flags),
          (t & 65536) !== 0 && (t & 128) === 0 ? ((e.flags = (t & -65537) | 128), e) : null
        );
      case 26:
      case 27:
      case 5:
        return (Cu(e), null);
      case 31:
        if (e.memoizedState !== null) {
          if ((oe(e), e.alternate === null)) throw Error(r(340));
          ql();
        }
        return ((t = e.flags), t & 65536 ? ((e.flags = (t & -65537) | 128), e) : null);
      case 13:
        if ((oe(e), (t = e.memoizedState), t !== null && t.dehydrated !== null)) {
          if (e.alternate === null) throw Error(r(340));
          ql();
        }
        return ((t = e.flags), t & 65536 ? ((e.flags = (t & -65537) | 128), e) : null);
      case 19:
        return (D(Mt), null);
      case 4:
        return (At(), null);
      case 10:
        return (Ze(e.type), null);
      case 22:
      case 23:
        return (
          oe(e),
          Ii(),
          t !== null && D(Bl),
          (t = e.flags),
          t & 65536 ? ((e.flags = (t & -65537) | 128), e) : null
        );
      case 24:
        return (Ze(Rt), null);
      case 25:
        return null;
      default:
        return null;
    }
  }
  function ho(t, e) {
    switch ((xi(e), e.tag)) {
      case 3:
        (Ze(Rt), At());
        break;
      case 26:
      case 27:
      case 5:
        Cu(e);
        break;
      case 4:
        At();
        break;
      case 31:
        e.memoizedState !== null && oe(e);
        break;
      case 13:
        oe(e);
        break;
      case 19:
        D(Mt);
        break;
      case 10:
        Ze(e.type);
        break;
      case 22:
      case 23:
        (oe(e), Ii(), t !== null && D(Bl));
        break;
      case 24:
        Ze(Rt);
    }
  }
  function su(t, e) {
    try {
      var l = e.updateQueue,
        a = l !== null ? l.lastEffect : null;
      if (a !== null) {
        var u = a.next;
        l = u;
        do {
          if ((l.tag & t) === t) {
            a = void 0;
            var n = l.create,
              c = l.inst;
            ((a = n()), (c.destroy = a));
          }
          l = l.next;
        } while (l !== u);
      }
    } catch (s) {
      ft(e, e.return, s);
    }
  }
  function dl(t, e, l) {
    try {
      var a = e.updateQueue,
        u = a !== null ? a.lastEffect : null;
      if (u !== null) {
        var n = u.next;
        a = n;
        do {
          if ((a.tag & t) === t) {
            var c = a.inst,
              s = c.destroy;
            if (s !== void 0) {
              ((c.destroy = void 0), (u = e));
              var h = l,
                S = s;
              try {
                S();
              } catch (E) {
                ft(u, h, E);
              }
            }
          }
          a = a.next;
        } while (a !== n);
      }
    } catch (E) {
      ft(e, e.return, E);
    }
  }
  function yo(t) {
    var e = t.updateQueue;
    if (e !== null) {
      var l = t.stateNode;
      try {
        ar(e, l);
      } catch (a) {
        ft(t, t.return, a);
      }
    }
  }
  function vo(t, e, l) {
    ((l.props = Zl(t.type, t.memoizedProps)), (l.state = t.memoizedState));
    try {
      l.componentWillUnmount();
    } catch (a) {
      ft(t, e, a);
    }
  }
  function ru(t, e) {
    try {
      var l = t.ref;
      if (l !== null) {
        switch (t.tag) {
          case 26:
          case 27:
          case 5:
            var a = t.stateNode;
            break;
          case 30:
            a = t.stateNode;
            break;
          default:
            a = t.stateNode;
        }
        typeof l == "function" ? (t.refCleanup = l(a)) : (l.current = a);
      }
    } catch (u) {
      ft(t, e, u);
    }
  }
  function je(t, e) {
    var l = t.ref,
      a = t.refCleanup;
    if (l !== null)
      if (typeof a == "function")
        try {
          a();
        } catch (u) {
          ft(t, e, u);
        } finally {
          ((t.refCleanup = null), (t = t.alternate), t != null && (t.refCleanup = null));
        }
      else if (typeof l == "function")
        try {
          l(null);
        } catch (u) {
          ft(t, e, u);
        }
      else l.current = null;
  }
  function mo(t) {
    var e = t.type,
      l = t.memoizedProps,
      a = t.stateNode;
    try {
      t: switch (e) {
        case "button":
        case "input":
        case "select":
        case "textarea":
          l.autoFocus && a.focus();
          break t;
        case "img":
          l.src ? (a.src = l.src) : l.srcSet && (a.srcset = l.srcSet);
      }
    } catch (u) {
      ft(t, t.return, u);
    }
  }
  function Rc(t, e, l) {
    try {
      var a = t.stateNode;
      (ov(a, t.type, l, e), (a[kt] = e));
    } catch (u) {
      ft(t, t.return, u);
    }
  }
  function go(t) {
    return (
      t.tag === 5 || t.tag === 3 || t.tag === 26 || (t.tag === 27 && bl(t.type)) || t.tag === 4
    );
  }
  function Cc(t) {
    t: for (;;) {
      for (; t.sibling === null; ) {
        if (t.return === null || go(t.return)) return null;
        t = t.return;
      }
      for (
        t.sibling.return = t.return, t = t.sibling;
        t.tag !== 5 && t.tag !== 6 && t.tag !== 18;
      ) {
        if ((t.tag === 27 && bl(t.type)) || t.flags & 2 || t.child === null || t.tag === 4)
          continue t;
        ((t.child.return = t), (t = t.child));
      }
      if (!(t.flags & 2)) return t.stateNode;
    }
  }
  function Nc(t, e, l) {
    var a = t.tag;
    if (a === 5 || a === 6)
      ((t = t.stateNode),
        e
          ? (l.nodeType === 9
              ? l.body
              : l.nodeName === "HTML"
                ? l.ownerDocument.body
                : l
            ).insertBefore(t, e)
          : ((e = l.nodeType === 9 ? l.body : l.nodeName === "HTML" ? l.ownerDocument.body : l),
            e.appendChild(t),
            (l = l._reactRootContainer),
            l != null || e.onclick !== null || (e.onclick = Be)));
    else if (
      a !== 4 &&
      (a === 27 && bl(t.type) && ((l = t.stateNode), (e = null)), (t = t.child), t !== null)
    )
      for (Nc(t, e, l), t = t.sibling; t !== null; ) (Nc(t, e, l), (t = t.sibling));
  }
  function Tn(t, e, l) {
    var a = t.tag;
    if (a === 5 || a === 6) ((t = t.stateNode), e ? l.insertBefore(t, e) : l.appendChild(t));
    else if (a !== 4 && (a === 27 && bl(t.type) && (l = t.stateNode), (t = t.child), t !== null))
      for (Tn(t, e, l), t = t.sibling; t !== null; ) (Tn(t, e, l), (t = t.sibling));
  }
  function So(t) {
    var e = t.stateNode,
      l = t.memoizedProps;
    try {
      for (var a = t.type, u = e.attributes; u.length; ) e.removeAttributeNode(u[0]);
      (Vt(e, a, l), (e[Xt] = t), (e[kt] = l));
    } catch (n) {
      ft(t, t.return, n);
    }
  }
  var Fe = !1,
    Ht = !1,
    Hc = !1,
    po = typeof WeakSet == "function" ? WeakSet : Set,
    Gt = null;
  function Ky(t, e) {
    if (((t = t.containerInfo), (tf = Zn), (t = Rs(t)), Mi(t))) {
      if ("selectionStart" in t) var l = { start: t.selectionStart, end: t.selectionEnd };
      else
        t: {
          l = ((l = t.ownerDocument) && l.defaultView) || window;
          var a = l.getSelection && l.getSelection();
          if (a && a.rangeCount !== 0) {
            l = a.anchorNode;
            var u = a.anchorOffset,
              n = a.focusNode;
            a = a.focusOffset;
            try {
              (l.nodeType, n.nodeType);
            } catch {
              l = null;
              break t;
            }
            var c = 0,
              s = -1,
              h = -1,
              S = 0,
              E = 0,
              _ = t,
              p = null;
            e: for (;;) {
              for (
                var b;
                _ !== l || (u !== 0 && _.nodeType !== 3) || (s = c + u),
                  _ !== n || (a !== 0 && _.nodeType !== 3) || (h = c + a),
                  _.nodeType === 3 && (c += _.nodeValue.length),
                  (b = _.firstChild) !== null;
              )
                ((p = _), (_ = b));
              for (;;) {
                if (_ === t) break e;
                if (
                  (p === l && ++S === u && (s = c),
                  p === n && ++E === a && (h = c),
                  (b = _.nextSibling) !== null)
                )
                  break;
                ((_ = p), (p = _.parentNode));
              }
              _ = b;
            }
            l = s === -1 || h === -1 ? null : { start: s, end: h };
          } else l = null;
        }
      l = l || { start: 0, end: 0 };
    } else l = null;
    for (ef = { focusedElem: t, selectionRange: l }, Zn = !1, Gt = e; Gt !== null; )
      if (((e = Gt), (t = e.child), (e.subtreeFlags & 1028) !== 0 && t !== null))
        ((t.return = e), (Gt = t));
      else
        for (; Gt !== null; ) {
          switch (((e = Gt), (n = e.alternate), (t = e.flags), e.tag)) {
            case 0:
              if (
                (t & 4) !== 0 &&
                ((t = e.updateQueue), (t = t !== null ? t.events : null), t !== null)
              )
                for (l = 0; l < t.length; l++) ((u = t[l]), (u.ref.impl = u.nextImpl));
              break;
            case 11:
            case 15:
              break;
            case 1:
              if ((t & 1024) !== 0 && n !== null) {
                ((t = void 0),
                  (l = e),
                  (u = n.memoizedProps),
                  (n = n.memoizedState),
                  (a = l.stateNode));
                try {
                  var x = Zl(l.type, u);
                  ((t = a.getSnapshotBeforeUpdate(x, n)),
                    (a.__reactInternalSnapshotBeforeUpdate = t));
                } catch (X) {
                  ft(l, l.return, X);
                }
              }
              break;
            case 3:
              if ((t & 1024) !== 0) {
                if (((t = e.stateNode.containerInfo), (l = t.nodeType), l === 9)) uf(t);
                else if (l === 1)
                  switch (t.nodeName) {
                    case "HEAD":
                    case "HTML":
                    case "BODY":
                      uf(t);
                      break;
                    default:
                      t.textContent = "";
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
            default:
              if ((t & 1024) !== 0) throw Error(r(163));
          }
          if (((t = e.sibling), t !== null)) {
            ((t.return = e.return), (Gt = t));
            break;
          }
          Gt = e.return;
        }
  }
  function bo(t, e, l) {
    var a = l.flags;
    switch (l.tag) {
      case 0:
      case 11:
      case 15:
        ($e(t, l), a & 4 && su(5, l));
        break;
      case 1:
        if (($e(t, l), a & 4))
          if (((t = l.stateNode), e === null))
            try {
              t.componentDidMount();
            } catch (c) {
              ft(l, l.return, c);
            }
          else {
            var u = Zl(l.type, e.memoizedProps);
            e = e.memoizedState;
            try {
              t.componentDidUpdate(u, e, t.__reactInternalSnapshotBeforeUpdate);
            } catch (c) {
              ft(l, l.return, c);
            }
          }
        (a & 64 && yo(l), a & 512 && ru(l, l.return));
        break;
      case 3:
        if (($e(t, l), a & 64 && ((t = l.updateQueue), t !== null))) {
          if (((e = null), l.child !== null))
            switch (l.child.tag) {
              case 27:
              case 5:
                e = l.child.stateNode;
                break;
              case 1:
                e = l.child.stateNode;
            }
          try {
            ar(t, e);
          } catch (c) {
            ft(l, l.return, c);
          }
        }
        break;
      case 27:
        e === null && a & 4 && So(l);
      case 26:
      case 5:
        ($e(t, l), e === null && a & 4 && mo(l), a & 512 && ru(l, l.return));
        break;
      case 12:
        $e(t, l);
        break;
      case 31:
        ($e(t, l), a & 4 && Oo(t, l));
        break;
      case 13:
        ($e(t, l),
          a & 4 && zo(t, l),
          a & 64 &&
            ((t = l.memoizedState),
            t !== null && ((t = t.dehydrated), t !== null && ((l = Py.bind(null, l)), pv(t, l)))));
        break;
      case 22:
        if (((a = l.memoizedState !== null || Fe), !a)) {
          ((e = (e !== null && e.memoizedState !== null) || Ht), (u = Fe));
          var n = Ht;
          ((Fe = a),
            (Ht = e) && !n ? ke(t, l, (l.subtreeFlags & 8772) !== 0) : $e(t, l),
            (Fe = u),
            (Ht = n));
        }
        break;
      case 30:
        break;
      default:
        $e(t, l);
    }
  }
  function Eo(t) {
    var e = t.alternate;
    (e !== null && ((t.alternate = null), Eo(e)),
      (t.child = null),
      (t.deletions = null),
      (t.sibling = null),
      t.tag === 5 && ((e = t.stateNode), e !== null && si(e)),
      (t.stateNode = null),
      (t.return = null),
      (t.dependencies = null),
      (t.memoizedProps = null),
      (t.memoizedState = null),
      (t.pendingProps = null),
      (t.stateNode = null),
      (t.updateQueue = null));
  }
  var Et = null,
    Pt = !1;
  function We(t, e, l) {
    for (l = l.child; l !== null; ) (To(t, e, l), (l = l.sibling));
  }
  function To(t, e, l) {
    if (ie && typeof ie.onCommitFiberUnmount == "function")
      try {
        ie.onCommitFiberUnmount(ja, l);
      } catch {}
    switch (l.tag) {
      case 26:
        (Ht || je(l, e),
          We(t, e, l),
          l.memoizedState
            ? l.memoizedState.count--
            : l.stateNode && ((l = l.stateNode), l.parentNode.removeChild(l)));
        break;
      case 27:
        Ht || je(l, e);
        var a = Et,
          u = Pt;
        (bl(l.type) && ((Et = l.stateNode), (Pt = !1)),
          We(t, e, l),
          pu(l.stateNode),
          (Et = a),
          (Pt = u));
        break;
      case 5:
        Ht || je(l, e);
      case 6:
        if (((a = Et), (u = Pt), (Et = null), We(t, e, l), (Et = a), (Pt = u), Et !== null))
          if (Pt)
            try {
              (Et.nodeType === 9
                ? Et.body
                : Et.nodeName === "HTML"
                  ? Et.ownerDocument.body
                  : Et
              ).removeChild(l.stateNode);
            } catch (n) {
              ft(l, e, n);
            }
          else
            try {
              Et.removeChild(l.stateNode);
            } catch (n) {
              ft(l, e, n);
            }
        break;
      case 18:
        Et !== null &&
          (Pt
            ? ((t = Et),
              yh(
                t.nodeType === 9 ? t.body : t.nodeName === "HTML" ? t.ownerDocument.body : t,
                l.stateNode,
              ),
              Ca(t))
            : yh(Et, l.stateNode));
        break;
      case 4:
        ((a = Et),
          (u = Pt),
          (Et = l.stateNode.containerInfo),
          (Pt = !0),
          We(t, e, l),
          (Et = a),
          (Pt = u));
        break;
      case 0:
      case 11:
      case 14:
      case 15:
        (dl(2, l, e), Ht || dl(4, l, e), We(t, e, l));
        break;
      case 1:
        (Ht ||
          (je(l, e), (a = l.stateNode), typeof a.componentWillUnmount == "function" && vo(l, e, a)),
          We(t, e, l));
        break;
      case 21:
        We(t, e, l);
        break;
      case 22:
        ((Ht = (a = Ht) || l.memoizedState !== null), We(t, e, l), (Ht = a));
        break;
      default:
        We(t, e, l);
    }
  }
  function Oo(t, e) {
    if (
      e.memoizedState === null &&
      ((t = e.alternate), t !== null && ((t = t.memoizedState), t !== null))
    ) {
      t = t.dehydrated;
      try {
        Ca(t);
      } catch (l) {
        ft(e, e.return, l);
      }
    }
  }
  function zo(t, e) {
    if (
      e.memoizedState === null &&
      ((t = e.alternate),
      t !== null && ((t = t.memoizedState), t !== null && ((t = t.dehydrated), t !== null)))
    )
      try {
        Ca(t);
      } catch (l) {
        ft(e, e.return, l);
      }
  }
  function Vy(t) {
    switch (t.tag) {
      case 31:
      case 13:
      case 19:
        var e = t.stateNode;
        return (e === null && (e = t.stateNode = new po()), e);
      case 22:
        return (
          (t = t.stateNode), (e = t._retryCache), e === null && (e = t._retryCache = new po()), e
        );
      default:
        throw Error(r(435, t.tag));
    }
  }
  function On(t, e) {
    var l = Vy(t);
    e.forEach(function (a) {
      if (!l.has(a)) {
        l.add(a);
        var u = tv.bind(null, t, a);
        a.then(u, u);
      }
    });
  }
  function te(t, e) {
    var l = e.deletions;
    if (l !== null)
      for (var a = 0; a < l.length; a++) {
        var u = l[a],
          n = t,
          c = e,
          s = c;
        t: for (; s !== null; ) {
          switch (s.tag) {
            case 27:
              if (bl(s.type)) {
                ((Et = s.stateNode), (Pt = !1));
                break t;
              }
              break;
            case 5:
              ((Et = s.stateNode), (Pt = !1));
              break t;
            case 3:
            case 4:
              ((Et = s.stateNode.containerInfo), (Pt = !0));
              break t;
          }
          s = s.return;
        }
        if (Et === null) throw Error(r(160));
        (To(n, c, u),
          (Et = null),
          (Pt = !1),
          (n = u.alternate),
          n !== null && (n.return = null),
          (u.return = null));
      }
    if (e.subtreeFlags & 13886) for (e = e.child; e !== null; ) (Ao(e, t), (e = e.sibling));
  }
  var Re = null;
  function Ao(t, e) {
    var l = t.alternate,
      a = t.flags;
    switch (t.tag) {
      case 0:
      case 11:
      case 14:
      case 15:
        (te(e, t), ee(t), a & 4 && (dl(3, t, t.return), su(3, t), dl(5, t, t.return)));
        break;
      case 1:
        (te(e, t),
          ee(t),
          a & 512 && (Ht || l === null || je(l, l.return)),
          a & 64 &&
            Fe &&
            ((t = t.updateQueue),
            t !== null &&
              ((a = t.callbacks),
              a !== null &&
                ((l = t.shared.hiddenCallbacks),
                (t.shared.hiddenCallbacks = l === null ? a : l.concat(a))))));
        break;
      case 26:
        var u = Re;
        if ((te(e, t), ee(t), a & 512 && (Ht || l === null || je(l, l.return)), a & 4)) {
          var n = l !== null ? l.memoizedState : null;
          if (((a = t.memoizedState), l === null))
            if (a === null)
              if (t.stateNode === null) {
                t: {
                  ((a = t.type), (l = t.memoizedProps), (u = u.ownerDocument || u));
                  e: switch (a) {
                    case "title":
                      ((n = u.getElementsByTagName("title")[0]),
                        (!n ||
                          n[xa] ||
                          n[Xt] ||
                          n.namespaceURI === "http://www.w3.org/2000/svg" ||
                          n.hasAttribute("itemprop")) &&
                          ((n = u.createElement(a)),
                          u.head.insertBefore(n, u.querySelector("head > title"))),
                        Vt(n, a, l),
                        (n[Xt] = t),
                        Yt(n),
                        (a = n));
                      break t;
                    case "link":
                      var c = Ah("link", "href", u).get(a + (l.href || ""));
                      if (c) {
                        for (var s = 0; s < c.length; s++)
                          if (
                            ((n = c[s]),
                            n.getAttribute("href") ===
                              (l.href == null || l.href === "" ? null : l.href) &&
                              n.getAttribute("rel") === (l.rel == null ? null : l.rel) &&
                              n.getAttribute("title") === (l.title == null ? null : l.title) &&
                              n.getAttribute("crossorigin") ===
                                (l.crossOrigin == null ? null : l.crossOrigin))
                          ) {
                            c.splice(s, 1);
                            break e;
                          }
                      }
                      ((n = u.createElement(a)), Vt(n, a, l), u.head.appendChild(n));
                      break;
                    case "meta":
                      if ((c = Ah("meta", "content", u).get(a + (l.content || "")))) {
                        for (s = 0; s < c.length; s++)
                          if (
                            ((n = c[s]),
                            n.getAttribute("content") ===
                              (l.content == null ? null : "" + l.content) &&
                              n.getAttribute("name") === (l.name == null ? null : l.name) &&
                              n.getAttribute("property") ===
                                (l.property == null ? null : l.property) &&
                              n.getAttribute("http-equiv") ===
                                (l.httpEquiv == null ? null : l.httpEquiv) &&
                              n.getAttribute("charset") === (l.charSet == null ? null : l.charSet))
                          ) {
                            c.splice(s, 1);
                            break e;
                          }
                      }
                      ((n = u.createElement(a)), Vt(n, a, l), u.head.appendChild(n));
                      break;
                    default:
                      throw Error(r(468, a));
                  }
                  ((n[Xt] = t), Yt(n), (a = n));
                }
                t.stateNode = a;
              } else Mh(u, t.type, t.stateNode);
            else t.stateNode = zh(u, a, t.memoizedProps);
          else
            n !== a
              ? (n === null
                  ? l.stateNode !== null && ((l = l.stateNode), l.parentNode.removeChild(l))
                  : n.count--,
                a === null ? Mh(u, t.type, t.stateNode) : zh(u, a, t.memoizedProps))
              : a === null && t.stateNode !== null && Rc(t, t.memoizedProps, l.memoizedProps);
        }
        break;
      case 27:
        (te(e, t),
          ee(t),
          a & 512 && (Ht || l === null || je(l, l.return)),
          l !== null && a & 4 && Rc(t, t.memoizedProps, l.memoizedProps));
        break;
      case 5:
        if ((te(e, t), ee(t), a & 512 && (Ht || l === null || je(l, l.return)), t.flags & 32)) {
          u = t.stateNode;
          try {
            ta(u, "");
          } catch (x) {
            ft(t, t.return, x);
          }
        }
        (a & 4 &&
          t.stateNode != null &&
          ((u = t.memoizedProps), Rc(t, u, l !== null ? l.memoizedProps : u)),
          a & 1024 && (Hc = !0));
        break;
      case 6:
        if ((te(e, t), ee(t), a & 4)) {
          if (t.stateNode === null) throw Error(r(162));
          ((a = t.memoizedProps), (l = t.stateNode));
          try {
            l.nodeValue = a;
          } catch (x) {
            ft(t, t.return, x);
          }
        }
        break;
      case 3:
        if (
          ((Yn = null),
          (u = Re),
          (Re = xn(e.containerInfo)),
          te(e, t),
          (Re = u),
          ee(t),
          a & 4 && l !== null && l.memoizedState.isDehydrated)
        )
          try {
            Ca(e.containerInfo);
          } catch (x) {
            ft(t, t.return, x);
          }
        Hc && ((Hc = !1), Mo(t));
        break;
      case 4:
        ((a = Re), (Re = xn(t.stateNode.containerInfo)), te(e, t), ee(t), (Re = a));
        break;
      case 12:
        (te(e, t), ee(t));
        break;
      case 31:
        (te(e, t),
          ee(t),
          a & 4 && ((a = t.updateQueue), a !== null && ((t.updateQueue = null), On(t, a))));
        break;
      case 13:
        (te(e, t),
          ee(t),
          t.child.flags & 8192 &&
            (t.memoizedState !== null) != (l !== null && l.memoizedState !== null) &&
            (An = ne()),
          a & 4 && ((a = t.updateQueue), a !== null && ((t.updateQueue = null), On(t, a))));
        break;
      case 22:
        u = t.memoizedState !== null;
        var h = l !== null && l.memoizedState !== null,
          S = Fe,
          E = Ht;
        if (((Fe = S || u), (Ht = E || h), te(e, t), (Ht = E), (Fe = S), ee(t), a & 8192))
          t: for (
            e = t.stateNode,
              e._visibility = u ? e._visibility & -2 : e._visibility | 1,
              u && (l === null || h || Fe || Ht || Kl(t)),
              l = null,
              e = t;
            ;
          ) {
            if (e.tag === 5 || e.tag === 26) {
              if (l === null) {
                h = l = e;
                try {
                  if (((n = h.stateNode), u))
                    ((c = n.style),
                      typeof c.setProperty == "function"
                        ? c.setProperty("display", "none", "important")
                        : (c.display = "none"));
                  else {
                    s = h.stateNode;
                    var _ = h.memoizedProps.style,
                      p = _ != null && _.hasOwnProperty("display") ? _.display : null;
                    s.style.display = p == null || typeof p == "boolean" ? "" : ("" + p).trim();
                  }
                } catch (x) {
                  ft(h, h.return, x);
                }
              }
            } else if (e.tag === 6) {
              if (l === null) {
                h = e;
                try {
                  h.stateNode.nodeValue = u ? "" : h.memoizedProps;
                } catch (x) {
                  ft(h, h.return, x);
                }
              }
            } else if (e.tag === 18) {
              if (l === null) {
                h = e;
                try {
                  var b = h.stateNode;
                  u ? vh(b, !0) : vh(h.stateNode, !1);
                } catch (x) {
                  ft(h, h.return, x);
                }
              }
            } else if (
              ((e.tag !== 22 && e.tag !== 23) || e.memoizedState === null || e === t) &&
              e.child !== null
            ) {
              ((e.child.return = e), (e = e.child));
              continue;
            }
            if (e === t) break t;
            for (; e.sibling === null; ) {
              if (e.return === null || e.return === t) break t;
              (l === e && (l = null), (e = e.return));
            }
            (l === e && (l = null), (e.sibling.return = e.return), (e = e.sibling));
          }
        a & 4 &&
          ((a = t.updateQueue),
          a !== null && ((l = a.retryQueue), l !== null && ((a.retryQueue = null), On(t, l))));
        break;
      case 19:
        (te(e, t),
          ee(t),
          a & 4 && ((a = t.updateQueue), a !== null && ((t.updateQueue = null), On(t, a))));
        break;
      case 30:
        break;
      case 21:
        break;
      default:
        (te(e, t), ee(t));
    }
  }
  function ee(t) {
    var e = t.flags;
    if (e & 2) {
      try {
        for (var l, a = t.return; a !== null; ) {
          if (go(a)) {
            l = a;
            break;
          }
          a = a.return;
        }
        if (l == null) throw Error(r(160));
        switch (l.tag) {
          case 27:
            var u = l.stateNode,
              n = Cc(t);
            Tn(t, n, u);
            break;
          case 5:
            var c = l.stateNode;
            l.flags & 32 && (ta(c, ""), (l.flags &= -33));
            var s = Cc(t);
            Tn(t, s, c);
            break;
          case 3:
          case 4:
            var h = l.stateNode.containerInfo,
              S = Cc(t);
            Nc(t, S, h);
            break;
          default:
            throw Error(r(161));
        }
      } catch (E) {
        ft(t, t.return, E);
      }
      t.flags &= -3;
    }
    e & 4096 && (t.flags &= -4097);
  }
  function Mo(t) {
    if (t.subtreeFlags & 1024)
      for (t = t.child; t !== null; ) {
        var e = t;
        (Mo(e), e.tag === 5 && e.flags & 1024 && e.stateNode.reset(), (t = t.sibling));
      }
  }
  function $e(t, e) {
    if (e.subtreeFlags & 8772)
      for (e = e.child; e !== null; ) (bo(t, e.alternate, e), (e = e.sibling));
  }
  function Kl(t) {
    for (t = t.child; t !== null; ) {
      var e = t;
      switch (e.tag) {
        case 0:
        case 11:
        case 14:
        case 15:
          (dl(4, e, e.return), Kl(e));
          break;
        case 1:
          je(e, e.return);
          var l = e.stateNode;
          (typeof l.componentWillUnmount == "function" && vo(e, e.return, l), Kl(e));
          break;
        case 27:
          pu(e.stateNode);
        case 26:
        case 5:
          (je(e, e.return), Kl(e));
          break;
        case 22:
          e.memoizedState === null && Kl(e);
          break;
        case 30:
          Kl(e);
          break;
        default:
          Kl(e);
      }
      t = t.sibling;
    }
  }
  function ke(t, e, l) {
    for (l = l && (e.subtreeFlags & 8772) !== 0, e = e.child; e !== null; ) {
      var a = e.alternate,
        u = t,
        n = e,
        c = n.flags;
      switch (n.tag) {
        case 0:
        case 11:
        case 15:
          (ke(u, n, l), su(4, n));
          break;
        case 1:
          if ((ke(u, n, l), (a = n), (u = a.stateNode), typeof u.componentDidMount == "function"))
            try {
              u.componentDidMount();
            } catch (S) {
              ft(a, a.return, S);
            }
          if (((a = n), (u = a.updateQueue), u !== null)) {
            var s = a.stateNode;
            try {
              var h = u.shared.hiddenCallbacks;
              if (h !== null)
                for (u.shared.hiddenCallbacks = null, u = 0; u < h.length; u++) lr(h[u], s);
            } catch (S) {
              ft(a, a.return, S);
            }
          }
          (l && c & 64 && yo(n), ru(n, n.return));
          break;
        case 27:
          So(n);
        case 26:
        case 5:
          (ke(u, n, l), l && a === null && c & 4 && mo(n), ru(n, n.return));
          break;
        case 12:
          ke(u, n, l);
          break;
        case 31:
          (ke(u, n, l), l && c & 4 && Oo(u, n));
          break;
        case 13:
          (ke(u, n, l), l && c & 4 && zo(u, n));
          break;
        case 22:
          (n.memoizedState === null && ke(u, n, l), ru(n, n.return));
          break;
        case 30:
          break;
        default:
          ke(u, n, l);
      }
      e = e.sibling;
    }
  }
  function jc(t, e) {
    var l = null;
    (t !== null &&
      t.memoizedState !== null &&
      t.memoizedState.cachePool !== null &&
      (l = t.memoizedState.cachePool.pool),
      (t = null),
      e.memoizedState !== null &&
        e.memoizedState.cachePool !== null &&
        (t = e.memoizedState.cachePool.pool),
      t !== l && (t != null && t.refCount++, l != null && $a(l)));
  }
  function qc(t, e) {
    ((t = null),
      e.alternate !== null && (t = e.alternate.memoizedState.cache),
      (e = e.memoizedState.cache),
      e !== t && (e.refCount++, t != null && $a(t)));
  }
  function Ce(t, e, l, a) {
    if (e.subtreeFlags & 10256) for (e = e.child; e !== null; ) (_o(t, e, l, a), (e = e.sibling));
  }
  function _o(t, e, l, a) {
    var u = e.flags;
    switch (e.tag) {
      case 0:
      case 11:
      case 15:
        (Ce(t, e, l, a), u & 2048 && su(9, e));
        break;
      case 1:
        Ce(t, e, l, a);
        break;
      case 3:
        (Ce(t, e, l, a),
          u & 2048 &&
            ((t = null),
            e.alternate !== null && (t = e.alternate.memoizedState.cache),
            (e = e.memoizedState.cache),
            e !== t && (e.refCount++, t != null && $a(t))));
        break;
      case 12:
        if (u & 2048) {
          (Ce(t, e, l, a), (t = e.stateNode));
          try {
            var n = e.memoizedProps,
              c = n.id,
              s = n.onPostCommit;
            typeof s == "function" &&
              s(c, e.alternate === null ? "mount" : "update", t.passiveEffectDuration, -0);
          } catch (h) {
            ft(e, e.return, h);
          }
        } else Ce(t, e, l, a);
        break;
      case 31:
        Ce(t, e, l, a);
        break;
      case 13:
        Ce(t, e, l, a);
        break;
      case 23:
        break;
      case 22:
        ((n = e.stateNode),
          (c = e.alternate),
          e.memoizedState !== null
            ? n._visibility & 2
              ? Ce(t, e, l, a)
              : ou(t, e)
            : n._visibility & 2
              ? Ce(t, e, l, a)
              : ((n._visibility |= 2), ba(t, e, l, a, (e.subtreeFlags & 10256) !== 0 || !1)),
          u & 2048 && jc(c, e));
        break;
      case 24:
        (Ce(t, e, l, a), u & 2048 && qc(e.alternate, e));
        break;
      default:
        Ce(t, e, l, a);
    }
  }
  function ba(t, e, l, a, u) {
    for (u = u && ((e.subtreeFlags & 10256) !== 0 || !1), e = e.child; e !== null; ) {
      var n = t,
        c = e,
        s = l,
        h = a,
        S = c.flags;
      switch (c.tag) {
        case 0:
        case 11:
        case 15:
          (ba(n, c, s, h, u), su(8, c));
          break;
        case 23:
          break;
        case 22:
          var E = c.stateNode;
          (c.memoizedState !== null
            ? E._visibility & 2
              ? ba(n, c, s, h, u)
              : ou(n, c)
            : ((E._visibility |= 2), ba(n, c, s, h, u)),
            u && S & 2048 && jc(c.alternate, c));
          break;
        case 24:
          (ba(n, c, s, h, u), u && S & 2048 && qc(c.alternate, c));
          break;
        default:
          ba(n, c, s, h, u);
      }
      e = e.sibling;
    }
  }
  function ou(t, e) {
    if (e.subtreeFlags & 10256)
      for (e = e.child; e !== null; ) {
        var l = t,
          a = e,
          u = a.flags;
        switch (a.tag) {
          case 22:
            (ou(l, a), u & 2048 && jc(a.alternate, a));
            break;
          case 24:
            (ou(l, a), u & 2048 && qc(a.alternate, a));
            break;
          default:
            ou(l, a);
        }
        e = e.sibling;
      }
  }
  var hu = 8192;
  function Ea(t, e, l) {
    if (t.subtreeFlags & hu) for (t = t.child; t !== null; ) (Do(t, e, l), (t = t.sibling));
  }
  function Do(t, e, l) {
    switch (t.tag) {
      case 26:
        (Ea(t, e, l),
          t.flags & hu && t.memoizedState !== null && Cv(l, Re, t.memoizedState, t.memoizedProps));
        break;
      case 5:
        Ea(t, e, l);
        break;
      case 3:
      case 4:
        var a = Re;
        ((Re = xn(t.stateNode.containerInfo)), Ea(t, e, l), (Re = a));
        break;
      case 22:
        t.memoizedState === null &&
          ((a = t.alternate),
          a !== null && a.memoizedState !== null
            ? ((a = hu), (hu = 16777216), Ea(t, e, l), (hu = a))
            : Ea(t, e, l));
        break;
      default:
        Ea(t, e, l);
    }
  }
  function Uo(t) {
    var e = t.alternate;
    if (e !== null && ((t = e.child), t !== null)) {
      e.child = null;
      do ((e = t.sibling), (t.sibling = null), (t = e));
      while (t !== null);
    }
  }
  function du(t) {
    var e = t.deletions;
    if ((t.flags & 16) !== 0) {
      if (e !== null)
        for (var l = 0; l < e.length; l++) {
          var a = e[l];
          ((Gt = a), Co(a, t));
        }
      Uo(t);
    }
    if (t.subtreeFlags & 10256) for (t = t.child; t !== null; ) (Ro(t), (t = t.sibling));
  }
  function Ro(t) {
    switch (t.tag) {
      case 0:
      case 11:
      case 15:
        (du(t), t.flags & 2048 && dl(9, t, t.return));
        break;
      case 3:
        du(t);
        break;
      case 12:
        du(t);
        break;
      case 22:
        var e = t.stateNode;
        t.memoizedState !== null && e._visibility & 2 && (t.return === null || t.return.tag !== 13)
          ? ((e._visibility &= -3), zn(t))
          : du(t);
        break;
      default:
        du(t);
    }
  }
  function zn(t) {
    var e = t.deletions;
    if ((t.flags & 16) !== 0) {
      if (e !== null)
        for (var l = 0; l < e.length; l++) {
          var a = e[l];
          ((Gt = a), Co(a, t));
        }
      Uo(t);
    }
    for (t = t.child; t !== null; ) {
      switch (((e = t), e.tag)) {
        case 0:
        case 11:
        case 15:
          (dl(8, e, e.return), zn(e));
          break;
        case 22:
          ((l = e.stateNode), l._visibility & 2 && ((l._visibility &= -3), zn(e)));
          break;
        default:
          zn(e);
      }
      t = t.sibling;
    }
  }
  function Co(t, e) {
    for (; Gt !== null; ) {
      var l = Gt;
      switch (l.tag) {
        case 0:
        case 11:
        case 15:
          dl(8, l, e);
          break;
        case 23:
        case 22:
          if (l.memoizedState !== null && l.memoizedState.cachePool !== null) {
            var a = l.memoizedState.cachePool.pool;
            a != null && a.refCount++;
          }
          break;
        case 24:
          $a(l.memoizedState.cache);
      }
      if (((a = l.child), a !== null)) ((a.return = l), (Gt = a));
      else
        t: for (l = t; Gt !== null; ) {
          a = Gt;
          var u = a.sibling,
            n = a.return;
          if ((Eo(a), a === l)) {
            Gt = null;
            break t;
          }
          if (u !== null) {
            ((u.return = n), (Gt = u));
            break t;
          }
          Gt = n;
        }
    }
  }
  var Jy = {
      getCacheForType: function (t) {
        var e = Zt(Rt),
          l = e.data.get(t);
        return (l === void 0 && ((l = t()), e.data.set(t, l)), l);
      },
      cacheSignal: function () {
        return Zt(Rt).controller.signal;
      },
    },
    wy = typeof WeakMap == "function" ? WeakMap : Map,
    nt = 0,
    mt = null,
    k = null,
    P = 0,
    ct = 0,
    he = null,
    yl = !1,
    Ta = !1,
    Qc = !1,
    Ie = 0,
    zt = 0,
    vl = 0,
    Vl = 0,
    xc = 0,
    de = 0,
    Oa = 0,
    yu = null,
    le = null,
    Bc = !1,
    An = 0,
    No = 0,
    Mn = 1 / 0,
    _n = null,
    ml = null,
    qt = 0,
    gl = null,
    za = null,
    Pe = 0,
    Yc = 0,
    Gc = null,
    Ho = null,
    vu = 0,
    Xc = null;
  function ye() {
    return (nt & 2) !== 0 && P !== 0 ? P & -P : O.T !== null ? wc() : Ff();
  }
  function jo() {
    if (de === 0)
      if ((P & 536870912) === 0 || et) {
        var t = ju;
        ((ju <<= 1), (ju & 3932160) === 0 && (ju = 262144), (de = t));
      } else de = 536870912;
    return ((t = re.current), t !== null && (t.flags |= 32), de);
  }
  function ae(t, e, l) {
    (((t === mt && (ct === 2 || ct === 9)) || t.cancelPendingCommit !== null) &&
      (Aa(t, 0), Sl(t, P, de, !1)),
      Qa(t, l),
      ((nt & 2) === 0 || t !== mt) &&
        (t === mt && ((nt & 2) === 0 && (Vl |= l), zt === 4 && Sl(t, P, de, !1)), qe(t)));
  }
  function qo(t, e, l) {
    if ((nt & 6) !== 0) throw Error(r(327));
    var a = (!l && (e & 127) === 0 && (e & t.expiredLanes) === 0) || qa(t, e),
      u = a ? $y(t, e) : Zc(t, e, !0),
      n = a;
    do {
      if (u === 0) {
        Ta && !a && Sl(t, e, 0, !1);
        break;
      } else {
        if (((l = t.current.alternate), n && !Fy(l))) {
          ((u = Zc(t, e, !1)), (n = !1));
          continue;
        }
        if (u === 2) {
          if (((n = e), t.errorRecoveryDisabledLanes & n)) var c = 0;
          else
            ((c = t.pendingLanes & -536870913), (c = c !== 0 ? c : c & 536870912 ? 536870912 : 0));
          if (c !== 0) {
            e = c;
            t: {
              var s = t;
              u = yu;
              var h = s.current.memoizedState.isDehydrated;
              if ((h && (Aa(s, c).flags |= 256), (c = Zc(s, c, !1)), c !== 2)) {
                if (Qc && !h) {
                  ((s.errorRecoveryDisabledLanes |= n), (Vl |= n), (u = 4));
                  break t;
                }
                ((n = le), (le = u), n !== null && (le === null ? (le = n) : le.push.apply(le, n)));
              }
              u = c;
            }
            if (((n = !1), u !== 2)) continue;
          }
        }
        if (u === 1) {
          (Aa(t, 0), Sl(t, e, 0, !0));
          break;
        }
        t: {
          switch (((a = t), (n = u), n)) {
            case 0:
            case 1:
              throw Error(r(345));
            case 4:
              if ((e & 4194048) !== e) break;
            case 6:
              Sl(a, e, de, !yl);
              break t;
            case 2:
              le = null;
              break;
            case 3:
            case 5:
              break;
            default:
              throw Error(r(329));
          }
          if ((e & 62914560) === e && ((u = An + 300 - ne()), 10 < u)) {
            if ((Sl(a, e, de, !yl), Qu(a, 0, !0) !== 0)) break t;
            ((Pe = e),
              (a.timeoutHandle = hh(
                Qo.bind(null, a, l, le, _n, Bc, e, de, Vl, Oa, yl, n, "Throttled", -0, 0),
                u,
              )));
            break t;
          }
          Qo(a, l, le, _n, Bc, e, de, Vl, Oa, yl, n, null, -0, 0);
        }
      }
      break;
    } while (!0);
    qe(t);
  }
  function Qo(t, e, l, a, u, n, c, s, h, S, E, _, p, b) {
    if (((t.timeoutHandle = -1), (_ = e.subtreeFlags), _ & 8192 || (_ & 16785408) === 16785408)) {
      ((_ = {
        stylesheets: null,
        count: 0,
        imgCount: 0,
        imgBytes: 0,
        suspenseyImages: [],
        waitingForImages: !0,
        waitingForViewTransition: !1,
        unsuspend: Be,
      }),
        Do(e, n, _));
      var x = (n & 62914560) === n ? An - ne() : (n & 4194048) === n ? No - ne() : 0;
      if (((x = Nv(_, x)), x !== null)) {
        ((Pe = n),
          (t.cancelPendingCommit = x(Ko.bind(null, t, e, n, l, a, u, c, s, h, E, _, null, p, b))),
          Sl(t, n, c, !S));
        return;
      }
    }
    Ko(t, e, n, l, a, u, c, s, h);
  }
  function Fy(t) {
    for (var e = t; ; ) {
      var l = e.tag;
      if (
        (l === 0 || l === 11 || l === 15) &&
        e.flags & 16384 &&
        ((l = e.updateQueue), l !== null && ((l = l.stores), l !== null))
      )
        for (var a = 0; a < l.length; a++) {
          var u = l[a],
            n = u.getSnapshot;
          u = u.value;
          try {
            if (!fe(n(), u)) return !1;
          } catch {
            return !1;
          }
        }
      if (((l = e.child), e.subtreeFlags & 16384 && l !== null)) ((l.return = e), (e = l));
      else {
        if (e === t) break;
        for (; e.sibling === null; ) {
          if (e.return === null || e.return === t) return !0;
          e = e.return;
        }
        ((e.sibling.return = e.return), (e = e.sibling));
      }
    }
    return !0;
  }
  function Sl(t, e, l, a) {
    ((e &= ~xc),
      (e &= ~Vl),
      (t.suspendedLanes |= e),
      (t.pingedLanes &= ~e),
      a && (t.warmLanes |= e),
      (a = t.expirationTimes));
    for (var u = e; 0 < u; ) {
      var n = 31 - ce(u),
        c = 1 << n;
      ((a[n] = -1), (u &= ~c));
    }
    l !== 0 && Vf(t, l, e);
  }
  function Dn() {
    return (nt & 6) === 0 ? (mu(0), !1) : !0;
  }
  function Lc() {
    if (k !== null) {
      if (ct === 0) var t = k.return;
      else ((t = k), (Le = Ql = null), uc(t), (va = null), (Ia = 0), (t = k));
      for (; t !== null; ) (ho(t.alternate, t), (t = t.return));
      k = null;
    }
  }
  function Aa(t, e) {
    var l = t.timeoutHandle;
    (l !== -1 && ((t.timeoutHandle = -1), yv(l)),
      (l = t.cancelPendingCommit),
      l !== null && ((t.cancelPendingCommit = null), l()),
      (Pe = 0),
      Lc(),
      (mt = t),
      (k = l = Ge(t.current, null)),
      (P = e),
      (ct = 0),
      (he = null),
      (yl = !1),
      (Ta = qa(t, e)),
      (Qc = !1),
      (Oa = de = xc = Vl = vl = zt = 0),
      (le = yu = null),
      (Bc = !1),
      (e & 8) !== 0 && (e |= e & 32));
    var a = t.entangledLanes;
    if (a !== 0)
      for (t = t.entanglements, a &= e; 0 < a; ) {
        var u = 31 - ce(a),
          n = 1 << u;
        ((e |= t[u]), (a &= ~n));
      }
    return ((Ie = e), Fu(), l);
  }
  function xo(t, e) {
    ((w = null),
      (O.H = iu),
      e === ya || e === ln
        ? ((e = Is()), (ct = 3))
        : e === Ji
          ? ((e = Is()), (ct = 4))
          : (ct =
              e === bc
                ? 8
                : e !== null && typeof e == "object" && typeof e.then == "function"
                  ? 6
                  : 1),
      (he = e),
      k === null && ((zt = 1), gn(t, be(e, t.current))));
  }
  function Bo() {
    var t = re.current;
    return t === null
      ? !0
      : (P & 4194048) === P
        ? ze === null
        : (P & 62914560) === P || (P & 536870912) !== 0
          ? t === ze
          : !1;
  }
  function Yo() {
    var t = O.H;
    return ((O.H = iu), t === null ? iu : t);
  }
  function Go() {
    var t = O.A;
    return ((O.A = Jy), t);
  }
  function Un() {
    ((zt = 4),
      yl || ((P & 4194048) !== P && re.current !== null) || (Ta = !0),
      ((vl & 134217727) === 0 && (Vl & 134217727) === 0) || mt === null || Sl(mt, P, de, !1));
  }
  function Zc(t, e, l) {
    var a = nt;
    nt |= 2;
    var u = Yo(),
      n = Go();
    ((mt !== t || P !== e) && ((_n = null), Aa(t, e)), (e = !1));
    var c = zt;
    t: do
      try {
        if (ct !== 0 && k !== null) {
          var s = k,
            h = he;
          switch (ct) {
            case 8:
              (Lc(), (c = 6));
              break t;
            case 3:
            case 2:
            case 9:
            case 6:
              re.current === null && (e = !0);
              var S = ct;
              if (((ct = 0), (he = null), Ma(t, s, h, S), l && Ta)) {
                c = 0;
                break t;
              }
              break;
            default:
              ((S = ct), (ct = 0), (he = null), Ma(t, s, h, S));
          }
        }
        (Wy(), (c = zt));
        break;
      } catch (E) {
        xo(t, E);
      }
    while (!0);
    return (
      e && t.shellSuspendCounter++,
      (Le = Ql = null),
      (nt = a),
      (O.H = u),
      (O.A = n),
      k === null && ((mt = null), (P = 0), Fu()),
      c
    );
  }
  function Wy() {
    for (; k !== null; ) Xo(k);
  }
  function $y(t, e) {
    var l = nt;
    nt |= 2;
    var a = Yo(),
      u = Go();
    mt !== t || P !== e ? ((_n = null), (Mn = ne() + 500), Aa(t, e)) : (Ta = qa(t, e));
    t: do
      try {
        if (ct !== 0 && k !== null) {
          e = k;
          var n = he;
          e: switch (ct) {
            case 1:
              ((ct = 0), (he = null), Ma(t, e, n, 1));
              break;
            case 2:
            case 9:
              if ($s(n)) {
                ((ct = 0), (he = null), Lo(e));
                break;
              }
              ((e = function () {
                ((ct !== 2 && ct !== 9) || mt !== t || (ct = 7), qe(t));
              }),
                n.then(e, e));
              break t;
            case 3:
              ct = 7;
              break t;
            case 4:
              ct = 5;
              break t;
            case 7:
              $s(n) ? ((ct = 0), (he = null), Lo(e)) : ((ct = 0), (he = null), Ma(t, e, n, 7));
              break;
            case 5:
              var c = null;
              switch (k.tag) {
                case 26:
                  c = k.memoizedState;
                case 5:
                case 27:
                  var s = k;
                  if (c ? _h(c) : s.stateNode.complete) {
                    ((ct = 0), (he = null));
                    var h = s.sibling;
                    if (h !== null) k = h;
                    else {
                      var S = s.return;
                      S !== null ? ((k = S), Rn(S)) : (k = null);
                    }
                    break e;
                  }
              }
              ((ct = 0), (he = null), Ma(t, e, n, 5));
              break;
            case 6:
              ((ct = 0), (he = null), Ma(t, e, n, 6));
              break;
            case 8:
              (Lc(), (zt = 6));
              break t;
            default:
              throw Error(r(462));
          }
        }
        ky();
        break;
      } catch (E) {
        xo(t, E);
      }
    while (!0);
    return (
      (Le = Ql = null),
      (O.H = a),
      (O.A = u),
      (nt = l),
      k !== null ? 0 : ((mt = null), (P = 0), Fu(), zt)
    );
  }
  function ky() {
    for (; k !== null && !bd(); ) Xo(k);
  }
  function Xo(t) {
    var e = ro(t.alternate, t, Ie);
    ((t.memoizedProps = t.pendingProps), e === null ? Rn(t) : (k = e));
  }
  function Lo(t) {
    var e = t,
      l = e.alternate;
    switch (e.tag) {
      case 15:
      case 0:
        e = uo(l, e, e.pendingProps, e.type, void 0, P);
        break;
      case 11:
        e = uo(l, e, e.pendingProps, e.type.render, e.ref, P);
        break;
      case 5:
        uc(e);
      default:
        (ho(l, e), (e = k = Ys(e, Ie)), (e = ro(l, e, Ie)));
    }
    ((t.memoizedProps = t.pendingProps), e === null ? Rn(t) : (k = e));
  }
  function Ma(t, e, l, a) {
    ((Le = Ql = null), uc(e), (va = null), (Ia = 0));
    var u = e.return;
    try {
      if (Yy(t, u, e, l, P)) {
        ((zt = 1), gn(t, be(l, t.current)), (k = null));
        return;
      }
    } catch (n) {
      if (u !== null) throw ((k = u), n);
      ((zt = 1), gn(t, be(l, t.current)), (k = null));
      return;
    }
    e.flags & 32768
      ? (et || a === 1
          ? (t = !0)
          : Ta || (P & 536870912) !== 0
            ? (t = !1)
            : ((yl = t = !0),
              (a === 2 || a === 9 || a === 3 || a === 6) &&
                ((a = re.current), a !== null && a.tag === 13 && (a.flags |= 16384))),
        Zo(e, t))
      : Rn(e);
  }
  function Rn(t) {
    var e = t;
    do {
      if ((e.flags & 32768) !== 0) {
        Zo(e, yl);
        return;
      }
      t = e.return;
      var l = Ly(e.alternate, e, Ie);
      if (l !== null) {
        k = l;
        return;
      }
      if (((e = e.sibling), e !== null)) {
        k = e;
        return;
      }
      k = e = t;
    } while (e !== null);
    zt === 0 && (zt = 5);
  }
  function Zo(t, e) {
    do {
      var l = Zy(t.alternate, t);
      if (l !== null) {
        ((l.flags &= 32767), (k = l));
        return;
      }
      if (
        ((l = t.return),
        l !== null && ((l.flags |= 32768), (l.subtreeFlags = 0), (l.deletions = null)),
        !e && ((t = t.sibling), t !== null))
      ) {
        k = t;
        return;
      }
      k = t = l;
    } while (t !== null);
    ((zt = 6), (k = null));
  }
  function Ko(t, e, l, a, u, n, c, s, h) {
    t.cancelPendingCommit = null;
    do Cn();
    while (qt !== 0);
    if ((nt & 6) !== 0) throw Error(r(327));
    if (e !== null) {
      if (e === t.current) throw Error(r(177));
      if (
        ((n = e.lanes | e.childLanes),
        (n |= Ci),
        Rd(t, l, n, c, s, h),
        t === mt && ((k = mt = null), (P = 0)),
        (za = e),
        (gl = t),
        (Pe = l),
        (Yc = n),
        (Gc = u),
        (Ho = a),
        (e.subtreeFlags & 10256) !== 0 || (e.flags & 10256) !== 0
          ? ((t.callbackNode = null),
            (t.callbackPriority = 0),
            ev(Nu, function () {
              return (Wo(), null);
            }))
          : ((t.callbackNode = null), (t.callbackPriority = 0)),
        (a = (e.flags & 13878) !== 0),
        (e.subtreeFlags & 13878) !== 0 || a)
      ) {
        ((a = O.T), (O.T = null), (u = H.p), (H.p = 2), (c = nt), (nt |= 4));
        try {
          Ky(t, e, l);
        } finally {
          ((nt = c), (H.p = u), (O.T = a));
        }
      }
      ((qt = 1), Vo(), Jo(), wo());
    }
  }
  function Vo() {
    if (qt === 1) {
      qt = 0;
      var t = gl,
        e = za,
        l = (e.flags & 13878) !== 0;
      if ((e.subtreeFlags & 13878) !== 0 || l) {
        ((l = O.T), (O.T = null));
        var a = H.p;
        H.p = 2;
        var u = nt;
        nt |= 4;
        try {
          Ao(e, t);
          var n = ef,
            c = Rs(t.containerInfo),
            s = n.focusedElem,
            h = n.selectionRange;
          if (c !== s && s && s.ownerDocument && Us(s.ownerDocument.documentElement, s)) {
            if (h !== null && Mi(s)) {
              var S = h.start,
                E = h.end;
              if ((E === void 0 && (E = S), "selectionStart" in s))
                ((s.selectionStart = S), (s.selectionEnd = Math.min(E, s.value.length)));
              else {
                var _ = s.ownerDocument || document,
                  p = (_ && _.defaultView) || window;
                if (p.getSelection) {
                  var b = p.getSelection(),
                    x = s.textContent.length,
                    X = Math.min(h.start, x),
                    dt = h.end === void 0 ? X : Math.min(h.end, x);
                  !b.extend && X > dt && ((c = dt), (dt = X), (X = c));
                  var v = Ds(s, X),
                    d = Ds(s, dt);
                  if (
                    v &&
                    d &&
                    (b.rangeCount !== 1 ||
                      b.anchorNode !== v.node ||
                      b.anchorOffset !== v.offset ||
                      b.focusNode !== d.node ||
                      b.focusOffset !== d.offset)
                  ) {
                    var m = _.createRange();
                    (m.setStart(v.node, v.offset),
                      b.removeAllRanges(),
                      X > dt
                        ? (b.addRange(m), b.extend(d.node, d.offset))
                        : (m.setEnd(d.node, d.offset), b.addRange(m)));
                  }
                }
              }
            }
            for (_ = [], b = s; (b = b.parentNode); )
              b.nodeType === 1 && _.push({ element: b, left: b.scrollLeft, top: b.scrollTop });
            for (typeof s.focus == "function" && s.focus(), s = 0; s < _.length; s++) {
              var A = _[s];
              ((A.element.scrollLeft = A.left), (A.element.scrollTop = A.top));
            }
          }
          ((Zn = !!tf), (ef = tf = null));
        } finally {
          ((nt = u), (H.p = a), (O.T = l));
        }
      }
      ((t.current = e), (qt = 2));
    }
  }
  function Jo() {
    if (qt === 2) {
      qt = 0;
      var t = gl,
        e = za,
        l = (e.flags & 8772) !== 0;
      if ((e.subtreeFlags & 8772) !== 0 || l) {
        ((l = O.T), (O.T = null));
        var a = H.p;
        H.p = 2;
        var u = nt;
        nt |= 4;
        try {
          bo(t, e.alternate, e);
        } finally {
          ((nt = u), (H.p = a), (O.T = l));
        }
      }
      qt = 3;
    }
  }
  function wo() {
    if (qt === 4 || qt === 3) {
      ((qt = 0), Ed());
      var t = gl,
        e = za,
        l = Pe,
        a = Ho;
      (e.subtreeFlags & 10256) !== 0 || (e.flags & 10256) !== 0
        ? (qt = 5)
        : ((qt = 0), (za = gl = null), Fo(t, t.pendingLanes));
      var u = t.pendingLanes;
      if (
        (u === 0 && (ml = null),
        ci(l),
        (e = e.stateNode),
        ie && typeof ie.onCommitFiberRoot == "function")
      )
        try {
          ie.onCommitFiberRoot(ja, e, void 0, (e.current.flags & 128) === 128);
        } catch {}
      if (a !== null) {
        ((e = O.T), (u = H.p), (H.p = 2), (O.T = null));
        try {
          for (var n = t.onRecoverableError, c = 0; c < a.length; c++) {
            var s = a[c];
            n(s.value, { componentStack: s.stack });
          }
        } finally {
          ((O.T = e), (H.p = u));
        }
      }
      ((Pe & 3) !== 0 && Cn(),
        qe(t),
        (u = t.pendingLanes),
        (l & 261930) !== 0 && (u & 42) !== 0 ? (t === Xc ? vu++ : ((vu = 0), (Xc = t))) : (vu = 0),
        mu(0));
    }
  }
  function Fo(t, e) {
    (t.pooledCacheLanes &= e) === 0 &&
      ((e = t.pooledCache), e != null && ((t.pooledCache = null), $a(e)));
  }
  function Cn() {
    return (Vo(), Jo(), wo(), Wo());
  }
  function Wo() {
    if (qt !== 5) return !1;
    var t = gl,
      e = Yc;
    Yc = 0;
    var l = ci(Pe),
      a = O.T,
      u = H.p;
    try {
      ((H.p = 32 > l ? 32 : l), (O.T = null), (l = Gc), (Gc = null));
      var n = gl,
        c = Pe;
      if (((qt = 0), (za = gl = null), (Pe = 0), (nt & 6) !== 0)) throw Error(r(331));
      var s = nt;
      if (
        ((nt |= 4),
        Ro(n.current),
        _o(n, n.current, c, l),
        (nt = s),
        mu(0, !1),
        ie && typeof ie.onPostCommitFiberRoot == "function")
      )
        try {
          ie.onPostCommitFiberRoot(ja, n);
        } catch {}
      return !0;
    } finally {
      ((H.p = u), (O.T = a), Fo(t, e));
    }
  }
  function $o(t, e, l) {
    ((e = be(l, e)),
      (e = pc(t.stateNode, e, 2)),
      (t = rl(t, e, 2)),
      t !== null && (Qa(t, 2), qe(t)));
  }
  function ft(t, e, l) {
    if (t.tag === 3) $o(t, t, l);
    else
      for (; e !== null; ) {
        if (e.tag === 3) {
          $o(e, t, l);
          break;
        } else if (e.tag === 1) {
          var a = e.stateNode;
          if (
            typeof e.type.getDerivedStateFromError == "function" ||
            (typeof a.componentDidCatch == "function" && (ml === null || !ml.has(a)))
          ) {
            ((t = be(l, t)),
              (l = $r(2)),
              (a = rl(e, l, 2)),
              a !== null && (kr(l, a, e, t), Qa(a, 2), qe(a)));
            break;
          }
        }
        e = e.return;
      }
  }
  function Kc(t, e, l) {
    var a = t.pingCache;
    if (a === null) {
      a = t.pingCache = new wy();
      var u = new Set();
      a.set(e, u);
    } else ((u = a.get(e)), u === void 0 && ((u = new Set()), a.set(e, u)));
    u.has(l) || ((Qc = !0), u.add(l), (t = Iy.bind(null, t, e, l)), e.then(t, t));
  }
  function Iy(t, e, l) {
    var a = t.pingCache;
    (a !== null && a.delete(e),
      (t.pingedLanes |= t.suspendedLanes & l),
      (t.warmLanes &= ~l),
      mt === t &&
        (P & l) === l &&
        (zt === 4 || (zt === 3 && (P & 62914560) === P && 300 > ne() - An)
          ? (nt & 2) === 0 && Aa(t, 0)
          : (xc |= l),
        Oa === P && (Oa = 0)),
      qe(t));
  }
  function ko(t, e) {
    (e === 0 && (e = Kf()), (t = Hl(t, e)), t !== null && (Qa(t, e), qe(t)));
  }
  function Py(t) {
    var e = t.memoizedState,
      l = 0;
    (e !== null && (l = e.retryLane), ko(t, l));
  }
  function tv(t, e) {
    var l = 0;
    switch (t.tag) {
      case 31:
      case 13:
        var a = t.stateNode,
          u = t.memoizedState;
        u !== null && (l = u.retryLane);
        break;
      case 19:
        a = t.stateNode;
        break;
      case 22:
        a = t.stateNode._retryCache;
        break;
      default:
        throw Error(r(314));
    }
    (a !== null && a.delete(e), ko(t, l));
  }
  function ev(t, e) {
    return ai(t, e);
  }
  var Nn = null,
    _a = null,
    Vc = !1,
    Hn = !1,
    Jc = !1,
    pl = 0;
  function qe(t) {
    (t !== _a && t.next === null && (_a === null ? (Nn = _a = t) : (_a = _a.next = t)),
      (Hn = !0),
      Vc || ((Vc = !0), av()));
  }
  function mu(t, e) {
    if (!Jc && Hn) {
      Jc = !0;
      do
        for (var l = !1, a = Nn; a !== null; ) {
          if (t !== 0) {
            var u = a.pendingLanes;
            if (u === 0) var n = 0;
            else {
              var c = a.suspendedLanes,
                s = a.pingedLanes;
              ((n = (1 << (31 - ce(42 | t) + 1)) - 1),
                (n &= u & ~(c & ~s)),
                (n = n & 201326741 ? (n & 201326741) | 1 : n ? n | 2 : 0));
            }
            n !== 0 && ((l = !0), eh(a, n));
          } else
            ((n = P),
              (n = Qu(
                a,
                a === mt ? n : 0,
                a.cancelPendingCommit !== null || a.timeoutHandle !== -1,
              )),
              (n & 3) === 0 || qa(a, n) || ((l = !0), eh(a, n)));
          a = a.next;
        }
      while (l);
      Jc = !1;
    }
  }
  function lv() {
    Io();
  }
  function Io() {
    Hn = Vc = !1;
    var t = 0;
    pl !== 0 && dv() && (t = pl);
    for (var e = ne(), l = null, a = Nn; a !== null; ) {
      var u = a.next,
        n = Po(a, e);
      (n === 0
        ? ((a.next = null), l === null ? (Nn = u) : (l.next = u), u === null && (_a = l))
        : ((l = a), (t !== 0 || (n & 3) !== 0) && (Hn = !0)),
        (a = u));
    }
    ((qt !== 0 && qt !== 5) || mu(t), pl !== 0 && (pl = 0));
  }
  function Po(t, e) {
    for (
      var l = t.suspendedLanes,
        a = t.pingedLanes,
        u = t.expirationTimes,
        n = t.pendingLanes & -62914561;
      0 < n;
    ) {
      var c = 31 - ce(n),
        s = 1 << c,
        h = u[c];
      (h === -1
        ? ((s & l) === 0 || (s & a) !== 0) && (u[c] = Ud(s, e))
        : h <= e && (t.expiredLanes |= s),
        (n &= ~s));
    }
    if (
      ((e = mt),
      (l = P),
      (l = Qu(t, t === e ? l : 0, t.cancelPendingCommit !== null || t.timeoutHandle !== -1)),
      (a = t.callbackNode),
      l === 0 || (t === e && (ct === 2 || ct === 9)) || t.cancelPendingCommit !== null)
    )
      return (a !== null && a !== null && ui(a), (t.callbackNode = null), (t.callbackPriority = 0));
    if ((l & 3) === 0 || qa(t, l)) {
      if (((e = l & -l), e === t.callbackPriority)) return e;
      switch ((a !== null && ui(a), ci(l))) {
        case 2:
        case 8:
          l = Lf;
          break;
        case 32:
          l = Nu;
          break;
        case 268435456:
          l = Zf;
          break;
        default:
          l = Nu;
      }
      return (
        (a = th.bind(null, t)), (l = ai(l, a)), (t.callbackPriority = e), (t.callbackNode = l), e
      );
    }
    return (
      a !== null && a !== null && ui(a), (t.callbackPriority = 2), (t.callbackNode = null), 2
    );
  }
  function th(t, e) {
    if (qt !== 0 && qt !== 5) return ((t.callbackNode = null), (t.callbackPriority = 0), null);
    var l = t.callbackNode;
    if (Cn() && t.callbackNode !== l) return null;
    var a = P;
    return (
      (a = Qu(t, t === mt ? a : 0, t.cancelPendingCommit !== null || t.timeoutHandle !== -1)),
      a === 0
        ? null
        : (qo(t, a, e),
          Po(t, ne()),
          t.callbackNode != null && t.callbackNode === l ? th.bind(null, t) : null)
    );
  }
  function eh(t, e) {
    if (Cn()) return null;
    qo(t, e, !0);
  }
  function av() {
    vv(function () {
      (nt & 6) !== 0 ? ai(Xf, lv) : Io();
    });
  }
  function wc() {
    if (pl === 0) {
      var t = ha;
      (t === 0 && ((t = Hu), (Hu <<= 1), (Hu & 261888) === 0 && (Hu = 256)), (pl = t));
    }
    return pl;
  }
  function lh(t) {
    return t == null || typeof t == "symbol" || typeof t == "boolean"
      ? null
      : typeof t == "function"
        ? t
        : Gu("" + t);
  }
  function ah(t, e) {
    var l = e.ownerDocument.createElement("input");
    return (
      (l.name = e.name),
      (l.value = e.value),
      t.id && l.setAttribute("form", t.id),
      e.parentNode.insertBefore(l, e),
      (t = new FormData(t)),
      l.parentNode.removeChild(l),
      t
    );
  }
  function uv(t, e, l, a, u) {
    if (e === "submit" && l && l.stateNode === u) {
      var n = lh((u[kt] || null).action),
        c = a.submitter;
      c &&
        ((e = (e = c[kt] || null) ? lh(e.formAction) : c.getAttribute("formAction")),
        e !== null && ((n = e), (c = null)));
      var s = new Ku("action", "action", null, a, u);
      t.push({
        event: s,
        listeners: [
          {
            instance: null,
            listener: function () {
              if (a.defaultPrevented) {
                if (pl !== 0) {
                  var h = c ? ah(u, c) : new FormData(u);
                  dc(l, { pending: !0, data: h, method: u.method, action: n }, null, h);
                }
              } else
                typeof n == "function" &&
                  (s.preventDefault(),
                  (h = c ? ah(u, c) : new FormData(u)),
                  dc(l, { pending: !0, data: h, method: u.method, action: n }, n, h));
            },
            currentTarget: u,
          },
        ],
      });
    }
  }
  for (var Fc = 0; Fc < Ri.length; Fc++) {
    var Wc = Ri[Fc],
      nv = Wc.toLowerCase(),
      iv = Wc[0].toUpperCase() + Wc.slice(1);
    Ue(nv, "on" + iv);
  }
  (Ue(Hs, "onAnimationEnd"),
    Ue(js, "onAnimationIteration"),
    Ue(qs, "onAnimationStart"),
    Ue("dblclick", "onDoubleClick"),
    Ue("focusin", "onFocus"),
    Ue("focusout", "onBlur"),
    Ue(Ty, "onTransitionRun"),
    Ue(Oy, "onTransitionStart"),
    Ue(zy, "onTransitionCancel"),
    Ue(Qs, "onTransitionEnd"),
    Il("onMouseEnter", ["mouseout", "mouseover"]),
    Il("onMouseLeave", ["mouseout", "mouseover"]),
    Il("onPointerEnter", ["pointerout", "pointerover"]),
    Il("onPointerLeave", ["pointerout", "pointerover"]),
    Ul("onChange", "change click focusin focusout input keydown keyup selectionchange".split(" ")),
    Ul(
      "onSelect",
      "focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange".split(
        " ",
      ),
    ),
    Ul("onBeforeInput", ["compositionend", "keypress", "textInput", "paste"]),
    Ul("onCompositionEnd", "compositionend focusout keydown keypress keyup mousedown".split(" ")),
    Ul(
      "onCompositionStart",
      "compositionstart focusout keydown keypress keyup mousedown".split(" "),
    ),
    Ul(
      "onCompositionUpdate",
      "compositionupdate focusout keydown keypress keyup mousedown".split(" "),
    ));
  var gu =
      "abort canplay canplaythrough durationchange emptied encrypted ended error loadeddata loadedmetadata loadstart pause play playing progress ratechange resize seeked seeking stalled suspend timeupdate volumechange waiting".split(
        " ",
      ),
    cv = new Set(
      "beforetoggle cancel close invalid load scroll scrollend toggle".split(" ").concat(gu),
    );
  function uh(t, e) {
    e = (e & 4) !== 0;
    for (var l = 0; l < t.length; l++) {
      var a = t[l],
        u = a.event;
      a = a.listeners;
      t: {
        var n = void 0;
        if (e)
          for (var c = a.length - 1; 0 <= c; c--) {
            var s = a[c],
              h = s.instance,
              S = s.currentTarget;
            if (((s = s.listener), h !== n && u.isPropagationStopped())) break t;
            ((n = s), (u.currentTarget = S));
            try {
              n(u);
            } catch (E) {
              wu(E);
            }
            ((u.currentTarget = null), (n = h));
          }
        else
          for (c = 0; c < a.length; c++) {
            if (
              ((s = a[c]),
              (h = s.instance),
              (S = s.currentTarget),
              (s = s.listener),
              h !== n && u.isPropagationStopped())
            )
              break t;
            ((n = s), (u.currentTarget = S));
            try {
              n(u);
            } catch (E) {
              wu(E);
            }
            ((u.currentTarget = null), (n = h));
          }
      }
    }
  }
  function I(t, e) {
    var l = e[fi];
    l === void 0 && (l = e[fi] = new Set());
    var a = t + "__bubble";
    l.has(a) || (nh(e, t, 2, !1), l.add(a));
  }
  function $c(t, e, l) {
    var a = 0;
    (e && (a |= 4), nh(l, t, a, e));
  }
  var jn = "_reactListening" + Math.random().toString(36).slice(2);
  function kc(t) {
    if (!t[jn]) {
      ((t[jn] = !0),
        kf.forEach(function (l) {
          l !== "selectionchange" && (cv.has(l) || $c(l, !1, t), $c(l, !0, t));
        }));
      var e = t.nodeType === 9 ? t : t.ownerDocument;
      e === null || e[jn] || ((e[jn] = !0), $c("selectionchange", !1, e));
    }
  }
  function nh(t, e, l, a) {
    switch (jh(e)) {
      case 2:
        var u = qv;
        break;
      case 8:
        u = Qv;
        break;
      default:
        u = df;
    }
    ((l = u.bind(null, e, l, t)),
      (u = void 0),
      !gi || (e !== "touchstart" && e !== "touchmove" && e !== "wheel") || (u = !0),
      a
        ? u !== void 0
          ? t.addEventListener(e, l, { capture: !0, passive: u })
          : t.addEventListener(e, l, !0)
        : u !== void 0
          ? t.addEventListener(e, l, { passive: u })
          : t.addEventListener(e, l, !1));
  }
  function Ic(t, e, l, a, u) {
    var n = a;
    if ((e & 1) === 0 && (e & 2) === 0 && a !== null)
      t: for (;;) {
        if (a === null) return;
        var c = a.tag;
        if (c === 3 || c === 4) {
          var s = a.stateNode.containerInfo;
          if (s === u) break;
          if (c === 4)
            for (c = a.return; c !== null; ) {
              var h = c.tag;
              if ((h === 3 || h === 4) && c.stateNode.containerInfo === u) return;
              c = c.return;
            }
          for (; s !== null; ) {
            if (((c = Wl(s)), c === null)) return;
            if (((h = c.tag), h === 5 || h === 6 || h === 26 || h === 27)) {
              a = n = c;
              continue t;
            }
            s = s.parentNode;
          }
        }
        a = a.return;
      }
    ss(function () {
      var S = n,
        E = vi(l),
        _ = [];
      t: {
        var p = xs.get(t);
        if (p !== void 0) {
          var b = Ku,
            x = t;
          switch (t) {
            case "keypress":
              if (Lu(l) === 0) break t;
            case "keydown":
            case "keyup":
              b = ty;
              break;
            case "focusin":
              ((x = "focus"), (b = Ei));
              break;
            case "focusout":
              ((x = "blur"), (b = Ei));
              break;
            case "beforeblur":
            case "afterblur":
              b = Ei;
              break;
            case "click":
              if (l.button === 2) break t;
            case "auxclick":
            case "dblclick":
            case "mousedown":
            case "mousemove":
            case "mouseup":
            case "mouseout":
            case "mouseover":
            case "contextmenu":
              b = hs;
              break;
            case "drag":
            case "dragend":
            case "dragenter":
            case "dragexit":
            case "dragleave":
            case "dragover":
            case "dragstart":
            case "drop":
              b = Ld;
              break;
            case "touchcancel":
            case "touchend":
            case "touchmove":
            case "touchstart":
              b = ay;
              break;
            case Hs:
            case js:
            case qs:
              b = Vd;
              break;
            case Qs:
              b = ny;
              break;
            case "scroll":
            case "scrollend":
              b = Gd;
              break;
            case "wheel":
              b = cy;
              break;
            case "copy":
            case "cut":
            case "paste":
              b = wd;
              break;
            case "gotpointercapture":
            case "lostpointercapture":
            case "pointercancel":
            case "pointerdown":
            case "pointermove":
            case "pointerout":
            case "pointerover":
            case "pointerup":
              b = ys;
              break;
            case "toggle":
            case "beforetoggle":
              b = sy;
          }
          var X = (e & 4) !== 0,
            dt = !X && (t === "scroll" || t === "scrollend"),
            v = X ? (p !== null ? p + "Capture" : null) : p;
          X = [];
          for (var d = S, m; d !== null; ) {
            var A = d;
            if (
              ((m = A.stateNode),
              (A = A.tag),
              (A !== 5 && A !== 26 && A !== 27) ||
                m === null ||
                v === null ||
                ((A = Ya(d, v)), A != null && X.push(Su(d, A, m))),
              dt)
            )
              break;
            d = d.return;
          }
          0 < X.length && ((p = new b(p, x, null, l, E)), _.push({ event: p, listeners: X }));
        }
      }
      if ((e & 7) === 0) {
        t: {
          if (
            ((p = t === "mouseover" || t === "pointerover"),
            (b = t === "mouseout" || t === "pointerout"),
            p && l !== yi && (x = l.relatedTarget || l.fromElement) && (Wl(x) || x[Fl]))
          )
            break t;
          if (
            (b || p) &&
            ((p =
              E.window === E
                ? E
                : (p = E.ownerDocument)
                  ? p.defaultView || p.parentWindow
                  : window),
            b
              ? ((x = l.relatedTarget || l.toElement),
                (b = S),
                (x = x ? Wl(x) : null),
                x !== null &&
                  ((dt = M(x)), (X = x.tag), x !== dt || (X !== 5 && X !== 27 && X !== 6)) &&
                  (x = null))
              : ((b = null), (x = S)),
            b !== x)
          ) {
            if (
              ((X = hs),
              (A = "onMouseLeave"),
              (v = "onMouseEnter"),
              (d = "mouse"),
              (t === "pointerout" || t === "pointerover") &&
                ((X = ys), (A = "onPointerLeave"), (v = "onPointerEnter"), (d = "pointer")),
              (dt = b == null ? p : Ba(b)),
              (m = x == null ? p : Ba(x)),
              (p = new X(A, d + "leave", b, l, E)),
              (p.target = dt),
              (p.relatedTarget = m),
              (A = null),
              Wl(E) === S &&
                ((X = new X(v, d + "enter", x, l, E)),
                (X.target = m),
                (X.relatedTarget = dt),
                (A = X)),
              (dt = A),
              b && x)
            )
              e: {
                for (X = fv, v = b, d = x, m = 0, A = v; A; A = X(A)) m++;
                A = 0;
                for (var G = d; G; G = X(G)) A++;
                for (; 0 < m - A; ) ((v = X(v)), m--);
                for (; 0 < A - m; ) ((d = X(d)), A--);
                for (; m--; ) {
                  if (v === d || (d !== null && v === d.alternate)) {
                    X = v;
                    break e;
                  }
                  ((v = X(v)), (d = X(d)));
                }
                X = null;
              }
            else X = null;
            (b !== null && ih(_, p, b, X, !1), x !== null && dt !== null && ih(_, dt, x, X, !0));
          }
        }
        t: {
          if (
            ((p = S ? Ba(S) : window),
            (b = p.nodeName && p.nodeName.toLowerCase()),
            b === "select" || (b === "input" && p.type === "file"))
          )
            var at = Ts;
          else if (bs(p))
            if (Os) at = py;
            else {
              at = gy;
              var Y = my;
            }
          else
            ((b = p.nodeName),
              !b || b.toLowerCase() !== "input" || (p.type !== "checkbox" && p.type !== "radio")
                ? S && di(S.elementType) && (at = Ts)
                : (at = Sy));
          if (at && (at = at(t, S))) {
            Es(_, at, l, E);
            break t;
          }
          (Y && Y(t, p, S),
            t === "focusout" &&
              S &&
              p.type === "number" &&
              S.memoizedProps.value != null &&
              hi(p, "number", p.value));
        }
        switch (((Y = S ? Ba(S) : window), t)) {
          case "focusin":
            (bs(Y) || Y.contentEditable === "true") && ((ua = Y), (_i = S), (wa = null));
            break;
          case "focusout":
            wa = _i = ua = null;
            break;
          case "mousedown":
            Di = !0;
            break;
          case "contextmenu":
          case "mouseup":
          case "dragend":
            ((Di = !1), Cs(_, l, E));
            break;
          case "selectionchange":
            if (Ey) break;
          case "keydown":
          case "keyup":
            Cs(_, l, E);
        }
        var F;
        if (Oi)
          t: {
            switch (t) {
              case "compositionstart":
                var tt = "onCompositionStart";
                break t;
              case "compositionend":
                tt = "onCompositionEnd";
                break t;
              case "compositionupdate":
                tt = "onCompositionUpdate";
                break t;
            }
            tt = void 0;
          }
        else
          aa
            ? Ss(t, l) && (tt = "onCompositionEnd")
            : t === "keydown" && l.keyCode === 229 && (tt = "onCompositionStart");
        (tt &&
          (vs &&
            l.locale !== "ko" &&
            (aa || tt !== "onCompositionStart"
              ? tt === "onCompositionEnd" && aa && (F = rs())
              : ((al = E), (Si = "value" in al ? al.value : al.textContent), (aa = !0))),
          (Y = qn(S, tt)),
          0 < Y.length &&
            ((tt = new ds(tt, t, null, l, E)),
            _.push({ event: tt, listeners: Y }),
            F ? (tt.data = F) : ((F = ps(l)), F !== null && (tt.data = F)))),
          (F = oy ? hy(t, l) : dy(t, l)) &&
            ((tt = qn(S, "onBeforeInput")),
            0 < tt.length &&
              ((Y = new ds("onBeforeInput", "beforeinput", null, l, E)),
              _.push({ event: Y, listeners: tt }),
              (Y.data = F))),
          uv(_, t, S, l, E));
      }
      uh(_, e);
    });
  }
  function Su(t, e, l) {
    return { instance: t, listener: e, currentTarget: l };
  }
  function qn(t, e) {
    for (var l = e + "Capture", a = []; t !== null; ) {
      var u = t,
        n = u.stateNode;
      if (
        ((u = u.tag),
        (u !== 5 && u !== 26 && u !== 27) ||
          n === null ||
          ((u = Ya(t, l)),
          u != null && a.unshift(Su(t, u, n)),
          (u = Ya(t, e)),
          u != null && a.push(Su(t, u, n))),
        t.tag === 3)
      )
        return a;
      t = t.return;
    }
    return [];
  }
  function fv(t) {
    if (t === null) return null;
    do t = t.return;
    while (t && t.tag !== 5 && t.tag !== 27);
    return t || null;
  }
  function ih(t, e, l, a, u) {
    for (var n = e._reactName, c = []; l !== null && l !== a; ) {
      var s = l,
        h = s.alternate,
        S = s.stateNode;
      if (((s = s.tag), h !== null && h === a)) break;
      ((s !== 5 && s !== 26 && s !== 27) ||
        S === null ||
        ((h = S),
        u
          ? ((S = Ya(l, n)), S != null && c.unshift(Su(l, S, h)))
          : u || ((S = Ya(l, n)), S != null && c.push(Su(l, S, h)))),
        (l = l.return));
    }
    c.length !== 0 && t.push({ event: e, listeners: c });
  }
  var sv = /\r\n?/g,
    rv = /\u0000|\uFFFD/g;
  function ch(t) {
    return (typeof t == "string" ? t : "" + t)
      .replace(
        sv,
        `
`,
      )
      .replace(rv, "");
  }
  function fh(t, e) {
    return ((e = ch(e)), ch(t) === e);
  }
  function ht(t, e, l, a, u, n) {
    switch (l) {
      case "children":
        typeof a == "string"
          ? e === "body" || (e === "textarea" && a === "") || ta(t, a)
          : (typeof a == "number" || typeof a == "bigint") && e !== "body" && ta(t, "" + a);
        break;
      case "className":
        Bu(t, "class", a);
        break;
      case "tabIndex":
        Bu(t, "tabindex", a);
        break;
      case "dir":
      case "role":
      case "viewBox":
      case "width":
      case "height":
        Bu(t, l, a);
        break;
      case "style":
        cs(t, a, n);
        break;
      case "data":
        if (e !== "object") {
          Bu(t, "data", a);
          break;
        }
      case "src":
      case "href":
        if (a === "" && (e !== "a" || l !== "href")) {
          t.removeAttribute(l);
          break;
        }
        if (a == null || typeof a == "function" || typeof a == "symbol" || typeof a == "boolean") {
          t.removeAttribute(l);
          break;
        }
        ((a = Gu("" + a)), t.setAttribute(l, a));
        break;
      case "action":
      case "formAction":
        if (typeof a == "function") {
          t.setAttribute(
            l,
            "javascript:throw new Error('A React form was unexpectedly submitted. If you called form.submit() manually, consider using form.requestSubmit() instead. If you\\'re trying to use event.stopPropagation() in a submit event handler, consider also calling event.preventDefault().')",
          );
          break;
        } else
          typeof n == "function" &&
            (l === "formAction"
              ? (e !== "input" && ht(t, e, "name", u.name, u, null),
                ht(t, e, "formEncType", u.formEncType, u, null),
                ht(t, e, "formMethod", u.formMethod, u, null),
                ht(t, e, "formTarget", u.formTarget, u, null))
              : (ht(t, e, "encType", u.encType, u, null),
                ht(t, e, "method", u.method, u, null),
                ht(t, e, "target", u.target, u, null)));
        if (a == null || typeof a == "symbol" || typeof a == "boolean") {
          t.removeAttribute(l);
          break;
        }
        ((a = Gu("" + a)), t.setAttribute(l, a));
        break;
      case "onClick":
        a != null && (t.onclick = Be);
        break;
      case "onScroll":
        a != null && I("scroll", t);
        break;
      case "onScrollEnd":
        a != null && I("scrollend", t);
        break;
      case "dangerouslySetInnerHTML":
        if (a != null) {
          if (typeof a != "object" || !("__html" in a)) throw Error(r(61));
          if (((l = a.__html), l != null)) {
            if (u.children != null) throw Error(r(60));
            t.innerHTML = l;
          }
        }
        break;
      case "multiple":
        t.multiple = a && typeof a != "function" && typeof a != "symbol";
        break;
      case "muted":
        t.muted = a && typeof a != "function" && typeof a != "symbol";
        break;
      case "suppressContentEditableWarning":
      case "suppressHydrationWarning":
      case "defaultValue":
      case "defaultChecked":
      case "innerHTML":
      case "ref":
        break;
      case "autoFocus":
        break;
      case "xlinkHref":
        if (a == null || typeof a == "function" || typeof a == "boolean" || typeof a == "symbol") {
          t.removeAttribute("xlink:href");
          break;
        }
        ((l = Gu("" + a)), t.setAttributeNS("http://www.w3.org/1999/xlink", "xlink:href", l));
        break;
      case "contentEditable":
      case "spellCheck":
      case "draggable":
      case "value":
      case "autoReverse":
      case "externalResourcesRequired":
      case "focusable":
      case "preserveAlpha":
        a != null && typeof a != "function" && typeof a != "symbol"
          ? t.setAttribute(l, "" + a)
          : t.removeAttribute(l);
        break;
      case "inert":
      case "allowFullScreen":
      case "async":
      case "autoPlay":
      case "controls":
      case "default":
      case "defer":
      case "disabled":
      case "disablePictureInPicture":
      case "disableRemotePlayback":
      case "formNoValidate":
      case "hidden":
      case "loop":
      case "noModule":
      case "noValidate":
      case "open":
      case "playsInline":
      case "readOnly":
      case "required":
      case "reversed":
      case "scoped":
      case "seamless":
      case "itemScope":
        a && typeof a != "function" && typeof a != "symbol"
          ? t.setAttribute(l, "")
          : t.removeAttribute(l);
        break;
      case "capture":
      case "download":
        a === !0
          ? t.setAttribute(l, "")
          : a !== !1 && a != null && typeof a != "function" && typeof a != "symbol"
            ? t.setAttribute(l, a)
            : t.removeAttribute(l);
        break;
      case "cols":
      case "rows":
      case "size":
      case "span":
        a != null && typeof a != "function" && typeof a != "symbol" && !isNaN(a) && 1 <= a
          ? t.setAttribute(l, a)
          : t.removeAttribute(l);
        break;
      case "rowSpan":
      case "start":
        a == null || typeof a == "function" || typeof a == "symbol" || isNaN(a)
          ? t.removeAttribute(l)
          : t.setAttribute(l, a);
        break;
      case "popover":
        (I("beforetoggle", t), I("toggle", t), xu(t, "popover", a));
        break;
      case "xlinkActuate":
        xe(t, "http://www.w3.org/1999/xlink", "xlink:actuate", a);
        break;
      case "xlinkArcrole":
        xe(t, "http://www.w3.org/1999/xlink", "xlink:arcrole", a);
        break;
      case "xlinkRole":
        xe(t, "http://www.w3.org/1999/xlink", "xlink:role", a);
        break;
      case "xlinkShow":
        xe(t, "http://www.w3.org/1999/xlink", "xlink:show", a);
        break;
      case "xlinkTitle":
        xe(t, "http://www.w3.org/1999/xlink", "xlink:title", a);
        break;
      case "xlinkType":
        xe(t, "http://www.w3.org/1999/xlink", "xlink:type", a);
        break;
      case "xmlBase":
        xe(t, "http://www.w3.org/XML/1998/namespace", "xml:base", a);
        break;
      case "xmlLang":
        xe(t, "http://www.w3.org/XML/1998/namespace", "xml:lang", a);
        break;
      case "xmlSpace":
        xe(t, "http://www.w3.org/XML/1998/namespace", "xml:space", a);
        break;
      case "is":
        xu(t, "is", a);
        break;
      case "innerText":
      case "textContent":
        break;
      default:
        (!(2 < l.length) || (l[0] !== "o" && l[0] !== "O") || (l[1] !== "n" && l[1] !== "N")) &&
          ((l = Bd.get(l) || l), xu(t, l, a));
    }
  }
  function Pc(t, e, l, a, u, n) {
    switch (l) {
      case "style":
        cs(t, a, n);
        break;
      case "dangerouslySetInnerHTML":
        if (a != null) {
          if (typeof a != "object" || !("__html" in a)) throw Error(r(61));
          if (((l = a.__html), l != null)) {
            if (u.children != null) throw Error(r(60));
            t.innerHTML = l;
          }
        }
        break;
      case "children":
        typeof a == "string"
          ? ta(t, a)
          : (typeof a == "number" || typeof a == "bigint") && ta(t, "" + a);
        break;
      case "onScroll":
        a != null && I("scroll", t);
        break;
      case "onScrollEnd":
        a != null && I("scrollend", t);
        break;
      case "onClick":
        a != null && (t.onclick = Be);
        break;
      case "suppressContentEditableWarning":
      case "suppressHydrationWarning":
      case "innerHTML":
      case "ref":
        break;
      case "innerText":
      case "textContent":
        break;
      default:
        if (!If.hasOwnProperty(l))
          t: {
            if (
              l[0] === "o" &&
              l[1] === "n" &&
              ((u = l.endsWith("Capture")),
              (e = l.slice(2, u ? l.length - 7 : void 0)),
              (n = t[kt] || null),
              (n = n != null ? n[l] : null),
              typeof n == "function" && t.removeEventListener(e, n, u),
              typeof a == "function")
            ) {
              (typeof n != "function" &&
                n !== null &&
                (l in t ? (t[l] = null) : t.hasAttribute(l) && t.removeAttribute(l)),
                t.addEventListener(e, a, u));
              break t;
            }
            l in t ? (t[l] = a) : a === !0 ? t.setAttribute(l, "") : xu(t, l, a);
          }
    }
  }
  function Vt(t, e, l) {
    switch (e) {
      case "div":
      case "span":
      case "svg":
      case "path":
      case "a":
      case "g":
      case "p":
      case "li":
        break;
      case "img":
        (I("error", t), I("load", t));
        var a = !1,
          u = !1,
          n;
        for (n in l)
          if (l.hasOwnProperty(n)) {
            var c = l[n];
            if (c != null)
              switch (n) {
                case "src":
                  a = !0;
                  break;
                case "srcSet":
                  u = !0;
                  break;
                case "children":
                case "dangerouslySetInnerHTML":
                  throw Error(r(137, e));
                default:
                  ht(t, e, n, c, l, null);
              }
          }
        (u && ht(t, e, "srcSet", l.srcSet, l, null), a && ht(t, e, "src", l.src, l, null));
        return;
      case "input":
        I("invalid", t);
        var s = (n = c = u = null),
          h = null,
          S = null;
        for (a in l)
          if (l.hasOwnProperty(a)) {
            var E = l[a];
            if (E != null)
              switch (a) {
                case "name":
                  u = E;
                  break;
                case "type":
                  c = E;
                  break;
                case "checked":
                  h = E;
                  break;
                case "defaultChecked":
                  S = E;
                  break;
                case "value":
                  n = E;
                  break;
                case "defaultValue":
                  s = E;
                  break;
                case "children":
                case "dangerouslySetInnerHTML":
                  if (E != null) throw Error(r(137, e));
                  break;
                default:
                  ht(t, e, a, E, l, null);
              }
          }
        as(t, n, s, h, S, c, u, !1);
        return;
      case "select":
        (I("invalid", t), (a = c = n = null));
        for (u in l)
          if (l.hasOwnProperty(u) && ((s = l[u]), s != null))
            switch (u) {
              case "value":
                n = s;
                break;
              case "defaultValue":
                c = s;
                break;
              case "multiple":
                a = s;
              default:
                ht(t, e, u, s, l, null);
            }
        ((e = n),
          (l = c),
          (t.multiple = !!a),
          e != null ? Pl(t, !!a, e, !1) : l != null && Pl(t, !!a, l, !0));
        return;
      case "textarea":
        (I("invalid", t), (n = u = a = null));
        for (c in l)
          if (l.hasOwnProperty(c) && ((s = l[c]), s != null))
            switch (c) {
              case "value":
                a = s;
                break;
              case "defaultValue":
                u = s;
                break;
              case "children":
                n = s;
                break;
              case "dangerouslySetInnerHTML":
                if (s != null) throw Error(r(91));
                break;
              default:
                ht(t, e, c, s, l, null);
            }
        ns(t, a, u, n);
        return;
      case "option":
        for (h in l)
          l.hasOwnProperty(h) &&
            ((a = l[h]), a != null) &&
            (h === "selected"
              ? (t.selected = a && typeof a != "function" && typeof a != "symbol")
              : ht(t, e, h, a, l, null));
        return;
      case "dialog":
        (I("beforetoggle", t), I("toggle", t), I("cancel", t), I("close", t));
        break;
      case "iframe":
      case "object":
        I("load", t);
        break;
      case "video":
      case "audio":
        for (a = 0; a < gu.length; a++) I(gu[a], t);
        break;
      case "image":
        (I("error", t), I("load", t));
        break;
      case "details":
        I("toggle", t);
        break;
      case "embed":
      case "source":
      case "link":
        (I("error", t), I("load", t));
      case "area":
      case "base":
      case "br":
      case "col":
      case "hr":
      case "keygen":
      case "meta":
      case "param":
      case "track":
      case "wbr":
      case "menuitem":
        for (S in l)
          if (l.hasOwnProperty(S) && ((a = l[S]), a != null))
            switch (S) {
              case "children":
              case "dangerouslySetInnerHTML":
                throw Error(r(137, e));
              default:
                ht(t, e, S, a, l, null);
            }
        return;
      default:
        if (di(e)) {
          for (E in l)
            l.hasOwnProperty(E) && ((a = l[E]), a !== void 0 && Pc(t, e, E, a, l, void 0));
          return;
        }
    }
    for (s in l) l.hasOwnProperty(s) && ((a = l[s]), a != null && ht(t, e, s, a, l, null));
  }
  function ov(t, e, l, a) {
    switch (e) {
      case "div":
      case "span":
      case "svg":
      case "path":
      case "a":
      case "g":
      case "p":
      case "li":
        break;
      case "input":
        var u = null,
          n = null,
          c = null,
          s = null,
          h = null,
          S = null,
          E = null;
        for (b in l) {
          var _ = l[b];
          if (l.hasOwnProperty(b) && _ != null)
            switch (b) {
              case "checked":
                break;
              case "value":
                break;
              case "defaultValue":
                h = _;
              default:
                a.hasOwnProperty(b) || ht(t, e, b, null, a, _);
            }
        }
        for (var p in a) {
          var b = a[p];
          if (((_ = l[p]), a.hasOwnProperty(p) && (b != null || _ != null)))
            switch (p) {
              case "type":
                n = b;
                break;
              case "name":
                u = b;
                break;
              case "checked":
                S = b;
                break;
              case "defaultChecked":
                E = b;
                break;
              case "value":
                c = b;
                break;
              case "defaultValue":
                s = b;
                break;
              case "children":
              case "dangerouslySetInnerHTML":
                if (b != null) throw Error(r(137, e));
                break;
              default:
                b !== _ && ht(t, e, p, b, a, _);
            }
        }
        oi(t, c, s, h, S, E, n, u);
        return;
      case "select":
        b = c = s = p = null;
        for (n in l)
          if (((h = l[n]), l.hasOwnProperty(n) && h != null))
            switch (n) {
              case "value":
                break;
              case "multiple":
                b = h;
              default:
                a.hasOwnProperty(n) || ht(t, e, n, null, a, h);
            }
        for (u in a)
          if (((n = a[u]), (h = l[u]), a.hasOwnProperty(u) && (n != null || h != null)))
            switch (u) {
              case "value":
                p = n;
                break;
              case "defaultValue":
                s = n;
                break;
              case "multiple":
                c = n;
              default:
                n !== h && ht(t, e, u, n, a, h);
            }
        ((e = s),
          (l = c),
          (a = b),
          p != null
            ? Pl(t, !!l, p, !1)
            : !!a != !!l && (e != null ? Pl(t, !!l, e, !0) : Pl(t, !!l, l ? [] : "", !1)));
        return;
      case "textarea":
        b = p = null;
        for (s in l)
          if (((u = l[s]), l.hasOwnProperty(s) && u != null && !a.hasOwnProperty(s)))
            switch (s) {
              case "value":
                break;
              case "children":
                break;
              default:
                ht(t, e, s, null, a, u);
            }
        for (c in a)
          if (((u = a[c]), (n = l[c]), a.hasOwnProperty(c) && (u != null || n != null)))
            switch (c) {
              case "value":
                p = u;
                break;
              case "defaultValue":
                b = u;
                break;
              case "children":
                break;
              case "dangerouslySetInnerHTML":
                if (u != null) throw Error(r(91));
                break;
              default:
                u !== n && ht(t, e, c, u, a, n);
            }
        us(t, p, b);
        return;
      case "option":
        for (var x in l)
          ((p = l[x]),
            l.hasOwnProperty(x) &&
              p != null &&
              !a.hasOwnProperty(x) &&
              (x === "selected" ? (t.selected = !1) : ht(t, e, x, null, a, p)));
        for (h in a)
          ((p = a[h]),
            (b = l[h]),
            a.hasOwnProperty(h) &&
              p !== b &&
              (p != null || b != null) &&
              (h === "selected"
                ? (t.selected = p && typeof p != "function" && typeof p != "symbol")
                : ht(t, e, h, p, a, b)));
        return;
      case "img":
      case "link":
      case "area":
      case "base":
      case "br":
      case "col":
      case "embed":
      case "hr":
      case "keygen":
      case "meta":
      case "param":
      case "source":
      case "track":
      case "wbr":
      case "menuitem":
        for (var X in l)
          ((p = l[X]),
            l.hasOwnProperty(X) && p != null && !a.hasOwnProperty(X) && ht(t, e, X, null, a, p));
        for (S in a)
          if (((p = a[S]), (b = l[S]), a.hasOwnProperty(S) && p !== b && (p != null || b != null)))
            switch (S) {
              case "children":
              case "dangerouslySetInnerHTML":
                if (p != null) throw Error(r(137, e));
                break;
              default:
                ht(t, e, S, p, a, b);
            }
        return;
      default:
        if (di(e)) {
          for (var dt in l)
            ((p = l[dt]),
              l.hasOwnProperty(dt) &&
                p !== void 0 &&
                !a.hasOwnProperty(dt) &&
                Pc(t, e, dt, void 0, a, p));
          for (E in a)
            ((p = a[E]),
              (b = l[E]),
              !a.hasOwnProperty(E) ||
                p === b ||
                (p === void 0 && b === void 0) ||
                Pc(t, e, E, p, a, b));
          return;
        }
    }
    for (var v in l)
      ((p = l[v]),
        l.hasOwnProperty(v) && p != null && !a.hasOwnProperty(v) && ht(t, e, v, null, a, p));
    for (_ in a)
      ((p = a[_]),
        (b = l[_]),
        !a.hasOwnProperty(_) || p === b || (p == null && b == null) || ht(t, e, _, p, a, b));
  }
  function sh(t) {
    switch (t) {
      case "css":
      case "script":
      case "font":
      case "img":
      case "image":
      case "input":
      case "link":
        return !0;
      default:
        return !1;
    }
  }
  function hv() {
    if (typeof performance.getEntriesByType == "function") {
      for (
        var t = 0, e = 0, l = performance.getEntriesByType("resource"), a = 0;
        a < l.length;
        a++
      ) {
        var u = l[a],
          n = u.transferSize,
          c = u.initiatorType,
          s = u.duration;
        if (n && s && sh(c)) {
          for (c = 0, s = u.responseEnd, a += 1; a < l.length; a++) {
            var h = l[a],
              S = h.startTime;
            if (S > s) break;
            var E = h.transferSize,
              _ = h.initiatorType;
            E && sh(_) && ((h = h.responseEnd), (c += E * (h < s ? 1 : (s - S) / (h - S))));
          }
          if ((--a, (e += (8 * (n + c)) / (u.duration / 1e3)), t++, 10 < t)) break;
        }
      }
      if (0 < t) return e / t / 1e6;
    }
    return navigator.connection && ((t = navigator.connection.downlink), typeof t == "number")
      ? t
      : 5;
  }
  var tf = null,
    ef = null;
  function Qn(t) {
    return t.nodeType === 9 ? t : t.ownerDocument;
  }
  function rh(t) {
    switch (t) {
      case "http://www.w3.org/2000/svg":
        return 1;
      case "http://www.w3.org/1998/Math/MathML":
        return 2;
      default:
        return 0;
    }
  }
  function oh(t, e) {
    if (t === 0)
      switch (e) {
        case "svg":
          return 1;
        case "math":
          return 2;
        default:
          return 0;
      }
    return t === 1 && e === "foreignObject" ? 0 : t;
  }
  function lf(t, e) {
    return (
      t === "textarea" ||
      t === "noscript" ||
      typeof e.children == "string" ||
      typeof e.children == "number" ||
      typeof e.children == "bigint" ||
      (typeof e.dangerouslySetInnerHTML == "object" &&
        e.dangerouslySetInnerHTML !== null &&
        e.dangerouslySetInnerHTML.__html != null)
    );
  }
  var af = null;
  function dv() {
    var t = window.event;
    return t && t.type === "popstate" ? (t === af ? !1 : ((af = t), !0)) : ((af = null), !1);
  }
  var hh = typeof setTimeout == "function" ? setTimeout : void 0,
    yv = typeof clearTimeout == "function" ? clearTimeout : void 0,
    dh = typeof Promise == "function" ? Promise : void 0,
    vv =
      typeof queueMicrotask == "function"
        ? queueMicrotask
        : typeof dh < "u"
          ? function (t) {
              return dh.resolve(null).then(t).catch(mv);
            }
          : hh;
  function mv(t) {
    setTimeout(function () {
      throw t;
    });
  }
  function bl(t) {
    return t === "head";
  }
  function yh(t, e) {
    var l = e,
      a = 0;
    do {
      var u = l.nextSibling;
      if ((t.removeChild(l), u && u.nodeType === 8))
        if (((l = u.data), l === "/$" || l === "/&")) {
          if (a === 0) {
            (t.removeChild(u), Ca(e));
            return;
          }
          a--;
        } else if (l === "$" || l === "$?" || l === "$~" || l === "$!" || l === "&") a++;
        else if (l === "html") pu(t.ownerDocument.documentElement);
        else if (l === "head") {
          ((l = t.ownerDocument.head), pu(l));
          for (var n = l.firstChild; n; ) {
            var c = n.nextSibling,
              s = n.nodeName;
            (n[xa] ||
              s === "SCRIPT" ||
              s === "STYLE" ||
              (s === "LINK" && n.rel.toLowerCase() === "stylesheet") ||
              l.removeChild(n),
              (n = c));
          }
        } else l === "body" && pu(t.ownerDocument.body);
      l = u;
    } while (l);
    Ca(e);
  }
  function vh(t, e) {
    var l = t;
    t = 0;
    do {
      var a = l.nextSibling;
      if (
        (l.nodeType === 1
          ? e
            ? ((l._stashedDisplay = l.style.display), (l.style.display = "none"))
            : ((l.style.display = l._stashedDisplay || ""),
              l.getAttribute("style") === "" && l.removeAttribute("style"))
          : l.nodeType === 3 &&
            (e
              ? ((l._stashedText = l.nodeValue), (l.nodeValue = ""))
              : (l.nodeValue = l._stashedText || "")),
        a && a.nodeType === 8)
      )
        if (((l = a.data), l === "/$")) {
          if (t === 0) break;
          t--;
        } else (l !== "$" && l !== "$?" && l !== "$~" && l !== "$!") || t++;
      l = a;
    } while (l);
  }
  function uf(t) {
    var e = t.firstChild;
    for (e && e.nodeType === 10 && (e = e.nextSibling); e; ) {
      var l = e;
      switch (((e = e.nextSibling), l.nodeName)) {
        case "HTML":
        case "HEAD":
        case "BODY":
          (uf(l), si(l));
          continue;
        case "SCRIPT":
        case "STYLE":
          continue;
        case "LINK":
          if (l.rel.toLowerCase() === "stylesheet") continue;
      }
      t.removeChild(l);
    }
  }
  function gv(t, e, l, a) {
    for (; t.nodeType === 1; ) {
      var u = l;
      if (t.nodeName.toLowerCase() !== e.toLowerCase()) {
        if (!a && (t.nodeName !== "INPUT" || t.type !== "hidden")) break;
      } else if (a) {
        if (!t[xa])
          switch (e) {
            case "meta":
              if (!t.hasAttribute("itemprop")) break;
              return t;
            case "link":
              if (
                ((n = t.getAttribute("rel")),
                n === "stylesheet" && t.hasAttribute("data-precedence"))
              )
                break;
              if (
                n !== u.rel ||
                t.getAttribute("href") !== (u.href == null || u.href === "" ? null : u.href) ||
                t.getAttribute("crossorigin") !== (u.crossOrigin == null ? null : u.crossOrigin) ||
                t.getAttribute("title") !== (u.title == null ? null : u.title)
              )
                break;
              return t;
            case "style":
              if (t.hasAttribute("data-precedence")) break;
              return t;
            case "script":
              if (
                ((n = t.getAttribute("src")),
                (n !== (u.src == null ? null : u.src) ||
                  t.getAttribute("type") !== (u.type == null ? null : u.type) ||
                  t.getAttribute("crossorigin") !==
                    (u.crossOrigin == null ? null : u.crossOrigin)) &&
                  n &&
                  t.hasAttribute("async") &&
                  !t.hasAttribute("itemprop"))
              )
                break;
              return t;
            default:
              return t;
          }
      } else if (e === "input" && t.type === "hidden") {
        var n = u.name == null ? null : "" + u.name;
        if (u.type === "hidden" && t.getAttribute("name") === n) return t;
      } else return t;
      if (((t = Ae(t.nextSibling)), t === null)) break;
    }
    return null;
  }
  function Sv(t, e, l) {
    if (e === "") return null;
    for (; t.nodeType !== 3; )
      if (
        ((t.nodeType !== 1 || t.nodeName !== "INPUT" || t.type !== "hidden") && !l) ||
        ((t = Ae(t.nextSibling)), t === null)
      )
        return null;
    return t;
  }
  function mh(t, e) {
    for (; t.nodeType !== 8; )
      if (
        ((t.nodeType !== 1 || t.nodeName !== "INPUT" || t.type !== "hidden") && !e) ||
        ((t = Ae(t.nextSibling)), t === null)
      )
        return null;
    return t;
  }
  function nf(t) {
    return t.data === "$?" || t.data === "$~";
  }
  function cf(t) {
    return t.data === "$!" || (t.data === "$?" && t.ownerDocument.readyState !== "loading");
  }
  function pv(t, e) {
    var l = t.ownerDocument;
    if (t.data === "$~") t._reactRetry = e;
    else if (t.data !== "$?" || l.readyState !== "loading") e();
    else {
      var a = function () {
        (e(), l.removeEventListener("DOMContentLoaded", a));
      };
      (l.addEventListener("DOMContentLoaded", a), (t._reactRetry = a));
    }
  }
  function Ae(t) {
    for (; t != null; t = t.nextSibling) {
      var e = t.nodeType;
      if (e === 1 || e === 3) break;
      if (e === 8) {
        if (
          ((e = t.data),
          e === "$" ||
            e === "$!" ||
            e === "$?" ||
            e === "$~" ||
            e === "&" ||
            e === "F!" ||
            e === "F")
        )
          break;
        if (e === "/$" || e === "/&") return null;
      }
    }
    return t;
  }
  var ff = null;
  function gh(t) {
    t = t.nextSibling;
    for (var e = 0; t; ) {
      if (t.nodeType === 8) {
        var l = t.data;
        if (l === "/$" || l === "/&") {
          if (e === 0) return Ae(t.nextSibling);
          e--;
        } else (l !== "$" && l !== "$!" && l !== "$?" && l !== "$~" && l !== "&") || e++;
      }
      t = t.nextSibling;
    }
    return null;
  }
  function Sh(t) {
    t = t.previousSibling;
    for (var e = 0; t; ) {
      if (t.nodeType === 8) {
        var l = t.data;
        if (l === "$" || l === "$!" || l === "$?" || l === "$~" || l === "&") {
          if (e === 0) return t;
          e--;
        } else (l !== "/$" && l !== "/&") || e++;
      }
      t = t.previousSibling;
    }
    return null;
  }
  function ph(t, e, l) {
    switch (((e = Qn(l)), t)) {
      case "html":
        if (((t = e.documentElement), !t)) throw Error(r(452));
        return t;
      case "head":
        if (((t = e.head), !t)) throw Error(r(453));
        return t;
      case "body":
        if (((t = e.body), !t)) throw Error(r(454));
        return t;
      default:
        throw Error(r(451));
    }
  }
  function pu(t) {
    for (var e = t.attributes; e.length; ) t.removeAttributeNode(e[0]);
    si(t);
  }
  var Me = new Map(),
    bh = new Set();
  function xn(t) {
    return typeof t.getRootNode == "function"
      ? t.getRootNode()
      : t.nodeType === 9
        ? t
        : t.ownerDocument;
  }
  var tl = H.d;
  H.d = { f: bv, r: Ev, D: Tv, C: Ov, L: zv, m: Av, X: _v, S: Mv, M: Dv };
  function bv() {
    var t = tl.f(),
      e = Dn();
    return t || e;
  }
  function Ev(t) {
    var e = $l(t);
    e !== null && e.tag === 5 && e.type === "form" ? Qr(e) : tl.r(t);
  }
  var Da = typeof document > "u" ? null : document;
  function Eh(t, e, l) {
    var a = Da;
    if (a && typeof e == "string" && e) {
      var u = Se(e);
      ((u = 'link[rel="' + t + '"][href="' + u + '"]'),
        typeof l == "string" && (u += '[crossorigin="' + l + '"]'),
        bh.has(u) ||
          (bh.add(u),
          (t = { rel: t, crossOrigin: l, href: e }),
          a.querySelector(u) === null &&
            ((e = a.createElement("link")), Vt(e, "link", t), Yt(e), a.head.appendChild(e))));
    }
  }
  function Tv(t) {
    (tl.D(t), Eh("dns-prefetch", t, null));
  }
  function Ov(t, e) {
    (tl.C(t, e), Eh("preconnect", t, e));
  }
  function zv(t, e, l) {
    tl.L(t, e, l);
    var a = Da;
    if (a && t && e) {
      var u = 'link[rel="preload"][as="' + Se(e) + '"]';
      e === "image" && l && l.imageSrcSet
        ? ((u += '[imagesrcset="' + Se(l.imageSrcSet) + '"]'),
          typeof l.imageSizes == "string" && (u += '[imagesizes="' + Se(l.imageSizes) + '"]'))
        : (u += '[href="' + Se(t) + '"]');
      var n = u;
      switch (e) {
        case "style":
          n = Ua(t);
          break;
        case "script":
          n = Ra(t);
      }
      Me.has(n) ||
        ((t = N(
          { rel: "preload", href: e === "image" && l && l.imageSrcSet ? void 0 : t, as: e },
          l,
        )),
        Me.set(n, t),
        a.querySelector(u) !== null ||
          (e === "style" && a.querySelector(bu(n))) ||
          (e === "script" && a.querySelector(Eu(n))) ||
          ((e = a.createElement("link")), Vt(e, "link", t), Yt(e), a.head.appendChild(e)));
    }
  }
  function Av(t, e) {
    tl.m(t, e);
    var l = Da;
    if (l && t) {
      var a = e && typeof e.as == "string" ? e.as : "script",
        u = 'link[rel="modulepreload"][as="' + Se(a) + '"][href="' + Se(t) + '"]',
        n = u;
      switch (a) {
        case "audioworklet":
        case "paintworklet":
        case "serviceworker":
        case "sharedworker":
        case "worker":
        case "script":
          n = Ra(t);
      }
      if (
        !Me.has(n) &&
        ((t = N({ rel: "modulepreload", href: t }, e)), Me.set(n, t), l.querySelector(u) === null)
      ) {
        switch (a) {
          case "audioworklet":
          case "paintworklet":
          case "serviceworker":
          case "sharedworker":
          case "worker":
          case "script":
            if (l.querySelector(Eu(n))) return;
        }
        ((a = l.createElement("link")), Vt(a, "link", t), Yt(a), l.head.appendChild(a));
      }
    }
  }
  function Mv(t, e, l) {
    tl.S(t, e, l);
    var a = Da;
    if (a && t) {
      var u = kl(a).hoistableStyles,
        n = Ua(t);
      e = e || "default";
      var c = u.get(n);
      if (!c) {
        var s = { loading: 0, preload: null };
        if ((c = a.querySelector(bu(n)))) s.loading = 5;
        else {
          ((t = N({ rel: "stylesheet", href: t, "data-precedence": e }, l)),
            (l = Me.get(n)) && sf(t, l));
          var h = (c = a.createElement("link"));
          (Yt(h),
            Vt(h, "link", t),
            (h._p = new Promise(function (S, E) {
              ((h.onload = S), (h.onerror = E));
            })),
            h.addEventListener("load", function () {
              s.loading |= 1;
            }),
            h.addEventListener("error", function () {
              s.loading |= 2;
            }),
            (s.loading |= 4),
            Bn(c, e, a));
        }
        ((c = { type: "stylesheet", instance: c, count: 1, state: s }), u.set(n, c));
      }
    }
  }
  function _v(t, e) {
    tl.X(t, e);
    var l = Da;
    if (l && t) {
      var a = kl(l).hoistableScripts,
        u = Ra(t),
        n = a.get(u);
      n ||
        ((n = l.querySelector(Eu(u))),
        n ||
          ((t = N({ src: t, async: !0 }, e)),
          (e = Me.get(u)) && rf(t, e),
          (n = l.createElement("script")),
          Yt(n),
          Vt(n, "link", t),
          l.head.appendChild(n)),
        (n = { type: "script", instance: n, count: 1, state: null }),
        a.set(u, n));
    }
  }
  function Dv(t, e) {
    tl.M(t, e);
    var l = Da;
    if (l && t) {
      var a = kl(l).hoistableScripts,
        u = Ra(t),
        n = a.get(u);
      n ||
        ((n = l.querySelector(Eu(u))),
        n ||
          ((t = N({ src: t, async: !0, type: "module" }, e)),
          (e = Me.get(u)) && rf(t, e),
          (n = l.createElement("script")),
          Yt(n),
          Vt(n, "link", t),
          l.head.appendChild(n)),
        (n = { type: "script", instance: n, count: 1, state: null }),
        a.set(u, n));
    }
  }
  function Th(t, e, l, a) {
    var u = (u = $.current) ? xn(u) : null;
    if (!u) throw Error(r(446));
    switch (t) {
      case "meta":
      case "title":
        return null;
      case "style":
        return typeof l.precedence == "string" && typeof l.href == "string"
          ? ((e = Ua(l.href)),
            (l = kl(u).hoistableStyles),
            (a = l.get(e)),
            a || ((a = { type: "style", instance: null, count: 0, state: null }), l.set(e, a)),
            a)
          : { type: "void", instance: null, count: 0, state: null };
      case "link":
        if (
          l.rel === "stylesheet" &&
          typeof l.href == "string" &&
          typeof l.precedence == "string"
        ) {
          t = Ua(l.href);
          var n = kl(u).hoistableStyles,
            c = n.get(t);
          if (
            (c ||
              ((u = u.ownerDocument || u),
              (c = {
                type: "stylesheet",
                instance: null,
                count: 0,
                state: { loading: 0, preload: null },
              }),
              n.set(t, c),
              (n = u.querySelector(bu(t))) && !n._p && ((c.instance = n), (c.state.loading = 5)),
              Me.has(t) ||
                ((l = {
                  rel: "preload",
                  as: "style",
                  href: l.href,
                  crossOrigin: l.crossOrigin,
                  integrity: l.integrity,
                  media: l.media,
                  hrefLang: l.hrefLang,
                  referrerPolicy: l.referrerPolicy,
                }),
                Me.set(t, l),
                n || Uv(u, t, l, c.state))),
            e && a === null)
          )
            throw Error(r(528, ""));
          return c;
        }
        if (e && a !== null) throw Error(r(529, ""));
        return null;
      case "script":
        return (
          (e = l.async),
          (l = l.src),
          typeof l == "string" && e && typeof e != "function" && typeof e != "symbol"
            ? ((e = Ra(l)),
              (l = kl(u).hoistableScripts),
              (a = l.get(e)),
              a || ((a = { type: "script", instance: null, count: 0, state: null }), l.set(e, a)),
              a)
            : { type: "void", instance: null, count: 0, state: null }
        );
      default:
        throw Error(r(444, t));
    }
  }
  function Ua(t) {
    return 'href="' + Se(t) + '"';
  }
  function bu(t) {
    return 'link[rel="stylesheet"][' + t + "]";
  }
  function Oh(t) {
    return N({}, t, { "data-precedence": t.precedence, precedence: null });
  }
  function Uv(t, e, l, a) {
    t.querySelector('link[rel="preload"][as="style"][' + e + "]")
      ? (a.loading = 1)
      : ((e = t.createElement("link")),
        (a.preload = e),
        e.addEventListener("load", function () {
          return (a.loading |= 1);
        }),
        e.addEventListener("error", function () {
          return (a.loading |= 2);
        }),
        Vt(e, "link", l),
        Yt(e),
        t.head.appendChild(e));
  }
  function Ra(t) {
    return '[src="' + Se(t) + '"]';
  }
  function Eu(t) {
    return "script[async]" + t;
  }
  function zh(t, e, l) {
    if ((e.count++, e.instance === null))
      switch (e.type) {
        case "style":
          var a = t.querySelector('style[data-href~="' + Se(l.href) + '"]');
          if (a) return ((e.instance = a), Yt(a), a);
          var u = N({}, l, {
            "data-href": l.href,
            "data-precedence": l.precedence,
            href: null,
            precedence: null,
          });
          return (
            (a = (t.ownerDocument || t).createElement("style")),
            Yt(a),
            Vt(a, "style", u),
            Bn(a, l.precedence, t),
            (e.instance = a)
          );
        case "stylesheet":
          u = Ua(l.href);
          var n = t.querySelector(bu(u));
          if (n) return ((e.state.loading |= 4), (e.instance = n), Yt(n), n);
          ((a = Oh(l)),
            (u = Me.get(u)) && sf(a, u),
            (n = (t.ownerDocument || t).createElement("link")),
            Yt(n));
          var c = n;
          return (
            (c._p = new Promise(function (s, h) {
              ((c.onload = s), (c.onerror = h));
            })),
            Vt(n, "link", a),
            (e.state.loading |= 4),
            Bn(n, l.precedence, t),
            (e.instance = n)
          );
        case "script":
          return (
            (n = Ra(l.src)),
            (u = t.querySelector(Eu(n)))
              ? ((e.instance = u), Yt(u), u)
              : ((a = l),
                (u = Me.get(n)) && ((a = N({}, l)), rf(a, u)),
                (t = t.ownerDocument || t),
                (u = t.createElement("script")),
                Yt(u),
                Vt(u, "link", a),
                t.head.appendChild(u),
                (e.instance = u))
          );
        case "void":
          return null;
        default:
          throw Error(r(443, e.type));
      }
    else
      e.type === "stylesheet" &&
        (e.state.loading & 4) === 0 &&
        ((a = e.instance), (e.state.loading |= 4), Bn(a, l.precedence, t));
    return e.instance;
  }
  function Bn(t, e, l) {
    for (
      var a = l.querySelectorAll('link[rel="stylesheet"][data-precedence],style[data-precedence]'),
        u = a.length ? a[a.length - 1] : null,
        n = u,
        c = 0;
      c < a.length;
      c++
    ) {
      var s = a[c];
      if (s.dataset.precedence === e) n = s;
      else if (n !== u) break;
    }
    n
      ? n.parentNode.insertBefore(t, n.nextSibling)
      : ((e = l.nodeType === 9 ? l.head : l), e.insertBefore(t, e.firstChild));
  }
  function sf(t, e) {
    (t.crossOrigin == null && (t.crossOrigin = e.crossOrigin),
      t.referrerPolicy == null && (t.referrerPolicy = e.referrerPolicy),
      t.title == null && (t.title = e.title));
  }
  function rf(t, e) {
    (t.crossOrigin == null && (t.crossOrigin = e.crossOrigin),
      t.referrerPolicy == null && (t.referrerPolicy = e.referrerPolicy),
      t.integrity == null && (t.integrity = e.integrity));
  }
  var Yn = null;
  function Ah(t, e, l) {
    if (Yn === null) {
      var a = new Map(),
        u = (Yn = new Map());
      u.set(l, a);
    } else ((u = Yn), (a = u.get(l)), a || ((a = new Map()), u.set(l, a)));
    if (a.has(t)) return a;
    for (a.set(t, null), l = l.getElementsByTagName(t), u = 0; u < l.length; u++) {
      var n = l[u];
      if (
        !(n[xa] || n[Xt] || (t === "link" && n.getAttribute("rel") === "stylesheet")) &&
        n.namespaceURI !== "http://www.w3.org/2000/svg"
      ) {
        var c = n.getAttribute(e) || "";
        c = t + c;
        var s = a.get(c);
        s ? s.push(n) : a.set(c, [n]);
      }
    }
    return a;
  }
  function Mh(t, e, l) {
    ((t = t.ownerDocument || t),
      t.head.insertBefore(l, e === "title" ? t.querySelector("head > title") : null));
  }
  function Rv(t, e, l) {
    if (l === 1 || e.itemProp != null) return !1;
    switch (t) {
      case "meta":
      case "title":
        return !0;
      case "style":
        if (typeof e.precedence != "string" || typeof e.href != "string" || e.href === "") break;
        return !0;
      case "link":
        if (
          typeof e.rel != "string" ||
          typeof e.href != "string" ||
          e.href === "" ||
          e.onLoad ||
          e.onError
        )
          break;
        return e.rel === "stylesheet"
          ? ((t = e.disabled), typeof e.precedence == "string" && t == null)
          : !0;
      case "script":
        if (
          e.async &&
          typeof e.async != "function" &&
          typeof e.async != "symbol" &&
          !e.onLoad &&
          !e.onError &&
          e.src &&
          typeof e.src == "string"
        )
          return !0;
    }
    return !1;
  }
  function _h(t) {
    return !(t.type === "stylesheet" && (t.state.loading & 3) === 0);
  }
  function Cv(t, e, l, a) {
    if (
      l.type === "stylesheet" &&
      (typeof a.media != "string" || matchMedia(a.media).matches !== !1) &&
      (l.state.loading & 4) === 0
    ) {
      if (l.instance === null) {
        var u = Ua(a.href),
          n = e.querySelector(bu(u));
        if (n) {
          ((e = n._p),
            e !== null &&
              typeof e == "object" &&
              typeof e.then == "function" &&
              (t.count++, (t = Gn.bind(t)), e.then(t, t)),
            (l.state.loading |= 4),
            (l.instance = n),
            Yt(n));
          return;
        }
        ((n = e.ownerDocument || e),
          (a = Oh(a)),
          (u = Me.get(u)) && sf(a, u),
          (n = n.createElement("link")),
          Yt(n));
        var c = n;
        ((c._p = new Promise(function (s, h) {
          ((c.onload = s), (c.onerror = h));
        })),
          Vt(n, "link", a),
          (l.instance = n));
      }
      (t.stylesheets === null && (t.stylesheets = new Map()),
        t.stylesheets.set(l, e),
        (e = l.state.preload) &&
          (l.state.loading & 3) === 0 &&
          (t.count++,
          (l = Gn.bind(t)),
          e.addEventListener("load", l),
          e.addEventListener("error", l)));
    }
  }
  var of = 0;
  function Nv(t, e) {
    return (
      t.stylesheets && t.count === 0 && Ln(t, t.stylesheets),
      0 < t.count || 0 < t.imgCount
        ? function (l) {
            var a = setTimeout(function () {
              if ((t.stylesheets && Ln(t, t.stylesheets), t.unsuspend)) {
                var n = t.unsuspend;
                ((t.unsuspend = null), n());
              }
            }, 6e4 + e);
            0 < t.imgBytes && of === 0 && (of = 62500 * hv());
            var u = setTimeout(
              function () {
                if (
                  ((t.waitingForImages = !1),
                  t.count === 0 && (t.stylesheets && Ln(t, t.stylesheets), t.unsuspend))
                ) {
                  var n = t.unsuspend;
                  ((t.unsuspend = null), n());
                }
              },
              (t.imgBytes > of ? 50 : 800) + e,
            );
            return (
              (t.unsuspend = l),
              function () {
                ((t.unsuspend = null), clearTimeout(a), clearTimeout(u));
              }
            );
          }
        : null
    );
  }
  function Gn() {
    if ((this.count--, this.count === 0 && (this.imgCount === 0 || !this.waitingForImages))) {
      if (this.stylesheets) Ln(this, this.stylesheets);
      else if (this.unsuspend) {
        var t = this.unsuspend;
        ((this.unsuspend = null), t());
      }
    }
  }
  var Xn = null;
  function Ln(t, e) {
    ((t.stylesheets = null),
      t.unsuspend !== null &&
        (t.count++, (Xn = new Map()), e.forEach(Hv, t), (Xn = null), Gn.call(t)));
  }
  function Hv(t, e) {
    if (!(e.state.loading & 4)) {
      var l = Xn.get(t);
      if (l) var a = l.get(null);
      else {
        ((l = new Map()), Xn.set(t, l));
        for (
          var u = t.querySelectorAll("link[data-precedence],style[data-precedence]"), n = 0;
          n < u.length;
          n++
        ) {
          var c = u[n];
          (c.nodeName === "LINK" || c.getAttribute("media") !== "not all") &&
            (l.set(c.dataset.precedence, c), (a = c));
        }
        a && l.set(null, a);
      }
      ((u = e.instance),
        (c = u.getAttribute("data-precedence")),
        (n = l.get(c) || a),
        n === a && l.set(null, u),
        l.set(c, u),
        this.count++,
        (a = Gn.bind(this)),
        u.addEventListener("load", a),
        u.addEventListener("error", a),
        n
          ? n.parentNode.insertBefore(u, n.nextSibling)
          : ((t = t.nodeType === 9 ? t.head : t), t.insertBefore(u, t.firstChild)),
        (e.state.loading |= 4));
    }
  }
  var Tu = {
    $$typeof: gt,
    Provider: null,
    Consumer: null,
    _currentValue: L,
    _currentValue2: L,
    _threadCount: 0,
  };
  function jv(t, e, l, a, u, n, c, s, h) {
    ((this.tag = 1),
      (this.containerInfo = t),
      (this.pingCache = this.current = this.pendingChildren = null),
      (this.timeoutHandle = -1),
      (this.callbackNode =
        this.next =
        this.pendingContext =
        this.context =
        this.cancelPendingCommit =
          null),
      (this.callbackPriority = 0),
      (this.expirationTimes = ni(-1)),
      (this.entangledLanes =
        this.shellSuspendCounter =
        this.errorRecoveryDisabledLanes =
        this.expiredLanes =
        this.warmLanes =
        this.pingedLanes =
        this.suspendedLanes =
        this.pendingLanes =
          0),
      (this.entanglements = ni(0)),
      (this.hiddenUpdates = ni(null)),
      (this.identifierPrefix = a),
      (this.onUncaughtError = u),
      (this.onCaughtError = n),
      (this.onRecoverableError = c),
      (this.pooledCache = null),
      (this.pooledCacheLanes = 0),
      (this.formState = h),
      (this.incompleteTransitions = new Map()));
  }
  function Dh(t, e, l, a, u, n, c, s, h, S, E, _) {
    return (
      (t = new jv(t, e, l, c, h, S, E, _, s)),
      (e = 1),
      n === !0 && (e |= 24),
      (n = se(3, null, null, e)),
      (t.current = n),
      (n.stateNode = t),
      (e = Zi()),
      e.refCount++,
      (t.pooledCache = e),
      e.refCount++,
      (n.memoizedState = { element: a, isDehydrated: l, cache: e }),
      wi(n),
      t
    );
  }
  function Uh(t) {
    return t ? ((t = ca), t) : ca;
  }
  function Rh(t, e, l, a, u, n) {
    ((u = Uh(u)),
      a.context === null ? (a.context = u) : (a.pendingContext = u),
      (a = sl(e)),
      (a.payload = { element: l }),
      (n = n === void 0 ? null : n),
      n !== null && (a.callback = n),
      (l = rl(t, a, e)),
      l !== null && (ae(l, t, e), tu(l, t, e)));
  }
  function Ch(t, e) {
    if (((t = t.memoizedState), t !== null && t.dehydrated !== null)) {
      var l = t.retryLane;
      t.retryLane = l !== 0 && l < e ? l : e;
    }
  }
  function hf(t, e) {
    (Ch(t, e), (t = t.alternate) && Ch(t, e));
  }
  function Nh(t) {
    if (t.tag === 13 || t.tag === 31) {
      var e = Hl(t, 67108864);
      (e !== null && ae(e, t, 67108864), hf(t, 67108864));
    }
  }
  function Hh(t) {
    if (t.tag === 13 || t.tag === 31) {
      var e = ye();
      e = ii(e);
      var l = Hl(t, e);
      (l !== null && ae(l, t, e), hf(t, e));
    }
  }
  var Zn = !0;
  function qv(t, e, l, a) {
    var u = O.T;
    O.T = null;
    var n = H.p;
    try {
      ((H.p = 2), df(t, e, l, a));
    } finally {
      ((H.p = n), (O.T = u));
    }
  }
  function Qv(t, e, l, a) {
    var u = O.T;
    O.T = null;
    var n = H.p;
    try {
      ((H.p = 8), df(t, e, l, a));
    } finally {
      ((H.p = n), (O.T = u));
    }
  }
  function df(t, e, l, a) {
    if (Zn) {
      var u = yf(a);
      if (u === null) (Ic(t, e, a, Kn, l), qh(t, a));
      else if (Bv(u, t, e, l, a)) a.stopPropagation();
      else if ((qh(t, a), e & 4 && -1 < xv.indexOf(t))) {
        for (; u !== null; ) {
          var n = $l(u);
          if (n !== null)
            switch (n.tag) {
              case 3:
                if (((n = n.stateNode), n.current.memoizedState.isDehydrated)) {
                  var c = Dl(n.pendingLanes);
                  if (c !== 0) {
                    var s = n;
                    for (s.pendingLanes |= 2, s.entangledLanes |= 2; c; ) {
                      var h = 1 << (31 - ce(c));
                      ((s.entanglements[1] |= h), (c &= ~h));
                    }
                    (qe(n), (nt & 6) === 0 && ((Mn = ne() + 500), mu(0)));
                  }
                }
                break;
              case 31:
              case 13:
                ((s = Hl(n, 2)), s !== null && ae(s, n, 2), Dn(), hf(n, 2));
            }
          if (((n = yf(a)), n === null && Ic(t, e, a, Kn, l), n === u)) break;
          u = n;
        }
        u !== null && a.stopPropagation();
      } else Ic(t, e, a, null, l);
    }
  }
  function yf(t) {
    return ((t = vi(t)), vf(t));
  }
  var Kn = null;
  function vf(t) {
    if (((Kn = null), (t = Wl(t)), t !== null)) {
      var e = M(t);
      if (e === null) t = null;
      else {
        var l = e.tag;
        if (l === 13) {
          if (((t = U(e)), t !== null)) return t;
          t = null;
        } else if (l === 31) {
          if (((t = Q(e)), t !== null)) return t;
          t = null;
        } else if (l === 3) {
          if (e.stateNode.current.memoizedState.isDehydrated)
            return e.tag === 3 ? e.stateNode.containerInfo : null;
          t = null;
        } else e !== t && (t = null);
      }
    }
    return ((Kn = t), null);
  }
  function jh(t) {
    switch (t) {
      case "beforetoggle":
      case "cancel":
      case "click":
      case "close":
      case "contextmenu":
      case "copy":
      case "cut":
      case "auxclick":
      case "dblclick":
      case "dragend":
      case "dragstart":
      case "drop":
      case "focusin":
      case "focusout":
      case "input":
      case "invalid":
      case "keydown":
      case "keypress":
      case "keyup":
      case "mousedown":
      case "mouseup":
      case "paste":
      case "pause":
      case "play":
      case "pointercancel":
      case "pointerdown":
      case "pointerup":
      case "ratechange":
      case "reset":
      case "resize":
      case "seeked":
      case "submit":
      case "toggle":
      case "touchcancel":
      case "touchend":
      case "touchstart":
      case "volumechange":
      case "change":
      case "selectionchange":
      case "textInput":
      case "compositionstart":
      case "compositionend":
      case "compositionupdate":
      case "beforeblur":
      case "afterblur":
      case "beforeinput":
      case "blur":
      case "fullscreenchange":
      case "focus":
      case "hashchange":
      case "popstate":
      case "select":
      case "selectstart":
        return 2;
      case "drag":
      case "dragenter":
      case "dragexit":
      case "dragleave":
      case "dragover":
      case "mousemove":
      case "mouseout":
      case "mouseover":
      case "pointermove":
      case "pointerout":
      case "pointerover":
      case "scroll":
      case "touchmove":
      case "wheel":
      case "mouseenter":
      case "mouseleave":
      case "pointerenter":
      case "pointerleave":
        return 8;
      case "message":
        switch (Td()) {
          case Xf:
            return 2;
          case Lf:
            return 8;
          case Nu:
          case Od:
            return 32;
          case Zf:
            return 268435456;
          default:
            return 32;
        }
      default:
        return 32;
    }
  }
  var mf = !1,
    El = null,
    Tl = null,
    Ol = null,
    Ou = new Map(),
    zu = new Map(),
    zl = [],
    xv =
      "mousedown mouseup touchcancel touchend touchstart auxclick dblclick pointercancel pointerdown pointerup dragend dragstart drop compositionend compositionstart keydown keypress keyup input textInput copy cut paste click change contextmenu reset".split(
        " ",
      );
  function qh(t, e) {
    switch (t) {
      case "focusin":
      case "focusout":
        El = null;
        break;
      case "dragenter":
      case "dragleave":
        Tl = null;
        break;
      case "mouseover":
      case "mouseout":
        Ol = null;
        break;
      case "pointerover":
      case "pointerout":
        Ou.delete(e.pointerId);
        break;
      case "gotpointercapture":
      case "lostpointercapture":
        zu.delete(e.pointerId);
    }
  }
  function Au(t, e, l, a, u, n) {
    return t === null || t.nativeEvent !== n
      ? ((t = {
          blockedOn: e,
          domEventName: l,
          eventSystemFlags: a,
          nativeEvent: n,
          targetContainers: [u],
        }),
        e !== null && ((e = $l(e)), e !== null && Nh(e)),
        t)
      : ((t.eventSystemFlags |= a),
        (e = t.targetContainers),
        u !== null && e.indexOf(u) === -1 && e.push(u),
        t);
  }
  function Bv(t, e, l, a, u) {
    switch (e) {
      case "focusin":
        return ((El = Au(El, t, e, l, a, u)), !0);
      case "dragenter":
        return ((Tl = Au(Tl, t, e, l, a, u)), !0);
      case "mouseover":
        return ((Ol = Au(Ol, t, e, l, a, u)), !0);
      case "pointerover":
        var n = u.pointerId;
        return (Ou.set(n, Au(Ou.get(n) || null, t, e, l, a, u)), !0);
      case "gotpointercapture":
        return ((n = u.pointerId), zu.set(n, Au(zu.get(n) || null, t, e, l, a, u)), !0);
    }
    return !1;
  }
  function Qh(t) {
    var e = Wl(t.target);
    if (e !== null) {
      var l = M(e);
      if (l !== null) {
        if (((e = l.tag), e === 13)) {
          if (((e = U(l)), e !== null)) {
            ((t.blockedOn = e),
              Wf(t.priority, function () {
                Hh(l);
              }));
            return;
          }
        } else if (e === 31) {
          if (((e = Q(l)), e !== null)) {
            ((t.blockedOn = e),
              Wf(t.priority, function () {
                Hh(l);
              }));
            return;
          }
        } else if (e === 3 && l.stateNode.current.memoizedState.isDehydrated) {
          t.blockedOn = l.tag === 3 ? l.stateNode.containerInfo : null;
          return;
        }
      }
    }
    t.blockedOn = null;
  }
  function Vn(t) {
    if (t.blockedOn !== null) return !1;
    for (var e = t.targetContainers; 0 < e.length; ) {
      var l = yf(t.nativeEvent);
      if (l === null) {
        l = t.nativeEvent;
        var a = new l.constructor(l.type, l);
        ((yi = a), l.target.dispatchEvent(a), (yi = null));
      } else return ((e = $l(l)), e !== null && Nh(e), (t.blockedOn = l), !1);
      e.shift();
    }
    return !0;
  }
  function xh(t, e, l) {
    Vn(t) && l.delete(e);
  }
  function Yv() {
    ((mf = !1),
      El !== null && Vn(El) && (El = null),
      Tl !== null && Vn(Tl) && (Tl = null),
      Ol !== null && Vn(Ol) && (Ol = null),
      Ou.forEach(xh),
      zu.forEach(xh));
  }
  function Jn(t, e) {
    t.blockedOn === e &&
      ((t.blockedOn = null),
      mf || ((mf = !0), i.unstable_scheduleCallback(i.unstable_NormalPriority, Yv)));
  }
  var wn = null;
  function Bh(t) {
    wn !== t &&
      ((wn = t),
      i.unstable_scheduleCallback(i.unstable_NormalPriority, function () {
        wn === t && (wn = null);
        for (var e = 0; e < t.length; e += 3) {
          var l = t[e],
            a = t[e + 1],
            u = t[e + 2];
          if (typeof a != "function") {
            if (vf(a || l) === null) continue;
            break;
          }
          var n = $l(l);
          n !== null &&
            (t.splice(e, 3),
            (e -= 3),
            dc(n, { pending: !0, data: u, method: l.method, action: a }, a, u));
        }
      }));
  }
  function Ca(t) {
    function e(h) {
      return Jn(h, t);
    }
    (El !== null && Jn(El, t),
      Tl !== null && Jn(Tl, t),
      Ol !== null && Jn(Ol, t),
      Ou.forEach(e),
      zu.forEach(e));
    for (var l = 0; l < zl.length; l++) {
      var a = zl[l];
      a.blockedOn === t && (a.blockedOn = null);
    }
    for (; 0 < zl.length && ((l = zl[0]), l.blockedOn === null); )
      (Qh(l), l.blockedOn === null && zl.shift());
    if (((l = (t.ownerDocument || t).$$reactFormReplay), l != null))
      for (a = 0; a < l.length; a += 3) {
        var u = l[a],
          n = l[a + 1],
          c = u[kt] || null;
        if (typeof n == "function") c || Bh(l);
        else if (c) {
          var s = null;
          if (n && n.hasAttribute("formAction")) {
            if (((u = n), (c = n[kt] || null))) s = c.formAction;
            else if (vf(u) !== null) continue;
          } else s = c.action;
          (typeof s == "function" ? (l[a + 1] = s) : (l.splice(a, 3), (a -= 3)), Bh(l));
        }
      }
  }
  function Yh() {
    function t(n) {
      n.canIntercept &&
        n.info === "react-transition" &&
        n.intercept({
          handler: function () {
            return new Promise(function (c) {
              return (u = c);
            });
          },
          focusReset: "manual",
          scroll: "manual",
        });
    }
    function e() {
      (u !== null && (u(), (u = null)), a || setTimeout(l, 20));
    }
    function l() {
      if (!a && !navigation.transition) {
        var n = navigation.currentEntry;
        n &&
          n.url != null &&
          navigation.navigate(n.url, {
            state: n.getState(),
            info: "react-transition",
            history: "replace",
          });
      }
    }
    if (typeof navigation == "object") {
      var a = !1,
        u = null;
      return (
        navigation.addEventListener("navigate", t),
        navigation.addEventListener("navigatesuccess", e),
        navigation.addEventListener("navigateerror", e),
        setTimeout(l, 100),
        function () {
          ((a = !0),
            navigation.removeEventListener("navigate", t),
            navigation.removeEventListener("navigatesuccess", e),
            navigation.removeEventListener("navigateerror", e),
            u !== null && (u(), (u = null)));
        }
      );
    }
  }
  function gf(t) {
    this._internalRoot = t;
  }
  ((Fn.prototype.render = gf.prototype.render =
    function (t) {
      var e = this._internalRoot;
      if (e === null) throw Error(r(409));
      var l = e.current,
        a = ye();
      Rh(l, a, t, e, null, null);
    }),
    (Fn.prototype.unmount = gf.prototype.unmount =
      function () {
        var t = this._internalRoot;
        if (t !== null) {
          this._internalRoot = null;
          var e = t.containerInfo;
          (Rh(t.current, 2, null, t, null, null), Dn(), (e[Fl] = null));
        }
      }));
  function Fn(t) {
    this._internalRoot = t;
  }
  Fn.prototype.unstable_scheduleHydration = function (t) {
    if (t) {
      var e = Ff();
      t = { blockedOn: null, target: t, priority: e };
      for (var l = 0; l < zl.length && e !== 0 && e < zl[l].priority; l++);
      (zl.splice(l, 0, t), l === 0 && Qh(t));
    }
  };
  var Gh = f.version;
  if (Gh !== "19.2.7") throw Error(r(527, Gh, "19.2.7"));
  H.findDOMNode = function (t) {
    var e = t._reactInternals;
    if (e === void 0)
      throw typeof t.render == "function"
        ? Error(r(188))
        : ((t = Object.keys(t).join(",")), Error(r(268, t)));
    return ((t = T(e)), (t = t !== null ? C(t) : null), (t = t === null ? null : t.stateNode), t);
  };
  var Gv = {
    bundleType: 0,
    version: "19.2.7",
    rendererPackageName: "react-dom",
    currentDispatcherRef: O,
    reconcilerVersion: "19.2.7",
  };
  if (typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ < "u") {
    var Wn = __REACT_DEVTOOLS_GLOBAL_HOOK__;
    if (!Wn.isDisabled && Wn.supportsFiber)
      try {
        ((ja = Wn.inject(Gv)), (ie = Wn));
      } catch {}
  }
  return (
    (_u.createRoot = function (t, e) {
      if (!g(t)) throw Error(r(299));
      var l = !1,
        a = "",
        u = Jr,
        n = wr,
        c = Fr;
      return (
        e != null &&
          (e.unstable_strictMode === !0 && (l = !0),
          e.identifierPrefix !== void 0 && (a = e.identifierPrefix),
          e.onUncaughtError !== void 0 && (u = e.onUncaughtError),
          e.onCaughtError !== void 0 && (n = e.onCaughtError),
          e.onRecoverableError !== void 0 && (c = e.onRecoverableError)),
        (e = Dh(t, 1, !1, null, null, l, a, null, u, n, c, Yh)),
        (t[Fl] = e.current),
        kc(t),
        new gf(e)
      );
    }),
    (_u.hydrateRoot = function (t, e, l) {
      if (!g(t)) throw Error(r(299));
      var a = !1,
        u = "",
        n = Jr,
        c = wr,
        s = Fr,
        h = null;
      return (
        l != null &&
          (l.unstable_strictMode === !0 && (a = !0),
          l.identifierPrefix !== void 0 && (u = l.identifierPrefix),
          l.onUncaughtError !== void 0 && (n = l.onUncaughtError),
          l.onCaughtError !== void 0 && (c = l.onCaughtError),
          l.onRecoverableError !== void 0 && (s = l.onRecoverableError),
          l.formState !== void 0 && (h = l.formState)),
        (e = Dh(t, 1, !0, e, l ?? null, a, u, h, n, c, s, Yh)),
        (e.context = Uh(null)),
        (l = e.current),
        (a = ye()),
        (a = ii(a)),
        (u = sl(a)),
        (u.callback = null),
        rl(l, u, a),
        (l = a),
        (e.current.lanes = l),
        Qa(e, l),
        qe(e),
        (t[Fl] = e.current),
        kc(t),
        new Fn(e)
      );
    }),
    (_u.version = "19.2.7"),
    _u
  );
}
var $h;
function $v() {
  if ($h) return bf.exports;
  $h = 1;
  function i() {
    if (
      !(
        typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ > "u" ||
        typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE != "function"
      )
    )
      try {
        __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(i);
      } catch (f) {
        console.error(f);
      }
  }
  return (i(), (bf.exports = Wv()), bf.exports);
}
var kv = $v(),
  Na = class {
    constructor() {
      ((this.listeners = new Set()), (this.subscribe = this.subscribe.bind(this)));
    }
    subscribe(i) {
      return (
        this.listeners.add(i),
        this.onSubscribe(),
        () => {
          (this.listeners.delete(i), this.onUnsubscribe());
        }
      );
    }
    hasListeners() {
      return this.listeners.size > 0;
    }
    onSubscribe() {}
    onUnsubscribe() {}
  },
  Iv = class extends Na {
    #t;
    #e;
    #l;
    constructor() {
      (super(),
        (this.#l = (i) => {
          if (typeof window < "u" && window.addEventListener) {
            const f = () => i();
            return (
              window.addEventListener("visibilitychange", f, !1),
              () => {
                window.removeEventListener("visibilitychange", f);
              }
            );
          }
        }));
    }
    onSubscribe() {
      this.#e || this.setEventListener(this.#l);
    }
    onUnsubscribe() {
      this.hasListeners() || (this.#e?.(), (this.#e = void 0));
    }
    setEventListener(i) {
      ((this.#l = i),
        this.#e?.(),
        (this.#e = i((f) => {
          typeof f == "boolean" ? this.setFocused(f) : this.onFocus();
        })));
    }
    setFocused(i) {
      this.#t !== i && ((this.#t = i), this.onFocus());
    }
    onFocus() {
      const i = this.isFocused();
      this.listeners.forEach((f) => {
        f(i);
      });
    }
    isFocused() {
      return typeof this.#t == "boolean"
        ? this.#t
        : globalThis.document?.visibilityState !== "hidden";
    }
  },
  Hf = new Iv(),
  Pv = {
    setTimeout: (i, f) => setTimeout(i, f),
    clearTimeout: (i) => clearTimeout(i),
    setInterval: (i, f) => setInterval(i, f),
    clearInterval: (i) => clearInterval(i),
  },
  tm = class {
    #t = Pv;
    #e = !1;
    setTimeoutProvider(i) {
      this.#t = i;
    }
    setTimeout(i, f) {
      return this.#t.setTimeout(i, f);
    }
    clearTimeout(i) {
      this.#t.clearTimeout(i);
    }
    setInterval(i, f) {
      return this.#t.setInterval(i, f);
    }
    clearInterval(i) {
      this.#t.clearInterval(i);
    }
  },
  Jl = new tm();
function em(i) {
  setTimeout(i, 0);
}
var lm = typeof window > "u" || "Deno" in globalThis;
function $t() {}
function am(i, f) {
  return typeof i == "function" ? i(f) : i;
}
function Af(i) {
  return typeof i == "number" && i >= 0 && i !== 1 / 0;
}
function sd(i, f) {
  return Math.max(i + (f || 0) - Date.now(), 0);
}
function Ml(i, f) {
  return typeof i == "function" ? i(f) : i;
}
function ve(i, f) {
  return typeof i == "function" ? i(f) : i;
}
function kh(i, f) {
  const { type: o = "all", exact: r, fetchStatus: g, predicate: M, queryKey: U, stale: Q } = i;
  if (U) {
    if (r) {
      if (f.queryHash !== jf(U, f.options)) return !1;
    } else if (!Uu(f.queryKey, U)) return !1;
  }
  if (o !== "all") {
    const z = f.isActive();
    if ((o === "active" && !z) || (o === "inactive" && z)) return !1;
  }
  return !(
    (typeof Q == "boolean" && f.isStale() !== Q) ||
    (g && g !== f.state.fetchStatus) ||
    (M && !M(f))
  );
}
function Ih(i, f) {
  const { exact: o, status: r, predicate: g, mutationKey: M } = i;
  if (M) {
    if (!f.options.mutationKey) return !1;
    if (o) {
      if (wl(f.options.mutationKey) !== wl(M)) return !1;
    } else if (!Uu(f.options.mutationKey, M)) return !1;
  }
  return !((r && f.state.status !== r) || (g && !g(f)));
}
function jf(i, f) {
  return (f?.queryKeyHashFn || wl)(i);
}
function wl(i) {
  return JSON.stringify(i, (f, o) =>
    Mf(o)
      ? Object.keys(o)
          .sort()
          .reduce((r, g) => ((r[g] = o[g]), r), {})
      : o,
  );
}
function Uu(i, f) {
  return i === f
    ? !0
    : typeof i != typeof f
      ? !1
      : i && f && typeof i == "object" && typeof f == "object"
        ? Object.keys(f).every((o) => Uu(i[o], f[o]))
        : !1;
}
var um = Object.prototype.hasOwnProperty;
function rd(i, f, o = 0) {
  if (i === f) return i;
  if (o > 500) return f;
  const r = Ph(i) && Ph(f);
  if (!r && !(Mf(i) && Mf(f))) return f;
  const M = (r ? i : Object.keys(i)).length,
    U = r ? f : Object.keys(f),
    Q = U.length,
    z = r ? new Array(Q) : {};
  let T = 0;
  for (let C = 0; C < Q; C++) {
    const N = r ? C : U[C],
      R = i[N],
      lt = f[N];
    if (R === lt) {
      ((z[N] = R), (r ? C < M : um.call(i, N)) && T++);
      continue;
    }
    if (R === null || lt === null || typeof R != "object" || typeof lt != "object") {
      z[N] = lt;
      continue;
    }
    const W = rd(R, lt, o + 1);
    ((z[N] = W), W === R && T++);
  }
  return M === Q && T === M ? i : z;
}
function kn(i, f) {
  if (!f || Object.keys(i).length !== Object.keys(f).length) return !1;
  for (const o in i) if (i[o] !== f[o]) return !1;
  return !0;
}
function Ph(i) {
  return Array.isArray(i) && i.length === Object.keys(i).length;
}
function Mf(i) {
  if (!td(i)) return !1;
  const f = i.constructor;
  if (f === void 0) return !0;
  const o = f.prototype;
  return !(
    !td(o) ||
    !o.hasOwnProperty("isPrototypeOf") ||
    Object.getPrototypeOf(i) !== Object.prototype
  );
}
function td(i) {
  return Object.prototype.toString.call(i) === "[object Object]";
}
function nm(i) {
  return new Promise((f) => {
    Jl.setTimeout(f, i);
  });
}
function _f(i, f, o) {
  return typeof o.structuralSharing == "function"
    ? o.structuralSharing(i, f)
    : o.structuralSharing !== !1
      ? rd(i, f)
      : f;
}
function im(i, f, o = 0) {
  const r = [...i, f];
  return o && r.length > o ? r.slice(1) : r;
}
function cm(i, f, o = 0) {
  const r = [f, ...i];
  return o && r.length > o ? r.slice(0, -1) : r;
}
var qf = Symbol();
function od(i, f) {
  return !i.queryFn && f?.initialPromise
    ? () => f.initialPromise
    : !i.queryFn || i.queryFn === qf
      ? () => Promise.reject(new Error(`Missing queryFn: '${i.queryHash}'`))
      : i.queryFn;
}
function Qf(i, f) {
  return typeof i == "function" ? i(...f) : !!i;
}
function fm(i, f, o) {
  let r = !1,
    g;
  return (
    Object.defineProperty(i, "signal", {
      enumerable: !0,
      get: () => (
        (g ??= f()),
        r || ((r = !0), g.aborted ? o() : g.addEventListener("abort", o, { once: !0 })),
        g
      ),
    }),
    i
  );
}
var Ru = (() => {
  let i = () => lm;
  return {
    isServer() {
      return i();
    },
    setIsServer(f) {
      i = f;
    },
  };
})();
function Df() {
  let i, f;
  const o = new Promise((g, M) => {
    ((i = g), (f = M));
  });
  ((o.status = "pending"), o.catch(() => {}));
  function r(g) {
    (Object.assign(o, g), delete o.resolve, delete o.reject);
  }
  return (
    (o.resolve = (g) => {
      (r({ status: "fulfilled", value: g }), i(g));
    }),
    (o.reject = (g) => {
      (r({ status: "rejected", reason: g }), f(g));
    }),
    o
  );
}
var sm = em;
function rm() {
  let i = [],
    f = 0,
    o = (Q) => {
      Q();
    },
    r = (Q) => {
      Q();
    },
    g = sm;
  const M = (Q) => {
      f
        ? i.push(Q)
        : g(() => {
            o(Q);
          });
    },
    U = () => {
      const Q = i;
      ((i = []),
        Q.length &&
          g(() => {
            r(() => {
              Q.forEach((z) => {
                o(z);
              });
            });
          }));
    };
  return {
    batch: (Q) => {
      let z;
      f++;
      try {
        z = Q();
      } finally {
        (f--, f || U());
      }
      return z;
    },
    batchCalls:
      (Q) =>
      (...z) => {
        M(() => {
          Q(...z);
        });
      },
    schedule: M,
    setNotifyFunction: (Q) => {
      o = Q;
    },
    setBatchNotifyFunction: (Q) => {
      r = Q;
    },
    setScheduler: (Q) => {
      g = Q;
    },
  };
}
var Qt = rm(),
  om = class extends Na {
    #t = !0;
    #e;
    #l;
    constructor() {
      (super(),
        (this.#l = (i) => {
          if (typeof window < "u" && window.addEventListener) {
            const f = () => i(!0),
              o = () => i(!1);
            return (
              window.addEventListener("online", f, !1),
              window.addEventListener("offline", o, !1),
              () => {
                (window.removeEventListener("online", f), window.removeEventListener("offline", o));
              }
            );
          }
        }));
    }
    onSubscribe() {
      this.#e || this.setEventListener(this.#l);
    }
    onUnsubscribe() {
      this.hasListeners() || (this.#e?.(), (this.#e = void 0));
    }
    setEventListener(i) {
      ((this.#l = i), this.#e?.(), (this.#e = i(this.setOnline.bind(this))));
    }
    setOnline(i) {
      this.#t !== i &&
        ((this.#t = i),
        this.listeners.forEach((o) => {
          o(i);
        }));
    }
    isOnline() {
      return this.#t;
    }
  },
  In = new om();
function hm(i) {
  return Math.min(1e3 * 2 ** i, 3e4);
}
function hd(i) {
  return (i ?? "online") === "online" ? In.isOnline() : !0;
}
var Uf = class extends Error {
  constructor(i) {
    (super("CancelledError"), (this.revert = i?.revert), (this.silent = i?.silent));
  }
};
function dd(i) {
  let f = !1,
    o = 0,
    r;
  const g = Df(),
    M = () => g.status !== "pending",
    U = (Z) => {
      if (!M()) {
        const St = new Uf(Z);
        (R(St), i.onCancel?.(St));
      }
    },
    Q = () => {
      f = !0;
    },
    z = () => {
      f = !1;
    },
    T = () => Hf.isFocused() && (i.networkMode === "always" || In.isOnline()) && i.canRun(),
    C = () => hd(i.networkMode) && i.canRun(),
    N = (Z) => {
      M() || (r?.(), g.resolve(Z));
    },
    R = (Z) => {
      M() || (r?.(), g.reject(Z));
    },
    lt = () =>
      new Promise((Z) => {
        ((r = (St) => {
          (M() || T()) && Z(St);
        }),
          i.onPause?.());
      }).then(() => {
        ((r = void 0), M() || i.onContinue?.());
      }),
    W = () => {
      if (M()) return;
      let Z;
      const St = o === 0 ? i.initialPromise : void 0;
      try {
        Z = St ?? i.fn();
      } catch (st) {
        Z = Promise.reject(st);
      }
      Promise.resolve(Z)
        .then(N)
        .catch((st) => {
          if (M()) return;
          const Dt = i.retry ?? (Ru.isServer() ? 0 : 3),
            gt = i.retryDelay ?? hm,
            Ut = typeof gt == "function" ? gt(o, st) : gt,
            xt =
              Dt === !0 ||
              (typeof Dt == "number" && o < Dt) ||
              (typeof Dt == "function" && Dt(o, st));
          if (f || !xt) {
            R(st);
            return;
          }
          (o++,
            i.onFail?.(o, st),
            nm(Ut)
              .then(() => (T() ? void 0 : lt()))
              .then(() => {
                f ? R(st) : W();
              }));
        });
    };
  return {
    promise: g,
    status: () => g.status,
    cancel: U,
    continue: () => (r?.(), g),
    cancelRetry: Q,
    continueRetry: z,
    canStart: C,
    start: () => (C() ? W() : lt().then(W), g),
  };
}
var yd = class {
  #t;
  destroy() {
    this.clearGcTimeout();
  }
  scheduleGc() {
    (this.clearGcTimeout(),
      Af(this.gcTime) &&
        (this.#t = Jl.setTimeout(() => {
          this.optionalRemove();
        }, this.gcTime)));
  }
  updateGcTime(i) {
    this.gcTime = Math.max(this.gcTime || 0, i ?? (Ru.isServer() ? 1 / 0 : 300 * 1e3));
  }
  clearGcTimeout() {
    this.#t !== void 0 && (Jl.clearTimeout(this.#t), (this.#t = void 0));
  }
};
function dm(i) {
  return {
    onFetch: (f, o) => {
      const r = f.options,
        g = f.fetchOptions?.meta?.fetchMore?.direction,
        M = f.state.data?.pages || [],
        U = f.state.data?.pageParams || [];
      let Q = { pages: [], pageParams: [] },
        z = 0;
      const T = async () => {
        let C = !1;
        const N = (W) => {
            fm(
              W,
              () => f.signal,
              () => (C = !0),
            );
          },
          R = od(f.options, f.fetchOptions),
          lt = async (W, Z, St) => {
            if (C) return Promise.reject(f.signal.reason);
            if (Z == null && W.pages.length) return Promise.resolve(W);
            const Dt = (() => {
                const Bt = {
                  client: f.client,
                  queryKey: f.queryKey,
                  pageParam: Z,
                  direction: St ? "backward" : "forward",
                  meta: f.options.meta,
                };
                return (N(Bt), Bt);
              })(),
              gt = await R(Dt),
              { maxPages: Ut } = f.options,
              xt = St ? cm : im;
            return { pages: xt(W.pages, gt, Ut), pageParams: xt(W.pageParams, Z, Ut) };
          };
        if (g && M.length) {
          const W = g === "backward",
            Z = W ? ym : ed,
            St = { pages: M, pageParams: U },
            st = Z(r, St);
          Q = await lt(St, st, W);
        } else {
          const W = i ?? M.length;
          do {
            const Z = z === 0 ? (U[0] ?? r.initialPageParam) : ed(r, Q);
            if (z > 0 && Z == null) break;
            ((Q = await lt(Q, Z)), z++);
          } while (z < W);
        }
        return Q;
      };
      f.options.persister
        ? (f.fetchFn = () =>
            f.options.persister?.(
              T,
              { client: f.client, queryKey: f.queryKey, meta: f.options.meta, signal: f.signal },
              o,
            ))
        : (f.fetchFn = T);
    },
  };
}
function ed(i, { pages: f, pageParams: o }) {
  const r = f.length - 1;
  return f.length > 0 ? i.getNextPageParam(f[r], f, o[r], o) : void 0;
}
function ym(i, { pages: f, pageParams: o }) {
  return f.length > 0 ? i.getPreviousPageParam?.(f[0], f, o[0], o) : void 0;
}
var vm = class extends yd {
  #t;
  #e;
  #l;
  #a;
  #n;
  #u;
  #c;
  #i;
  constructor(i) {
    (super(),
      (this.#i = !1),
      (this.#c = i.defaultOptions),
      this.setOptions(i.options),
      (this.observers = []),
      (this.#n = i.client),
      (this.#a = this.#n.getQueryCache()),
      (this.queryKey = i.queryKey),
      (this.queryHash = i.queryHash),
      (this.#e = ad(this.options)),
      (this.state = i.state ?? this.#e),
      this.scheduleGc());
  }
  get meta() {
    return this.options.meta;
  }
  get queryType() {
    return this.#t;
  }
  get promise() {
    return this.#u?.promise;
  }
  setOptions(i) {
    if (
      ((this.options = { ...this.#c, ...i }),
      i?._type && (this.#t = i._type),
      this.updateGcTime(this.options.gcTime),
      this.state && this.state.data === void 0)
    ) {
      const f = ad(this.options);
      f.data !== void 0 && (this.setState(ld(f.data, f.dataUpdatedAt)), (this.#e = f));
    }
  }
  optionalRemove() {
    !this.observers.length && this.state.fetchStatus === "idle" && this.#a.remove(this);
  }
  setData(i, f) {
    const o = _f(this.state.data, i, this.options);
    return (
      this.#f({ data: o, type: "success", dataUpdatedAt: f?.updatedAt, manual: f?.manual }), o
    );
  }
  setState(i) {
    this.#f({ type: "setState", state: i });
  }
  cancel(i) {
    const f = this.#u?.promise;
    return (this.#u?.cancel(i), f ? f.then($t).catch($t) : Promise.resolve());
  }
  destroy() {
    (super.destroy(), this.cancel({ silent: !0 }));
  }
  get resetState() {
    return this.#e;
  }
  reset() {
    (this.destroy(), this.setState(this.resetState));
  }
  isActive() {
    return this.observers.some((i) => ve(i.options.enabled, this) !== !1);
  }
  isDisabled() {
    return this.getObserversCount() > 0
      ? !this.isActive()
      : this.options.queryFn === qf || !this.isFetched();
  }
  isFetched() {
    return this.state.dataUpdateCount + this.state.errorUpdateCount > 0;
  }
  isStatic() {
    return this.getObserversCount() > 0
      ? this.observers.some((i) => Ml(i.options.staleTime, this) === "static")
      : !1;
  }
  isStale() {
    return this.getObserversCount() > 0
      ? this.observers.some((i) => i.getCurrentResult().isStale)
      : this.state.data === void 0 || this.state.isInvalidated;
  }
  isStaleByTime(i = 0) {
    return this.state.data === void 0
      ? !0
      : i === "static"
        ? !1
        : this.state.isInvalidated
          ? !0
          : !sd(this.state.dataUpdatedAt, i);
  }
  onFocus() {
    (this.observers.find((f) => f.shouldFetchOnWindowFocus())?.refetch({ cancelRefetch: !1 }),
      this.#u?.continue());
  }
  onOnline() {
    (this.observers.find((f) => f.shouldFetchOnReconnect())?.refetch({ cancelRefetch: !1 }),
      this.#u?.continue());
  }
  addObserver(i) {
    this.observers.includes(i) ||
      (this.observers.push(i),
      this.clearGcTimeout(),
      this.#a.notify({ type: "observerAdded", query: this, observer: i }));
  }
  removeObserver(i) {
    this.observers.includes(i) &&
      ((this.observers = this.observers.filter((f) => f !== i)),
      this.observers.length ||
        (this.#u && (this.#i || this.#r() ? this.#u.cancel({ revert: !0 }) : this.#u.cancelRetry()),
        this.scheduleGc()),
      this.#a.notify({ type: "observerRemoved", query: this, observer: i }));
  }
  getObserversCount() {
    return this.observers.length;
  }
  #r() {
    return this.state.fetchStatus === "paused" && this.state.status === "pending";
  }
  invalidate() {
    this.state.isInvalidated || this.#f({ type: "invalidate" });
  }
  async fetch(i, f) {
    if (this.state.fetchStatus !== "idle" && this.#u?.status() !== "rejected") {
      if (this.state.data !== void 0 && f?.cancelRefetch) this.cancel({ silent: !0 });
      else if (this.#u) return (this.#u.continueRetry(), this.#u.promise);
    }
    if ((i && this.setOptions(i), !this.options.queryFn)) {
      const z = this.observers.find((T) => T.options.queryFn);
      z && this.setOptions(z.options);
    }
    const o = new AbortController(),
      r = (z) => {
        Object.defineProperty(z, "signal", {
          enumerable: !0,
          get: () => ((this.#i = !0), o.signal),
        });
      },
      g = () => {
        const z = od(this.options, f),
          C = (() => {
            const N = { client: this.#n, queryKey: this.queryKey, meta: this.meta };
            return (r(N), N);
          })();
        return ((this.#i = !1), this.options.persister ? this.options.persister(z, C, this) : z(C));
      },
      U = (() => {
        const z = {
          fetchOptions: f,
          options: this.options,
          queryKey: this.queryKey,
          client: this.#n,
          state: this.state,
          fetchFn: g,
        };
        return (r(z), z);
      })();
    ((this.#t === "infinite" ? dm(this.options.pages) : this.options.behavior)?.onFetch(U, this),
      (this.#l = this.state),
      (this.state.fetchStatus === "idle" || this.state.fetchMeta !== U.fetchOptions?.meta) &&
        this.#f({ type: "fetch", meta: U.fetchOptions?.meta }),
      (this.#u = dd({
        initialPromise: f?.initialPromise,
        fn: U.fetchFn,
        onCancel: (z) => {
          (z instanceof Uf && z.revert && this.setState({ ...this.#l, fetchStatus: "idle" }),
            o.abort());
        },
        onFail: (z, T) => {
          this.#f({ type: "failed", failureCount: z, error: T });
        },
        onPause: () => {
          this.#f({ type: "pause" });
        },
        onContinue: () => {
          this.#f({ type: "continue" });
        },
        retry: U.options.retry,
        retryDelay: U.options.retryDelay,
        networkMode: U.options.networkMode,
        canRun: () => !0,
      })));
    try {
      const z = await this.#u.start();
      if (z === void 0) throw new Error(`${this.queryHash} data is undefined`);
      return (
        this.setData(z),
        this.#a.config.onSuccess?.(z, this),
        this.#a.config.onSettled?.(z, this.state.error, this),
        z
      );
    } catch (z) {
      if (z instanceof Uf) {
        if (z.silent) return this.#u.promise;
        if (z.revert) {
          if (this.state.data === void 0) throw z;
          return this.state.data;
        }
      }
      throw (
        this.#f({ type: "error", error: z }),
        this.#a.config.onError?.(z, this),
        this.#a.config.onSettled?.(this.state.data, z, this),
        z
      );
    } finally {
      this.scheduleGc();
    }
  }
  #f(i) {
    const f = (o) => {
      switch (i.type) {
        case "failed":
          return { ...o, fetchFailureCount: i.failureCount, fetchFailureReason: i.error };
        case "pause":
          return { ...o, fetchStatus: "paused" };
        case "continue":
          return { ...o, fetchStatus: "fetching" };
        case "fetch":
          return { ...o, ...vd(o.data, this.options), fetchMeta: i.meta ?? null };
        case "success":
          const r = {
            ...o,
            ...ld(i.data, i.dataUpdatedAt),
            dataUpdateCount: o.dataUpdateCount + 1,
            ...(!i.manual && {
              fetchStatus: "idle",
              fetchFailureCount: 0,
              fetchFailureReason: null,
            }),
          };
          return ((this.#l = i.manual ? r : void 0), r);
        case "error":
          const g = i.error;
          return {
            ...o,
            error: g,
            errorUpdateCount: o.errorUpdateCount + 1,
            errorUpdatedAt: Date.now(),
            fetchFailureCount: o.fetchFailureCount + 1,
            fetchFailureReason: g,
            fetchStatus: "idle",
            status: "error",
            isInvalidated: !0,
          };
        case "invalidate":
          return { ...o, isInvalidated: !0 };
        case "setState":
          return { ...o, ...i.state };
      }
    };
    ((this.state = f(this.state)),
      Qt.batch(() => {
        (this.observers.forEach((o) => {
          o.onQueryUpdate();
        }),
          this.#a.notify({ query: this, type: "updated", action: i }));
      }));
  }
};
function vd(i, f) {
  return {
    fetchFailureCount: 0,
    fetchFailureReason: null,
    fetchStatus: hd(f.networkMode) ? "fetching" : "paused",
    ...(i === void 0 && { error: null, status: "pending" }),
  };
}
function ld(i, f) {
  return {
    data: i,
    dataUpdatedAt: f ?? Date.now(),
    error: null,
    isInvalidated: !1,
    status: "success",
  };
}
function ad(i) {
  const f = typeof i.initialData == "function" ? i.initialData() : i.initialData,
    o = f !== void 0,
    r = o
      ? typeof i.initialDataUpdatedAt == "function"
        ? i.initialDataUpdatedAt()
        : i.initialDataUpdatedAt
      : 0;
  return {
    data: f,
    dataUpdateCount: 0,
    dataUpdatedAt: o ? (r ?? Date.now()) : 0,
    error: null,
    errorUpdateCount: 0,
    errorUpdatedAt: 0,
    fetchFailureCount: 0,
    fetchFailureReason: null,
    fetchMeta: null,
    isInvalidated: !1,
    status: o ? "success" : "pending",
    fetchStatus: "idle",
  };
}
var mm = class extends Na {
  constructor(i, f) {
    (super(),
      (this.options = f),
      (this.#t = i),
      (this.#i = null),
      (this.#c = Df()),
      this.bindMethods(),
      this.setOptions(f));
  }
  #t;
  #e = void 0;
  #l = void 0;
  #a = void 0;
  #n;
  #u;
  #c;
  #i;
  #r;
  #f;
  #y;
  #o;
  #h;
  #s;
  #v = new Set();
  bindMethods() {
    this.refetch = this.refetch.bind(this);
  }
  onSubscribe() {
    this.listeners.size === 1 &&
      (this.#e.addObserver(this),
      ud(this.#e, this.options) ? this.#d() : this.updateResult(),
      this.#p());
  }
  onUnsubscribe() {
    this.hasListeners() || this.destroy();
  }
  shouldFetchOnReconnect() {
    return Rf(this.#e, this.options, this.options.refetchOnReconnect);
  }
  shouldFetchOnWindowFocus() {
    return Rf(this.#e, this.options, this.options.refetchOnWindowFocus);
  }
  destroy() {
    ((this.listeners = new Set()), this.#b(), this.#E(), this.#e.removeObserver(this));
  }
  setOptions(i) {
    const f = this.options,
      o = this.#e;
    if (
      ((this.options = this.#t.defaultQueryOptions(i)),
      this.options.enabled !== void 0 &&
        typeof this.options.enabled != "boolean" &&
        typeof this.options.enabled != "function" &&
        typeof ve(this.options.enabled, this.#e) != "boolean")
    )
      throw new Error("Expected enabled to be a boolean or a callback that returns a boolean");
    (this.#T(),
      this.#e.setOptions(this.options),
      f._defaulted &&
        !kn(this.options, f) &&
        this.#t
          .getQueryCache()
          .notify({ type: "observerOptionsUpdated", query: this.#e, observer: this }));
    const r = this.hasListeners();
    (r && nd(this.#e, o, this.options, f) && this.#d(),
      this.updateResult(),
      r &&
        (this.#e !== o ||
          ve(this.options.enabled, this.#e) !== ve(f.enabled, this.#e) ||
          Ml(this.options.staleTime, this.#e) !== Ml(f.staleTime, this.#e)) &&
        this.#m());
    const g = this.#g();
    r &&
      (this.#e !== o ||
        ve(this.options.enabled, this.#e) !== ve(f.enabled, this.#e) ||
        g !== this.#s) &&
      this.#S(g);
  }
  getOptimisticResult(i) {
    const f = this.#t.getQueryCache().build(this.#t, i),
      o = this.createResult(f, i);
    return (Sm(this, o) && ((this.#a = o), (this.#u = this.options), (this.#n = this.#e.state)), o);
  }
  getCurrentResult() {
    return this.#a;
  }
  trackResult(i, f) {
    return new Proxy(i, {
      get: (o, r) => (
        this.trackProp(r),
        f?.(r),
        r === "promise" &&
          (this.trackProp("data"),
          !this.options.experimental_prefetchInRender &&
            this.#c.status === "pending" &&
            this.#c.reject(new Error("experimental_prefetchInRender feature flag is not enabled"))),
        Reflect.get(o, r)
      ),
    });
  }
  trackProp(i) {
    this.#v.add(i);
  }
  getCurrentQuery() {
    return this.#e;
  }
  refetch({ ...i } = {}) {
    return this.fetch({ ...i });
  }
  fetchOptimistic(i) {
    const f = this.#t.defaultQueryOptions(i),
      o = this.#t.getQueryCache().build(this.#t, f);
    return o.fetch().then(() => this.createResult(o, f));
  }
  fetch(i) {
    return this.#d({ ...i, cancelRefetch: i.cancelRefetch ?? !0 }).then(
      () => (this.updateResult(), this.#a),
    );
  }
  #d(i) {
    this.#T();
    let f = this.#e.fetch(this.options, i);
    return (i?.throwOnError || (f = f.catch($t)), f);
  }
  #m() {
    this.#b();
    const i = Ml(this.options.staleTime, this.#e);
    if (Ru.isServer() || this.#a.isStale || !Af(i)) return;
    const o = sd(this.#a.dataUpdatedAt, i) + 1;
    this.#o = Jl.setTimeout(() => {
      this.#a.isStale || this.updateResult();
    }, o);
  }
  #g() {
    return (
      (typeof this.options.refetchInterval == "function"
        ? this.options.refetchInterval(this.#e)
        : this.options.refetchInterval) ?? !1
    );
  }
  #S(i) {
    (this.#E(),
      (this.#s = i),
      !(
        Ru.isServer() ||
        ve(this.options.enabled, this.#e) === !1 ||
        !Af(this.#s) ||
        this.#s === 0
      ) &&
        (this.#h = Jl.setInterval(() => {
          (this.options.refetchIntervalInBackground || Hf.isFocused()) && this.#d();
        }, this.#s)));
  }
  #p() {
    (this.#m(), this.#S(this.#g()));
  }
  #b() {
    this.#o !== void 0 && (Jl.clearTimeout(this.#o), (this.#o = void 0));
  }
  #E() {
    this.#h !== void 0 && (Jl.clearInterval(this.#h), (this.#h = void 0));
  }
  createResult(i, f) {
    const o = this.#e,
      r = this.options,
      g = this.#a,
      M = this.#n,
      U = this.#u,
      z = i !== o ? i.state : this.#l,
      { state: T } = i;
    let C = { ...T },
      N = !1,
      R;
    if (f._optimisticResults) {
      const yt = this.hasListeners(),
        Jt = !yt && ud(i, f),
        _e = yt && nd(i, o, f, r);
      ((Jt || _e) && (C = { ...C, ...vd(T.data, i.options) }),
        f._optimisticResults === "isRestoring" && (C.fetchStatus = "idle"));
    }
    let { error: lt, errorUpdatedAt: W, status: Z } = C;
    R = C.data;
    let St = !1;
    if (f.placeholderData !== void 0 && R === void 0 && Z === "pending") {
      let yt;
      (g?.isPlaceholderData && f.placeholderData === U?.placeholderData
        ? ((yt = g.data), (St = !0))
        : (yt =
            typeof f.placeholderData == "function"
              ? f.placeholderData(this.#y?.state.data, this.#y)
              : f.placeholderData),
        yt !== void 0 && ((Z = "success"), (R = _f(g?.data, yt, f)), (N = !0)));
    }
    if (f.select && R !== void 0 && !St)
      if (g && R === M?.data && f.select === this.#r) R = this.#f;
      else
        try {
          ((this.#r = f.select),
            (R = f.select(R)),
            (R = _f(g?.data, R, f)),
            (this.#f = R),
            (this.#i = null));
        } catch (yt) {
          this.#i = yt;
        }
    this.#i && ((lt = this.#i), (R = this.#f), (W = Date.now()), (Z = "error"));
    const st = C.fetchStatus === "fetching",
      Dt = Z === "pending",
      gt = Z === "error",
      Ut = Dt && st,
      xt = R !== void 0,
      K = {
        status: Z,
        fetchStatus: C.fetchStatus,
        isPending: Dt,
        isSuccess: Z === "success",
        isError: gt,
        isInitialLoading: Ut,
        isLoading: Ut,
        data: R,
        dataUpdatedAt: C.dataUpdatedAt,
        error: lt,
        errorUpdatedAt: W,
        failureCount: C.fetchFailureCount,
        failureReason: C.fetchFailureReason,
        errorUpdateCount: C.errorUpdateCount,
        isFetched: i.isFetched(),
        isFetchedAfterMount:
          C.dataUpdateCount > z.dataUpdateCount || C.errorUpdateCount > z.errorUpdateCount,
        isFetching: st,
        isRefetching: st && !Dt,
        isLoadingError: gt && !xt,
        isPaused: C.fetchStatus === "paused",
        isPlaceholderData: N,
        isRefetchError: gt && xt,
        isStale: xf(i, f),
        refetch: this.refetch,
        promise: this.#c,
        isEnabled: ve(f.enabled, i) !== !1,
      };
    if (this.options.experimental_prefetchInRender) {
      const yt = K.data !== void 0,
        Jt = K.status === "error" && !yt,
        _e = (me) => {
          Jt ? me.reject(K.error) : yt && me.resolve(K.data);
        },
        ue = () => {
          const me = (this.#c = K.promise = Df());
          _e(me);
        },
        jt = this.#c;
      switch (jt.status) {
        case "pending":
          i.queryHash === o.queryHash && _e(jt);
          break;
        case "fulfilled":
          (Jt || K.data !== jt.value) && ue();
          break;
        case "rejected":
          (!Jt || K.error !== jt.reason) && ue();
          break;
      }
    }
    return K;
  }
  updateResult() {
    const i = this.#a,
      f = this.createResult(this.#e, this.options);
    if (
      ((this.#n = this.#e.state),
      (this.#u = this.options),
      this.#n.data !== void 0 && (this.#y = this.#e),
      kn(f, i))
    )
      return;
    this.#a = f;
    const o = () => {
      if (!i) return !0;
      const { notifyOnChangeProps: r } = this.options,
        g = typeof r == "function" ? r() : r;
      if (g === "all" || (!g && !this.#v.size)) return !0;
      const M = new Set(g ?? this.#v);
      return (
        this.options.throwOnError && M.add("error"),
        Object.keys(this.#a).some((U) => {
          const Q = U;
          return this.#a[Q] !== i[Q] && M.has(Q);
        })
      );
    };
    this.#O({ listeners: o() });
  }
  #T() {
    const i = this.#t.getQueryCache().build(this.#t, this.options);
    if (i === this.#e) return;
    const f = this.#e;
    ((this.#e = i),
      (this.#l = i.state),
      this.hasListeners() && (f?.removeObserver(this), i.addObserver(this)));
  }
  onQueryUpdate() {
    (this.updateResult(), this.hasListeners() && this.#p());
  }
  #O(i) {
    Qt.batch(() => {
      (i.listeners &&
        this.listeners.forEach((f) => {
          f(this.#a);
        }),
        this.#t.getQueryCache().notify({ query: this.#e, type: "observerResultsUpdated" }));
    });
  }
};
function gm(i, f) {
  return (
    ve(f.enabled, i) !== !1 &&
    i.state.data === void 0 &&
    !(i.state.status === "error" && ve(f.retryOnMount, i) === !1)
  );
}
function ud(i, f) {
  return gm(i, f) || (i.state.data !== void 0 && Rf(i, f, f.refetchOnMount));
}
function Rf(i, f, o) {
  if (ve(f.enabled, i) !== !1 && Ml(f.staleTime, i) !== "static") {
    const r = typeof o == "function" ? o(i) : o;
    return r === "always" || (r !== !1 && xf(i, f));
  }
  return !1;
}
function nd(i, f, o, r) {
  return (
    (i !== f || ve(r.enabled, i) === !1) && (!o.suspense || i.state.status !== "error") && xf(i, o)
  );
}
function xf(i, f) {
  return ve(f.enabled, i) !== !1 && i.isStaleByTime(Ml(f.staleTime, i));
}
function Sm(i, f) {
  return !kn(i.getCurrentResult(), f);
}
var pm = class extends yd {
  #t;
  #e;
  #l;
  #a;
  constructor(i) {
    (super(),
      (this.#t = i.client),
      (this.mutationId = i.mutationId),
      (this.#l = i.mutationCache),
      (this.#e = []),
      (this.state = i.state || md()),
      this.setOptions(i.options),
      this.scheduleGc());
  }
  setOptions(i) {
    ((this.options = i), this.updateGcTime(this.options.gcTime));
  }
  get meta() {
    return this.options.meta;
  }
  addObserver(i) {
    this.#e.includes(i) ||
      (this.#e.push(i),
      this.clearGcTimeout(),
      this.#l.notify({ type: "observerAdded", mutation: this, observer: i }));
  }
  removeObserver(i) {
    ((this.#e = this.#e.filter((f) => f !== i)),
      this.scheduleGc(),
      this.#l.notify({ type: "observerRemoved", mutation: this, observer: i }));
  }
  optionalRemove() {
    this.#e.length || (this.state.status === "pending" ? this.scheduleGc() : this.#l.remove(this));
  }
  continue() {
    return this.#a?.continue() ?? this.execute(this.state.variables);
  }
  async execute(i) {
    const f = () => {
        this.#n({ type: "continue" });
      },
      o = { client: this.#t, meta: this.options.meta, mutationKey: this.options.mutationKey };
    this.#a = dd({
      fn: () =>
        this.options.mutationFn
          ? this.options.mutationFn(i, o)
          : Promise.reject(new Error("No mutationFn found")),
      onFail: (M, U) => {
        this.#n({ type: "failed", failureCount: M, error: U });
      },
      onPause: () => {
        this.#n({ type: "pause" });
      },
      onContinue: f,
      retry: this.options.retry ?? 0,
      retryDelay: this.options.retryDelay,
      networkMode: this.options.networkMode,
      canRun: () => this.#l.canRun(this),
    });
    const r = this.state.status === "pending",
      g = !this.#a.canStart();
    try {
      if (r) f();
      else {
        (this.#n({ type: "pending", variables: i, isPaused: g }),
          this.#l.config.onMutate && (await this.#l.config.onMutate(i, this, o)));
        const U = await this.options.onMutate?.(i, o);
        U !== this.state.context &&
          this.#n({ type: "pending", context: U, variables: i, isPaused: g });
      }
      const M = await this.#a.start();
      return (
        await this.#l.config.onSuccess?.(M, i, this.state.context, this, o),
        await this.options.onSuccess?.(M, i, this.state.context, o),
        await this.#l.config.onSettled?.(
          M,
          null,
          this.state.variables,
          this.state.context,
          this,
          o,
        ),
        await this.options.onSettled?.(M, null, i, this.state.context, o),
        this.#n({ type: "success", data: M }),
        M
      );
    } catch (M) {
      try {
        await this.#l.config.onError?.(M, i, this.state.context, this, o);
      } catch (U) {
        Promise.reject(U);
      }
      try {
        await this.options.onError?.(M, i, this.state.context, o);
      } catch (U) {
        Promise.reject(U);
      }
      try {
        await this.#l.config.onSettled?.(
          void 0,
          M,
          this.state.variables,
          this.state.context,
          this,
          o,
        );
      } catch (U) {
        Promise.reject(U);
      }
      try {
        await this.options.onSettled?.(void 0, M, i, this.state.context, o);
      } catch (U) {
        Promise.reject(U);
      }
      throw (this.#n({ type: "error", error: M }), M);
    } finally {
      this.#l.runNext(this);
    }
  }
  #n(i) {
    const f = (o) => {
      switch (i.type) {
        case "failed":
          return { ...o, failureCount: i.failureCount, failureReason: i.error };
        case "pause":
          return { ...o, isPaused: !0 };
        case "continue":
          return { ...o, isPaused: !1 };
        case "pending":
          return {
            ...o,
            context: i.context,
            data: void 0,
            failureCount: 0,
            failureReason: null,
            error: null,
            isPaused: i.isPaused,
            status: "pending",
            variables: i.variables,
            submittedAt: Date.now(),
          };
        case "success":
          return {
            ...o,
            data: i.data,
            failureCount: 0,
            failureReason: null,
            error: null,
            status: "success",
            isPaused: !1,
          };
        case "error":
          return {
            ...o,
            data: void 0,
            error: i.error,
            failureCount: o.failureCount + 1,
            failureReason: i.error,
            isPaused: !1,
            status: "error",
          };
      }
    };
    ((this.state = f(this.state)),
      Qt.batch(() => {
        (this.#e.forEach((o) => {
          o.onMutationUpdate(i);
        }),
          this.#l.notify({ mutation: this, type: "updated", action: i }));
      }));
  }
};
function md() {
  return {
    context: void 0,
    data: void 0,
    error: null,
    failureCount: 0,
    failureReason: null,
    isPaused: !1,
    status: "idle",
    variables: void 0,
    submittedAt: 0,
  };
}
var bm = class extends Na {
  constructor(i = {}) {
    (super(), (this.config = i), (this.#t = new Set()), (this.#e = new Map()), (this.#l = 0));
  }
  #t;
  #e;
  #l;
  build(i, f, o) {
    const r = new pm({
      client: i,
      mutationCache: this,
      mutationId: ++this.#l,
      options: i.defaultMutationOptions(f),
      state: o,
    });
    return (this.add(r), r);
  }
  add(i) {
    this.#t.add(i);
    const f = $n(i);
    if (typeof f == "string") {
      const o = this.#e.get(f);
      o ? o.push(i) : this.#e.set(f, [i]);
    }
    this.notify({ type: "added", mutation: i });
  }
  remove(i) {
    if (this.#t.delete(i)) {
      const f = $n(i);
      if (typeof f == "string") {
        const o = this.#e.get(f);
        if (o)
          if (o.length > 1) {
            const r = o.indexOf(i);
            r !== -1 && o.splice(r, 1);
          } else o[0] === i && this.#e.delete(f);
      }
    }
    this.notify({ type: "removed", mutation: i });
  }
  canRun(i) {
    const f = $n(i);
    if (typeof f == "string") {
      const r = this.#e.get(f)?.find((g) => g.state.status === "pending");
      return !r || r === i;
    } else return !0;
  }
  runNext(i) {
    const f = $n(i);
    return typeof f == "string"
      ? (this.#e
          .get(f)
          ?.find((r) => r !== i && r.state.isPaused)
          ?.continue() ?? Promise.resolve())
      : Promise.resolve();
  }
  clear() {
    Qt.batch(() => {
      (this.#t.forEach((i) => {
        this.notify({ type: "removed", mutation: i });
      }),
        this.#t.clear(),
        this.#e.clear());
    });
  }
  getAll() {
    return Array.from(this.#t);
  }
  find(i) {
    const f = { exact: !0, ...i };
    return this.getAll().find((o) => Ih(f, o));
  }
  findAll(i = {}) {
    return this.getAll().filter((f) => Ih(i, f));
  }
  notify(i) {
    Qt.batch(() => {
      this.listeners.forEach((f) => {
        f(i);
      });
    });
  }
  resumePausedMutations() {
    const i = this.getAll().filter((f) => f.state.isPaused);
    return Qt.batch(() => Promise.all(i.map((f) => f.continue().catch($t))));
  }
};
function $n(i) {
  return i.options.scope?.id;
}
var Em = class extends Na {
    #t;
    #e = void 0;
    #l;
    #a;
    constructor(f, o) {
      (super(), (this.#t = f), this.setOptions(o), this.bindMethods(), this.#n());
    }
    bindMethods() {
      ((this.mutate = this.mutate.bind(this)), (this.reset = this.reset.bind(this)));
    }
    setOptions(f) {
      const o = this.options;
      ((this.options = this.#t.defaultMutationOptions(f)),
        kn(this.options, o) ||
          this.#t
            .getMutationCache()
            .notify({ type: "observerOptionsUpdated", mutation: this.#l, observer: this }),
        o?.mutationKey &&
        this.options.mutationKey &&
        wl(o.mutationKey) !== wl(this.options.mutationKey)
          ? this.reset()
          : this.#l?.state.status === "pending" && this.#l.setOptions(this.options));
    }
    onUnsubscribe() {
      this.hasListeners() || this.#l?.removeObserver(this);
    }
    onMutationUpdate(f) {
      (this.#n(), this.#u(f));
    }
    getCurrentResult() {
      return this.#e;
    }
    reset() {
      (this.#l?.removeObserver(this), (this.#l = void 0), this.#n(), this.#u());
    }
    mutate(f, o) {
      return (
        (this.#a = o),
        this.#l?.removeObserver(this),
        (this.#l = this.#t.getMutationCache().build(this.#t, this.options)),
        this.#l.addObserver(this),
        this.#l.execute(f)
      );
    }
    #n() {
      const f = this.#l?.state ?? md();
      this.#e = {
        ...f,
        isPending: f.status === "pending",
        isSuccess: f.status === "success",
        isError: f.status === "error",
        isIdle: f.status === "idle",
        mutate: this.mutate,
        reset: this.reset,
      };
    }
    #u(f) {
      Qt.batch(() => {
        if (this.#a && this.hasListeners()) {
          const o = this.#e.variables,
            r = this.#e.context,
            g = { client: this.#t, meta: this.options.meta, mutationKey: this.options.mutationKey };
          if (f?.type === "success") {
            try {
              this.#a.onSuccess?.(f.data, o, r, g);
            } catch (M) {
              Promise.reject(M);
            }
            try {
              this.#a.onSettled?.(f.data, null, o, r, g);
            } catch (M) {
              Promise.reject(M);
            }
          } else if (f?.type === "error") {
            try {
              this.#a.onError?.(f.error, o, r, g);
            } catch (M) {
              Promise.reject(M);
            }
            try {
              this.#a.onSettled?.(void 0, f.error, o, r, g);
            } catch (M) {
              Promise.reject(M);
            }
          }
        }
        this.listeners.forEach((o) => {
          o(this.#e);
        });
      });
    }
  },
  Tm = class extends Na {
    constructor(i = {}) {
      (super(), (this.config = i), (this.#t = new Map()));
    }
    #t;
    build(i, f, o) {
      const r = f.queryKey,
        g = f.queryHash ?? jf(r, f);
      let M = this.get(g);
      return (
        M ||
          ((M = new vm({
            client: i,
            queryKey: r,
            queryHash: g,
            options: i.defaultQueryOptions(f),
            state: o,
            defaultOptions: i.getQueryDefaults(r),
          })),
          this.add(M)),
        M
      );
    }
    add(i) {
      this.#t.has(i.queryHash) ||
        (this.#t.set(i.queryHash, i), this.notify({ type: "added", query: i }));
    }
    remove(i) {
      const f = this.#t.get(i.queryHash);
      f &&
        (i.destroy(),
        f === i && this.#t.delete(i.queryHash),
        this.notify({ type: "removed", query: i }));
    }
    clear() {
      Qt.batch(() => {
        this.getAll().forEach((i) => {
          this.remove(i);
        });
      });
    }
    get(i) {
      return this.#t.get(i);
    }
    getAll() {
      return [...this.#t.values()];
    }
    find(i) {
      const f = { exact: !0, ...i };
      return this.getAll().find((o) => kh(f, o));
    }
    findAll(i = {}) {
      const f = this.getAll();
      return Object.keys(i).length > 0 ? f.filter((o) => kh(i, o)) : f;
    }
    notify(i) {
      Qt.batch(() => {
        this.listeners.forEach((f) => {
          f(i);
        });
      });
    }
    onFocus() {
      Qt.batch(() => {
        this.getAll().forEach((i) => {
          i.onFocus();
        });
      });
    }
    onOnline() {
      Qt.batch(() => {
        this.getAll().forEach((i) => {
          i.onOnline();
        });
      });
    }
  },
  Om = class {
    #t;
    #e;
    #l;
    #a;
    #n;
    #u;
    #c;
    #i;
    constructor(i = {}) {
      ((this.#t = i.queryCache || new Tm()),
        (this.#e = i.mutationCache || new bm()),
        (this.#l = i.defaultOptions || {}),
        (this.#a = new Map()),
        (this.#n = new Map()),
        (this.#u = 0));
    }
    mount() {
      (this.#u++,
        this.#u === 1 &&
          ((this.#c = Hf.subscribe(async (i) => {
            i && (await this.resumePausedMutations(), this.#t.onFocus());
          })),
          (this.#i = In.subscribe(async (i) => {
            i && (await this.resumePausedMutations(), this.#t.onOnline());
          }))));
    }
    unmount() {
      (this.#u--,
        this.#u === 0 && (this.#c?.(), (this.#c = void 0), this.#i?.(), (this.#i = void 0)));
    }
    isFetching(i) {
      return this.#t.findAll({ ...i, fetchStatus: "fetching" }).length;
    }
    isMutating(i) {
      return this.#e.findAll({ ...i, status: "pending" }).length;
    }
    getQueryData(i) {
      const f = this.defaultQueryOptions({ queryKey: i });
      return this.#t.get(f.queryHash)?.state.data;
    }
    ensureQueryData(i) {
      const f = this.defaultQueryOptions(i),
        o = this.#t.build(this, f),
        r = o.state.data;
      return r === void 0
        ? this.fetchQuery(i)
        : (i.revalidateIfStale && o.isStaleByTime(Ml(f.staleTime, o)) && this.prefetchQuery(f),
          Promise.resolve(r));
    }
    getQueriesData(i) {
      return this.#t.findAll(i).map(({ queryKey: f, state: o }) => {
        const r = o.data;
        return [f, r];
      });
    }
    setQueryData(i, f, o) {
      const r = this.defaultQueryOptions({ queryKey: i }),
        M = this.#t.get(r.queryHash)?.state.data,
        U = am(f, M);
      if (U !== void 0) return this.#t.build(this, r).setData(U, { ...o, manual: !0 });
    }
    setQueriesData(i, f, o) {
      return Qt.batch(() =>
        this.#t.findAll(i).map(({ queryKey: r }) => [r, this.setQueryData(r, f, o)]),
      );
    }
    getQueryState(i) {
      const f = this.defaultQueryOptions({ queryKey: i });
      return this.#t.get(f.queryHash)?.state;
    }
    removeQueries(i) {
      const f = this.#t;
      Qt.batch(() => {
        f.findAll(i).forEach((o) => {
          f.remove(o);
        });
      });
    }
    resetQueries(i, f) {
      const o = this.#t;
      return Qt.batch(
        () => (
          o.findAll(i).forEach((r) => {
            r.reset();
          }),
          this.refetchQueries({ type: "active", ...i }, f)
        ),
      );
    }
    cancelQueries(i, f = {}) {
      const o = { revert: !0, ...f },
        r = Qt.batch(() => this.#t.findAll(i).map((g) => g.cancel(o)));
      return Promise.all(r).then($t).catch($t);
    }
    invalidateQueries(i, f = {}) {
      return Qt.batch(
        () => (
          this.#t.findAll(i).forEach((o) => {
            o.invalidate();
          }),
          i?.refetchType === "none"
            ? Promise.resolve()
            : this.refetchQueries({ ...i, type: i?.refetchType ?? i?.type ?? "active" }, f)
        ),
      );
    }
    refetchQueries(i, f = {}) {
      const o = { ...f, cancelRefetch: f.cancelRefetch ?? !0 },
        r = Qt.batch(() =>
          this.#t
            .findAll(i)
            .filter((g) => !g.isDisabled() && !g.isStatic())
            .map((g) => {
              let M = g.fetch(void 0, o);
              return (
                o.throwOnError || (M = M.catch($t)),
                g.state.fetchStatus === "paused" ? Promise.resolve() : M
              );
            }),
        );
      return Promise.all(r).then($t);
    }
    fetchQuery(i) {
      const f = this.defaultQueryOptions(i);
      f.retry === void 0 && (f.retry = !1);
      const o = this.#t.build(this, f);
      return o.isStaleByTime(Ml(f.staleTime, o)) ? o.fetch(f) : Promise.resolve(o.state.data);
    }
    prefetchQuery(i) {
      return this.fetchQuery(i).then($t).catch($t);
    }
    fetchInfiniteQuery(i) {
      return ((i._type = "infinite"), this.fetchQuery(i));
    }
    prefetchInfiniteQuery(i) {
      return this.fetchInfiniteQuery(i).then($t).catch($t);
    }
    ensureInfiniteQueryData(i) {
      return ((i._type = "infinite"), this.ensureQueryData(i));
    }
    resumePausedMutations() {
      return In.isOnline() ? this.#e.resumePausedMutations() : Promise.resolve();
    }
    getQueryCache() {
      return this.#t;
    }
    getMutationCache() {
      return this.#e;
    }
    getDefaultOptions() {
      return this.#l;
    }
    setDefaultOptions(i) {
      this.#l = i;
    }
    setQueryDefaults(i, f) {
      this.#a.set(wl(i), { queryKey: i, defaultOptions: f });
    }
    getQueryDefaults(i) {
      const f = [...this.#a.values()],
        o = {};
      return (
        f.forEach((r) => {
          Uu(i, r.queryKey) && Object.assign(o, r.defaultOptions);
        }),
        o
      );
    }
    setMutationDefaults(i, f) {
      this.#n.set(wl(i), { mutationKey: i, defaultOptions: f });
    }
    getMutationDefaults(i) {
      const f = [...this.#n.values()],
        o = {};
      return (
        f.forEach((r) => {
          Uu(i, r.mutationKey) && Object.assign(o, r.defaultOptions);
        }),
        o
      );
    }
    defaultQueryOptions(i) {
      if (i._defaulted) return i;
      const f = { ...this.#l.queries, ...this.getQueryDefaults(i.queryKey), ...i, _defaulted: !0 };
      return (
        f.queryHash || (f.queryHash = jf(f.queryKey, f)),
        f.refetchOnReconnect === void 0 && (f.refetchOnReconnect = f.networkMode !== "always"),
        f.throwOnError === void 0 && (f.throwOnError = !!f.suspense),
        !f.networkMode && f.persister && (f.networkMode = "offlineFirst"),
        f.queryFn === qf && (f.enabled = !1),
        f
      );
    }
    defaultMutationOptions(i) {
      return i?._defaulted
        ? i
        : {
            ...this.#l.mutations,
            ...(i?.mutationKey && this.getMutationDefaults(i.mutationKey)),
            ...i,
            _defaulted: !0,
          };
    }
    clear() {
      (this.#t.clear(), this.#e.clear());
    }
  },
  gd = Tt.createContext(void 0),
  Bf = (i) => {
    const f = Tt.useContext(gd);
    if (!f) throw new Error("No QueryClient set, use QueryClientProvider to set one");
    return f;
  },
  zm = ({ client: i, children: f }) => (
    Tt.useEffect(
      () => (
        i.mount(),
        () => {
          i.unmount();
        }
      ),
      [i],
    ),
    q.jsx(gd.Provider, { value: i, children: f })
  ),
  Sd = Tt.createContext(!1),
  Am = () => Tt.useContext(Sd);
Sd.Provider;
function Mm() {
  let i = !1;
  return {
    clearReset: () => {
      i = !1;
    },
    reset: () => {
      i = !0;
    },
    isReset: () => i,
  };
}
var _m = Tt.createContext(Mm()),
  Dm = () => Tt.useContext(_m),
  Um = (i, f, o) => {
    const r =
      o?.state.error && typeof i.throwOnError == "function"
        ? Qf(i.throwOnError, [o.state.error, o])
        : i.throwOnError;
    (i.suspense || i.experimental_prefetchInRender || r) && (f.isReset() || (i.retryOnMount = !1));
  },
  Rm = (i) => {
    Tt.useEffect(() => {
      i.clearReset();
    }, [i]);
  },
  Cm = ({ result: i, errorResetBoundary: f, throwOnError: o, query: r, suspense: g }) =>
    i.isError &&
    !f.isReset() &&
    !i.isFetching &&
    r &&
    ((g && i.data === void 0) || Qf(o, [i.error, r])),
  Nm = (i) => {
    if (i.suspense) {
      const o = (g) => (g === "static" ? g : Math.max(g ?? 1e3, 1e3)),
        r = i.staleTime;
      ((i.staleTime = typeof r == "function" ? (...g) => o(r(...g)) : o(r)),
        typeof i.gcTime == "number" && (i.gcTime = Math.max(i.gcTime, 1e3)));
    }
  },
  Hm = (i, f) => i.isLoading && i.isFetching && !f,
  jm = (i, f) => i?.suspense && f.isPending,
  id = (i, f, o) =>
    f.fetchOptimistic(i).catch(() => {
      o.clearReset();
    });
function qm(i, f, o) {
  const r = Am(),
    g = Dm(),
    M = Bf(),
    U = M.defaultQueryOptions(i);
  M.getDefaultOptions().queries?._experimental_beforeQuery?.(U);
  const Q = M.getQueryCache().get(U.queryHash),
    z = i.subscribed !== !1;
  ((U._optimisticResults = r ? "isRestoring" : z ? "optimistic" : void 0),
    Nm(U),
    Um(U, g, Q),
    Rm(g));
  const T = !M.getQueryCache().get(U.queryHash),
    [C] = Tt.useState(() => new f(M, U)),
    N = C.getOptimisticResult(U),
    R = !r && z;
  if (
    (Tt.useSyncExternalStore(
      Tt.useCallback(
        (lt) => {
          const W = R ? C.subscribe(Qt.batchCalls(lt)) : $t;
          return (C.updateResult(), W);
        },
        [C, R],
      ),
      () => C.getCurrentResult(),
      () => C.getCurrentResult(),
    ),
    Tt.useEffect(() => {
      C.setOptions(U);
    }, [U, C]),
    jm(U, N))
  )
    throw id(U, C, g);
  if (
    Cm({
      result: N,
      errorResetBoundary: g,
      throwOnError: U.throwOnError,
      query: Q,
      suspense: U.suspense,
    })
  )
    throw N.error;
  return (
    M.getDefaultOptions().queries?._experimental_afterQuery?.(U, N),
    U.experimental_prefetchInRender &&
      !Ru.isServer() &&
      Hm(N, r) &&
      (T ? id(U, C, g) : Q?.promise)?.catch($t).finally(() => {
        C.updateResult();
      }),
    U.notifyOnChangeProps ? N : C.trackResult(N)
  );
}
function Qm(i, f) {
  return qm(i, mm);
}
function cd(i, f) {
  const o = Bf(),
    [r] = Tt.useState(() => new Em(o, i));
  Tt.useEffect(() => {
    r.setOptions(i);
  }, [r, i]);
  const g = Tt.useSyncExternalStore(
      Tt.useCallback((U) => r.subscribe(Qt.batchCalls(U)), [r]),
      () => r.getCurrentResult(),
      () => r.getCurrentResult(),
    ),
    M = Tt.useCallback(
      (U, Q) => {
        r.mutate(U, Q).catch($t);
      },
      [r],
    );
  if (g.error && Qf(r.options.throwOnError, [g.error])) throw g.error;
  return { ...g, mutate: M, mutateAsync: g.mutate };
}
const xm = {
  scope: "local",
  path: "/workspace/.native-whisperx/speakers",
  library: {
    path: "/workspace/.native-whisperx/speakers/library.json",
    status: "valid",
    profileCount: 1,
  },
  profiles: [
    { id: "speaker-a", label: "Speaker A", metadata: { status: "confirmed", note: "fixture" } },
  ],
  trace: {
    path: "/workspace/.native-whisperx/speakers/speaker-trace.json",
    status: "valid",
    scanRoot: "/workspace",
    speakers: [
      {
        kind: "enrolled",
        profileId: "speaker-a",
        label: "Speaker A",
        files: [
          {
            sourceFile: "/workspace/interview.json",
            segmentCount: 2,
            totalDuration: 7.5,
            spans: [
              { startSeconds: 0, endSeconds: 2.4, snippet: "Welcome to the native-whisperx demo." },
            ],
          },
        ],
      },
      { kind: "anonymous", anonymousLabel: "speaker_1", files: [] },
    ],
    errors: [],
  },
};
structuredClone(xm);
const Bm = "X-Native-Whisperx-Session-Token";
async function Du(i, f) {
  const o = await fetch(i, {
    ...f,
    headers: {
      ...(f?.body ? { "Content-Type": "application/json" } : {}),
      ...(window.nativeWhisperxSessionToken ? { [Bm]: window.nativeWhisperxSessionToken } : {}),
      ...f?.headers,
    },
  });
  if (!o.ok) throw new Error(await o.text());
  return await o.json();
}
const Cf = {
    getState: () => Du("/api/state"),
    updateProfile: async (i, f) => (
      await Du(`/api/profiles/${encodeURIComponent(i)}`, {
        method: "PUT",
        body: JSON.stringify(f),
      }),
      Cf.getState()
    ),
    deleteProfile: async (i) => (
      await Du(`/api/profiles/${encodeURIComponent(i)}`, { method: "DELETE" }),
      Cf.getState()
    ),
    createProfile: () => Du("/api/profiles", { method: "POST", body: JSON.stringify({}) }),
    rebuildTrace: (i) =>
      Du("/api/trace/rebuild", { method: "POST", body: JSON.stringify(i.scanRoot ? i : {}) }),
  },
  Ym = Cf;
function Gm({ api: i = Ym }) {
  const [f] = Tt.useState(() => new Om());
  return q.jsx(zm, { client: f, children: q.jsx(Xm, { api: i }) });
}
function Xm({ api: i }) {
  const f = Qm({ queryKey: ["speaker-directory-state"], queryFn: () => i.getState() });
  return f.isLoading
    ? q.jsx("main", { className: "page", children: "Loading Speaker Directory..." })
    : f.isError || !f.data
      ? q.jsxs("main", {
          className: "page",
          children: [
            q.jsx("h1", { children: "Speaker Directory" }),
            q.jsx("p", { role: "alert", children: "Failed to load Speaker Directory state." }),
          ],
        })
      : q.jsx(Lm, { api: i, state: f.data });
}
function Lm({ api: i, state: f }) {
  const [o] = Tt.useState(f.trace.scanRoot ?? ""),
    r = f.trace.speakers.filter((g) => g.kind === "anonymous");
  return q.jsxs("main", {
    className: "page",
    children: [
      q.jsxs("header", {
        className: "header",
        children: [
          q.jsx("p", { className: "eyebrow", children: "CLI workspace" }),
          q.jsx("h1", { children: "Speaker Directory" }),
          q.jsx("p", { className: "path", children: f.path }),
        ],
      }),
      q.jsxs("section", {
        className: "summaryGrid",
        "aria-label": "Speaker Directory summary",
        children: [
          q.jsx(zf, {
            title: "Speaker Library",
            status: f.library.status,
            detail: `${f.library.profileCount} profile${f.library.profileCount === 1 ? "" : "s"}`,
          }),
          q.jsx(zf, {
            title: "Speaker Trace",
            status: f.trace.status,
            detail: f.trace.scanRoot ?? "No scan root",
          }),
          q.jsx(zf, { title: "Scope", status: f.scope, detail: "Speaker Directory" }),
        ],
      }),
      q.jsxs("section", {
        children: [
          q.jsxs("div", {
            className: "sectionHeading",
            children: [
              q.jsx("h2", { children: "Speaker Library profiles" }),
              q.jsx("span", { children: f.profiles.length }),
            ],
          }),
          q.jsx("div", {
            className: "profileList",
            children: f.profiles.map((g) => q.jsx(Zm, { api: i, profile: g }, g.id)),
          }),
        ],
      }),
      q.jsxs("section", {
        children: [
          q.jsxs("div", {
            className: "sectionHeading",
            children: [
              q.jsx("h2", { children: "Speaker Trace" }),
              q.jsx("span", { children: f.trace.speakers.length }),
            ],
          }),
          q.jsxs("div", {
            className: "traceMeta",
            children: [
              q.jsx("span", { children: "Scan root" }),
              q.jsx("code", { children: o || "Not available" }),
            ],
          }),
          q.jsx("div", {
            className: "profileList",
            children: f.trace.speakers.map((g) =>
              q.jsxs(
                "article",
                {
                  className: "profile",
                  children: [
                    q.jsx("h3", {
                      children: g.label ?? g.anonymousLabel ?? "Anonymous Speaker Label",
                    }),
                    q.jsx("p", {
                      className: "mono",
                      children: g.kind === "anonymous" ? "Anonymous Speaker Label" : g.profileId,
                    }),
                    q.jsxs("p", { children: [g.files.length, " traced file(s)"] }),
                  ],
                },
                g.profileId ?? g.anonymousLabel,
              ),
            ),
          }),
        ],
      }),
      q.jsxs("section", {
        children: [
          q.jsxs("div", {
            className: "sectionHeading",
            children: [
              q.jsx("h2", { children: "Anonymous Speaker Label" }),
              q.jsx("span", { children: r.length }),
            ],
          }),
          q.jsx("p", {
            className: "muted",
            children:
              "Anonymous Speaker Labels are Speaker Trace data, not enrolled Speaker Library identities.",
          }),
        ],
      }),
    ],
  });
}
function zf({ title: i, status: f, detail: o }) {
  return q.jsxs("article", {
    className: "statusPanel",
    children: [
      q.jsx("h2", { children: i }),
      q.jsx("p", { className: "status", children: f }),
      q.jsx("p", { children: o }),
    ],
  });
}
function Zm({ api: i, profile: f }) {
  const o = Bf(),
    [r, g] = Tt.useState(f.label),
    [M, U] = Tt.useState(Km(f.metadata)),
    [Q, z] = Tt.useState(null),
    T = cd({
      mutationFn: () => {
        const R = fd(M);
        return (z(null), i.updateProfile(f.id, { id: f.id, label: r, metadata: R }));
      },
      onSuccess: (R) => {
        o.setQueryData(["speaker-directory-state"], R);
      },
      onError: (R) => {
        z(R instanceof Error ? R.message : "Failed to save Speaker Library profile.");
      },
    }),
    C = cd({
      mutationFn: () => (z(null), i.deleteProfile(f.id)),
      onSuccess: (R) => {
        o.setQueryData(["speaker-directory-state"], R);
      },
      onError: (R) => {
        z(R instanceof Error ? R.message : "Failed to delete Speaker Library profile.");
      },
    }),
    N = () => {
      try {
        fd(M);
      } catch (R) {
        z(R instanceof Error ? R.message : "Speaker Library profile metadata is malformed.");
        return;
      }
      T.mutate();
    };
  return q.jsxs("article", {
    className: "profile",
    children: [
      q.jsxs("div", {
        className: "profileIdentity",
        children: [
          q.jsxs("div", {
            children: [
              q.jsx("h3", { children: f.label }),
              q.jsx("p", { className: "identityLabel", children: "Stable profile id" }),
              q.jsx("p", { className: "mono profileId", children: f.id }),
            ],
          }),
          q.jsx("span", { className: "identityBadge", children: "Speaker Library profile" }),
        ],
      }),
      q.jsx("dl", {
        children: Object.entries(f.metadata).map(([R, lt]) =>
          q.jsxs(
            "div",
            { children: [q.jsx("dt", { children: R }), q.jsx("dd", { children: lt })] },
            R,
          ),
        ),
      }),
      q.jsxs("div", {
        className: "profileForm",
        children: [
          q.jsxs("label", {
            children: [
              "Label",
              q.jsx("input", {
                "aria-label": `${f.id} label`,
                value: r,
                onChange: (R) => g(R.currentTarget.value),
              }),
            ],
          }),
          q.jsxs("label", {
            children: [
              "Metadata",
              q.jsx("textarea", {
                "aria-label": `${f.id} metadata`,
                rows: 4,
                value: M,
                onChange: (R) => U(R.currentTarget.value),
              }),
            ],
          }),
          Q ? q.jsx("p", { role: "alert", children: Q }) : null,
          q.jsxs("div", {
            className: "profileActions",
            children: [
              q.jsx("button", {
                disabled: T.isPending,
                type: "button",
                onClick: N,
                children: "Save profile",
              }),
              q.jsx("button", {
                disabled: C.isPending,
                type: "button",
                onClick: () => C.mutate(),
                children: "Delete profile",
              }),
            ],
          }),
        ],
      }),
    ],
  });
}
function Km(i) {
  return Object.entries(i).map(([f, o]) => `${f}=${o}`).join(`
`);
}
function fd(i) {
  return i
    ? i
        .split(`
`)
        .reduce((f, o, r) => {
          if (!o.trim())
            throw new Error(`Speaker Library profile metadata line ${r + 1} must be key=value.`);
          const g = o.indexOf("=");
          if (g <= 0 || g === o.length - 1)
            throw new Error(`Speaker Library profile metadata line ${r + 1} must be key=value.`);
          const M = o.slice(0, g).trim(),
            U = o.slice(g + 1).trim();
          if (!M || !U)
            throw new Error(`Speaker Library profile metadata line ${r + 1} must be key=value.`);
          return ((f[M] = U), f);
        }, {})
    : {};
}
kv.createRoot(document.getElementById("root")).render(
  q.jsx(Tt.StrictMode, { children: q.jsx(Gm, {}) }),
);
