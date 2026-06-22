(function () {
  const f = document.createElement("link").relList;
  if (f && f.supports && f.supports("modulepreload")) return;
  for (const p of document.querySelectorAll('link[rel="modulepreload"]')) r(p);
  new MutationObserver((p) => {
    for (const M of p)
      if (M.type === "childList")
        for (const C of M.addedNodes) C.tagName === "LINK" && C.rel === "modulepreload" && r(C);
  }).observe(document, { childList: !0, subtree: !0 });
  function o(p) {
    const M = {};
    return (
      p.integrity && (M.integrity = p.integrity),
      p.referrerPolicy && (M.referrerPolicy = p.referrerPolicy),
      p.crossOrigin === "use-credentials"
        ? (M.credentials = "include")
        : p.crossOrigin === "anonymous"
          ? (M.credentials = "omit")
          : (M.credentials = "same-origin"),
      M
    );
  }
  function r(p) {
    if (p.ep) return;
    p.ep = !0;
    const M = o(p);
    fetch(p.href, M);
  }
})();
var pf = { exports: {} },
  Mu = {};
var Lh;
function Zm() {
  if (Lh) return Mu;
  Lh = 1;
  var i = Symbol.for("react.transitional.element"),
    f = Symbol.for("react.fragment");
  function o(r, p, M) {
    var C = null;
    if ((M !== void 0 && (C = "" + M), p.key !== void 0 && (C = "" + p.key), "key" in p)) {
      M = {};
      for (var q in p) q !== "key" && (M[q] = p[q]);
    } else M = p;
    return ((p = M.ref), { $$typeof: i, type: r, key: C, ref: p !== void 0 ? p : null, props: M });
  }
  return ((Mu.Fragment = f), (Mu.jsx = o), (Mu.jsxs = o), Mu);
}
var Zh;
function Km() {
  return (Zh || ((Zh = 1), (pf.exports = Zm())), pf.exports);
}
var O = Km(),
  bf = { exports: {} },
  V = {};
var Kh;
function Vm() {
  if (Kh) return V;
  Kh = 1;
  var i = Symbol.for("react.transitional.element"),
    f = Symbol.for("react.portal"),
    o = Symbol.for("react.fragment"),
    r = Symbol.for("react.strict_mode"),
    p = Symbol.for("react.profiler"),
    M = Symbol.for("react.consumer"),
    C = Symbol.for("react.context"),
    q = Symbol.for("react.forward_ref"),
    A = Symbol.for("react.suspense"),
    E = Symbol.for("react.memo"),
    j = Symbol.for("react.lazy"),
    N = Symbol.for("react.activity"),
    U = Symbol.iterator;
  function lt(y) {
    return y === null || typeof y != "object"
      ? null
      : ((y = (U && y[U]) || y["@@iterator"]), typeof y == "function" ? y : null);
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
  function st(y, R, H) {
    ((this.props = y), (this.context = R), (this.refs = St), (this.updater = H || W));
  }
  ((st.prototype.isReactComponent = {}),
    (st.prototype.setState = function (y, R) {
      if (typeof y != "object" && typeof y != "function" && y != null)
        throw Error(
          "takes an object of state variables to update or a function which returns an object of state variables.",
        );
      this.updater.enqueueSetState(this, y, R, "setState");
    }),
    (st.prototype.forceUpdate = function (y) {
      this.updater.enqueueForceUpdate(this, y, "forceUpdate");
    }));
  function Dt() {}
  Dt.prototype = st.prototype;
  function gt(y, R, H) {
    ((this.props = y), (this.context = R), (this.refs = St), (this.updater = H || W));
  }
  var Rt = (gt.prototype = new Dt());
  ((Rt.constructor = gt), Z(Rt, st.prototype), (Rt.isPureReactComponent = !0));
  var Qt = Array.isArray;
  function Bt() {}
  var K = { H: null, A: null, T: null, S: null },
    yt = Object.prototype.hasOwnProperty;
  function Jt(y, R, H) {
    var B = H.ref;
    return { $$typeof: i, type: y, key: R, ref: B !== void 0 ? B : null, props: H };
  }
  function _e(y, R) {
    return Jt(y.type, R, y.props);
  }
  function ue(y) {
    return typeof y == "object" && y !== null && y.$$typeof === i;
  }
  function xt(y) {
    var R = { "=": "=0", ":": "=2" };
    return (
      "$" +
      y.replace(/[=:]/g, function (H) {
        return R[H];
      })
    );
  }
  var ve = /\/+/g;
  function qe(y, R) {
    return typeof y == "object" && y !== null && y.key != null ? xt("" + y.key) : R.toString(36);
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
                function (R) {
                  y.status === "pending" && ((y.status = "fulfilled"), (y.value = R));
                },
                function (R) {
                  y.status === "pending" && ((y.status = "rejected"), (y.reason = R));
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
  function z(y, R, H, B, J) {
    var k = typeof y;
    (k === "undefined" || k === "boolean") && (y = null);
    var it = !1;
    if (y === null) it = !0;
    else
      switch (k) {
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
            case j:
              return ((it = y._init), z(it(y._payload), R, H, B, J));
          }
      }
    if (it)
      return (
        (J = J(y)),
        (it = B === "" ? "." + qe(y, 0) : B),
        Qt(J)
          ? ((H = ""),
            it != null && (H = it.replace(ve, "$&/") + "/"),
            z(J, R, H, "", function (Na) {
              return Na;
            }))
          : J != null &&
            (ue(J) &&
              (J = _e(
                J,
                H +
                  (J.key == null || (y && y.key === J.key)
                    ? ""
                    : ("" + J.key).replace(ve, "$&/") + "/") +
                  it,
              )),
            R.push(J)),
        1
      );
    it = 0;
    var Ft = B === "" ? "." : B + ":";
    if (Qt(y))
      for (var At = 0; At < y.length; At++)
        ((B = y[At]), (k = Ft + qe(B, At)), (it += z(B, R, H, k, J)));
    else if (((At = lt(y)), typeof At == "function"))
      for (y = At.call(y), At = 0; !(B = y.next()).done; )
        ((B = B.value), (k = Ft + qe(B, At++)), (it += z(B, R, H, k, J)));
    else if (k === "object") {
      if (typeof y.then == "function") return z(De(y), R, H, B, J);
      throw (
        (R = String(y)),
        Error(
          "Objects are not valid as a React child (found: " +
            (R === "[object Object]" ? "object with keys {" + Object.keys(y).join(", ") + "}" : R) +
            "). If you meant to render a collection of children, use an array instead.",
        )
      );
    }
    return it;
  }
  function x(y, R, H) {
    if (y == null) return y;
    var B = [],
      J = 0;
    return (
      z(y, B, "", "", function (k) {
        return R.call(H, k, J++);
      }),
      B
    );
  }
  function L(y) {
    if (y._status === -1) {
      var R = y._result;
      ((R = R()),
        R.then(
          function (H) {
            (y._status === 0 || y._status === -1) && ((y._status = 1), (y._result = H));
          },
          function (H) {
            (y._status === 0 || y._status === -1) && ((y._status = 2), (y._result = H));
          },
        ),
        y._status === -1 && ((y._status = 0), (y._result = R)));
    }
    if (y._status === 1) return y._result.default;
    throw y._result;
  }
  var rt =
      typeof reportError == "function"
        ? reportError
        : function (y) {
            if (typeof window == "object" && typeof window.ErrorEvent == "function") {
              var R = new window.ErrorEvent("error", {
                bubbles: !0,
                cancelable: !0,
                message:
                  typeof y == "object" && y !== null && typeof y.message == "string"
                    ? String(y.message)
                    : String(y),
                error: y,
              });
              if (!window.dispatchEvent(R)) return;
            } else if (typeof process == "object" && typeof process.emit == "function") {
              process.emit("uncaughtException", y);
              return;
            }
            console.error(y);
          },
    mt = {
      map: x,
      forEach: function (y, R, H) {
        x(
          y,
          function () {
            R.apply(this, arguments);
          },
          H,
        );
      },
      count: function (y) {
        var R = 0;
        return (
          x(y, function () {
            R++;
          }),
          R
        );
      },
      toArray: function (y) {
        return (
          x(y, function (R) {
            return R;
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
    (V.Children = mt),
    (V.Component = st),
    (V.Fragment = o),
    (V.Profiler = p),
    (V.PureComponent = gt),
    (V.StrictMode = r),
    (V.Suspense = A),
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
    (V.cloneElement = function (y, R, H) {
      if (y == null) throw Error("The argument must be a React element, but you passed " + y + ".");
      var B = Z({}, y.props),
        J = y.key;
      if (R != null)
        for (k in (R.key !== void 0 && (J = "" + R.key), R))
          !yt.call(R, k) ||
            k === "key" ||
            k === "__self" ||
            k === "__source" ||
            (k === "ref" && R.ref === void 0) ||
            (B[k] = R[k]);
      var k = arguments.length - 2;
      if (k === 1) B.children = H;
      else if (1 < k) {
        for (var it = Array(k), Ft = 0; Ft < k; Ft++) it[Ft] = arguments[Ft + 2];
        B.children = it;
      }
      return Jt(y.type, J, B);
    }),
    (V.createContext = function (y) {
      return (
        (y = {
          $$typeof: C,
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
    (V.createElement = function (y, R, H) {
      var B,
        J = {},
        k = null;
      if (R != null)
        for (B in (R.key !== void 0 && (k = "" + R.key), R))
          yt.call(R, B) && B !== "key" && B !== "__self" && B !== "__source" && (J[B] = R[B]);
      var it = arguments.length - 2;
      if (it === 1) J.children = H;
      else if (1 < it) {
        for (var Ft = Array(it), At = 0; At < it; At++) Ft[At] = arguments[At + 2];
        J.children = Ft;
      }
      if (y && y.defaultProps)
        for (B in ((it = y.defaultProps), it)) J[B] === void 0 && (J[B] = it[B]);
      return Jt(y, k, J);
    }),
    (V.createRef = function () {
      return { current: null };
    }),
    (V.forwardRef = function (y) {
      return { $$typeof: q, render: y };
    }),
    (V.isValidElement = ue),
    (V.lazy = function (y) {
      return { $$typeof: j, _payload: { _status: -1, _result: y }, _init: L };
    }),
    (V.memo = function (y, R) {
      return { $$typeof: E, type: y, compare: R === void 0 ? null : R };
    }),
    (V.startTransition = function (y) {
      var R = K.T,
        H = {};
      K.T = H;
      try {
        var B = y(),
          J = K.S;
        (J !== null && J(H, B),
          typeof B == "object" && B !== null && typeof B.then == "function" && B.then(Bt, rt));
      } catch (k) {
        rt(k);
      } finally {
        (R !== null && H.types !== null && (R.types = H.types), (K.T = R));
      }
    }),
    (V.unstable_useCacheRefresh = function () {
      return K.H.useCacheRefresh();
    }),
    (V.use = function (y) {
      return K.H.use(y);
    }),
    (V.useActionState = function (y, R, H) {
      return K.H.useActionState(y, R, H);
    }),
    (V.useCallback = function (y, R) {
      return K.H.useCallback(y, R);
    }),
    (V.useContext = function (y) {
      return K.H.useContext(y);
    }),
    (V.useDebugValue = function () {}),
    (V.useDeferredValue = function (y, R) {
      return K.H.useDeferredValue(y, R);
    }),
    (V.useEffect = function (y, R) {
      return K.H.useEffect(y, R);
    }),
    (V.useEffectEvent = function (y) {
      return K.H.useEffectEvent(y);
    }),
    (V.useId = function () {
      return K.H.useId();
    }),
    (V.useImperativeHandle = function (y, R, H) {
      return K.H.useImperativeHandle(y, R, H);
    }),
    (V.useInsertionEffect = function (y, R) {
      return K.H.useInsertionEffect(y, R);
    }),
    (V.useLayoutEffect = function (y, R) {
      return K.H.useLayoutEffect(y, R);
    }),
    (V.useMemo = function (y, R) {
      return K.H.useMemo(y, R);
    }),
    (V.useOptimistic = function (y, R) {
      return K.H.useOptimistic(y, R);
    }),
    (V.useReducer = function (y, R, H) {
      return K.H.useReducer(y, R, H);
    }),
    (V.useRef = function (y) {
      return K.H.useRef(y);
    }),
    (V.useState = function (y) {
      return K.H.useState(y);
    }),
    (V.useSyncExternalStore = function (y, R, H) {
      return K.H.useSyncExternalStore(y, R, H);
    }),
    (V.useTransition = function () {
      return K.H.useTransition();
    }),
    (V.version = "19.2.7"),
    V
  );
}
var Vh;
function xf() {
  return (Vh || ((Vh = 1), (bf.exports = Vm())), bf.exports);
}
var Et = xf(),
  Ef = { exports: {} },
  _u = {},
  Tf = { exports: {} },
  Of = {};
var Jh;
function Jm() {
  return (
    Jh ||
      ((Jh = 1),
      (function (i) {
        function f(z, x) {
          var L = z.length;
          z.push(x);
          t: for (; 0 < L; ) {
            var rt = (L - 1) >>> 1,
              mt = z[rt];
            if (0 < p(mt, x)) ((z[rt] = x), (z[L] = mt), (L = rt));
            else break t;
          }
        }
        function o(z) {
          return z.length === 0 ? null : z[0];
        }
        function r(z) {
          if (z.length === 0) return null;
          var x = z[0],
            L = z.pop();
          if (L !== x) {
            z[0] = L;
            t: for (var rt = 0, mt = z.length, y = mt >>> 1; rt < y; ) {
              var R = 2 * (rt + 1) - 1,
                H = z[R],
                B = R + 1,
                J = z[B];
              if (0 > p(H, L))
                B < mt && 0 > p(J, H)
                  ? ((z[rt] = J), (z[B] = L), (rt = B))
                  : ((z[rt] = H), (z[R] = L), (rt = R));
              else if (B < mt && 0 > p(J, L)) ((z[rt] = J), (z[B] = L), (rt = B));
              else break t;
            }
          }
          return x;
        }
        function p(z, x) {
          var L = z.sortIndex - x.sortIndex;
          return L !== 0 ? L : z.id - x.id;
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
          var C = Date,
            q = C.now();
          i.unstable_now = function () {
            return C.now() - q;
          };
        }
        var A = [],
          E = [],
          j = 1,
          N = null,
          U = 3,
          lt = !1,
          W = !1,
          Z = !1,
          St = !1,
          st = typeof setTimeout == "function" ? setTimeout : null,
          Dt = typeof clearTimeout == "function" ? clearTimeout : null,
          gt = typeof setImmediate < "u" ? setImmediate : null;
        function Rt(z) {
          for (var x = o(E); x !== null; ) {
            if (x.callback === null) r(E);
            else if (x.startTime <= z) (r(E), (x.sortIndex = x.expirationTime), f(A, x));
            else break;
            x = o(E);
          }
        }
        function Qt(z) {
          if (((Z = !1), Rt(z), !W))
            if (o(A) !== null) ((W = !0), Bt || ((Bt = !0), xt()));
            else {
              var x = o(E);
              x !== null && De(Qt, x.startTime - z);
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
            var z = i.unstable_now();
            Jt = z;
            var x = !0;
            try {
              t: {
                ((W = !1), Z && ((Z = !1), Dt(K), (K = -1)), (lt = !0));
                var L = U;
                try {
                  e: {
                    for (Rt(z), N = o(A); N !== null && !(N.expirationTime > z && _e()); ) {
                      var rt = N.callback;
                      if (typeof rt == "function") {
                        ((N.callback = null), (U = N.priorityLevel));
                        var mt = rt(N.expirationTime <= z);
                        if (((z = i.unstable_now()), typeof mt == "function")) {
                          ((N.callback = mt), Rt(z), (x = !0));
                          break e;
                        }
                        (N === o(A) && r(A), Rt(z));
                      } else r(A);
                      N = o(A);
                    }
                    if (N !== null) x = !0;
                    else {
                      var y = o(E);
                      (y !== null && De(Qt, y.startTime - z), (x = !1));
                    }
                  }
                  break t;
                } finally {
                  ((N = null), (U = L), (lt = !1));
                }
                x = void 0;
              }
            } finally {
              x ? xt() : (Bt = !1);
            }
          }
        }
        var xt;
        if (typeof gt == "function")
          xt = function () {
            gt(ue);
          };
        else if (typeof MessageChannel < "u") {
          var ve = new MessageChannel(),
            qe = ve.port2;
          ((ve.port1.onmessage = ue),
            (xt = function () {
              qe.postMessage(null);
            }));
        } else
          xt = function () {
            st(ue, 0);
          };
        function De(z, x) {
          K = st(function () {
            z(i.unstable_now());
          }, x);
        }
        ((i.unstable_IdlePriority = 5),
          (i.unstable_ImmediatePriority = 1),
          (i.unstable_LowPriority = 4),
          (i.unstable_NormalPriority = 3),
          (i.unstable_Profiling = null),
          (i.unstable_UserBlockingPriority = 2),
          (i.unstable_cancelCallback = function (z) {
            z.callback = null;
          }),
          (i.unstable_forceFrameRate = function (z) {
            0 > z || 125 < z
              ? console.error(
                  "forceFrameRate takes a positive int between 0 and 125, forcing frame rates higher than 125 fps is not supported",
                )
              : (yt = 0 < z ? Math.floor(1e3 / z) : 5);
          }),
          (i.unstable_getCurrentPriorityLevel = function () {
            return U;
          }),
          (i.unstable_next = function (z) {
            switch (U) {
              case 1:
              case 2:
              case 3:
                var x = 3;
                break;
              default:
                x = U;
            }
            var L = U;
            U = x;
            try {
              return z();
            } finally {
              U = L;
            }
          }),
          (i.unstable_requestPaint = function () {
            St = !0;
          }),
          (i.unstable_runWithPriority = function (z, x) {
            switch (z) {
              case 1:
              case 2:
              case 3:
              case 4:
              case 5:
                break;
              default:
                z = 3;
            }
            var L = U;
            U = z;
            try {
              return x();
            } finally {
              U = L;
            }
          }),
          (i.unstable_scheduleCallback = function (z, x, L) {
            var rt = i.unstable_now();
            switch (
              (typeof L == "object" && L !== null
                ? ((L = L.delay), (L = typeof L == "number" && 0 < L ? rt + L : rt))
                : (L = rt),
              z)
            ) {
              case 1:
                var mt = -1;
                break;
              case 2:
                mt = 250;
                break;
              case 5:
                mt = 1073741823;
                break;
              case 4:
                mt = 1e4;
                break;
              default:
                mt = 5e3;
            }
            return (
              (mt = L + mt),
              (z = {
                id: j++,
                callback: x,
                priorityLevel: z,
                startTime: L,
                expirationTime: mt,
                sortIndex: -1,
              }),
              L > rt
                ? ((z.sortIndex = L),
                  f(E, z),
                  o(A) === null && z === o(E) && (Z ? (Dt(K), (K = -1)) : (Z = !0), De(Qt, L - rt)))
                : ((z.sortIndex = mt), f(A, z), W || lt || ((W = !0), Bt || ((Bt = !0), xt()))),
              z
            );
          }),
          (i.unstable_shouldYield = _e),
          (i.unstable_wrapCallback = function (z) {
            var x = U;
            return function () {
              var L = U;
              U = x;
              try {
                return z.apply(this, arguments);
              } finally {
                U = L;
              }
            };
          }));
      })(Of)),
    Of
  );
}
var wh;
function wm() {
  return (wh || ((wh = 1), (Tf.exports = Jm())), Tf.exports);
}
var zf = { exports: {} },
  wt = {};
var Fh;
function Fm() {
  if (Fh) return wt;
  Fh = 1;
  var i = xf();
  function f(A) {
    var E = "https://react.dev/errors/" + A;
    if (1 < arguments.length) {
      E += "?args[]=" + encodeURIComponent(arguments[1]);
      for (var j = 2; j < arguments.length; j++) E += "&args[]=" + encodeURIComponent(arguments[j]);
    }
    return (
      "Minified React error #" +
      A +
      "; visit " +
      E +
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
    p = Symbol.for("react.portal");
  function M(A, E, j) {
    var N = 3 < arguments.length && arguments[3] !== void 0 ? arguments[3] : null;
    return {
      $$typeof: p,
      key: N == null ? null : "" + N,
      children: A,
      containerInfo: E,
      implementation: j,
    };
  }
  var C = i.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE;
  function q(A, E) {
    if (A === "font") return "";
    if (typeof E == "string") return E === "use-credentials" ? E : "";
  }
  return (
    (wt.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE = r),
    (wt.createPortal = function (A, E) {
      var j = 2 < arguments.length && arguments[2] !== void 0 ? arguments[2] : null;
      if (!E || (E.nodeType !== 1 && E.nodeType !== 9 && E.nodeType !== 11)) throw Error(f(299));
      return M(A, E, null, j);
    }),
    (wt.flushSync = function (A) {
      var E = C.T,
        j = r.p;
      try {
        if (((C.T = null), (r.p = 2), A)) return A();
      } finally {
        ((C.T = E), (r.p = j), r.d.f());
      }
    }),
    (wt.preconnect = function (A, E) {
      typeof A == "string" &&
        (E
          ? ((E = E.crossOrigin),
            (E = typeof E == "string" ? (E === "use-credentials" ? E : "") : void 0))
          : (E = null),
        r.d.C(A, E));
    }),
    (wt.prefetchDNS = function (A) {
      typeof A == "string" && r.d.D(A);
    }),
    (wt.preinit = function (A, E) {
      if (typeof A == "string" && E && typeof E.as == "string") {
        var j = E.as,
          N = q(j, E.crossOrigin),
          U = typeof E.integrity == "string" ? E.integrity : void 0,
          lt = typeof E.fetchPriority == "string" ? E.fetchPriority : void 0;
        j === "style"
          ? r.d.S(A, typeof E.precedence == "string" ? E.precedence : void 0, {
              crossOrigin: N,
              integrity: U,
              fetchPriority: lt,
            })
          : j === "script" &&
            r.d.X(A, {
              crossOrigin: N,
              integrity: U,
              fetchPriority: lt,
              nonce: typeof E.nonce == "string" ? E.nonce : void 0,
            });
      }
    }),
    (wt.preinitModule = function (A, E) {
      if (typeof A == "string")
        if (typeof E == "object" && E !== null) {
          if (E.as == null || E.as === "script") {
            var j = q(E.as, E.crossOrigin);
            r.d.M(A, {
              crossOrigin: j,
              integrity: typeof E.integrity == "string" ? E.integrity : void 0,
              nonce: typeof E.nonce == "string" ? E.nonce : void 0,
            });
          }
        } else E == null && r.d.M(A);
    }),
    (wt.preload = function (A, E) {
      if (typeof A == "string" && typeof E == "object" && E !== null && typeof E.as == "string") {
        var j = E.as,
          N = q(j, E.crossOrigin);
        r.d.L(A, j, {
          crossOrigin: N,
          integrity: typeof E.integrity == "string" ? E.integrity : void 0,
          nonce: typeof E.nonce == "string" ? E.nonce : void 0,
          type: typeof E.type == "string" ? E.type : void 0,
          fetchPriority: typeof E.fetchPriority == "string" ? E.fetchPriority : void 0,
          referrerPolicy: typeof E.referrerPolicy == "string" ? E.referrerPolicy : void 0,
          imageSrcSet: typeof E.imageSrcSet == "string" ? E.imageSrcSet : void 0,
          imageSizes: typeof E.imageSizes == "string" ? E.imageSizes : void 0,
          media: typeof E.media == "string" ? E.media : void 0,
        });
      }
    }),
    (wt.preloadModule = function (A, E) {
      if (typeof A == "string")
        if (E) {
          var j = q(E.as, E.crossOrigin);
          r.d.m(A, {
            as: typeof E.as == "string" && E.as !== "script" ? E.as : void 0,
            crossOrigin: j,
            integrity: typeof E.integrity == "string" ? E.integrity : void 0,
          });
        } else r.d.m(A);
    }),
    (wt.requestFormReset = function (A) {
      r.d.r(A);
    }),
    (wt.unstable_batchedUpdates = function (A, E) {
      return A(E);
    }),
    (wt.useFormState = function (A, E, j) {
      return C.H.useFormState(A, E, j);
    }),
    (wt.useFormStatus = function () {
      return C.H.useHostTransitionStatus();
    }),
    (wt.version = "19.2.7"),
    wt
  );
}
var Wh;
function Wm() {
  if (Wh) return zf.exports;
  Wh = 1;
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
  return (i(), (zf.exports = Fm()), zf.exports);
}
var kh;
function km() {
  if (kh) return _u;
  kh = 1;
  var i = wm(),
    f = xf(),
    o = Wm();
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
  function p(t) {
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
  function C(t) {
    if (t.tag === 13) {
      var e = t.memoizedState;
      if ((e === null && ((t = t.alternate), t !== null && (e = t.memoizedState)), e !== null))
        return e.dehydrated;
    }
    return null;
  }
  function q(t) {
    if (t.tag === 31) {
      var e = t.memoizedState;
      if ((e === null && ((t = t.alternate), t !== null && (e = t.memoizedState)), e !== null))
        return e.dehydrated;
    }
    return null;
  }
  function A(t) {
    if (M(t) !== t) throw Error(r(188));
  }
  function E(t) {
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
          if (n === l) return (A(u), t);
          if (n === a) return (A(u), e);
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
  function j(t) {
    var e = t.tag;
    if (e === 5 || e === 26 || e === 27 || e === 6) return t;
    for (t = t.child; t !== null; ) {
      if (((e = j(t)), e !== null)) return e;
      t = t.sibling;
    }
    return null;
  }
  var N = Object.assign,
    U = Symbol.for("react.element"),
    lt = Symbol.for("react.transitional.element"),
    W = Symbol.for("react.portal"),
    Z = Symbol.for("react.fragment"),
    St = Symbol.for("react.strict_mode"),
    st = Symbol.for("react.profiler"),
    Dt = Symbol.for("react.consumer"),
    gt = Symbol.for("react.context"),
    Rt = Symbol.for("react.forward_ref"),
    Qt = Symbol.for("react.suspense"),
    Bt = Symbol.for("react.suspense_list"),
    K = Symbol.for("react.memo"),
    yt = Symbol.for("react.lazy"),
    Jt = Symbol.for("react.activity"),
    _e = Symbol.for("react.memo_cache_sentinel"),
    ue = Symbol.iterator;
  function xt(t) {
    return t === null || typeof t != "object"
      ? null
      : ((t = (ue && t[ue]) || t["@@iterator"]), typeof t == "function" ? t : null);
  }
  var ve = Symbol.for("react.client.reference");
  function qe(t) {
    if (t == null) return null;
    if (typeof t == "function") return t.$$typeof === ve ? null : t.displayName || t.name || null;
    if (typeof t == "string") return t;
    switch (t) {
      case Z:
        return "Fragment";
      case st:
        return "Profiler";
      case St:
        return "StrictMode";
      case Qt:
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
        case Rt:
          var e = t.render;
          return (
            (t = t.displayName),
            t ||
              ((t = e.displayName || e.name || ""),
              (t = t !== "" ? "ForwardRef(" + t + ")" : "ForwardRef")),
            t
          );
        case K:
          return ((e = t.displayName || null), e !== null ? e : qe(t.type) || "Memo");
        case yt:
          ((e = t._payload), (t = t._init));
          try {
            return qe(t(e));
          } catch {}
      }
    return null;
  }
  var De = Array.isArray,
    z = f.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,
    x = o.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,
    L = { pending: !1, data: null, method: null, action: null },
    rt = [],
    mt = -1;
  function y(t) {
    return { current: t };
  }
  function R(t) {
    0 > mt || ((t.current = rt[mt]), (rt[mt] = null), mt--);
  }
  function H(t, e) {
    (mt++, (rt[mt] = t.current), (t.current = e));
  }
  var B = y(null),
    J = y(null),
    k = y(null),
    it = y(null);
  function Ft(t, e) {
    switch ((H(k, e), H(J, t), H(B, null), e.nodeType)) {
      case 9:
      case 11:
        t = (t = e.documentElement) && (t = t.namespaceURI) ? oh(t) : 0;
        break;
      default:
        if (((t = e.tagName), (e = e.namespaceURI))) ((e = oh(e)), (t = hh(e, t)));
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
    (R(B), H(B, t));
  }
  function At() {
    (R(B), R(J), R(k));
  }
  function Na(t) {
    t.memoizedState !== null && H(it, t);
    var e = B.current,
      l = hh(e, t.type);
    e !== l && (H(J, t), H(B, l));
  }
  function Cu(t) {
    (J.current === t && (R(B), R(J)), it.current === t && (R(it), (Tu._currentValue = L)));
  }
  var ti, Gf;
  function _l(t) {
    if (ti === void 0)
      try {
        throw Error();
      } catch (l) {
        var e = l.stack.trim().match(/\n( *(at )?)/);
        ((ti = (e && e[1]) || ""),
          (Gf =
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
      ti +
      t +
      Gf
    );
  }
  var ei = !1;
  function li(t, e) {
    if (!t || ei) return "";
    ei = !0;
    var l = Error.prepareStackTrace;
    Error.prepareStackTrace = void 0;
    try {
      var a = {
        DetermineComponentFrameRoot: function () {
          try {
            if (e) {
              var D = function () {
                throw Error();
              };
              if (
                (Object.defineProperty(D.prototype, "props", {
                  set: function () {
                    throw Error();
                  },
                }),
                typeof Reflect == "object" && Reflect.construct)
              ) {
                try {
                  Reflect.construct(D, []);
                } catch (b) {
                  var S = b;
                }
                Reflect.construct(t, [], D);
              } else {
                try {
                  D.call();
                } catch (b) {
                  S = b;
                }
                t.call(D.prototype);
              }
            } else {
              try {
                throw Error();
              } catch (b) {
                S = b;
              }
              (D = t()) && typeof D.catch == "function" && D.catch(function () {});
            }
          } catch (b) {
            if (b && S && typeof b.stack == "string") return [b.stack, S.stack];
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
          g = s.split(`
`);
        for (u = a = 0; a < h.length && !h[a].includes("DetermineComponentFrameRoot"); ) a++;
        for (; u < g.length && !g[u].includes("DetermineComponentFrameRoot"); ) u++;
        if (a === h.length || u === g.length)
          for (a = h.length - 1, u = g.length - 1; 1 <= a && 0 <= u && h[a] !== g[u]; ) u--;
        for (; 1 <= a && 0 <= u; a--, u--)
          if (h[a] !== g[u]) {
            if (a !== 1 || u !== 1)
              do
                if ((a--, u--, 0 > u || h[a] !== g[u])) {
                  var T =
                    `
` + h[a].replace(" at new ", " at ");
                  return (
                    t.displayName &&
                      T.includes("<anonymous>") &&
                      (T = T.replace("<anonymous>", t.displayName)),
                    T
                  );
                }
              while (1 <= a && 0 <= u);
            break;
          }
      }
    } finally {
      ((ei = !1), (Error.prepareStackTrace = l));
    }
    return (l = t ? t.displayName || t.name : "") ? _l(l) : "";
  }
  function bd(t, e) {
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
        return li(t.type, !1);
      case 11:
        return li(t.type.render, !1);
      case 1:
        return li(t.type, !0);
      case 31:
        return _l("Activity");
      default:
        return "";
    }
  }
  function Xf(t) {
    try {
      var e = "",
        l = null;
      do ((e += bd(t, l)), (l = t), (t = t.return));
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
  var ai = Object.prototype.hasOwnProperty,
    ui = i.unstable_scheduleCallback,
    ni = i.unstable_cancelCallback,
    Ed = i.unstable_shouldYield,
    Td = i.unstable_requestPaint,
    ne = i.unstable_now,
    Od = i.unstable_getCurrentPriorityLevel,
    Lf = i.unstable_ImmediatePriority,
    Zf = i.unstable_UserBlockingPriority,
    ju = i.unstable_NormalPriority,
    zd = i.unstable_LowPriority,
    Kf = i.unstable_IdlePriority,
    Ad = i.log,
    Md = i.unstable_setDisableYieldValue,
    xa = null,
    ie = null;
  function el(t) {
    if ((typeof Ad == "function" && Md(t), ie && typeof ie.setStrictMode == "function"))
      try {
        ie.setStrictMode(xa, t);
      } catch {}
  }
  var ce = Math.clz32 ? Math.clz32 : Rd,
    _d = Math.log,
    Dd = Math.LN2;
  function Rd(t) {
    return ((t >>>= 0), t === 0 ? 32 : (31 - ((_d(t) / Dd) | 0)) | 0);
  }
  var Nu = 256,
    xu = 262144,
    Hu = 4194304;
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
  function qu(t, e, l) {
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
  function Ha(t, e) {
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
  function Vf() {
    var t = Hu;
    return ((Hu <<= 1), (Hu & 62914560) === 0 && (Hu = 4194304), t);
  }
  function ii(t) {
    for (var e = [], l = 0; 31 > l; l++) e.push(t);
    return e;
  }
  function qa(t, e) {
    ((t.pendingLanes |= e),
      e !== 268435456 && ((t.suspendedLanes = 0), (t.pingedLanes = 0), (t.warmLanes = 0)));
  }
  function Cd(t, e, l, a, u, n) {
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
      g = t.hiddenUpdates;
    for (l = c & ~l; 0 < l; ) {
      var T = 31 - ce(l),
        D = 1 << T;
      ((s[T] = 0), (h[T] = -1));
      var S = g[T];
      if (S !== null)
        for (g[T] = null, T = 0; T < S.length; T++) {
          var b = S[T];
          b !== null && (b.lane &= -536870913);
        }
      l &= ~D;
    }
    (a !== 0 && Jf(t, a, 0),
      n !== 0 && u === 0 && t.tag !== 0 && (t.suspendedLanes |= n & ~(c & ~e)));
  }
  function Jf(t, e, l) {
    ((t.pendingLanes |= e), (t.suspendedLanes &= ~e));
    var a = 31 - ce(e);
    ((t.entangledLanes |= e),
      (t.entanglements[a] = t.entanglements[a] | 1073741824 | (l & 261930)));
  }
  function wf(t, e) {
    var l = (t.entangledLanes |= e);
    for (t = t.entanglements; l; ) {
      var a = 31 - ce(l),
        u = 1 << a;
      ((u & e) | (t[a] & e) && (t[a] |= e), (l &= ~u));
    }
  }
  function Ff(t, e) {
    var l = e & -e;
    return ((l = (l & 42) !== 0 ? 1 : ci(l)), (l & (t.suspendedLanes | e)) !== 0 ? 0 : l);
  }
  function ci(t) {
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
  function fi(t) {
    return ((t &= -t), 2 < t ? (8 < t ? ((t & 134217727) !== 0 ? 32 : 268435456) : 8) : 2);
  }
  function Wf() {
    var t = x.p;
    return t !== 0 ? t : ((t = window.event), t === void 0 ? 32 : Hh(t.type));
  }
  function kf(t, e) {
    var l = x.p;
    try {
      return ((x.p = t), e());
    } finally {
      x.p = l;
    }
  }
  var ll = Math.random().toString(36).slice(2),
    Xt = "__reactFiber$" + ll,
    $t = "__reactProps$" + ll,
    Fl = "__reactContainer$" + ll,
    si = "__reactEvents$" + ll,
    jd = "__reactListeners$" + ll,
    Nd = "__reactHandles$" + ll,
    $f = "__reactResources$" + ll,
    Qa = "__reactMarker$" + ll;
  function ri(t) {
    (delete t[Xt], delete t[$t], delete t[si], delete t[jd], delete t[Nd]);
  }
  function Wl(t) {
    var e = t[Xt];
    if (e) return e;
    for (var l = t.parentNode; l; ) {
      if ((e = l[Fl] || l[Xt])) {
        if (((l = e.alternate), e.child !== null || (l !== null && l.child !== null)))
          for (t = ph(t); t !== null; ) {
            if ((l = t[Xt])) return l;
            t = ph(t);
          }
        return e;
      }
      ((t = l), (l = t.parentNode));
    }
    return null;
  }
  function kl(t) {
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
  function $l(t) {
    var e = t[$f];
    return (e || (e = t[$f] = { hoistableStyles: new Map(), hoistableScripts: new Map() }), e);
  }
  function Yt(t) {
    t[Qa] = !0;
  }
  var If = new Set(),
    Pf = {};
  function Rl(t, e) {
    (Il(t, e), Il(t + "Capture", e));
  }
  function Il(t, e) {
    for (Pf[t] = e, t = 0; t < e.length; t++) If.add(e[t]);
  }
  var xd = RegExp(
      "^[:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD][:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040]*$",
    ),
    ts = {},
    es = {};
  function Hd(t) {
    return ai.call(es, t)
      ? !0
      : ai.call(ts, t)
        ? !1
        : xd.test(t)
          ? (es[t] = !0)
          : ((ts[t] = !0), !1);
  }
  function Qu(t, e, l) {
    if (Hd(e))
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
  function Qe(t, e, l, a) {
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
  function ls(t) {
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
  function oi(t) {
    if (!t._valueTracker) {
      var e = ls(t) ? "checked" : "value";
      t._valueTracker = qd(t, e, "" + t[e]);
    }
  }
  function as(t) {
    if (!t) return !1;
    var e = t._valueTracker;
    if (!e) return !0;
    var l = e.getValue(),
      a = "";
    return (
      t && (a = ls(t) ? (t.checked ? "true" : "false") : t.value),
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
  function hi(t, e, l, a, u, n, c, s) {
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
        ? di(t, c, ge(e))
        : l != null
          ? di(t, c, ge(l))
          : a != null && t.removeAttribute("value"),
      u == null && n != null && (t.defaultChecked = !!n),
      u != null && (t.checked = u && typeof u != "function" && typeof u != "symbol"),
      s != null && typeof s != "function" && typeof s != "symbol" && typeof s != "boolean"
        ? (t.name = "" + ge(s))
        : t.removeAttribute("name"));
  }
  function us(t, e, l, a, u, n, c, s) {
    if (
      (n != null &&
        typeof n != "function" &&
        typeof n != "symbol" &&
        typeof n != "boolean" &&
        (t.type = n),
      e != null || l != null)
    ) {
      if (!((n !== "submit" && n !== "reset") || e != null)) {
        oi(t);
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
      oi(t));
  }
  function di(t, e, l) {
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
  function ns(t, e, l) {
    if (e != null && ((e = "" + ge(e)), e !== t.value && (t.value = e), l == null)) {
      t.defaultValue !== e && (t.defaultValue = e);
      return;
    }
    t.defaultValue = l != null ? "" + ge(l) : "";
  }
  function is(t, e, l, a) {
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
      oi(t));
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
  var Bd = new Set(
    "animationIterationCount aspectRatio borderImageOutset borderImageSlice borderImageWidth boxFlex boxFlexGroup boxOrdinalGroup columnCount columns flex flexGrow flexPositive flexShrink flexNegative flexOrder gridArea gridRow gridRowEnd gridRowSpan gridRowStart gridColumn gridColumnEnd gridColumnSpan gridColumnStart fontWeight lineClamp lineHeight opacity order orphans scale tabSize widows zIndex zoom fillOpacity floodOpacity stopOpacity strokeDasharray strokeDashoffset strokeMiterlimit strokeOpacity strokeWidth MozAnimationIterationCount MozBoxFlex MozBoxFlexGroup MozLineClamp msAnimationIterationCount msFlex msZoom msFlexGrow msFlexNegative msFlexOrder msFlexPositive msFlexShrink msGridColumn msGridColumnSpan msGridRow msGridRowSpan WebkitAnimationIterationCount WebkitBoxFlex WebKitBoxFlexGroup WebkitBoxOrdinalGroup WebkitColumnCount WebkitColumns WebkitFlex WebkitFlexGrow WebkitFlexPositive WebkitFlexShrink WebkitLineClamp".split(
      " ",
    ),
  );
  function cs(t, e, l) {
    var a = e.indexOf("--") === 0;
    l == null || typeof l == "boolean" || l === ""
      ? a
        ? t.setProperty(e, "")
        : e === "float"
          ? (t.cssFloat = "")
          : (t[e] = "")
      : a
        ? t.setProperty(e, l)
        : typeof l != "number" || l === 0 || Bd.has(e)
          ? e === "float"
            ? (t.cssFloat = l)
            : (t[e] = ("" + l).trim())
          : (t[e] = l + "px");
  }
  function fs(t, e, l) {
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
      for (var u in e) ((a = e[u]), e.hasOwnProperty(u) && l[u] !== a && cs(t, u, a));
    } else for (var n in e) e.hasOwnProperty(n) && cs(t, n, e[n]);
  }
  function yi(t) {
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
  var Yd = new Map([
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
    Gd =
      /^[\u0000-\u001F ]*j[\r\n\t]*a[\r\n\t]*v[\r\n\t]*a[\r\n\t]*s[\r\n\t]*c[\r\n\t]*r[\r\n\t]*i[\r\n\t]*p[\r\n\t]*t[\r\n\t]*:/i;
  function Gu(t) {
    return Gd.test("" + t)
      ? "javascript:throw new Error('React has blocked a javascript: URL as a security precaution.')"
      : t;
  }
  function Be() {}
  var mi = null;
  function vi(t) {
    return (
      (t = t.target || t.srcElement || window),
      t.correspondingUseElement && (t = t.correspondingUseElement),
      t.nodeType === 3 ? t.parentNode : t
    );
  }
  var ea = null,
    la = null;
  function ss(t) {
    var e = kl(t);
    if (e && (t = e.stateNode)) {
      var l = t[$t] || null;
      t: switch (((t = e.stateNode), e.type)) {
        case "input":
          if (
            (hi(
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
                var u = a[$t] || null;
                if (!u) throw Error(r(90));
                hi(
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
            for (e = 0; e < l.length; e++) ((a = l[e]), a.form === t.form && as(a));
          }
          break t;
        case "textarea":
          ns(t, l.value, l.defaultValue);
          break t;
        case "select":
          ((e = l.value), e != null && Pl(t, !!l.multiple, e, !1));
      }
    }
  }
  var gi = !1;
  function rs(t, e, l) {
    if (gi) return t(e, l);
    gi = !0;
    try {
      var a = t(e);
      return a;
    } finally {
      if (
        ((gi = !1),
        (ea !== null || la !== null) &&
          (Dn(), ea && ((e = ea), (t = la), (la = ea = null), ss(e), t)))
      )
        for (e = 0; e < t.length; e++) ss(t[e]);
    }
  }
  function Ya(t, e) {
    var l = t.stateNode;
    if (l === null) return null;
    var a = l[$t] || null;
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
    Si = !1;
  if (Ye)
    try {
      var Ga = {};
      (Object.defineProperty(Ga, "passive", {
        get: function () {
          Si = !0;
        },
      }),
        window.addEventListener("test", Ga, Ga),
        window.removeEventListener("test", Ga, Ga));
    } catch {
      Si = !1;
    }
  var al = null,
    pi = null,
    Xu = null;
  function os() {
    if (Xu) return Xu;
    var t,
      e = pi,
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
  function hs() {
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
          : hs),
        (this.isPropagationStopped = hs),
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
  var Ul = {
      eventPhase: 0,
      bubbles: 0,
      cancelable: 0,
      timeStamp: function (t) {
        return t.timeStamp || Date.now();
      },
      defaultPrevented: 0,
      isTrusted: 0,
    },
    Ku = It(Ul),
    Xa = N({}, Ul, { view: 0, detail: 0 }),
    Xd = It(Xa),
    bi,
    Ei,
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
      getModifierState: Oi,
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
                ? ((bi = t.screenX - La.screenX), (Ei = t.screenY - La.screenY))
                : (Ei = bi = 0),
              (La = t)),
            bi);
      },
      movementY: function (t) {
        return "movementY" in t ? t.movementY : Ei;
      },
    }),
    ds = It(Vu),
    Ld = N({}, Vu, { dataTransfer: 0 }),
    Zd = It(Ld),
    Kd = N({}, Xa, { relatedTarget: 0 }),
    Ti = It(Kd),
    Vd = N({}, Ul, { animationName: 0, elapsedTime: 0, pseudoElement: 0 }),
    Jd = It(Vd),
    wd = N({}, Ul, {
      clipboardData: function (t) {
        return "clipboardData" in t ? t.clipboardData : window.clipboardData;
      },
    }),
    Fd = It(wd),
    Wd = N({}, Ul, { data: 0 }),
    ys = It(Wd),
    kd = {
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
    Id = { Alt: "altKey", Control: "ctrlKey", Meta: "metaKey", Shift: "shiftKey" };
  function Pd(t) {
    var e = this.nativeEvent;
    return e.getModifierState ? e.getModifierState(t) : (t = Id[t]) ? !!e[t] : !1;
  }
  function Oi() {
    return Pd;
  }
  var ty = N({}, Xa, {
      key: function (t) {
        if (t.key) {
          var e = kd[t.key] || t.key;
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
      getModifierState: Oi,
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
    ey = It(ty),
    ly = N({}, Vu, {
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
    ms = It(ly),
    ay = N({}, Xa, {
      touches: 0,
      targetTouches: 0,
      changedTouches: 0,
      altKey: 0,
      metaKey: 0,
      ctrlKey: 0,
      shiftKey: 0,
      getModifierState: Oi,
    }),
    uy = It(ay),
    ny = N({}, Ul, { propertyName: 0, elapsedTime: 0, pseudoElement: 0 }),
    iy = It(ny),
    cy = N({}, Vu, {
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
    fy = It(cy),
    sy = N({}, Ul, { newState: 0, oldState: 0 }),
    ry = It(sy),
    oy = [9, 13, 27, 32],
    zi = Ye && "CompositionEvent" in window,
    Za = null;
  Ye && "documentMode" in document && (Za = document.documentMode);
  var hy = Ye && "TextEvent" in window && !Za,
    vs = Ye && (!zi || (Za && 8 < Za && 11 >= Za)),
    gs = " ",
    Ss = !1;
  function ps(t, e) {
    switch (t) {
      case "keyup":
        return oy.indexOf(e.keyCode) !== -1;
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
  function bs(t) {
    return ((t = t.detail), typeof t == "object" && "data" in t ? t.data : null);
  }
  var aa = !1;
  function dy(t, e) {
    switch (t) {
      case "compositionend":
        return bs(e);
      case "keypress":
        return e.which !== 32 ? null : ((Ss = !0), gs);
      case "textInput":
        return ((t = e.data), t === gs && Ss ? null : t);
      default:
        return null;
    }
  }
  function yy(t, e) {
    if (aa)
      return t === "compositionend" || (!zi && ps(t, e))
        ? ((t = os()), (Xu = pi = al = null), (aa = !1), t)
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
  var my = {
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
  function Es(t) {
    var e = t && t.nodeName && t.nodeName.toLowerCase();
    return e === "input" ? !!my[t.type] : e === "textarea";
  }
  function Ts(t, e, l, a) {
    (ea ? (la ? la.push(a) : (la = [a])) : (ea = a),
      (e = Hn(e, "onChange")),
      0 < e.length &&
        ((l = new Ku("onChange", "change", null, l, a)), t.push({ event: l, listeners: e })));
  }
  var Ka = null,
    Va = null;
  function vy(t) {
    nh(t, 0);
  }
  function Ju(t) {
    var e = Ba(t);
    if (as(e)) return t;
  }
  function Os(t, e) {
    if (t === "change") return e;
  }
  var zs = !1;
  if (Ye) {
    var Ai;
    if (Ye) {
      var Mi = "oninput" in document;
      if (!Mi) {
        var As = document.createElement("div");
        (As.setAttribute("oninput", "return;"), (Mi = typeof As.oninput == "function"));
      }
      Ai = Mi;
    } else Ai = !1;
    zs = Ai && (!document.documentMode || 9 < document.documentMode);
  }
  function Ms() {
    Ka && (Ka.detachEvent("onpropertychange", _s), (Va = Ka = null));
  }
  function _s(t) {
    if (t.propertyName === "value" && Ju(Va)) {
      var e = [];
      (Ts(e, Va, t, vi(t)), rs(vy, e));
    }
  }
  function gy(t, e, l) {
    t === "focusin"
      ? (Ms(), (Ka = e), (Va = l), Ka.attachEvent("onpropertychange", _s))
      : t === "focusout" && Ms();
  }
  function Sy(t) {
    if (t === "selectionchange" || t === "keyup" || t === "keydown") return Ju(Va);
  }
  function py(t, e) {
    if (t === "click") return Ju(e);
  }
  function by(t, e) {
    if (t === "input" || t === "change") return Ju(e);
  }
  function Ey(t, e) {
    return (t === e && (t !== 0 || 1 / t === 1 / e)) || (t !== t && e !== e);
  }
  var fe = typeof Object.is == "function" ? Object.is : Ey;
  function Ja(t, e) {
    if (fe(t, e)) return !0;
    if (typeof t != "object" || t === null || typeof e != "object" || e === null) return !1;
    var l = Object.keys(t),
      a = Object.keys(e);
    if (l.length !== a.length) return !1;
    for (a = 0; a < l.length; a++) {
      var u = l[a];
      if (!ai.call(e, u) || !fe(t[u], e[u])) return !1;
    }
    return !0;
  }
  function Ds(t) {
    for (; t && t.firstChild; ) t = t.firstChild;
    return t;
  }
  function Rs(t, e) {
    var l = Ds(t);
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
      l = Ds(l);
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
  function Cs(t) {
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
  function _i(t) {
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
  var Ty = Ye && "documentMode" in document && 11 >= document.documentMode,
    ua = null,
    Di = null,
    wa = null,
    Ri = !1;
  function js(t, e, l) {
    var a = l.window === l ? l.document : l.nodeType === 9 ? l : l.ownerDocument;
    Ri ||
      ua == null ||
      ua !== Yu(a) ||
      ((a = ua),
      "selectionStart" in a && _i(a)
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
        (a = Hn(Di, "onSelect")),
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
  function jl(t) {
    if (Ui[t]) return Ui[t];
    if (!na[t]) return t;
    var e = na[t],
      l;
    for (l in e) if (e.hasOwnProperty(l) && l in Ns) return (Ui[t] = e[l]);
    return t;
  }
  var xs = jl("animationend"),
    Hs = jl("animationiteration"),
    qs = jl("animationstart"),
    Oy = jl("transitionrun"),
    zy = jl("transitionstart"),
    Ay = jl("transitioncancel"),
    Qs = jl("transitionend"),
    Bs = new Map(),
    Ci =
      "abort auxClick beforeToggle cancel canPlay canPlayThrough click close contextMenu copy cut drag dragEnd dragEnter dragExit dragLeave dragOver dragStart drop durationChange emptied encrypted ended error gotPointerCapture input invalid keyDown keyPress keyUp load loadedData loadedMetadata loadStart lostPointerCapture mouseDown mouseMove mouseOut mouseOver mouseUp paste pause play playing pointerCancel pointerDown pointerMove pointerOut pointerOver pointerUp progress rateChange reset resize seeked seeking stalled submit suspend timeUpdate touchCancel touchEnd touchStart volumeChange scroll toggle touchMove waiting wheel".split(
        " ",
      );
  Ci.push("scrollEnd");
  function Re(t, e) {
    (Bs.set(t, e), Rl(e, [t]));
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
    ji = 0;
  function Fu() {
    for (var t = ia, e = (ji = ia = 0); e < t; ) {
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
      n !== 0 && Ys(l, u, n);
    }
  }
  function Wu(t, e, l, a) {
    ((pe[ia++] = t),
      (pe[ia++] = e),
      (pe[ia++] = l),
      (pe[ia++] = a),
      (ji |= a),
      (t.lanes |= a),
      (t = t.alternate),
      t !== null && (t.lanes |= a));
  }
  function Ni(t, e, l, a) {
    return (Wu(t, e, l, a), ku(t));
  }
  function Nl(t, e) {
    return (Wu(t, null, null, e), ku(t));
  }
  function Ys(t, e, l) {
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
  function ku(t) {
    if (50 < mu) throw ((mu = 0), (Lc = null), Error(r(185)));
    for (var e = t.return; e !== null; ) ((t = e), (e = t.return));
    return t.tag === 3 ? t.stateNode : null;
  }
  var ca = {};
  function My(t, e, l, a) {
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
    return new My(t, e, l, a);
  }
  function xi(t) {
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
  function Gs(t, e) {
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
  function $u(t, e, l, a, u, n) {
    var c = 0;
    if (((a = t), typeof t == "function")) xi(t) && (c = 1);
    else if (typeof t == "string")
      c = Cm(t, l, B.current) ? 26 : t === "html" || t === "head" || t === "body" ? 27 : 5;
    else
      t: switch (t) {
        case Jt:
          return ((t = se(31, l, e, u)), (t.elementType = Jt), (t.lanes = n), t);
        case Z:
          return xl(l.children, u, n, e);
        case St:
          ((c = 8), (u |= 24));
          break;
        case st:
          return ((t = se(12, l, e, u | 2)), (t.elementType = st), (t.lanes = n), t);
        case Qt:
          return ((t = se(13, l, e, u)), (t.elementType = Qt), (t.lanes = n), t);
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
              case Rt:
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
  function xl(t, e, l, a) {
    return ((t = se(7, t, a, e)), (t.lanes = l), t);
  }
  function Hi(t, e, l) {
    return ((t = se(6, t, null, e)), (t.lanes = l), t);
  }
  function Xs(t) {
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
  var Ls = new WeakMap();
  function be(t, e) {
    if (typeof t == "object" && t !== null) {
      var l = Ls.get(t);
      return l !== void 0 ? l : ((e = { value: t, source: e, stack: Xf(e) }), Ls.set(t, e), e);
    }
    return { value: t, source: e, stack: Xf(e) };
  }
  var fa = [],
    sa = 0,
    Iu = null,
    Fa = 0,
    Ee = [],
    Te = 0,
    ul = null,
    je = 1,
    Ne = "";
  function Xe(t, e) {
    ((fa[sa++] = Fa), (fa[sa++] = Iu), (Iu = t), (Fa = e));
  }
  function Zs(t, e, l) {
    ((Ee[Te++] = je), (Ee[Te++] = Ne), (Ee[Te++] = ul), (ul = t));
    var a = je;
    t = Ne;
    var u = 32 - ce(a) - 1;
    ((a &= ~(1 << u)), (l += 1));
    var n = 32 - ce(e) + u;
    if (30 < n) {
      var c = u - (u % 5);
      ((n = (a & ((1 << c) - 1)).toString(32)),
        (a >>= c),
        (u -= c),
        (je = (1 << (32 - ce(e) + u)) | (l << u) | a),
        (Ne = n + t));
    } else ((je = (1 << n) | (l << u) | a), (Ne = t));
  }
  function Qi(t) {
    t.return !== null && (Xe(t, 1), Zs(t, 1, 0));
  }
  function Bi(t) {
    for (; t === Iu; ) ((Iu = fa[--sa]), (fa[sa] = null), (Fa = fa[--sa]), (fa[sa] = null));
    for (; t === ul; )
      ((ul = Ee[--Te]),
        (Ee[Te] = null),
        (Ne = Ee[--Te]),
        (Ee[Te] = null),
        (je = Ee[--Te]),
        (Ee[Te] = null));
  }
  function Ks(t, e) {
    ((Ee[Te++] = je), (Ee[Te++] = Ne), (Ee[Te++] = ul), (je = e.id), (Ne = e.overflow), (ul = t));
  }
  var Lt = null,
    pt = null,
    et = !1,
    nl = null,
    Oe = !1,
    Yi = Error(r(519));
  function il(t) {
    var e = Error(
      r(418, 1 < arguments.length && arguments[1] !== void 0 && arguments[1] ? "text" : "HTML", ""),
    );
    throw (Wa(be(e, t)), Yi);
  }
  function Vs(t) {
    var e = t.stateNode,
      l = t.type,
      a = t.memoizedProps;
    switch (((e[Xt] = t), (e[$t] = a), l)) {
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
          us(e, a.value, a.defaultValue, a.checked, a.defaultChecked, a.type, a.name, !0));
        break;
      case "select":
        I("invalid", e);
        break;
      case "textarea":
        (I("invalid", e), is(e, a.value, a.defaultValue, a.children));
    }
    ((l = a.children),
      (typeof l != "string" && typeof l != "number" && typeof l != "bigint") ||
      e.textContent === "" + l ||
      a.suppressHydrationWarning === !0 ||
      sh(e.textContent, l)
        ? (a.popover != null && (I("beforetoggle", e), I("toggle", e)),
          a.onScroll != null && I("scroll", e),
          a.onScrollEnd != null && I("scrollend", e),
          a.onClick != null && (e.onclick = Be),
          (e = !0))
        : (e = !1),
      e || il(t, !0));
  }
  function Js(t) {
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
    if (!et) return (Js(t), (et = !0), !1);
    var e = t.tag,
      l;
    if (
      ((l = e !== 3 && e !== 27) &&
        ((l = e === 5) &&
          ((l = t.type), (l = !(l !== "form" && l !== "button") || af(t.type, t.memoizedProps))),
        (l = !l)),
      l && pt && il(t),
      Js(t),
      e === 13)
    ) {
      if (((t = t.memoizedState), (t = t !== null ? t.dehydrated : null), !t)) throw Error(r(317));
      pt = Sh(t);
    } else if (e === 31) {
      if (((t = t.memoizedState), (t = t !== null ? t.dehydrated : null), !t)) throw Error(r(317));
      pt = Sh(t);
    } else
      e === 27
        ? ((e = pt), bl(t.type) ? ((t = sf), (sf = null), (pt = t)) : (pt = e))
        : (pt = Lt ? Ae(t.stateNode.nextSibling) : null);
    return !0;
  }
  function Hl() {
    ((pt = Lt = null), (et = !1));
  }
  function Gi() {
    var t = nl;
    return (t !== null && (le === null ? (le = t) : le.push.apply(le, t), (nl = null)), t);
  }
  function Wa(t) {
    nl === null ? (nl = [t]) : nl.push(t);
  }
  var Xi = y(null),
    ql = null,
    Le = null;
  function cl(t, e, l) {
    (H(Xi, e._currentValue), (e._currentValue = l));
  }
  function Ze(t) {
    ((t._currentValue = Xi.current), R(Xi));
  }
  function Li(t, e, l) {
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
  function Zi(t, e, l, a) {
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
                Li(n.return, l, t),
                a || (c = null));
              break t;
            }
          n = s.next;
        }
      } else if (u.tag === 18) {
        if (((c = u.return), c === null)) throw Error(r(341));
        ((c.lanes |= l), (n = c.alternate), n !== null && (n.lanes |= l), Li(c, l, t), (c = null));
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
    (t !== null && Zi(e, t, l, a), (e.flags |= 262144));
  }
  function Pu(t) {
    for (t = t.firstContext; t !== null; ) {
      if (!fe(t.context._currentValue, t.memoizedValue)) return !0;
      t = t.next;
    }
    return !1;
  }
  function Ql(t) {
    ((ql = t), (Le = null), (t = t.dependencies), t !== null && (t.firstContext = null));
  }
  function Zt(t) {
    return ws(ql, t);
  }
  function tn(t, e) {
    return (ql === null && Ql(t), ws(t, e));
  }
  function ws(t, e) {
    var l = e._currentValue;
    if (((e = { context: e, memoizedValue: l, next: null }), Le === null)) {
      if (t === null) throw Error(r(308));
      ((Le = e), (t.dependencies = { lanes: 0, firstContext: e }), (t.flags |= 524288));
    } else Le = Le.next = e;
    return l;
  }
  var _y =
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
    Dy = i.unstable_scheduleCallback,
    Ry = i.unstable_NormalPriority,
    Ut = {
      $$typeof: gt,
      Consumer: null,
      Provider: null,
      _currentValue: null,
      _currentValue2: null,
      _threadCount: 0,
    };
  function Ki() {
    return { controller: new _y(), data: new Map(), refCount: 0 };
  }
  function ka(t) {
    (t.refCount--,
      t.refCount === 0 &&
        Dy(Ry, function () {
          t.controller.abort();
        }));
  }
  var $a = null,
    Vi = 0,
    ha = 0,
    da = null;
  function Uy(t, e) {
    if ($a === null) {
      var l = ($a = []);
      ((Vi = 0),
        (ha = Fc()),
        (da = {
          status: "pending",
          value: void 0,
          then: function (a) {
            l.push(a);
          },
        }));
    }
    return (Vi++, e.then(Fs, Fs), e);
  }
  function Fs() {
    if (--Vi === 0 && $a !== null) {
      da !== null && (da.status = "fulfilled");
      var t = $a;
      (($a = null), (ha = 0), (da = null));
      for (var e = 0; e < t.length; e++) (0, t[e])();
    }
  }
  function Cy(t, e) {
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
  var Ws = z.S;
  z.S = function (t, e) {
    ((No = ne()),
      typeof e == "object" && e !== null && typeof e.then == "function" && Uy(t, e),
      Ws !== null && Ws(t, e));
  };
  var Bl = y(null);
  function Ji() {
    var t = Bl.current;
    return t !== null ? t : vt.pooledCache;
  }
  function en(t, e) {
    e === null ? H(Bl, Bl.current) : H(Bl, e.pool);
  }
  function ks() {
    var t = Ji();
    return t === null ? null : { parent: Ut._currentValue, pool: t };
  }
  var ya = Error(r(460)),
    wi = Error(r(474)),
    ln = Error(r(542)),
    an = { then: function () {} };
  function $s(t) {
    return ((t = t.status), t === "fulfilled" || t === "rejected");
  }
  function Is(t, e, l) {
    switch (
      ((l = t[l]), l === void 0 ? t.push(e) : l !== e && (e.then(Be, Be), (e = l)), e.status)
    ) {
      case "fulfilled":
        return e.value;
      case "rejected":
        throw ((t = e.reason), tr(t), t);
      default:
        if (typeof e.status == "string") e.then(Be, Be);
        else {
          if (((t = vt), t !== null && 100 < t.shellSuspendCounter)) throw Error(r(482));
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
            throw ((t = e.reason), tr(t), t);
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
  function Ps() {
    if (Gl === null) throw Error(r(459));
    var t = Gl;
    return ((Gl = null), t);
  }
  function tr(t) {
    if (t === ya || t === ln) throw Error(r(483));
  }
  var ma = null,
    Ia = 0;
  function un(t) {
    var e = Ia;
    return ((Ia += 1), ma === null && (ma = []), Is(ma, t, e));
  }
  function Pa(t, e) {
    ((e = e.props.ref), (t.ref = e !== void 0 ? e : null));
  }
  function nn(t, e) {
    throw e.$$typeof === U
      ? Error(r(525))
      : ((t = Object.prototype.toString.call(e)),
        Error(
          r(
            31,
            t === "[object Object]" ? "object with keys {" + Object.keys(e).join(", ") + "}" : t,
          ),
        ));
  }
  function er(t) {
    function e(m, d) {
      if (t) {
        var v = m.deletions;
        v === null ? ((m.deletions = [d]), (m.flags |= 16)) : v.push(d);
      }
    }
    function l(m, d) {
      if (!t) return null;
      for (; d !== null; ) (e(m, d), (d = d.sibling));
      return null;
    }
    function a(m) {
      for (var d = new Map(); m !== null; )
        (m.key !== null ? d.set(m.key, m) : d.set(m.index, m), (m = m.sibling));
      return d;
    }
    function u(m, d) {
      return ((m = Ge(m, d)), (m.index = 0), (m.sibling = null), m);
    }
    function n(m, d, v) {
      return (
        (m.index = v),
        t
          ? ((v = m.alternate),
            v !== null
              ? ((v = v.index), v < d ? ((m.flags |= 67108866), d) : v)
              : ((m.flags |= 67108866), d))
          : ((m.flags |= 1048576), d)
      );
    }
    function c(m) {
      return (t && m.alternate === null && (m.flags |= 67108866), m);
    }
    function s(m, d, v, _) {
      return d === null || d.tag !== 6
        ? ((d = Hi(v, m.mode, _)), (d.return = m), d)
        : ((d = u(d, v)), (d.return = m), d);
    }
    function h(m, d, v, _) {
      var G = v.type;
      return G === Z
        ? T(m, d, v.props.children, _, v.key)
        : d !== null &&
            (d.elementType === G ||
              (typeof G == "object" && G !== null && G.$$typeof === yt && Yl(G) === d.type))
          ? ((d = u(d, v.props)), Pa(d, v), (d.return = m), d)
          : ((d = $u(v.type, v.key, v.props, null, m.mode, _)), Pa(d, v), (d.return = m), d);
    }
    function g(m, d, v, _) {
      return d === null ||
        d.tag !== 4 ||
        d.stateNode.containerInfo !== v.containerInfo ||
        d.stateNode.implementation !== v.implementation
        ? ((d = qi(v, m.mode, _)), (d.return = m), d)
        : ((d = u(d, v.children || [])), (d.return = m), d);
    }
    function T(m, d, v, _, G) {
      return d === null || d.tag !== 7
        ? ((d = xl(v, m.mode, _, G)), (d.return = m), d)
        : ((d = u(d, v)), (d.return = m), d);
    }
    function D(m, d, v) {
      if ((typeof d == "string" && d !== "") || typeof d == "number" || typeof d == "bigint")
        return ((d = Hi("" + d, m.mode, v)), (d.return = m), d);
      if (typeof d == "object" && d !== null) {
        switch (d.$$typeof) {
          case lt:
            return ((v = $u(d.type, d.key, d.props, null, m.mode, v)), Pa(v, d), (v.return = m), v);
          case W:
            return ((d = qi(d, m.mode, v)), (d.return = m), d);
          case yt:
            return ((d = Yl(d)), D(m, d, v));
        }
        if (De(d) || xt(d)) return ((d = xl(d, m.mode, v, null)), (d.return = m), d);
        if (typeof d.then == "function") return D(m, un(d), v);
        if (d.$$typeof === gt) return D(m, tn(m, d), v);
        nn(m, d);
      }
      return null;
    }
    function S(m, d, v, _) {
      var G = d !== null ? d.key : null;
      if ((typeof v == "string" && v !== "") || typeof v == "number" || typeof v == "bigint")
        return G !== null ? null : s(m, d, "" + v, _);
      if (typeof v == "object" && v !== null) {
        switch (v.$$typeof) {
          case lt:
            return v.key === G ? h(m, d, v, _) : null;
          case W:
            return v.key === G ? g(m, d, v, _) : null;
          case yt:
            return ((v = Yl(v)), S(m, d, v, _));
        }
        if (De(v) || xt(v)) return G !== null ? null : T(m, d, v, _, null);
        if (typeof v.then == "function") return S(m, d, un(v), _);
        if (v.$$typeof === gt) return S(m, d, tn(m, v), _);
        nn(m, v);
      }
      return null;
    }
    function b(m, d, v, _, G) {
      if ((typeof _ == "string" && _ !== "") || typeof _ == "number" || typeof _ == "bigint")
        return ((m = m.get(v) || null), s(d, m, "" + _, G));
      if (typeof _ == "object" && _ !== null) {
        switch (_.$$typeof) {
          case lt:
            return ((m = m.get(_.key === null ? v : _.key) || null), h(d, m, _, G));
          case W:
            return ((m = m.get(_.key === null ? v : _.key) || null), g(d, m, _, G));
          case yt:
            return ((_ = Yl(_)), b(m, d, v, _, G));
        }
        if (De(_) || xt(_)) return ((m = m.get(v) || null), T(d, m, _, G, null));
        if (typeof _.then == "function") return b(m, d, v, un(_), G);
        if (_.$$typeof === gt) return b(m, d, v, tn(d, _), G);
        nn(d, _);
      }
      return null;
    }
    function Q(m, d, v, _) {
      for (
        var G = null, at = null, Y = d, F = (d = 0), tt = null;
        Y !== null && F < v.length;
        F++
      ) {
        Y.index > F ? ((tt = Y), (Y = null)) : (tt = Y.sibling);
        var ut = S(m, Y, v[F], _);
        if (ut === null) {
          Y === null && (Y = tt);
          break;
        }
        (t && Y && ut.alternate === null && e(m, Y),
          (d = n(ut, d, F)),
          at === null ? (G = ut) : (at.sibling = ut),
          (at = ut),
          (Y = tt));
      }
      if (F === v.length) return (l(m, Y), et && Xe(m, F), G);
      if (Y === null) {
        for (; F < v.length; F++)
          ((Y = D(m, v[F], _)),
            Y !== null && ((d = n(Y, d, F)), at === null ? (G = Y) : (at.sibling = Y), (at = Y)));
        return (et && Xe(m, F), G);
      }
      for (Y = a(Y); F < v.length; F++)
        ((tt = b(Y, m, F, v[F], _)),
          tt !== null &&
            (t && tt.alternate !== null && Y.delete(tt.key === null ? F : tt.key),
            (d = n(tt, d, F)),
            at === null ? (G = tt) : (at.sibling = tt),
            (at = tt)));
      return (
        t &&
          Y.forEach(function (Al) {
            return e(m, Al);
          }),
        et && Xe(m, F),
        G
      );
    }
    function X(m, d, v, _) {
      if (v == null) throw Error(r(151));
      for (
        var G = null, at = null, Y = d, F = (d = 0), tt = null, ut = v.next();
        Y !== null && !ut.done;
        F++, ut = v.next()
      ) {
        Y.index > F ? ((tt = Y), (Y = null)) : (tt = Y.sibling);
        var Al = S(m, Y, ut.value, _);
        if (Al === null) {
          Y === null && (Y = tt);
          break;
        }
        (t && Y && Al.alternate === null && e(m, Y),
          (d = n(Al, d, F)),
          at === null ? (G = Al) : (at.sibling = Al),
          (at = Al),
          (Y = tt));
      }
      if (ut.done) return (l(m, Y), et && Xe(m, F), G);
      if (Y === null) {
        for (; !ut.done; F++, ut = v.next())
          ((ut = D(m, ut.value, _)),
            ut !== null &&
              ((d = n(ut, d, F)), at === null ? (G = ut) : (at.sibling = ut), (at = ut)));
        return (et && Xe(m, F), G);
      }
      for (Y = a(Y); !ut.done; F++, ut = v.next())
        ((ut = b(Y, m, F, ut.value, _)),
          ut !== null &&
            (t && ut.alternate !== null && Y.delete(ut.key === null ? F : ut.key),
            (d = n(ut, d, F)),
            at === null ? (G = ut) : (at.sibling = ut),
            (at = ut)));
      return (
        t &&
          Y.forEach(function (Lm) {
            return e(m, Lm);
          }),
        et && Xe(m, F),
        G
      );
    }
    function dt(m, d, v, _) {
      if (
        (typeof v == "object" &&
          v !== null &&
          v.type === Z &&
          v.key === null &&
          (v = v.props.children),
        typeof v == "object" && v !== null)
      ) {
        switch (v.$$typeof) {
          case lt:
            t: {
              for (var G = v.key; d !== null; ) {
                if (d.key === G) {
                  if (((G = v.type), G === Z)) {
                    if (d.tag === 7) {
                      (l(m, d.sibling), (_ = u(d, v.props.children)), (_.return = m), (m = _));
                      break t;
                    }
                  } else if (
                    d.elementType === G ||
                    (typeof G == "object" && G !== null && G.$$typeof === yt && Yl(G) === d.type)
                  ) {
                    (l(m, d.sibling), (_ = u(d, v.props)), Pa(_, v), (_.return = m), (m = _));
                    break t;
                  }
                  l(m, d);
                  break;
                } else e(m, d);
                d = d.sibling;
              }
              v.type === Z
                ? ((_ = xl(v.props.children, m.mode, _, v.key)), (_.return = m), (m = _))
                : ((_ = $u(v.type, v.key, v.props, null, m.mode, _)),
                  Pa(_, v),
                  (_.return = m),
                  (m = _));
            }
            return c(m);
          case W:
            t: {
              for (G = v.key; d !== null; ) {
                if (d.key === G)
                  if (
                    d.tag === 4 &&
                    d.stateNode.containerInfo === v.containerInfo &&
                    d.stateNode.implementation === v.implementation
                  ) {
                    (l(m, d.sibling), (_ = u(d, v.children || [])), (_.return = m), (m = _));
                    break t;
                  } else {
                    l(m, d);
                    break;
                  }
                else e(m, d);
                d = d.sibling;
              }
              ((_ = qi(v, m.mode, _)), (_.return = m), (m = _));
            }
            return c(m);
          case yt:
            return ((v = Yl(v)), dt(m, d, v, _));
        }
        if (De(v)) return Q(m, d, v, _);
        if (xt(v)) {
          if (((G = xt(v)), typeof G != "function")) throw Error(r(150));
          return ((v = G.call(v)), X(m, d, v, _));
        }
        if (typeof v.then == "function") return dt(m, d, un(v), _);
        if (v.$$typeof === gt) return dt(m, d, tn(m, v), _);
        nn(m, v);
      }
      return (typeof v == "string" && v !== "") || typeof v == "number" || typeof v == "bigint"
        ? ((v = "" + v),
          d !== null && d.tag === 6
            ? (l(m, d.sibling), (_ = u(d, v)), (_.return = m), (m = _))
            : (l(m, d), (_ = Hi(v, m.mode, _)), (_.return = m), (m = _)),
          c(m))
        : l(m, d);
    }
    return function (m, d, v, _) {
      try {
        Ia = 0;
        var G = dt(m, d, v, _);
        return ((ma = null), G);
      } catch (Y) {
        if (Y === ya || Y === ln) throw Y;
        var at = se(29, Y, null, m.mode);
        return ((at.lanes = _), (at.return = m), at);
      }
    };
  }
  var Xl = er(!0),
    lr = er(!1),
    fl = !1;
  function Fi(t) {
    t.updateQueue = {
      baseState: t.memoizedState,
      firstBaseUpdate: null,
      lastBaseUpdate: null,
      shared: { pending: null, lanes: 0, hiddenCallbacks: null },
      callbacks: null,
    };
  }
  function Wi(t, e) {
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
        (e = ku(t)),
        Ys(t, null, l),
        e
      );
    }
    return (Wu(t, a, e, l), ku(t));
  }
  function tu(t, e, l) {
    if (((e = e.updateQueue), e !== null && ((e = e.shared), (l & 4194048) !== 0))) {
      var a = e.lanes;
      ((a &= t.pendingLanes), (l |= a), (e.lanes = l), wf(t, l));
    }
  }
  function ki(t, e) {
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
        g = h.next;
      ((h.next = null), c === null ? (n = g) : (c.next = g), (c = h));
      var T = t.alternate;
      T !== null &&
        ((T = T.updateQueue),
        (s = T.lastBaseUpdate),
        s !== c && (s === null ? (T.firstBaseUpdate = g) : (s.next = g), (T.lastBaseUpdate = h)));
    }
    if (n !== null) {
      var D = u.baseState;
      ((c = 0), (T = g = h = null), (s = n));
      do {
        var S = s.lane & -536870913,
          b = S !== s.lane;
        if (b ? (P & S) === S : (a & S) === S) {
          (S !== 0 && S === ha && ($i = !0),
            T !== null &&
              (T = T.next =
                { lane: 0, tag: s.tag, payload: s.payload, callback: null, next: null }));
          t: {
            var Q = t,
              X = s;
            S = e;
            var dt = l;
            switch (X.tag) {
              case 1:
                if (((Q = X.payload), typeof Q == "function")) {
                  D = Q.call(dt, D, S);
                  break t;
                }
                D = Q;
                break t;
              case 3:
                Q.flags = (Q.flags & -65537) | 128;
              case 0:
                if (
                  ((Q = X.payload), (S = typeof Q == "function" ? Q.call(dt, D, S) : Q), S == null)
                )
                  break t;
                D = N({}, D, S);
                break t;
              case 2:
                fl = !0;
            }
          }
          ((S = s.callback),
            S !== null &&
              ((t.flags |= 64),
              b && (t.flags |= 8192),
              (b = u.callbacks),
              b === null ? (u.callbacks = [S]) : b.push(S)));
        } else
          ((b = { lane: S, tag: s.tag, payload: s.payload, callback: s.callback, next: null }),
            T === null ? ((g = T = b), (h = D)) : (T = T.next = b),
            (c |= S));
        if (((s = s.next), s === null)) {
          if (((s = u.shared.pending), s === null)) break;
          ((b = s),
            (s = b.next),
            (b.next = null),
            (u.lastBaseUpdate = b),
            (u.shared.pending = null));
        }
      } while (!0);
      (T === null && (h = D),
        (u.baseState = h),
        (u.firstBaseUpdate = g),
        (u.lastBaseUpdate = T),
        n === null && (u.shared.lanes = 0),
        (ml |= c),
        (t.lanes = c),
        (t.memoizedState = D));
    }
  }
  function ar(t, e) {
    if (typeof t != "function") throw Error(r(191, t));
    t.call(e);
  }
  function ur(t, e) {
    var l = t.callbacks;
    if (l !== null) for (t.callbacks = null, t = 0; t < l.length; t++) ar(l[t], e);
  }
  var va = y(null),
    cn = y(0);
  function nr(t, e) {
    ((t = Ie), H(cn, t), H(va, e), (Ie = t | e.baseLanes));
  }
  function Ii() {
    (H(cn, Ie), H(va, va.current));
  }
  function Pi() {
    ((Ie = cn.current), R(va), R(cn));
  }
  var re = y(null),
    ze = null;
  function ol(t) {
    var e = t.alternate;
    (H(Mt, Mt.current & 1),
      H(re, t),
      ze === null && (e === null || va.current !== null || e.memoizedState !== null) && (ze = t));
  }
  function tc(t) {
    (H(Mt, Mt.current), H(re, t), ze === null && (ze = t));
  }
  function ir(t) {
    t.tag === 22 ? (H(Mt, Mt.current), H(re, t), ze === null && (ze = t)) : hl();
  }
  function hl() {
    (H(Mt, Mt.current), H(re, re.current));
  }
  function oe(t) {
    (R(re), ze === t && (ze = null), R(Mt));
  }
  var Mt = y(0);
  function fn(t) {
    for (var e = t; e !== null; ) {
      if (e.tag === 13) {
        var l = e.memoizedState;
        if (l !== null && ((l = l.dehydrated), l === null || cf(l) || ff(l))) return e;
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
    jy = 0;
  function Ot() {
    throw Error(r(321));
  }
  function ec(t, e) {
    if (e === null) return !1;
    for (var l = 0; l < e.length && l < t.length; l++) if (!fe(t[l], e[l])) return !1;
    return !0;
  }
  function lc(t, e, l, a, u, n) {
    return (
      (Ke = n),
      (w = e),
      (e.memoizedState = null),
      (e.updateQueue = null),
      (e.lanes = 0),
      (z.H = t === null || t.memoizedState === null ? Zr : gc),
      (Ll = !1),
      (n = l(a, u)),
      (Ll = !1),
      ga && (n = fr(e, l, a, u)),
      cr(t),
      n
    );
  }
  function cr(t) {
    z.H = iu;
    var e = ot !== null && ot.next !== null;
    if (((Ke = 0), (Ct = ot = w = null), (sn = !1), (au = 0), (Sa = null), e)) throw Error(r(300));
    t === null || jt || ((t = t.dependencies), t !== null && Pu(t) && (jt = !0));
  }
  function fr(t, e, l, a) {
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
      ((z.H = Kr), (n = e(l, a)));
    } while (ga);
    return n;
  }
  function Ny() {
    var t = z.H,
      e = t.useState()[0];
    return (
      (e = typeof e.then == "function" ? uu(e) : e),
      (t = t.useState()[0]),
      (ot !== null ? ot.memoizedState : null) !== t && (w.flags |= 1024),
      e
    );
  }
  function ac() {
    var t = rn !== 0;
    return ((rn = 0), t);
  }
  function uc(t, e, l) {
    ((e.updateQueue = t.updateQueue), (e.flags &= -2053), (t.lanes &= ~l));
  }
  function nc(t) {
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
      (t = Is(Sa, t, e)),
      (e = w),
      (Ct === null ? e.memoizedState : Ct.next) === null &&
        ((e = e.alternate), (z.H = e === null || e.memoizedState === null ? Zr : gc)),
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
  function ic(t) {
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
    return cc(e, ot, t);
  }
  function cc(t, e, l) {
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
        g = e,
        T = !1;
      do {
        var D = g.lane & -536870913;
        if (D !== g.lane ? (P & D) === D : (Ke & D) === D) {
          var S = g.revertLane;
          if (S === 0)
            (h !== null &&
              (h = h.next =
                {
                  lane: 0,
                  revertLane: 0,
                  gesture: null,
                  action: g.action,
                  hasEagerState: g.hasEagerState,
                  eagerState: g.eagerState,
                  next: null,
                }),
              D === ha && (T = !0));
          else if ((Ke & S) === S) {
            ((g = g.next), S === ha && (T = !0));
            continue;
          } else
            ((D = {
              lane: 0,
              revertLane: g.revertLane,
              gesture: null,
              action: g.action,
              hasEagerState: g.hasEagerState,
              eagerState: g.eagerState,
              next: null,
            }),
              h === null ? ((s = h = D), (c = n)) : (h = h.next = D),
              (w.lanes |= S),
              (ml |= S));
          ((D = g.action), Ll && l(n, D), (n = g.hasEagerState ? g.eagerState : l(n, D)));
        } else
          ((S = {
            lane: D,
            revertLane: g.revertLane,
            gesture: g.gesture,
            action: g.action,
            hasEagerState: g.hasEagerState,
            eagerState: g.eagerState,
            next: null,
          }),
            h === null ? ((s = h = S), (c = n)) : (h = h.next = S),
            (w.lanes |= D),
            (ml |= D));
        g = g.next;
      } while (g !== null && g !== e);
      if (
        (h === null ? (c = n) : (h.next = s),
        !fe(n, t.memoizedState) && ((jt = !0), T && ((l = da), l !== null)))
      )
        throw l;
      ((t.memoizedState = n), (t.baseState = c), (t.baseQueue = h), (a.lastRenderedState = n));
    }
    return (u === null && (a.lanes = 0), [t.memoizedState, a.dispatch]);
  }
  function fc(t) {
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
      (fe(n, e.memoizedState) || (jt = !0),
        (e.memoizedState = n),
        e.baseQueue === null && (e.baseState = n),
        (l.lastRenderedState = n));
    }
    return [n, a];
  }
  function sr(t, e, l) {
    var a = w,
      u = _t(),
      n = et;
    if (n) {
      if (l === void 0) throw Error(r(407));
      l = l();
    } else l = e();
    var c = !fe((ot || u).memoizedState, l);
    if (
      (c && ((u.memoizedState = l), (jt = !0)),
      (u = u.queue),
      oc(hr.bind(null, a, u, t), [t]),
      u.getSnapshot !== e || c || (Ct !== null && Ct.memoizedState.tag & 1))
    ) {
      if (
        ((a.flags |= 2048),
        pa(9, { destroy: void 0 }, or.bind(null, a, u, l, e), null),
        vt === null)
      )
        throw Error(r(349));
      n || (Ke & 127) !== 0 || rr(a, e, l);
    }
    return l;
  }
  function rr(t, e, l) {
    ((t.flags |= 16384),
      (t = { getSnapshot: e, value: l }),
      (e = w.updateQueue),
      e === null
        ? ((e = on()), (w.updateQueue = e), (e.stores = [t]))
        : ((l = e.stores), l === null ? (e.stores = [t]) : l.push(t)));
  }
  function or(t, e, l, a) {
    ((e.value = l), (e.getSnapshot = a), dr(e) && yr(t));
  }
  function hr(t, e, l) {
    return l(function () {
      dr(e) && yr(t);
    });
  }
  function dr(t) {
    var e = t.getSnapshot;
    t = t.value;
    try {
      var l = e();
      return !fe(t, l);
    } catch {
      return !0;
    }
  }
  function yr(t) {
    var e = Nl(t, 2);
    e !== null && ae(e, t, 2);
  }
  function sc(t) {
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
  function mr(t, e, l, a) {
    return ((t.baseState = l), cc(t, ot, typeof a == "function" ? a : Ve));
  }
  function xy(t, e, l, a, u) {
    if (vn(t)) throw Error(r(485));
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
      (z.T !== null ? l(!0) : (n.isTransition = !1),
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
      var n = z.T,
        c = {};
      z.T = c;
      try {
        var s = l(u, a),
          h = z.S;
        (h !== null && h(c, s), gr(t, e, s));
      } catch (g) {
        rc(t, e, g);
      } finally {
        (n !== null && c.types !== null && (n.types = c.types), (z.T = n));
      }
    } else
      try {
        ((n = l(u, a)), gr(t, e, n));
      } catch (g) {
        rc(t, e, g);
      }
  }
  function gr(t, e, l) {
    l !== null && typeof l == "object" && typeof l.then == "function"
      ? l.then(
          function (a) {
            Sr(t, e, a);
          },
          function (a) {
            return rc(t, e, a);
          },
        )
      : Sr(t, e, l);
  }
  function Sr(t, e, l) {
    ((e.status = "fulfilled"),
      (e.value = l),
      pr(e),
      (t.state = l),
      (e = t.pending),
      e !== null &&
        ((l = e.next), l === e ? (t.pending = null) : ((l = l.next), (e.next = l), vr(t, l))));
  }
  function rc(t, e, l) {
    var a = t.pending;
    if (((t.pending = null), a !== null)) {
      a = a.next;
      do ((e.status = "rejected"), (e.reason = l), pr(e), (e = e.next));
      while (e !== a);
    }
    t.action = null;
  }
  function pr(t) {
    t = t.listeners;
    for (var e = 0; e < t.length; e++) (0, t[e])();
  }
  function br(t, e) {
    return e;
  }
  function Er(t, e) {
    if (et) {
      var l = vt.formState;
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
        lastRenderedReducer: br,
        lastRenderedState: e,
      }),
      (l.queue = a),
      (l = Gr.bind(null, w, a)),
      (a.dispatch = l),
      (a = sc(!1)),
      (n = vc.bind(null, w, !1, a.queue)),
      (a = Wt()),
      (u = { state: e, dispatch: null, action: t, pending: null }),
      (a.queue = u),
      (l = xy.bind(null, w, u, n, l)),
      (u.dispatch = l),
      (a.memoizedState = t),
      [e, l, !1]
    );
  }
  function Tr(t) {
    var e = _t();
    return Or(e, ot, t);
  }
  function Or(t, e, l) {
    if (
      ((e = cc(t, e, br)[0]),
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
        ((w.flags |= 2048), pa(9, { destroy: void 0 }, Hy.bind(null, u, l), null)),
      [a, n, t]
    );
  }
  function Hy(t, e) {
    t.action = e;
  }
  function zr(t) {
    var e = _t(),
      l = ot;
    if (l !== null) return Or(e, l, t);
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
  function Ar() {
    return _t().memoizedState;
  }
  function yn(t, e, l, a) {
    var u = Wt();
    ((w.flags |= t),
      (u.memoizedState = pa(1 | e, { destroy: void 0 }, l, a === void 0 ? null : a)));
  }
  function mn(t, e, l, a) {
    var u = _t();
    a = a === void 0 ? null : a;
    var n = u.memoizedState.inst;
    ot !== null && a !== null && ec(a, ot.memoizedState.deps)
      ? (u.memoizedState = pa(e, n, l, a))
      : ((w.flags |= t), (u.memoizedState = pa(1 | e, n, l, a)));
  }
  function Mr(t, e) {
    yn(8390656, 8, t, e);
  }
  function oc(t, e) {
    mn(2048, 8, t, e);
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
  function _r(t) {
    var e = _t().memoizedState;
    return (
      qy({ ref: e, nextImpl: t }),
      function () {
        if ((nt & 2) !== 0) throw Error(r(440));
        return e.impl.apply(void 0, arguments);
      }
    );
  }
  function Dr(t, e) {
    return mn(4, 2, t, e);
  }
  function Rr(t, e) {
    return mn(4, 4, t, e);
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
  function Cr(t, e, l) {
    ((l = l != null ? l.concat([t]) : null), mn(4, 4, Ur.bind(null, e, t), l));
  }
  function hc() {}
  function jr(t, e) {
    var l = _t();
    e = e === void 0 ? null : e;
    var a = l.memoizedState;
    return e !== null && ec(e, a[1]) ? a[0] : ((l.memoizedState = [t, e]), t);
  }
  function Nr(t, e) {
    var l = _t();
    e = e === void 0 ? null : e;
    var a = l.memoizedState;
    if (e !== null && ec(e, a[1])) return a[0];
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
  function dc(t, e, l) {
    return l === void 0 || ((Ke & 1073741824) !== 0 && (P & 261930) === 0)
      ? (t.memoizedState = e)
      : ((t.memoizedState = l), (t = Ho()), (w.lanes |= t), (ml |= t), l);
  }
  function xr(t, e, l, a) {
    return fe(l, e)
      ? l
      : va.current !== null
        ? ((t = dc(t, l, a)), fe(t, e) || (jt = !0), t)
        : (Ke & 42) === 0 || ((Ke & 1073741824) !== 0 && (P & 261930) === 0)
          ? ((jt = !0), (t.memoizedState = l))
          : ((t = Ho()), (w.lanes |= t), (ml |= t), e);
  }
  function Hr(t, e, l, a, u) {
    var n = x.p;
    x.p = n !== 0 && 8 > n ? n : 8;
    var c = z.T,
      s = {};
    ((z.T = s), vc(t, !1, e, l));
    try {
      var h = u(),
        g = z.S;
      if (
        (g !== null && g(s, h), h !== null && typeof h == "object" && typeof h.then == "function")
      ) {
        var T = Cy(h, a);
        nu(t, e, T, ye(t));
      } else nu(t, e, a, ye(t));
    } catch (D) {
      nu(t, e, { then: function () {}, status: "rejected", reason: D }, ye());
    } finally {
      ((x.p = n), c !== null && s.types !== null && (c.types = s.types), (z.T = c));
    }
  }
  function Qy() {}
  function yc(t, e, l, a) {
    if (t.tag !== 5) throw Error(r(476));
    var u = qr(t).queue;
    Hr(
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
  function mc() {
    return Zt(Tu);
  }
  function Br() {
    return _t().memoizedState;
  }
  function Yr() {
    return _t().memoizedState;
  }
  function By(t) {
    for (var e = t.return; e !== null; ) {
      switch (e.tag) {
        case 24:
        case 3:
          var l = ye();
          t = sl(l);
          var a = rl(e, t, l);
          (a !== null && (ae(a, e, l), tu(a, e, l)), (e = { cache: Ki() }), (t.payload = e));
          return;
      }
      e = e.return;
    }
  }
  function Yy(t, e, l) {
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
      vn(t) ? Xr(e, l) : ((l = Ni(t, e, l, a)), l !== null && (ae(l, t, a), Lr(l, e, a))));
  }
  function Gr(t, e, l) {
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
    if (vn(t)) Xr(e, u);
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
            return (Wu(t, e, u, 0), vt === null && Fu(), !1);
        } catch {}
      if (((l = Ni(t, e, u, a)), l !== null)) return (ae(l, t, a), Lr(l, e, a), !0);
    }
    return !1;
  }
  function vc(t, e, l, a) {
    if (
      ((a = {
        lane: 2,
        revertLane: Fc(),
        gesture: null,
        action: a,
        hasEagerState: !1,
        eagerState: null,
        next: null,
      }),
      vn(t))
    ) {
      if (e) throw Error(r(479));
    } else ((e = Ni(t, l, a, 2)), e !== null && ae(e, t, 2));
  }
  function vn(t) {
    var e = t.alternate;
    return t === w || (e !== null && e === w);
  }
  function Xr(t, e) {
    ga = sn = !0;
    var l = t.pending;
    (l === null ? (e.next = e) : ((e.next = l.next), (l.next = e)), (t.pending = e));
  }
  function Lr(t, e, l) {
    if ((l & 4194048) !== 0) {
      var a = e.lanes;
      ((a &= t.pendingLanes), (l |= a), (e.lanes = l), wf(t, l));
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
  var Zr = {
      readContext: Zt,
      use: hn,
      useCallback: function (t, e) {
        return ((Wt().memoizedState = [t, e === void 0 ? null : e]), t);
      },
      useContext: Zt,
      useEffect: Mr,
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
          (t = t.dispatch = Yy.bind(null, w, t)),
          [a.memoizedState, t]
        );
      },
      useRef: function (t) {
        var e = Wt();
        return ((t = { current: t }), (e.memoizedState = t));
      },
      useState: function (t) {
        t = sc(t);
        var e = t.queue,
          l = Gr.bind(null, w, e);
        return ((e.dispatch = l), [t.memoizedState, l]);
      },
      useDebugValue: hc,
      useDeferredValue: function (t, e) {
        var l = Wt();
        return dc(l, t, e);
      },
      useTransition: function () {
        var t = sc(!1);
        return ((t = Hr.bind(null, w, t.queue, !0, !1)), (Wt().memoizedState = t), [!1, t]);
      },
      useSyncExternalStore: function (t, e, l) {
        var a = w,
          u = Wt();
        if (et) {
          if (l === void 0) throw Error(r(407));
          l = l();
        } else {
          if (((l = e()), vt === null)) throw Error(r(349));
          (P & 127) !== 0 || rr(a, e, l);
        }
        u.memoizedState = l;
        var n = { value: l, getSnapshot: e };
        return (
          (u.queue = n),
          Mr(hr.bind(null, a, n, t), [t]),
          (a.flags |= 2048),
          pa(9, { destroy: void 0 }, or.bind(null, a, n, l, e), null),
          l
        );
      },
      useId: function () {
        var t = Wt(),
          e = vt.identifierPrefix;
        if (et) {
          var l = Ne,
            a = je;
          ((l = (a & ~(1 << (32 - ce(a) - 1))).toString(32) + l),
            (e = "_" + e + "R_" + l),
            (l = rn++),
            0 < l && (e += "H" + l.toString(32)),
            (e += "_"));
        } else ((l = jy++), (e = "_" + e + "r_" + l.toString(32) + "_"));
        return (t.memoizedState = e);
      },
      useHostTransitionStatus: mc,
      useFormState: Er,
      useActionState: Er,
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
      useMemoCache: ic,
      useCacheRefresh: function () {
        return (Wt().memoizedState = By.bind(null, w));
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
    gc = {
      readContext: Zt,
      use: hn,
      useCallback: jr,
      useContext: Zt,
      useEffect: oc,
      useImperativeHandle: Cr,
      useInsertionEffect: Dr,
      useLayoutEffect: Rr,
      useMemo: Nr,
      useReducer: dn,
      useRef: Ar,
      useState: function () {
        return dn(Ve);
      },
      useDebugValue: hc,
      useDeferredValue: function (t, e) {
        var l = _t();
        return xr(l, ot.memoizedState, t, e);
      },
      useTransition: function () {
        var t = dn(Ve)[0],
          e = _t().memoizedState;
        return [typeof t == "boolean" ? t : uu(t), e];
      },
      useSyncExternalStore: sr,
      useId: Br,
      useHostTransitionStatus: mc,
      useFormState: Tr,
      useActionState: Tr,
      useOptimistic: function (t, e) {
        var l = _t();
        return mr(l, ot, t, e);
      },
      useMemoCache: ic,
      useCacheRefresh: Yr,
    };
  gc.useEffectEvent = _r;
  var Kr = {
    readContext: Zt,
    use: hn,
    useCallback: jr,
    useContext: Zt,
    useEffect: oc,
    useImperativeHandle: Cr,
    useInsertionEffect: Dr,
    useLayoutEffect: Rr,
    useMemo: Nr,
    useReducer: fc,
    useRef: Ar,
    useState: function () {
      return fc(Ve);
    },
    useDebugValue: hc,
    useDeferredValue: function (t, e) {
      var l = _t();
      return ot === null ? dc(l, t, e) : xr(l, ot.memoizedState, t, e);
    },
    useTransition: function () {
      var t = fc(Ve)[0],
        e = _t().memoizedState;
      return [typeof t == "boolean" ? t : uu(t), e];
    },
    useSyncExternalStore: sr,
    useId: Br,
    useHostTransitionStatus: mc,
    useFormState: zr,
    useActionState: zr,
    useOptimistic: function (t, e) {
      var l = _t();
      return ot !== null ? mr(l, ot, t, e) : ((l.baseState = t), [t, l.queue.dispatch]);
    },
    useMemoCache: ic,
    useCacheRefresh: Yr,
  };
  Kr.useEffectEvent = _r;
  function Sc(t, e, l, a) {
    ((e = t.memoizedState),
      (l = l(a, e)),
      (l = l == null ? e : N({}, e, l)),
      (t.memoizedState = l),
      t.lanes === 0 && (t.updateQueue.baseState = l));
  }
  var pc = {
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
  function Vr(t, e, l, a, u, n, c) {
    return (
      (t = t.stateNode),
      typeof t.shouldComponentUpdate == "function"
        ? t.shouldComponentUpdate(a, n, c)
        : e.prototype && e.prototype.isPureReactComponent
          ? !Ja(l, a) || !Ja(u, n)
          : !0
    );
  }
  function Jr(t, e, l, a) {
    ((t = e.state),
      typeof e.componentWillReceiveProps == "function" && e.componentWillReceiveProps(l, a),
      typeof e.UNSAFE_componentWillReceiveProps == "function" &&
        e.UNSAFE_componentWillReceiveProps(l, a),
      e.state !== t && pc.enqueueReplaceState(e, e.state, null));
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
  function wr(t) {
    wu(t);
  }
  function Fr(t) {
    console.error(t);
  }
  function Wr(t) {
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
  function kr(t, e, l) {
    try {
      var a = t.onCaughtError;
      a(l.value, { componentStack: l.stack, errorBoundary: e.tag === 1 ? e.stateNode : null });
    } catch (u) {
      setTimeout(function () {
        throw u;
      });
    }
  }
  function bc(t, e, l) {
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
  function Ir(t, e, l, a) {
    var u = l.type.getDerivedStateFromError;
    if (typeof u == "function") {
      var n = a.value;
      ((t.payload = function () {
        return u(n);
      }),
        (t.callback = function () {
          kr(e, l, a);
        }));
    }
    var c = l.stateNode;
    c !== null &&
      typeof c.componentDidCatch == "function" &&
      (t.callback = function () {
        (kr(e, l, a),
          typeof u != "function" && (vl === null ? (vl = new Set([this])) : vl.add(this)));
        var s = a.stack;
        this.componentDidCatch(a.value, { componentStack: s !== null ? s : "" });
      });
  }
  function Gy(t, e, l, a, u) {
    if (((l.flags |= 32768), a !== null && typeof a == "object" && typeof a.then == "function")) {
      if (((e = l.alternate), e !== null && oa(e, l, u, !0), (l = re.current), l !== null)) {
        switch (l.tag) {
          case 31:
          case 13:
            return (
              ze === null ? Rn() : l.alternate === null && zt === 0 && (zt = 3),
              (l.flags &= -257),
              (l.flags |= 65536),
              (l.lanes = u),
              a === an
                ? (l.flags |= 16384)
                : ((e = l.updateQueue),
                  e === null ? (l.updateQueue = new Set([a])) : e.add(a),
                  Vc(t, a, u)),
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
                  Vc(t, a, u)),
              !1
            );
        }
        throw Error(r(435, l.tag));
      }
      return (Vc(t, a, u), Rn(), !1);
    }
    if (et)
      return (
        (e = re.current),
        e !== null
          ? ((e.flags & 65536) === 0 && (e.flags |= 256),
            (e.flags |= 65536),
            (e.lanes = u),
            a !== Yi && ((t = Error(r(422), { cause: a })), Wa(be(t, l))))
          : (a !== Yi && ((e = Error(r(423), { cause: a })), Wa(be(e, l))),
            (t = t.current.alternate),
            (t.flags |= 65536),
            (u &= -u),
            (t.lanes |= u),
            (a = be(a, l)),
            (u = bc(t.stateNode, a, u)),
            ki(t, u),
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
            (t = bc(l.stateNode, a, t)),
            ki(l, t),
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
                  (vl === null || !vl.has(n)))))
          )
            return (
              (l.flags |= 65536),
              (u &= -u),
              (l.lanes |= u),
              (u = $r(u)),
              Ir(u, t, l, a),
              ki(l, u),
              !1
            );
      }
      l = l.return;
    } while (l !== null);
    return !1;
  }
  var Ec = Error(r(461)),
    jt = !1;
  function Kt(t, e, l, a) {
    e.child = t === null ? lr(e, null, l, a) : Xl(e, t.child, l, a);
  }
  function Pr(t, e, l, a, u) {
    l = l.render;
    var n = e.ref;
    if ("ref" in a) {
      var c = {};
      for (var s in a) s !== "ref" && (c[s] = a[s]);
    } else c = a;
    return (
      Ql(e),
      (a = lc(t, e, l, c, n, u)),
      (s = ac()),
      t !== null && !jt
        ? (uc(t, e, u), Je(t, e, u))
        : (et && s && Qi(e), (e.flags |= 1), Kt(t, e, a, u), e.child)
    );
  }
  function to(t, e, l, a, u) {
    if (t === null) {
      var n = l.type;
      return typeof n == "function" && !xi(n) && n.defaultProps === void 0 && l.compare === null
        ? ((e.tag = 15), (e.type = n), eo(t, e, n, a, u))
        : ((t = $u(l.type, null, a, e, e.mode, u)), (t.ref = e.ref), (t.return = e), (e.child = t));
    }
    if (((n = t.child), !Rc(t, u))) {
      var c = n.memoizedProps;
      if (((l = l.compare), (l = l !== null ? l : Ja), l(c, a) && t.ref === e.ref))
        return Je(t, e, u);
    }
    return ((e.flags |= 1), (t = Ge(n, a)), (t.ref = e.ref), (t.return = e), (e.child = t));
  }
  function eo(t, e, l, a, u) {
    if (t !== null) {
      var n = t.memoizedProps;
      if (Ja(n, a) && t.ref === e.ref)
        if (((jt = !1), (e.pendingProps = a = n), Rc(t, u))) (t.flags & 131072) !== 0 && (jt = !0);
        else return ((e.lanes = t.lanes), Je(t, e, u));
    }
    return Tc(t, e, l, a, u);
  }
  function lo(t, e, l, a) {
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
        return ao(t, e, n, l, a);
      }
      if ((l & 536870912) !== 0)
        ((e.memoizedState = { baseLanes: 0, cachePool: null }),
          t !== null && en(e, n !== null ? n.cachePool : null),
          n !== null ? nr(e, n) : Ii(),
          ir(e));
      else return ((a = e.lanes = 536870912), ao(t, e, n !== null ? n.baseLanes | l : l, l, a));
    } else
      n !== null
        ? (en(e, n.cachePool), nr(e, n), hl(), (e.memoizedState = null))
        : (t !== null && en(e, null), Ii(), hl());
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
  function ao(t, e, l, a, u) {
    var n = Ji();
    return (
      (n = n === null ? null : { parent: Ut._currentValue, pool: n }),
      (e.memoizedState = { baseLanes: l, cachePool: n }),
      t !== null && en(e, null),
      Ii(),
      ir(e),
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
  function uo(t, e, l) {
    return (
      Xl(e, t.child, null, l),
      (t = Sn(e, e.pendingProps)),
      (t.flags |= 2),
      oe(e),
      (e.memoizedState = null),
      t
    );
  }
  function Xy(t, e, l) {
    var a = e.pendingProps,
      u = (e.flags & 128) !== 0;
    if (((e.flags &= -129), t === null)) {
      if (et) {
        if (a.mode === "hidden") return ((t = Sn(e, a)), (e.lanes = 536870912), cu(null, t));
        if (
          (tc(e),
          (t = pt)
            ? ((t = gh(t, Oe)),
              (t = t !== null && t.data === "&" ? t : null),
              t !== null &&
                ((e.memoizedState = {
                  dehydrated: t,
                  treeContext: ul !== null ? { id: je, overflow: Ne } : null,
                  retryLane: 536870912,
                  hydrationErrors: null,
                }),
                (l = Xs(t)),
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
      if ((tc(e), u))
        if (e.flags & 256) ((e.flags &= -257), (e = uo(t, e, l)));
        else if (e.memoizedState !== null) ((e.child = t.child), (e.flags |= 128), (e = null));
        else throw Error(r(558));
      else if ((jt || oa(t, e, l, !1), (u = (l & t.childLanes) !== 0), jt || u)) {
        if (((a = vt), a !== null && ((c = Ff(a, l)), c !== 0 && c !== n.retryLane)))
          throw ((n.retryLane = c), Nl(t, c), ae(a, t, c), Ec);
        (Rn(), (e = uo(t, e, l)));
      } else
        ((t = n.treeContext),
          (pt = Ae(c.nextSibling)),
          (Lt = e),
          (et = !0),
          (nl = null),
          (Oe = !1),
          t !== null && Ks(e, t),
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
  function Tc(t, e, l, a, u) {
    return (
      Ql(e),
      (l = lc(t, e, l, a, void 0, u)),
      (a = ac()),
      t !== null && !jt
        ? (uc(t, e, u), Je(t, e, u))
        : (et && a && Qi(e), (e.flags |= 1), Kt(t, e, l, u), e.child)
    );
  }
  function no(t, e, l, a, u, n) {
    return (
      Ql(e),
      (e.updateQueue = null),
      (l = fr(e, a, l, u)),
      cr(t),
      (a = ac()),
      t !== null && !jt
        ? (uc(t, e, n), Je(t, e, n))
        : (et && a && Qi(e), (e.flags |= 1), Kt(t, e, l, n), e.child)
    );
  }
  function io(t, e, l, a, u) {
    if ((Ql(e), e.stateNode === null)) {
      var n = ca,
        c = l.contextType;
      (typeof c == "object" && c !== null && (n = Zt(c)),
        (n = new l(a, n)),
        (e.memoizedState = n.state !== null && n.state !== void 0 ? n.state : null),
        (n.updater = pc),
        (e.stateNode = n),
        (n._reactInternals = e),
        (n = e.stateNode),
        (n.props = a),
        (n.state = e.memoizedState),
        (n.refs = {}),
        Fi(e),
        (c = l.contextType),
        (n.context = typeof c == "object" && c !== null ? Zt(c) : ca),
        (n.state = e.memoizedState),
        (c = l.getDerivedStateFromProps),
        typeof c == "function" && (Sc(e, l, c, a), (n.state = e.memoizedState)),
        typeof l.getDerivedStateFromProps == "function" ||
          typeof n.getSnapshotBeforeUpdate == "function" ||
          (typeof n.UNSAFE_componentWillMount != "function" &&
            typeof n.componentWillMount != "function") ||
          ((c = n.state),
          typeof n.componentWillMount == "function" && n.componentWillMount(),
          typeof n.UNSAFE_componentWillMount == "function" && n.UNSAFE_componentWillMount(),
          c !== n.state && pc.enqueueReplaceState(n, n.state, null),
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
      var g = n.context,
        T = l.contextType;
      ((c = ca), typeof T == "object" && T !== null && (c = Zt(T)));
      var D = l.getDerivedStateFromProps;
      ((T = typeof D == "function" || typeof n.getSnapshotBeforeUpdate == "function"),
        (s = e.pendingProps !== s),
        T ||
          (typeof n.UNSAFE_componentWillReceiveProps != "function" &&
            typeof n.componentWillReceiveProps != "function") ||
          ((s || g !== c) && Jr(e, n, a, c)),
        (fl = !1));
      var S = e.memoizedState;
      ((n.state = S),
        lu(e, a, n, u),
        eu(),
        (g = e.memoizedState),
        s || S !== g || fl
          ? (typeof D == "function" && (Sc(e, l, D, a), (g = e.memoizedState)),
            (h = fl || Vr(e, l, h, a, S, g, c))
              ? (T ||
                  (typeof n.UNSAFE_componentWillMount != "function" &&
                    typeof n.componentWillMount != "function") ||
                  (typeof n.componentWillMount == "function" && n.componentWillMount(),
                  typeof n.UNSAFE_componentWillMount == "function" &&
                    n.UNSAFE_componentWillMount()),
                typeof n.componentDidMount == "function" && (e.flags |= 4194308))
              : (typeof n.componentDidMount == "function" && (e.flags |= 4194308),
                (e.memoizedProps = a),
                (e.memoizedState = g)),
            (n.props = a),
            (n.state = g),
            (n.context = c),
            (a = h))
          : (typeof n.componentDidMount == "function" && (e.flags |= 4194308), (a = !1)));
    } else {
      ((n = e.stateNode),
        Wi(t, e),
        (c = e.memoizedProps),
        (T = Zl(l, c)),
        (n.props = T),
        (D = e.pendingProps),
        (S = n.context),
        (g = l.contextType),
        (h = ca),
        typeof g == "object" && g !== null && (h = Zt(g)),
        (s = l.getDerivedStateFromProps),
        (g = typeof s == "function" || typeof n.getSnapshotBeforeUpdate == "function") ||
          (typeof n.UNSAFE_componentWillReceiveProps != "function" &&
            typeof n.componentWillReceiveProps != "function") ||
          ((c !== D || S !== h) && Jr(e, n, a, h)),
        (fl = !1),
        (S = e.memoizedState),
        (n.state = S),
        lu(e, a, n, u),
        eu());
      var b = e.memoizedState;
      c !== D || S !== b || fl || (t !== null && t.dependencies !== null && Pu(t.dependencies))
        ? (typeof s == "function" && (Sc(e, l, s, a), (b = e.memoizedState)),
          (T =
            fl ||
            Vr(e, l, T, a, S, b, h) ||
            (t !== null && t.dependencies !== null && Pu(t.dependencies)))
            ? (g ||
                (typeof n.UNSAFE_componentWillUpdate != "function" &&
                  typeof n.componentWillUpdate != "function") ||
                (typeof n.componentWillUpdate == "function" && n.componentWillUpdate(a, b, h),
                typeof n.UNSAFE_componentWillUpdate == "function" &&
                  n.UNSAFE_componentWillUpdate(a, b, h)),
              typeof n.componentDidUpdate == "function" && (e.flags |= 4),
              typeof n.getSnapshotBeforeUpdate == "function" && (e.flags |= 1024))
            : (typeof n.componentDidUpdate != "function" ||
                (c === t.memoizedProps && S === t.memoizedState) ||
                (e.flags |= 4),
              typeof n.getSnapshotBeforeUpdate != "function" ||
                (c === t.memoizedProps && S === t.memoizedState) ||
                (e.flags |= 1024),
              (e.memoizedProps = a),
              (e.memoizedState = b)),
          (n.props = a),
          (n.state = b),
          (n.context = h),
          (a = T))
        : (typeof n.componentDidUpdate != "function" ||
            (c === t.memoizedProps && S === t.memoizedState) ||
            (e.flags |= 4),
          typeof n.getSnapshotBeforeUpdate != "function" ||
            (c === t.memoizedProps && S === t.memoizedState) ||
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
  function co(t, e, l, a) {
    return (Hl(), (e.flags |= 256), Kt(t, e, l, a), e.child);
  }
  var Oc = { dehydrated: null, treeContext: null, retryLane: 0, hydrationErrors: null };
  function zc(t) {
    return { baseLanes: t, cachePool: ks() };
  }
  function Ac(t, e, l) {
    return ((t = t !== null ? t.childLanes & ~l : 0), e && (t |= de), t);
  }
  function fo(t, e, l) {
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
            ? ((t = gh(t, Oe)),
              (t = t !== null && t.data !== "&" ? t : null),
              t !== null &&
                ((e.memoizedState = {
                  dehydrated: t,
                  treeContext: ul !== null ? { id: je, overflow: Ne } : null,
                  retryLane: 536870912,
                  hydrationErrors: null,
                }),
                (l = Xs(t)),
                (l.return = e),
                (e.child = l),
                (Lt = e),
                (pt = null)))
            : (t = null),
          t === null)
        )
          throw il(e);
        return (ff(t) ? (e.lanes = 32) : (e.lanes = 536870912), null);
      }
      var s = a.children;
      return (
        (a = a.fallback),
        u
          ? (hl(),
            (u = e.mode),
            (s = bn({ mode: "hidden", children: s }, u)),
            (a = xl(a, u, l, null)),
            (s.return = e),
            (a.return = e),
            (s.sibling = a),
            (e.child = s),
            (a = e.child),
            (a.memoizedState = zc(l)),
            (a.childLanes = Ac(t, c, l)),
            (e.memoizedState = Oc),
            cu(null, a))
          : (ol(e), Mc(e, s))
      );
    }
    var h = t.memoizedState;
    if (h !== null && ((s = h.dehydrated), s !== null)) {
      if (n)
        e.flags & 256
          ? (ol(e), (e.flags &= -257), (e = _c(t, e, l)))
          : e.memoizedState !== null
            ? (hl(), (e.child = t.child), (e.flags |= 128), (e = null))
            : (hl(),
              (s = a.fallback),
              (u = e.mode),
              (a = bn({ mode: "visible", children: a.children }, u)),
              (s = xl(s, u, l, null)),
              (s.flags |= 2),
              (a.return = e),
              (s.return = e),
              (a.sibling = s),
              (e.child = a),
              Xl(e, t.child, null, l),
              (a = e.child),
              (a.memoizedState = zc(l)),
              (a.childLanes = Ac(t, c, l)),
              (e.memoizedState = Oc),
              (e = cu(null, a)));
      else if ((ol(e), ff(s))) {
        if (((c = s.nextSibling && s.nextSibling.dataset), c)) var g = c.dgst;
        ((c = g),
          (a = Error(r(419))),
          (a.stack = ""),
          (a.digest = c),
          Wa({ value: a, source: null, stack: null }),
          (e = _c(t, e, l)));
      } else if ((jt || oa(t, e, l, !1), (c = (l & t.childLanes) !== 0), jt || c)) {
        if (((c = vt), c !== null && ((a = Ff(c, l)), a !== 0 && a !== h.retryLane)))
          throw ((h.retryLane = a), Nl(t, a), ae(c, t, a), Ec);
        (cf(s) || Rn(), (e = _c(t, e, l)));
      } else
        cf(s)
          ? ((e.flags |= 192), (e.child = t.child), (e = null))
          : ((t = h.treeContext),
            (pt = Ae(s.nextSibling)),
            (Lt = e),
            (et = !0),
            (nl = null),
            (Oe = !1),
            t !== null && Ks(e, t),
            (e = Mc(e, a.children)),
            (e.flags |= 4096));
      return e;
    }
    return u
      ? (hl(),
        (s = a.fallback),
        (u = e.mode),
        (h = t.child),
        (g = h.sibling),
        (a = Ge(h, { mode: "hidden", children: a.children })),
        (a.subtreeFlags = h.subtreeFlags & 65011712),
        g !== null ? (s = Ge(g, s)) : ((s = xl(s, u, l, null)), (s.flags |= 2)),
        (s.return = e),
        (a.return = e),
        (a.sibling = s),
        (e.child = a),
        cu(null, a),
        (a = e.child),
        (s = t.child.memoizedState),
        s === null
          ? (s = zc(l))
          : ((u = s.cachePool),
            u !== null
              ? ((h = Ut._currentValue), (u = u.parent !== h ? { parent: h, pool: h } : u))
              : (u = ks()),
            (s = { baseLanes: s.baseLanes | l, cachePool: u })),
        (a.memoizedState = s),
        (a.childLanes = Ac(t, c, l)),
        (e.memoizedState = Oc),
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
  function Mc(t, e) {
    return ((e = bn({ mode: "visible", children: e }, t.mode)), (e.return = t), (t.child = e));
  }
  function bn(t, e) {
    return ((t = se(22, t, null, e)), (t.lanes = 0), t);
  }
  function _c(t, e, l) {
    return (
      Xl(e, t.child, null, l),
      (t = Mc(e, e.pendingProps.children)),
      (t.flags |= 2),
      (e.memoizedState = null),
      t
    );
  }
  function so(t, e, l) {
    t.lanes |= e;
    var a = t.alternate;
    (a !== null && (a.lanes |= e), Li(t.return, e, l));
  }
  function Dc(t, e, l, a, u, n) {
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
  function ro(t, e, l) {
    var a = e.pendingProps,
      u = a.revealOrder,
      n = a.tail;
    a = a.children;
    var c = Mt.current,
      s = (c & 2) !== 0;
    if (
      (s ? ((c = (c & 1) | 2), (e.flags |= 128)) : (c &= 1),
      H(Mt, c),
      Kt(t, e, a, l),
      (a = et ? Fa : 0),
      !s && t !== null && (t.flags & 128) !== 0)
    )
      t: for (t = e.child; t !== null; ) {
        if (t.tag === 13) t.memoizedState !== null && so(t, l, e);
        else if (t.tag === 19) so(t, l, e);
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
          Dc(e, !1, u, l, n, a));
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
        Dc(e, !0, l, null, n, a);
        break;
      case "together":
        Dc(e, !1, null, null, void 0, a);
        break;
      default:
        e.memoizedState = null;
    }
    return e.child;
  }
  function Je(t, e, l) {
    if (
      (t !== null && (e.dependencies = t.dependencies), (ml |= e.lanes), (l & e.childLanes) === 0)
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
  function Rc(t, e) {
    return (t.lanes & e) !== 0 ? !0 : ((t = t.dependencies), !!(t !== null && Pu(t)));
  }
  function Ly(t, e, l) {
    switch (e.tag) {
      case 3:
        (Ft(e, e.stateNode.containerInfo), cl(e, Ut, t.memoizedState.cache), Hl());
        break;
      case 27:
      case 5:
        Na(e);
        break;
      case 4:
        Ft(e, e.stateNode.containerInfo);
        break;
      case 10:
        cl(e, e.type, e.memoizedProps.value);
        break;
      case 31:
        if (e.memoizedState !== null) return ((e.flags |= 128), tc(e), null);
        break;
      case 13:
        var a = e.memoizedState;
        if (a !== null)
          return a.dehydrated !== null
            ? (ol(e), (e.flags |= 128), null)
            : (l & e.child.childLanes) !== 0
              ? fo(t, e, l)
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
          if (a) return ro(t, e, l);
          e.flags |= 128;
        }
        if (
          ((u = e.memoizedState),
          u !== null && ((u.rendering = null), (u.tail = null), (u.lastEffect = null)),
          H(Mt, Mt.current),
          a)
        )
          break;
        return null;
      case 22:
        return ((e.lanes = 0), lo(t, e, l, e.pendingProps));
      case 24:
        cl(e, Ut, t.memoizedState.cache);
    }
    return Je(t, e, l);
  }
  function oo(t, e, l) {
    if (t !== null)
      if (t.memoizedProps !== e.pendingProps) jt = !0;
      else {
        if (!Rc(t, l) && (e.flags & 128) === 0) return ((jt = !1), Ly(t, e, l));
        jt = (t.flags & 131072) !== 0;
      }
    else ((jt = !1), et && (e.flags & 1048576) !== 0 && Zs(e, Fa, e.index));
    switch (((e.lanes = 0), e.tag)) {
      case 16:
        t: {
          var a = e.pendingProps;
          if (((t = Yl(e.elementType)), (e.type = t), typeof t == "function"))
            xi(t)
              ? ((a = Zl(t, a)), (e.tag = 1), (e = io(null, e, t, a, l)))
              : ((e.tag = 0), (e = Tc(null, e, t, a, l)));
          else {
            if (t != null) {
              var u = t.$$typeof;
              if (u === Rt) {
                ((e.tag = 11), (e = Pr(null, e, t, a, l)));
                break t;
              } else if (u === K) {
                ((e.tag = 14), (e = to(null, e, t, a, l)));
                break t;
              }
            }
            throw ((e = qe(t) || t), Error(r(306, e, "")));
          }
        }
        return e;
      case 0:
        return Tc(t, e, e.type, e.pendingProps, l);
      case 1:
        return ((a = e.type), (u = Zl(a, e.pendingProps)), io(t, e, a, u, l));
      case 3:
        t: {
          if ((Ft(e, e.stateNode.containerInfo), t === null)) throw Error(r(387));
          a = e.pendingProps;
          var n = e.memoizedState;
          ((u = n.element), Wi(t, e), lu(e, a, null, l));
          var c = e.memoizedState;
          if (
            ((a = c.cache),
            cl(e, Ut, a),
            a !== n.cache && Zi(e, [Ut], l, !0),
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
              e = co(t, e, a, l);
              break t;
            } else if (a !== u) {
              ((u = be(Error(r(424)), e)), Wa(u), (e = co(t, e, a, l)));
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
                  l = lr(e, null, a, l),
                  e.child = l;
                l;
              )
                ((l.flags = (l.flags & -3) | 4096), (l = l.sibling));
          else {
            if ((Hl(), a === u)) {
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
            ? (l = Oh(e.type, null, e.pendingProps, null))
              ? (e.memoizedState = l)
              : et ||
                ((l = e.type),
                (t = e.pendingProps),
                (a = qn(k.current).createElement(l)),
                (a[Xt] = e),
                (a[$t] = t),
                Vt(a, l, t),
                Yt(a),
                (e.stateNode = a))
            : (e.memoizedState = Oh(e.type, t.memoizedProps, e.pendingProps, t.memoizedState)),
          null
        );
      case 27:
        return (
          Na(e),
          t === null &&
            et &&
            ((a = e.stateNode = bh(e.type, e.pendingProps, k.current)),
            (Lt = e),
            (Oe = !0),
            (u = pt),
            bl(e.type) ? ((sf = u), (pt = Ae(a.firstChild))) : (pt = u)),
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
              ((a = Sm(a, e.type, e.pendingProps, Oe)),
              a !== null
                ? ((e.stateNode = a), (Lt = e), (pt = Ae(a.firstChild)), (Oe = !1), (u = !0))
                : (u = !1)),
            u || il(e)),
          Na(e),
          (u = e.type),
          (n = e.pendingProps),
          (c = t !== null ? t.memoizedProps : null),
          (a = n.children),
          af(u, n) ? (a = null) : c !== null && af(u, c) && (e.flags |= 32),
          e.memoizedState !== null && ((u = lc(t, e, Ny, null, null, l)), (Tu._currentValue = u)),
          pn(t, e),
          Kt(t, e, a, l),
          e.child
        );
      case 6:
        return (
          t === null &&
            et &&
            ((t = l = pt) &&
              ((l = pm(l, e.pendingProps, Oe)),
              l !== null ? ((e.stateNode = l), (Lt = e), (pt = null), (t = !0)) : (t = !1)),
            t || il(e)),
          null
        );
      case 13:
        return fo(t, e, l);
      case 4:
        return (
          Ft(e, e.stateNode.containerInfo),
          (a = e.pendingProps),
          t === null ? (e.child = Xl(e, null, a, l)) : Kt(t, e, a, l),
          e.child
        );
      case 11:
        return Pr(t, e, e.type, e.pendingProps, l);
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
          Ql(e),
          (u = Zt(u)),
          (a = a(u)),
          (e.flags |= 1),
          Kt(t, e, a, l),
          e.child
        );
      case 14:
        return to(t, e, e.type, e.pendingProps, l);
      case 15:
        return eo(t, e, e.type, e.pendingProps, l);
      case 19:
        return ro(t, e, l);
      case 31:
        return Xy(t, e, l);
      case 22:
        return lo(t, e, l, e.pendingProps);
      case 24:
        return (
          Ql(e),
          (a = Zt(Ut)),
          t === null
            ? ((u = Ji()),
              u === null &&
                ((u = vt),
                (n = Ki()),
                (u.pooledCache = n),
                n.refCount++,
                n !== null && (u.pooledCacheLanes |= l),
                (u = n)),
              (e.memoizedState = { parent: a, cache: u }),
              Fi(e),
              cl(e, Ut, u))
            : ((t.lanes & l) !== 0 && (Wi(t, e), lu(e, null, null, l), eu()),
              (u = t.memoizedState),
              (n = e.memoizedState),
              u.parent !== a
                ? ((u = { parent: a, cache: a }),
                  (e.memoizedState = u),
                  e.lanes === 0 && (e.memoizedState = e.updateQueue.baseState = u),
                  cl(e, Ut, a))
                : ((a = n.cache), cl(e, Ut, a), a !== u.cache && Zi(e, [Ut], l, !0))),
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
        else if (Yo()) t.flags |= 8192;
        else throw ((Gl = an), wi);
    } else t.flags &= -16777217;
  }
  function ho(t, e) {
    if (e.type !== "stylesheet" || (e.state.loading & 4) !== 0) t.flags &= -16777217;
    else if (((t.flags |= 16777216), !Dh(e)))
      if (Yo()) t.flags |= 8192;
      else throw ((Gl = an), wi);
  }
  function En(t, e) {
    (e !== null && (t.flags |= 4),
      t.flags & 16384 && ((e = t.tag !== 22 ? Vf() : 536870912), (t.lanes |= e), (Oa |= e)));
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
  function Zy(t, e, l) {
    var a = e.pendingProps;
    switch ((Bi(e), e.tag)) {
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
          Ze(Ut),
          At(),
          l.pendingContext && ((l.context = l.pendingContext), (l.pendingContext = null)),
          (t === null || t.child === null) &&
            (ra(e)
              ? we(e)
              : t === null ||
                (t.memoizedState.isDehydrated && (e.flags & 256) === 0) ||
                ((e.flags |= 1024), Gi())),
          bt(e),
          null
        );
      case 26:
        var u = e.type,
          n = e.memoizedState;
        return (
          t === null
            ? (we(e), n !== null ? (bt(e), ho(e, n)) : (bt(e), Uc(e, u, null, a, l)))
            : n
              ? n !== t.memoizedState
                ? (we(e), bt(e), ho(e, n))
                : (bt(e), (e.flags &= -16777217))
              : ((t = t.memoizedProps), t !== a && we(e), bt(e), Uc(e, u, t, a, l)),
          null
        );
      case 27:
        if ((Cu(e), (l = k.current), (u = e.type), t !== null && e.stateNode != null))
          t.memoizedProps !== a && we(e);
        else {
          if (!a) {
            if (e.stateNode === null) throw Error(r(166));
            return (bt(e), null);
          }
          ((t = B.current), ra(e) ? Vs(e) : ((t = bh(u, a, l)), (e.stateNode = t), we(e)));
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
          if (((n = B.current), ra(e))) Vs(e);
          else {
            var c = qn(k.current);
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
            ((n[Xt] = e), (n[$t] = a));
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
          if (((t = k.current), ra(e))) {
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
                sh(t.nodeValue, l)
              )),
              t || il(e, !0));
          } else ((t = qn(t).createTextNode(a)), (t[Xt] = e), (e.stateNode = t));
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
            } else (Hl(), (e.flags & 128) === 0 && (e.memoizedState = null), (e.flags |= 4));
            (bt(e), (t = !1));
          } else
            ((l = Gi()),
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
            } else (Hl(), (e.flags & 128) === 0 && (e.memoizedState = null), (e.flags |= 4));
            (bt(e), (u = !1));
          } else
            ((u = Gi()),
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
        return (At(), t === null && Ic(e.stateNode.containerInfo), bt(e), null);
      case 10:
        return (Ze(e.type), bt(e), null);
      case 19:
        if ((R(Mt), (a = e.memoizedState), a === null)) return (bt(e), null);
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
                    (Gs(l, t), (l = l.sibling));
                  return (H(Mt, (Mt.current & 1) | 2), et && Xe(e, a.treeForkCount), e.child);
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
            H(Mt, u ? (l & 1) | 2 : l & 1),
            et && Xe(e, a.treeForkCount),
            t)
          : (bt(e), null);
      case 22:
      case 23:
        return (
          oe(e),
          Pi(),
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
          t !== null && R(Bl),
          null
        );
      case 24:
        return (
          (l = null),
          t !== null && (l = t.memoizedState.cache),
          e.memoizedState.cache !== l && (e.flags |= 2048),
          Ze(Ut),
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
  function Ky(t, e) {
    switch ((Bi(e), e.tag)) {
      case 1:
        return ((t = e.flags), t & 65536 ? ((e.flags = (t & -65537) | 128), e) : null);
      case 3:
        return (
          Ze(Ut),
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
          Hl();
        }
        return ((t = e.flags), t & 65536 ? ((e.flags = (t & -65537) | 128), e) : null);
      case 13:
        if ((oe(e), (t = e.memoizedState), t !== null && t.dehydrated !== null)) {
          if (e.alternate === null) throw Error(r(340));
          Hl();
        }
        return ((t = e.flags), t & 65536 ? ((e.flags = (t & -65537) | 128), e) : null);
      case 19:
        return (R(Mt), null);
      case 4:
        return (At(), null);
      case 10:
        return (Ze(e.type), null);
      case 22:
      case 23:
        return (
          oe(e),
          Pi(),
          t !== null && R(Bl),
          (t = e.flags),
          t & 65536 ? ((e.flags = (t & -65537) | 128), e) : null
        );
      case 24:
        return (Ze(Ut), null);
      case 25:
        return null;
      default:
        return null;
    }
  }
  function yo(t, e) {
    switch ((Bi(e), e.tag)) {
      case 3:
        (Ze(Ut), At());
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
        R(Mt);
        break;
      case 10:
        Ze(e.type);
        break;
      case 22:
      case 23:
        (oe(e), Pi(), t !== null && R(Bl));
        break;
      case 24:
        Ze(Ut);
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
                g = s;
              try {
                g();
              } catch (T) {
                ft(u, h, T);
              }
            }
          }
          a = a.next;
        } while (a !== n);
      }
    } catch (T) {
      ft(e, e.return, T);
    }
  }
  function mo(t) {
    var e = t.updateQueue;
    if (e !== null) {
      var l = t.stateNode;
      try {
        ur(e, l);
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
  function xe(t, e) {
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
  function go(t) {
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
  function Cc(t, e, l) {
    try {
      var a = t.stateNode;
      (hm(a, t.type, l, e), (a[$t] = e));
    } catch (u) {
      ft(t, t.return, u);
    }
  }
  function So(t) {
    return (
      t.tag === 5 || t.tag === 3 || t.tag === 26 || (t.tag === 27 && bl(t.type)) || t.tag === 4
    );
  }
  function jc(t) {
    t: for (;;) {
      for (; t.sibling === null; ) {
        if (t.return === null || So(t.return)) return null;
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
  function po(t) {
    var e = t.stateNode,
      l = t.memoizedProps;
    try {
      for (var a = t.type, u = e.attributes; u.length; ) e.removeAttributeNode(u[0]);
      (Vt(e, a, l), (e[Xt] = t), (e[$t] = l));
    } catch (n) {
      ft(t, t.return, n);
    }
  }
  var Fe = !1,
    Nt = !1,
    xc = !1,
    bo = typeof WeakSet == "function" ? WeakSet : Set,
    Gt = null;
  function Vy(t, e) {
    if (((t = t.containerInfo), (ef = Zn), (t = Cs(t)), _i(t))) {
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
              g = 0,
              T = 0,
              D = t,
              S = null;
            e: for (;;) {
              for (
                var b;
                D !== l || (u !== 0 && D.nodeType !== 3) || (s = c + u),
                  D !== n || (a !== 0 && D.nodeType !== 3) || (h = c + a),
                  D.nodeType === 3 && (c += D.nodeValue.length),
                  (b = D.firstChild) !== null;
              )
                ((S = D), (D = b));
              for (;;) {
                if (D === t) break e;
                if (
                  (S === l && ++g === u && (s = c),
                  S === n && ++T === a && (h = c),
                  (b = D.nextSibling) !== null)
                )
                  break;
                ((D = S), (S = D.parentNode));
              }
              D = b;
            }
            l = s === -1 || h === -1 ? null : { start: s, end: h };
          } else l = null;
        }
      l = l || { start: 0, end: 0 };
    } else l = null;
    for (lf = { focusedElem: t, selectionRange: l }, Zn = !1, Gt = e; Gt !== null; )
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
                  var Q = Zl(l.type, u);
                  ((t = a.getSnapshotBeforeUpdate(Q, n)),
                    (a.__reactInternalSnapshotBeforeUpdate = t));
                } catch (X) {
                  ft(l, l.return, X);
                }
              }
              break;
            case 3:
              if ((t & 1024) !== 0) {
                if (((t = e.stateNode.containerInfo), (l = t.nodeType), l === 9)) nf(t);
                else if (l === 1)
                  switch (t.nodeName) {
                    case "HEAD":
                    case "HTML":
                    case "BODY":
                      nf(t);
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
  function Eo(t, e, l) {
    var a = l.flags;
    switch (l.tag) {
      case 0:
      case 11:
      case 15:
        (ke(t, l), a & 4 && su(5, l));
        break;
      case 1:
        if ((ke(t, l), a & 4))
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
        (a & 64 && mo(l), a & 512 && ru(l, l.return));
        break;
      case 3:
        if ((ke(t, l), a & 64 && ((t = l.updateQueue), t !== null))) {
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
            ur(t, e);
          } catch (c) {
            ft(l, l.return, c);
          }
        }
        break;
      case 27:
        e === null && a & 4 && po(l);
      case 26:
      case 5:
        (ke(t, l), e === null && a & 4 && go(l), a & 512 && ru(l, l.return));
        break;
      case 12:
        ke(t, l);
        break;
      case 31:
        (ke(t, l), a & 4 && zo(t, l));
        break;
      case 13:
        (ke(t, l),
          a & 4 && Ao(t, l),
          a & 64 &&
            ((t = l.memoizedState),
            t !== null && ((t = t.dehydrated), t !== null && ((l = tm.bind(null, l)), bm(t, l)))));
        break;
      case 22:
        if (((a = l.memoizedState !== null || Fe), !a)) {
          ((e = (e !== null && e.memoizedState !== null) || Nt), (u = Fe));
          var n = Nt;
          ((Fe = a),
            (Nt = e) && !n ? $e(t, l, (l.subtreeFlags & 8772) !== 0) : ke(t, l),
            (Fe = u),
            (Nt = n));
        }
        break;
      case 30:
        break;
      default:
        ke(t, l);
    }
  }
  function To(t) {
    var e = t.alternate;
    (e !== null && ((t.alternate = null), To(e)),
      (t.child = null),
      (t.deletions = null),
      (t.sibling = null),
      t.tag === 5 && ((e = t.stateNode), e !== null && ri(e)),
      (t.stateNode = null),
      (t.return = null),
      (t.dependencies = null),
      (t.memoizedProps = null),
      (t.memoizedState = null),
      (t.pendingProps = null),
      (t.stateNode = null),
      (t.updateQueue = null));
  }
  var Tt = null,
    Pt = !1;
  function We(t, e, l) {
    for (l = l.child; l !== null; ) (Oo(t, e, l), (l = l.sibling));
  }
  function Oo(t, e, l) {
    if (ie && typeof ie.onCommitFiberUnmount == "function")
      try {
        ie.onCommitFiberUnmount(xa, l);
      } catch {}
    switch (l.tag) {
      case 26:
        (Nt || xe(l, e),
          We(t, e, l),
          l.memoizedState
            ? l.memoizedState.count--
            : l.stateNode && ((l = l.stateNode), l.parentNode.removeChild(l)));
        break;
      case 27:
        Nt || xe(l, e);
        var a = Tt,
          u = Pt;
        (bl(l.type) && ((Tt = l.stateNode), (Pt = !1)),
          We(t, e, l),
          pu(l.stateNode),
          (Tt = a),
          (Pt = u));
        break;
      case 5:
        Nt || xe(l, e);
      case 6:
        if (((a = Tt), (u = Pt), (Tt = null), We(t, e, l), (Tt = a), (Pt = u), Tt !== null))
          if (Pt)
            try {
              (Tt.nodeType === 9
                ? Tt.body
                : Tt.nodeName === "HTML"
                  ? Tt.ownerDocument.body
                  : Tt
              ).removeChild(l.stateNode);
            } catch (n) {
              ft(l, e, n);
            }
          else
            try {
              Tt.removeChild(l.stateNode);
            } catch (n) {
              ft(l, e, n);
            }
        break;
      case 18:
        Tt !== null &&
          (Pt
            ? ((t = Tt),
              mh(
                t.nodeType === 9 ? t.body : t.nodeName === "HTML" ? t.ownerDocument.body : t,
                l.stateNode,
              ),
              Ca(t))
            : mh(Tt, l.stateNode));
        break;
      case 4:
        ((a = Tt),
          (u = Pt),
          (Tt = l.stateNode.containerInfo),
          (Pt = !0),
          We(t, e, l),
          (Tt = a),
          (Pt = u));
        break;
      case 0:
      case 11:
      case 14:
      case 15:
        (dl(2, l, e), Nt || dl(4, l, e), We(t, e, l));
        break;
      case 1:
        (Nt ||
          (xe(l, e), (a = l.stateNode), typeof a.componentWillUnmount == "function" && vo(l, e, a)),
          We(t, e, l));
        break;
      case 21:
        We(t, e, l);
        break;
      case 22:
        ((Nt = (a = Nt) || l.memoizedState !== null), We(t, e, l), (Nt = a));
        break;
      default:
        We(t, e, l);
    }
  }
  function zo(t, e) {
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
  function Ao(t, e) {
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
  function Jy(t) {
    switch (t.tag) {
      case 31:
      case 13:
      case 19:
        var e = t.stateNode;
        return (e === null && (e = t.stateNode = new bo()), e);
      case 22:
        return (
          (t = t.stateNode), (e = t._retryCache), e === null && (e = t._retryCache = new bo()), e
        );
      default:
        throw Error(r(435, t.tag));
    }
  }
  function On(t, e) {
    var l = Jy(t);
    e.forEach(function (a) {
      if (!l.has(a)) {
        l.add(a);
        var u = em.bind(null, t, a);
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
                ((Tt = s.stateNode), (Pt = !1));
                break t;
              }
              break;
            case 5:
              ((Tt = s.stateNode), (Pt = !1));
              break t;
            case 3:
            case 4:
              ((Tt = s.stateNode.containerInfo), (Pt = !0));
              break t;
          }
          s = s.return;
        }
        if (Tt === null) throw Error(r(160));
        (Oo(n, c, u),
          (Tt = null),
          (Pt = !1),
          (n = u.alternate),
          n !== null && (n.return = null),
          (u.return = null));
      }
    if (e.subtreeFlags & 13886) for (e = e.child; e !== null; ) (Mo(e, t), (e = e.sibling));
  }
  var Ue = null;
  function Mo(t, e) {
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
          a & 512 && (Nt || l === null || xe(l, l.return)),
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
        var u = Ue;
        if ((te(e, t), ee(t), a & 512 && (Nt || l === null || xe(l, l.return)), a & 4)) {
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
                          n[Qa] ||
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
                      var c = Mh("link", "href", u).get(a + (l.href || ""));
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
                      if ((c = Mh("meta", "content", u).get(a + (l.content || "")))) {
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
              } else _h(u, t.type, t.stateNode);
            else t.stateNode = Ah(u, a, t.memoizedProps);
          else
            n !== a
              ? (n === null
                  ? l.stateNode !== null && ((l = l.stateNode), l.parentNode.removeChild(l))
                  : n.count--,
                a === null ? _h(u, t.type, t.stateNode) : Ah(u, a, t.memoizedProps))
              : a === null && t.stateNode !== null && Cc(t, t.memoizedProps, l.memoizedProps);
        }
        break;
      case 27:
        (te(e, t),
          ee(t),
          a & 512 && (Nt || l === null || xe(l, l.return)),
          l !== null && a & 4 && Cc(t, t.memoizedProps, l.memoizedProps));
        break;
      case 5:
        if ((te(e, t), ee(t), a & 512 && (Nt || l === null || xe(l, l.return)), t.flags & 32)) {
          u = t.stateNode;
          try {
            ta(u, "");
          } catch (Q) {
            ft(t, t.return, Q);
          }
        }
        (a & 4 &&
          t.stateNode != null &&
          ((u = t.memoizedProps), Cc(t, u, l !== null ? l.memoizedProps : u)),
          a & 1024 && (xc = !0));
        break;
      case 6:
        if ((te(e, t), ee(t), a & 4)) {
          if (t.stateNode === null) throw Error(r(162));
          ((a = t.memoizedProps), (l = t.stateNode));
          try {
            l.nodeValue = a;
          } catch (Q) {
            ft(t, t.return, Q);
          }
        }
        break;
      case 3:
        if (
          ((Yn = null),
          (u = Ue),
          (Ue = Qn(e.containerInfo)),
          te(e, t),
          (Ue = u),
          ee(t),
          a & 4 && l !== null && l.memoizedState.isDehydrated)
        )
          try {
            Ca(e.containerInfo);
          } catch (Q) {
            ft(t, t.return, Q);
          }
        xc && ((xc = !1), _o(t));
        break;
      case 4:
        ((a = Ue), (Ue = Qn(t.stateNode.containerInfo)), te(e, t), ee(t), (Ue = a));
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
          g = Fe,
          T = Nt;
        if (((Fe = g || u), (Nt = T || h), te(e, t), (Nt = T), (Fe = g), ee(t), a & 8192))
          t: for (
            e = t.stateNode,
              e._visibility = u ? e._visibility & -2 : e._visibility | 1,
              u && (l === null || h || Fe || Nt || Kl(t)),
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
                    var D = h.memoizedProps.style,
                      S = D != null && D.hasOwnProperty("display") ? D.display : null;
                    s.style.display = S == null || typeof S == "boolean" ? "" : ("" + S).trim();
                  }
                } catch (Q) {
                  ft(h, h.return, Q);
                }
              }
            } else if (e.tag === 6) {
              if (l === null) {
                h = e;
                try {
                  h.stateNode.nodeValue = u ? "" : h.memoizedProps;
                } catch (Q) {
                  ft(h, h.return, Q);
                }
              }
            } else if (e.tag === 18) {
              if (l === null) {
                h = e;
                try {
                  var b = h.stateNode;
                  u ? vh(b, !0) : vh(h.stateNode, !1);
                } catch (Q) {
                  ft(h, h.return, Q);
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
          if (So(a)) {
            l = a;
            break;
          }
          a = a.return;
        }
        if (l == null) throw Error(r(160));
        switch (l.tag) {
          case 27:
            var u = l.stateNode,
              n = jc(t);
            Tn(t, n, u);
            break;
          case 5:
            var c = l.stateNode;
            l.flags & 32 && (ta(c, ""), (l.flags &= -33));
            var s = jc(t);
            Tn(t, s, c);
            break;
          case 3:
          case 4:
            var h = l.stateNode.containerInfo,
              g = jc(t);
            Nc(t, g, h);
            break;
          default:
            throw Error(r(161));
        }
      } catch (T) {
        ft(t, t.return, T);
      }
      t.flags &= -3;
    }
    e & 4096 && (t.flags &= -4097);
  }
  function _o(t) {
    if (t.subtreeFlags & 1024)
      for (t = t.child; t !== null; ) {
        var e = t;
        (_o(e), e.tag === 5 && e.flags & 1024 && e.stateNode.reset(), (t = t.sibling));
      }
  }
  function ke(t, e) {
    if (e.subtreeFlags & 8772)
      for (e = e.child; e !== null; ) (Eo(t, e.alternate, e), (e = e.sibling));
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
          xe(e, e.return);
          var l = e.stateNode;
          (typeof l.componentWillUnmount == "function" && vo(e, e.return, l), Kl(e));
          break;
        case 27:
          pu(e.stateNode);
        case 26:
        case 5:
          (xe(e, e.return), Kl(e));
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
  function $e(t, e, l) {
    for (l = l && (e.subtreeFlags & 8772) !== 0, e = e.child; e !== null; ) {
      var a = e.alternate,
        u = t,
        n = e,
        c = n.flags;
      switch (n.tag) {
        case 0:
        case 11:
        case 15:
          ($e(u, n, l), su(4, n));
          break;
        case 1:
          if (($e(u, n, l), (a = n), (u = a.stateNode), typeof u.componentDidMount == "function"))
            try {
              u.componentDidMount();
            } catch (g) {
              ft(a, a.return, g);
            }
          if (((a = n), (u = a.updateQueue), u !== null)) {
            var s = a.stateNode;
            try {
              var h = u.shared.hiddenCallbacks;
              if (h !== null)
                for (u.shared.hiddenCallbacks = null, u = 0; u < h.length; u++) ar(h[u], s);
            } catch (g) {
              ft(a, a.return, g);
            }
          }
          (l && c & 64 && mo(n), ru(n, n.return));
          break;
        case 27:
          po(n);
        case 26:
        case 5:
          ($e(u, n, l), l && a === null && c & 4 && go(n), ru(n, n.return));
          break;
        case 12:
          $e(u, n, l);
          break;
        case 31:
          ($e(u, n, l), l && c & 4 && zo(u, n));
          break;
        case 13:
          ($e(u, n, l), l && c & 4 && Ao(u, n));
          break;
        case 22:
          (n.memoizedState === null && $e(u, n, l), ru(n, n.return));
          break;
        case 30:
          break;
        default:
          $e(u, n, l);
      }
      e = e.sibling;
    }
  }
  function Hc(t, e) {
    var l = null;
    (t !== null &&
      t.memoizedState !== null &&
      t.memoizedState.cachePool !== null &&
      (l = t.memoizedState.cachePool.pool),
      (t = null),
      e.memoizedState !== null &&
        e.memoizedState.cachePool !== null &&
        (t = e.memoizedState.cachePool.pool),
      t !== l && (t != null && t.refCount++, l != null && ka(l)));
  }
  function qc(t, e) {
    ((t = null),
      e.alternate !== null && (t = e.alternate.memoizedState.cache),
      (e = e.memoizedState.cache),
      e !== t && (e.refCount++, t != null && ka(t)));
  }
  function Ce(t, e, l, a) {
    if (e.subtreeFlags & 10256) for (e = e.child; e !== null; ) (Do(t, e, l, a), (e = e.sibling));
  }
  function Do(t, e, l, a) {
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
            e !== t && (e.refCount++, t != null && ka(t))));
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
          u & 2048 && Hc(c, e));
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
        g = c.flags;
      switch (c.tag) {
        case 0:
        case 11:
        case 15:
          (ba(n, c, s, h, u), su(8, c));
          break;
        case 23:
          break;
        case 22:
          var T = c.stateNode;
          (c.memoizedState !== null
            ? T._visibility & 2
              ? ba(n, c, s, h, u)
              : ou(n, c)
            : ((T._visibility |= 2), ba(n, c, s, h, u)),
            u && g & 2048 && Hc(c.alternate, c));
          break;
        case 24:
          (ba(n, c, s, h, u), u && g & 2048 && qc(c.alternate, c));
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
            (ou(l, a), u & 2048 && Hc(a.alternate, a));
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
    if (t.subtreeFlags & hu) for (t = t.child; t !== null; ) (Ro(t, e, l), (t = t.sibling));
  }
  function Ro(t, e, l) {
    switch (t.tag) {
      case 26:
        (Ea(t, e, l),
          t.flags & hu && t.memoizedState !== null && jm(l, Ue, t.memoizedState, t.memoizedProps));
        break;
      case 5:
        Ea(t, e, l);
        break;
      case 3:
      case 4:
        var a = Ue;
        ((Ue = Qn(t.stateNode.containerInfo)), Ea(t, e, l), (Ue = a));
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
          ((Gt = a), jo(a, t));
        }
      Uo(t);
    }
    if (t.subtreeFlags & 10256) for (t = t.child; t !== null; ) (Co(t), (t = t.sibling));
  }
  function Co(t) {
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
          ((Gt = a), jo(a, t));
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
  function jo(t, e) {
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
          ka(l.memoizedState.cache);
      }
      if (((a = l.child), a !== null)) ((a.return = l), (Gt = a));
      else
        t: for (l = t; Gt !== null; ) {
          a = Gt;
          var u = a.sibling,
            n = a.return;
          if ((To(a), a === l)) {
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
  var wy = {
      getCacheForType: function (t) {
        var e = Zt(Ut),
          l = e.data.get(t);
        return (l === void 0 && ((l = t()), e.data.set(t, l)), l);
      },
      cacheSignal: function () {
        return Zt(Ut).controller.signal;
      },
    },
    Fy = typeof WeakMap == "function" ? WeakMap : Map,
    nt = 0,
    vt = null,
    $ = null,
    P = 0,
    ct = 0,
    he = null,
    yl = !1,
    Ta = !1,
    Qc = !1,
    Ie = 0,
    zt = 0,
    ml = 0,
    Vl = 0,
    Bc = 0,
    de = 0,
    Oa = 0,
    yu = null,
    le = null,
    Yc = !1,
    An = 0,
    No = 0,
    Mn = 1 / 0,
    _n = null,
    vl = null,
    Ht = 0,
    gl = null,
    za = null,
    Pe = 0,
    Gc = 0,
    Xc = null,
    xo = null,
    mu = 0,
    Lc = null;
  function ye() {
    return (nt & 2) !== 0 && P !== 0 ? P & -P : z.T !== null ? Fc() : Wf();
  }
  function Ho() {
    if (de === 0)
      if ((P & 536870912) === 0 || et) {
        var t = xu;
        ((xu <<= 1), (xu & 3932160) === 0 && (xu = 262144), (de = t));
      } else de = 536870912;
    return ((t = re.current), t !== null && (t.flags |= 32), de);
  }
  function ae(t, e, l) {
    (((t === vt && (ct === 2 || ct === 9)) || t.cancelPendingCommit !== null) &&
      (Aa(t, 0), Sl(t, P, de, !1)),
      qa(t, l),
      ((nt & 2) === 0 || t !== vt) &&
        (t === vt && ((nt & 2) === 0 && (Vl |= l), zt === 4 && Sl(t, P, de, !1)), He(t)));
  }
  function qo(t, e, l) {
    if ((nt & 6) !== 0) throw Error(r(327));
    var a = (!l && (e & 127) === 0 && (e & t.expiredLanes) === 0) || Ha(t, e),
      u = a ? $y(t, e) : Kc(t, e, !0),
      n = a;
    do {
      if (u === 0) {
        Ta && !a && Sl(t, e, 0, !1);
        break;
      } else {
        if (((l = t.current.alternate), n && !Wy(l))) {
          ((u = Kc(t, e, !1)), (n = !1));
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
              if ((h && (Aa(s, c).flags |= 256), (c = Kc(s, c, !1)), c !== 2)) {
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
            if ((Sl(a, e, de, !yl), qu(a, 0, !0) !== 0)) break t;
            ((Pe = e),
              (a.timeoutHandle = dh(
                Qo.bind(null, a, l, le, _n, Yc, e, de, Vl, Oa, yl, n, "Throttled", -0, 0),
                u,
              )));
            break t;
          }
          Qo(a, l, le, _n, Yc, e, de, Vl, Oa, yl, n, null, -0, 0);
        }
      }
      break;
    } while (!0);
    He(t);
  }
  function Qo(t, e, l, a, u, n, c, s, h, g, T, D, S, b) {
    if (((t.timeoutHandle = -1), (D = e.subtreeFlags), D & 8192 || (D & 16785408) === 16785408)) {
      ((D = {
        stylesheets: null,
        count: 0,
        imgCount: 0,
        imgBytes: 0,
        suspenseyImages: [],
        waitingForImages: !0,
        waitingForViewTransition: !1,
        unsuspend: Be,
      }),
        Ro(e, n, D));
      var Q = (n & 62914560) === n ? An - ne() : (n & 4194048) === n ? No - ne() : 0;
      if (((Q = Nm(D, Q)), Q !== null)) {
        ((Pe = n),
          (t.cancelPendingCommit = Q(Vo.bind(null, t, e, n, l, a, u, c, s, h, T, D, null, S, b))),
          Sl(t, n, c, !g));
        return;
      }
    }
    Vo(t, e, n, l, a, u, c, s, h);
  }
  function Wy(t) {
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
    ((e &= ~Bc),
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
    l !== 0 && Jf(t, l, e);
  }
  function Dn() {
    return (nt & 6) === 0 ? (vu(0), !1) : !0;
  }
  function Zc() {
    if ($ !== null) {
      if (ct === 0) var t = $.return;
      else ((t = $), (Le = ql = null), nc(t), (ma = null), (Ia = 0), (t = $));
      for (; t !== null; ) (yo(t.alternate, t), (t = t.return));
      $ = null;
    }
  }
  function Aa(t, e) {
    var l = t.timeoutHandle;
    (l !== -1 && ((t.timeoutHandle = -1), mm(l)),
      (l = t.cancelPendingCommit),
      l !== null && ((t.cancelPendingCommit = null), l()),
      (Pe = 0),
      Zc(),
      (vt = t),
      ($ = l = Ge(t.current, null)),
      (P = e),
      (ct = 0),
      (he = null),
      (yl = !1),
      (Ta = Ha(t, e)),
      (Qc = !1),
      (Oa = de = Bc = Vl = ml = zt = 0),
      (le = yu = null),
      (Yc = !1),
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
  function Bo(t, e) {
    ((w = null),
      (z.H = iu),
      e === ya || e === ln
        ? ((e = Ps()), (ct = 3))
        : e === wi
          ? ((e = Ps()), (ct = 4))
          : (ct =
              e === Ec
                ? 8
                : e !== null && typeof e == "object" && typeof e.then == "function"
                  ? 6
                  : 1),
      (he = e),
      $ === null && ((zt = 1), gn(t, be(e, t.current))));
  }
  function Yo() {
    var t = re.current;
    return t === null
      ? !0
      : (P & 4194048) === P
        ? ze === null
        : (P & 62914560) === P || (P & 536870912) !== 0
          ? t === ze
          : !1;
  }
  function Go() {
    var t = z.H;
    return ((z.H = iu), t === null ? iu : t);
  }
  function Xo() {
    var t = z.A;
    return ((z.A = wy), t);
  }
  function Rn() {
    ((zt = 4),
      yl || ((P & 4194048) !== P && re.current !== null) || (Ta = !0),
      ((ml & 134217727) === 0 && (Vl & 134217727) === 0) || vt === null || Sl(vt, P, de, !1));
  }
  function Kc(t, e, l) {
    var a = nt;
    nt |= 2;
    var u = Go(),
      n = Xo();
    ((vt !== t || P !== e) && ((_n = null), Aa(t, e)), (e = !1));
    var c = zt;
    t: do
      try {
        if (ct !== 0 && $ !== null) {
          var s = $,
            h = he;
          switch (ct) {
            case 8:
              (Zc(), (c = 6));
              break t;
            case 3:
            case 2:
            case 9:
            case 6:
              re.current === null && (e = !0);
              var g = ct;
              if (((ct = 0), (he = null), Ma(t, s, h, g), l && Ta)) {
                c = 0;
                break t;
              }
              break;
            default:
              ((g = ct), (ct = 0), (he = null), Ma(t, s, h, g));
          }
        }
        (ky(), (c = zt));
        break;
      } catch (T) {
        Bo(t, T);
      }
    while (!0);
    return (
      e && t.shellSuspendCounter++,
      (Le = ql = null),
      (nt = a),
      (z.H = u),
      (z.A = n),
      $ === null && ((vt = null), (P = 0), Fu()),
      c
    );
  }
  function ky() {
    for (; $ !== null; ) Lo($);
  }
  function $y(t, e) {
    var l = nt;
    nt |= 2;
    var a = Go(),
      u = Xo();
    vt !== t || P !== e ? ((_n = null), (Mn = ne() + 500), Aa(t, e)) : (Ta = Ha(t, e));
    t: do
      try {
        if (ct !== 0 && $ !== null) {
          e = $;
          var n = he;
          e: switch (ct) {
            case 1:
              ((ct = 0), (he = null), Ma(t, e, n, 1));
              break;
            case 2:
            case 9:
              if ($s(n)) {
                ((ct = 0), (he = null), Zo(e));
                break;
              }
              ((e = function () {
                ((ct !== 2 && ct !== 9) || vt !== t || (ct = 7), He(t));
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
              $s(n) ? ((ct = 0), (he = null), Zo(e)) : ((ct = 0), (he = null), Ma(t, e, n, 7));
              break;
            case 5:
              var c = null;
              switch ($.tag) {
                case 26:
                  c = $.memoizedState;
                case 5:
                case 27:
                  var s = $;
                  if (c ? Dh(c) : s.stateNode.complete) {
                    ((ct = 0), (he = null));
                    var h = s.sibling;
                    if (h !== null) $ = h;
                    else {
                      var g = s.return;
                      g !== null ? (($ = g), Un(g)) : ($ = null);
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
              (Zc(), (zt = 6));
              break t;
            default:
              throw Error(r(462));
          }
        }
        Iy();
        break;
      } catch (T) {
        Bo(t, T);
      }
    while (!0);
    return (
      (Le = ql = null),
      (z.H = a),
      (z.A = u),
      (nt = l),
      $ !== null ? 0 : ((vt = null), (P = 0), Fu(), zt)
    );
  }
  function Iy() {
    for (; $ !== null && !Ed(); ) Lo($);
  }
  function Lo(t) {
    var e = oo(t.alternate, t, Ie);
    ((t.memoizedProps = t.pendingProps), e === null ? Un(t) : ($ = e));
  }
  function Zo(t) {
    var e = t,
      l = e.alternate;
    switch (e.tag) {
      case 15:
      case 0:
        e = no(l, e, e.pendingProps, e.type, void 0, P);
        break;
      case 11:
        e = no(l, e, e.pendingProps, e.type.render, e.ref, P);
        break;
      case 5:
        nc(e);
      default:
        (yo(l, e), (e = $ = Gs(e, Ie)), (e = oo(l, e, Ie)));
    }
    ((t.memoizedProps = t.pendingProps), e === null ? Un(t) : ($ = e));
  }
  function Ma(t, e, l, a) {
    ((Le = ql = null), nc(e), (ma = null), (Ia = 0));
    var u = e.return;
    try {
      if (Gy(t, u, e, l, P)) {
        ((zt = 1), gn(t, be(l, t.current)), ($ = null));
        return;
      }
    } catch (n) {
      if (u !== null) throw (($ = u), n);
      ((zt = 1), gn(t, be(l, t.current)), ($ = null));
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
        Ko(e, t))
      : Un(e);
  }
  function Un(t) {
    var e = t;
    do {
      if ((e.flags & 32768) !== 0) {
        Ko(e, yl);
        return;
      }
      t = e.return;
      var l = Zy(e.alternate, e, Ie);
      if (l !== null) {
        $ = l;
        return;
      }
      if (((e = e.sibling), e !== null)) {
        $ = e;
        return;
      }
      $ = e = t;
    } while (e !== null);
    zt === 0 && (zt = 5);
  }
  function Ko(t, e) {
    do {
      var l = Ky(t.alternate, t);
      if (l !== null) {
        ((l.flags &= 32767), ($ = l));
        return;
      }
      if (
        ((l = t.return),
        l !== null && ((l.flags |= 32768), (l.subtreeFlags = 0), (l.deletions = null)),
        !e && ((t = t.sibling), t !== null))
      ) {
        $ = t;
        return;
      }
      $ = t = l;
    } while (t !== null);
    ((zt = 6), ($ = null));
  }
  function Vo(t, e, l, a, u, n, c, s, h) {
    t.cancelPendingCommit = null;
    do Cn();
    while (Ht !== 0);
    if ((nt & 6) !== 0) throw Error(r(327));
    if (e !== null) {
      if (e === t.current) throw Error(r(177));
      if (
        ((n = e.lanes | e.childLanes),
        (n |= ji),
        Cd(t, l, n, c, s, h),
        t === vt && (($ = vt = null), (P = 0)),
        (za = e),
        (gl = t),
        (Pe = l),
        (Gc = n),
        (Xc = u),
        (xo = a),
        (e.subtreeFlags & 10256) !== 0 || (e.flags & 10256) !== 0
          ? ((t.callbackNode = null),
            (t.callbackPriority = 0),
            lm(ju, function () {
              return (ko(), null);
            }))
          : ((t.callbackNode = null), (t.callbackPriority = 0)),
        (a = (e.flags & 13878) !== 0),
        (e.subtreeFlags & 13878) !== 0 || a)
      ) {
        ((a = z.T), (z.T = null), (u = x.p), (x.p = 2), (c = nt), (nt |= 4));
        try {
          Vy(t, e, l);
        } finally {
          ((nt = c), (x.p = u), (z.T = a));
        }
      }
      ((Ht = 1), Jo(), wo(), Fo());
    }
  }
  function Jo() {
    if (Ht === 1) {
      Ht = 0;
      var t = gl,
        e = za,
        l = (e.flags & 13878) !== 0;
      if ((e.subtreeFlags & 13878) !== 0 || l) {
        ((l = z.T), (z.T = null));
        var a = x.p;
        x.p = 2;
        var u = nt;
        nt |= 4;
        try {
          Mo(e, t);
          var n = lf,
            c = Cs(t.containerInfo),
            s = n.focusedElem,
            h = n.selectionRange;
          if (c !== s && s && s.ownerDocument && Us(s.ownerDocument.documentElement, s)) {
            if (h !== null && _i(s)) {
              var g = h.start,
                T = h.end;
              if ((T === void 0 && (T = g), "selectionStart" in s))
                ((s.selectionStart = g), (s.selectionEnd = Math.min(T, s.value.length)));
              else {
                var D = s.ownerDocument || document,
                  S = (D && D.defaultView) || window;
                if (S.getSelection) {
                  var b = S.getSelection(),
                    Q = s.textContent.length,
                    X = Math.min(h.start, Q),
                    dt = h.end === void 0 ? X : Math.min(h.end, Q);
                  !b.extend && X > dt && ((c = dt), (dt = X), (X = c));
                  var m = Rs(s, X),
                    d = Rs(s, dt);
                  if (
                    m &&
                    d &&
                    (b.rangeCount !== 1 ||
                      b.anchorNode !== m.node ||
                      b.anchorOffset !== m.offset ||
                      b.focusNode !== d.node ||
                      b.focusOffset !== d.offset)
                  ) {
                    var v = D.createRange();
                    (v.setStart(m.node, m.offset),
                      b.removeAllRanges(),
                      X > dt
                        ? (b.addRange(v), b.extend(d.node, d.offset))
                        : (v.setEnd(d.node, d.offset), b.addRange(v)));
                  }
                }
              }
            }
            for (D = [], b = s; (b = b.parentNode); )
              b.nodeType === 1 && D.push({ element: b, left: b.scrollLeft, top: b.scrollTop });
            for (typeof s.focus == "function" && s.focus(), s = 0; s < D.length; s++) {
              var _ = D[s];
              ((_.element.scrollLeft = _.left), (_.element.scrollTop = _.top));
            }
          }
          ((Zn = !!ef), (lf = ef = null));
        } finally {
          ((nt = u), (x.p = a), (z.T = l));
        }
      }
      ((t.current = e), (Ht = 2));
    }
  }
  function wo() {
    if (Ht === 2) {
      Ht = 0;
      var t = gl,
        e = za,
        l = (e.flags & 8772) !== 0;
      if ((e.subtreeFlags & 8772) !== 0 || l) {
        ((l = z.T), (z.T = null));
        var a = x.p;
        x.p = 2;
        var u = nt;
        nt |= 4;
        try {
          Eo(t, e.alternate, e);
        } finally {
          ((nt = u), (x.p = a), (z.T = l));
        }
      }
      Ht = 3;
    }
  }
  function Fo() {
    if (Ht === 4 || Ht === 3) {
      ((Ht = 0), Td());
      var t = gl,
        e = za,
        l = Pe,
        a = xo;
      (e.subtreeFlags & 10256) !== 0 || (e.flags & 10256) !== 0
        ? (Ht = 5)
        : ((Ht = 0), (za = gl = null), Wo(t, t.pendingLanes));
      var u = t.pendingLanes;
      if (
        (u === 0 && (vl = null),
        fi(l),
        (e = e.stateNode),
        ie && typeof ie.onCommitFiberRoot == "function")
      )
        try {
          ie.onCommitFiberRoot(xa, e, void 0, (e.current.flags & 128) === 128);
        } catch {}
      if (a !== null) {
        ((e = z.T), (u = x.p), (x.p = 2), (z.T = null));
        try {
          for (var n = t.onRecoverableError, c = 0; c < a.length; c++) {
            var s = a[c];
            n(s.value, { componentStack: s.stack });
          }
        } finally {
          ((z.T = e), (x.p = u));
        }
      }
      ((Pe & 3) !== 0 && Cn(),
        He(t),
        (u = t.pendingLanes),
        (l & 261930) !== 0 && (u & 42) !== 0 ? (t === Lc ? mu++ : ((mu = 0), (Lc = t))) : (mu = 0),
        vu(0));
    }
  }
  function Wo(t, e) {
    (t.pooledCacheLanes &= e) === 0 &&
      ((e = t.pooledCache), e != null && ((t.pooledCache = null), ka(e)));
  }
  function Cn() {
    return (Jo(), wo(), Fo(), ko());
  }
  function ko() {
    if (Ht !== 5) return !1;
    var t = gl,
      e = Gc;
    Gc = 0;
    var l = fi(Pe),
      a = z.T,
      u = x.p;
    try {
      ((x.p = 32 > l ? 32 : l), (z.T = null), (l = Xc), (Xc = null));
      var n = gl,
        c = Pe;
      if (((Ht = 0), (za = gl = null), (Pe = 0), (nt & 6) !== 0)) throw Error(r(331));
      var s = nt;
      if (
        ((nt |= 4),
        Co(n.current),
        Do(n, n.current, c, l),
        (nt = s),
        vu(0, !1),
        ie && typeof ie.onPostCommitFiberRoot == "function")
      )
        try {
          ie.onPostCommitFiberRoot(xa, n);
        } catch {}
      return !0;
    } finally {
      ((x.p = u), (z.T = a), Wo(t, e));
    }
  }
  function $o(t, e, l) {
    ((e = be(l, e)),
      (e = bc(t.stateNode, e, 2)),
      (t = rl(t, e, 2)),
      t !== null && (qa(t, 2), He(t)));
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
            (typeof a.componentDidCatch == "function" && (vl === null || !vl.has(a)))
          ) {
            ((t = be(l, t)),
              (l = $r(2)),
              (a = rl(e, l, 2)),
              a !== null && (Ir(l, a, e, t), qa(a, 2), He(a)));
            break;
          }
        }
        e = e.return;
      }
  }
  function Vc(t, e, l) {
    var a = t.pingCache;
    if (a === null) {
      a = t.pingCache = new Fy();
      var u = new Set();
      a.set(e, u);
    } else ((u = a.get(e)), u === void 0 && ((u = new Set()), a.set(e, u)));
    u.has(l) || ((Qc = !0), u.add(l), (t = Py.bind(null, t, e, l)), e.then(t, t));
  }
  function Py(t, e, l) {
    var a = t.pingCache;
    (a !== null && a.delete(e),
      (t.pingedLanes |= t.suspendedLanes & l),
      (t.warmLanes &= ~l),
      vt === t &&
        (P & l) === l &&
        (zt === 4 || (zt === 3 && (P & 62914560) === P && 300 > ne() - An)
          ? (nt & 2) === 0 && Aa(t, 0)
          : (Bc |= l),
        Oa === P && (Oa = 0)),
      He(t));
  }
  function Io(t, e) {
    (e === 0 && (e = Vf()), (t = Nl(t, e)), t !== null && (qa(t, e), He(t)));
  }
  function tm(t) {
    var e = t.memoizedState,
      l = 0;
    (e !== null && (l = e.retryLane), Io(t, l));
  }
  function em(t, e) {
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
    (a !== null && a.delete(e), Io(t, l));
  }
  function lm(t, e) {
    return ui(t, e);
  }
  var jn = null,
    _a = null,
    Jc = !1,
    Nn = !1,
    wc = !1,
    pl = 0;
  function He(t) {
    (t !== _a && t.next === null && (_a === null ? (jn = _a = t) : (_a = _a.next = t)),
      (Nn = !0),
      Jc || ((Jc = !0), um()));
  }
  function vu(t, e) {
    if (!wc && Nn) {
      wc = !0;
      do
        for (var l = !1, a = jn; a !== null; ) {
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
            n !== 0 && ((l = !0), lh(a, n));
          } else
            ((n = P),
              (n = qu(
                a,
                a === vt ? n : 0,
                a.cancelPendingCommit !== null || a.timeoutHandle !== -1,
              )),
              (n & 3) === 0 || Ha(a, n) || ((l = !0), lh(a, n)));
          a = a.next;
        }
      while (l);
      wc = !1;
    }
  }
  function am() {
    Po();
  }
  function Po() {
    Nn = Jc = !1;
    var t = 0;
    pl !== 0 && ym() && (t = pl);
    for (var e = ne(), l = null, a = jn; a !== null; ) {
      var u = a.next,
        n = th(a, e);
      (n === 0
        ? ((a.next = null), l === null ? (jn = u) : (l.next = u), u === null && (_a = l))
        : ((l = a), (t !== 0 || (n & 3) !== 0) && (Nn = !0)),
        (a = u));
    }
    ((Ht !== 0 && Ht !== 5) || vu(t), pl !== 0 && (pl = 0));
  }
  function th(t, e) {
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
      ((e = vt),
      (l = P),
      (l = qu(t, t === e ? l : 0, t.cancelPendingCommit !== null || t.timeoutHandle !== -1)),
      (a = t.callbackNode),
      l === 0 || (t === e && (ct === 2 || ct === 9)) || t.cancelPendingCommit !== null)
    )
      return (a !== null && a !== null && ni(a), (t.callbackNode = null), (t.callbackPriority = 0));
    if ((l & 3) === 0 || Ha(t, l)) {
      if (((e = l & -l), e === t.callbackPriority)) return e;
      switch ((a !== null && ni(a), fi(l))) {
        case 2:
        case 8:
          l = Zf;
          break;
        case 32:
          l = ju;
          break;
        case 268435456:
          l = Kf;
          break;
        default:
          l = ju;
      }
      return (
        (a = eh.bind(null, t)), (l = ui(l, a)), (t.callbackPriority = e), (t.callbackNode = l), e
      );
    }
    return (
      a !== null && a !== null && ni(a), (t.callbackPriority = 2), (t.callbackNode = null), 2
    );
  }
  function eh(t, e) {
    if (Ht !== 0 && Ht !== 5) return ((t.callbackNode = null), (t.callbackPriority = 0), null);
    var l = t.callbackNode;
    if (Cn() && t.callbackNode !== l) return null;
    var a = P;
    return (
      (a = qu(t, t === vt ? a : 0, t.cancelPendingCommit !== null || t.timeoutHandle !== -1)),
      a === 0
        ? null
        : (qo(t, a, e),
          th(t, ne()),
          t.callbackNode != null && t.callbackNode === l ? eh.bind(null, t) : null)
    );
  }
  function lh(t, e) {
    if (Cn()) return null;
    qo(t, e, !0);
  }
  function um() {
    vm(function () {
      (nt & 6) !== 0 ? ui(Lf, am) : Po();
    });
  }
  function Fc() {
    if (pl === 0) {
      var t = ha;
      (t === 0 && ((t = Nu), (Nu <<= 1), (Nu & 261888) === 0 && (Nu = 256)), (pl = t));
    }
    return pl;
  }
  function ah(t) {
    return t == null || typeof t == "symbol" || typeof t == "boolean"
      ? null
      : typeof t == "function"
        ? t
        : Gu("" + t);
  }
  function uh(t, e) {
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
  function nm(t, e, l, a, u) {
    if (e === "submit" && l && l.stateNode === u) {
      var n = ah((u[$t] || null).action),
        c = a.submitter;
      c &&
        ((e = (e = c[$t] || null) ? ah(e.formAction) : c.getAttribute("formAction")),
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
                  var h = c ? uh(u, c) : new FormData(u);
                  yc(l, { pending: !0, data: h, method: u.method, action: n }, null, h);
                }
              } else
                typeof n == "function" &&
                  (s.preventDefault(),
                  (h = c ? uh(u, c) : new FormData(u)),
                  yc(l, { pending: !0, data: h, method: u.method, action: n }, n, h));
            },
            currentTarget: u,
          },
        ],
      });
    }
  }
  for (var Wc = 0; Wc < Ci.length; Wc++) {
    var kc = Ci[Wc],
      im = kc.toLowerCase(),
      cm = kc[0].toUpperCase() + kc.slice(1);
    Re(im, "on" + cm);
  }
  (Re(xs, "onAnimationEnd"),
    Re(Hs, "onAnimationIteration"),
    Re(qs, "onAnimationStart"),
    Re("dblclick", "onDoubleClick"),
    Re("focusin", "onFocus"),
    Re("focusout", "onBlur"),
    Re(Oy, "onTransitionRun"),
    Re(zy, "onTransitionStart"),
    Re(Ay, "onTransitionCancel"),
    Re(Qs, "onTransitionEnd"),
    Il("onMouseEnter", ["mouseout", "mouseover"]),
    Il("onMouseLeave", ["mouseout", "mouseover"]),
    Il("onPointerEnter", ["pointerout", "pointerover"]),
    Il("onPointerLeave", ["pointerout", "pointerover"]),
    Rl("onChange", "change click focusin focusout input keydown keyup selectionchange".split(" ")),
    Rl(
      "onSelect",
      "focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange".split(
        " ",
      ),
    ),
    Rl("onBeforeInput", ["compositionend", "keypress", "textInput", "paste"]),
    Rl("onCompositionEnd", "compositionend focusout keydown keypress keyup mousedown".split(" ")),
    Rl(
      "onCompositionStart",
      "compositionstart focusout keydown keypress keyup mousedown".split(" "),
    ),
    Rl(
      "onCompositionUpdate",
      "compositionupdate focusout keydown keypress keyup mousedown".split(" "),
    ));
  var gu =
      "abort canplay canplaythrough durationchange emptied encrypted ended error loadeddata loadedmetadata loadstart pause play playing progress ratechange resize seeked seeking stalled suspend timeupdate volumechange waiting".split(
        " ",
      ),
    fm = new Set(
      "beforetoggle cancel close invalid load scroll scrollend toggle".split(" ").concat(gu),
    );
  function nh(t, e) {
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
              g = s.currentTarget;
            if (((s = s.listener), h !== n && u.isPropagationStopped())) break t;
            ((n = s), (u.currentTarget = g));
            try {
              n(u);
            } catch (T) {
              wu(T);
            }
            ((u.currentTarget = null), (n = h));
          }
        else
          for (c = 0; c < a.length; c++) {
            if (
              ((s = a[c]),
              (h = s.instance),
              (g = s.currentTarget),
              (s = s.listener),
              h !== n && u.isPropagationStopped())
            )
              break t;
            ((n = s), (u.currentTarget = g));
            try {
              n(u);
            } catch (T) {
              wu(T);
            }
            ((u.currentTarget = null), (n = h));
          }
      }
    }
  }
  function I(t, e) {
    var l = e[si];
    l === void 0 && (l = e[si] = new Set());
    var a = t + "__bubble";
    l.has(a) || (ih(e, t, 2, !1), l.add(a));
  }
  function $c(t, e, l) {
    var a = 0;
    (e && (a |= 4), ih(l, t, a, e));
  }
  var xn = "_reactListening" + Math.random().toString(36).slice(2);
  function Ic(t) {
    if (!t[xn]) {
      ((t[xn] = !0),
        If.forEach(function (l) {
          l !== "selectionchange" && (fm.has(l) || $c(l, !1, t), $c(l, !0, t));
        }));
      var e = t.nodeType === 9 ? t : t.ownerDocument;
      e === null || e[xn] || ((e[xn] = !0), $c("selectionchange", !1, e));
    }
  }
  function ih(t, e, l, a) {
    switch (Hh(e)) {
      case 2:
        var u = qm;
        break;
      case 8:
        u = Qm;
        break;
      default:
        u = yf;
    }
    ((l = u.bind(null, e, l, t)),
      (u = void 0),
      !Si || (e !== "touchstart" && e !== "touchmove" && e !== "wheel") || (u = !0),
      a
        ? u !== void 0
          ? t.addEventListener(e, l, { capture: !0, passive: u })
          : t.addEventListener(e, l, !0)
        : u !== void 0
          ? t.addEventListener(e, l, { passive: u })
          : t.addEventListener(e, l, !1));
  }
  function Pc(t, e, l, a, u) {
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
    rs(function () {
      var g = n,
        T = vi(l),
        D = [];
      t: {
        var S = Bs.get(t);
        if (S !== void 0) {
          var b = Ku,
            Q = t;
          switch (t) {
            case "keypress":
              if (Lu(l) === 0) break t;
            case "keydown":
            case "keyup":
              b = ey;
              break;
            case "focusin":
              ((Q = "focus"), (b = Ti));
              break;
            case "focusout":
              ((Q = "blur"), (b = Ti));
              break;
            case "beforeblur":
            case "afterblur":
              b = Ti;
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
              b = ds;
              break;
            case "drag":
            case "dragend":
            case "dragenter":
            case "dragexit":
            case "dragleave":
            case "dragover":
            case "dragstart":
            case "drop":
              b = Zd;
              break;
            case "touchcancel":
            case "touchend":
            case "touchmove":
            case "touchstart":
              b = uy;
              break;
            case xs:
            case Hs:
            case qs:
              b = Jd;
              break;
            case Qs:
              b = iy;
              break;
            case "scroll":
            case "scrollend":
              b = Xd;
              break;
            case "wheel":
              b = fy;
              break;
            case "copy":
            case "cut":
            case "paste":
              b = Fd;
              break;
            case "gotpointercapture":
            case "lostpointercapture":
            case "pointercancel":
            case "pointerdown":
            case "pointermove":
            case "pointerout":
            case "pointerover":
            case "pointerup":
              b = ms;
              break;
            case "toggle":
            case "beforetoggle":
              b = ry;
          }
          var X = (e & 4) !== 0,
            dt = !X && (t === "scroll" || t === "scrollend"),
            m = X ? (S !== null ? S + "Capture" : null) : S;
          X = [];
          for (var d = g, v; d !== null; ) {
            var _ = d;
            if (
              ((v = _.stateNode),
              (_ = _.tag),
              (_ !== 5 && _ !== 26 && _ !== 27) ||
                v === null ||
                m === null ||
                ((_ = Ya(d, m)), _ != null && X.push(Su(d, _, v))),
              dt)
            )
              break;
            d = d.return;
          }
          0 < X.length && ((S = new b(S, Q, null, l, T)), D.push({ event: S, listeners: X }));
        }
      }
      if ((e & 7) === 0) {
        t: {
          if (
            ((S = t === "mouseover" || t === "pointerover"),
            (b = t === "mouseout" || t === "pointerout"),
            S && l !== mi && (Q = l.relatedTarget || l.fromElement) && (Wl(Q) || Q[Fl]))
          )
            break t;
          if (
            (b || S) &&
            ((S =
              T.window === T
                ? T
                : (S = T.ownerDocument)
                  ? S.defaultView || S.parentWindow
                  : window),
            b
              ? ((Q = l.relatedTarget || l.toElement),
                (b = g),
                (Q = Q ? Wl(Q) : null),
                Q !== null &&
                  ((dt = M(Q)), (X = Q.tag), Q !== dt || (X !== 5 && X !== 27 && X !== 6)) &&
                  (Q = null))
              : ((b = null), (Q = g)),
            b !== Q)
          ) {
            if (
              ((X = ds),
              (_ = "onMouseLeave"),
              (m = "onMouseEnter"),
              (d = "mouse"),
              (t === "pointerout" || t === "pointerover") &&
                ((X = ms), (_ = "onPointerLeave"), (m = "onPointerEnter"), (d = "pointer")),
              (dt = b == null ? S : Ba(b)),
              (v = Q == null ? S : Ba(Q)),
              (S = new X(_, d + "leave", b, l, T)),
              (S.target = dt),
              (S.relatedTarget = v),
              (_ = null),
              Wl(T) === g &&
                ((X = new X(m, d + "enter", Q, l, T)),
                (X.target = v),
                (X.relatedTarget = dt),
                (_ = X)),
              (dt = _),
              b && Q)
            )
              e: {
                for (X = sm, m = b, d = Q, v = 0, _ = m; _; _ = X(_)) v++;
                _ = 0;
                for (var G = d; G; G = X(G)) _++;
                for (; 0 < v - _; ) ((m = X(m)), v--);
                for (; 0 < _ - v; ) ((d = X(d)), _--);
                for (; v--; ) {
                  if (m === d || (d !== null && m === d.alternate)) {
                    X = m;
                    break e;
                  }
                  ((m = X(m)), (d = X(d)));
                }
                X = null;
              }
            else X = null;
            (b !== null && ch(D, S, b, X, !1), Q !== null && dt !== null && ch(D, dt, Q, X, !0));
          }
        }
        t: {
          if (
            ((S = g ? Ba(g) : window),
            (b = S.nodeName && S.nodeName.toLowerCase()),
            b === "select" || (b === "input" && S.type === "file"))
          )
            var at = Os;
          else if (Es(S))
            if (zs) at = by;
            else {
              at = Sy;
              var Y = gy;
            }
          else
            ((b = S.nodeName),
              !b || b.toLowerCase() !== "input" || (S.type !== "checkbox" && S.type !== "radio")
                ? g && yi(g.elementType) && (at = Os)
                : (at = py));
          if (at && (at = at(t, g))) {
            Ts(D, at, l, T);
            break t;
          }
          (Y && Y(t, S, g),
            t === "focusout" &&
              g &&
              S.type === "number" &&
              g.memoizedProps.value != null &&
              di(S, "number", S.value));
        }
        switch (((Y = g ? Ba(g) : window), t)) {
          case "focusin":
            (Es(Y) || Y.contentEditable === "true") && ((ua = Y), (Di = g), (wa = null));
            break;
          case "focusout":
            wa = Di = ua = null;
            break;
          case "mousedown":
            Ri = !0;
            break;
          case "contextmenu":
          case "mouseup":
          case "dragend":
            ((Ri = !1), js(D, l, T));
            break;
          case "selectionchange":
            if (Ty) break;
          case "keydown":
          case "keyup":
            js(D, l, T);
        }
        var F;
        if (zi)
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
            ? ps(t, l) && (tt = "onCompositionEnd")
            : t === "keydown" && l.keyCode === 229 && (tt = "onCompositionStart");
        (tt &&
          (vs &&
            l.locale !== "ko" &&
            (aa || tt !== "onCompositionStart"
              ? tt === "onCompositionEnd" && aa && (F = os())
              : ((al = T), (pi = "value" in al ? al.value : al.textContent), (aa = !0))),
          (Y = Hn(g, tt)),
          0 < Y.length &&
            ((tt = new ys(tt, t, null, l, T)),
            D.push({ event: tt, listeners: Y }),
            F ? (tt.data = F) : ((F = bs(l)), F !== null && (tt.data = F)))),
          (F = hy ? dy(t, l) : yy(t, l)) &&
            ((tt = Hn(g, "onBeforeInput")),
            0 < tt.length &&
              ((Y = new ys("onBeforeInput", "beforeinput", null, l, T)),
              D.push({ event: Y, listeners: tt }),
              (Y.data = F))),
          nm(D, t, g, l, T));
      }
      nh(D, e);
    });
  }
  function Su(t, e, l) {
    return { instance: t, listener: e, currentTarget: l };
  }
  function Hn(t, e) {
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
  function sm(t) {
    if (t === null) return null;
    do t = t.return;
    while (t && t.tag !== 5 && t.tag !== 27);
    return t || null;
  }
  function ch(t, e, l, a, u) {
    for (var n = e._reactName, c = []; l !== null && l !== a; ) {
      var s = l,
        h = s.alternate,
        g = s.stateNode;
      if (((s = s.tag), h !== null && h === a)) break;
      ((s !== 5 && s !== 26 && s !== 27) ||
        g === null ||
        ((h = g),
        u
          ? ((g = Ya(l, n)), g != null && c.unshift(Su(l, g, h)))
          : u || ((g = Ya(l, n)), g != null && c.push(Su(l, g, h)))),
        (l = l.return));
    }
    c.length !== 0 && t.push({ event: e, listeners: c });
  }
  var rm = /\r\n?/g,
    om = /\u0000|\uFFFD/g;
  function fh(t) {
    return (typeof t == "string" ? t : "" + t)
      .replace(
        rm,
        `
`,
      )
      .replace(om, "");
  }
  function sh(t, e) {
    return ((e = fh(e)), fh(t) === e);
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
        fs(t, a, n);
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
        (I("beforetoggle", t), I("toggle", t), Qu(t, "popover", a));
        break;
      case "xlinkActuate":
        Qe(t, "http://www.w3.org/1999/xlink", "xlink:actuate", a);
        break;
      case "xlinkArcrole":
        Qe(t, "http://www.w3.org/1999/xlink", "xlink:arcrole", a);
        break;
      case "xlinkRole":
        Qe(t, "http://www.w3.org/1999/xlink", "xlink:role", a);
        break;
      case "xlinkShow":
        Qe(t, "http://www.w3.org/1999/xlink", "xlink:show", a);
        break;
      case "xlinkTitle":
        Qe(t, "http://www.w3.org/1999/xlink", "xlink:title", a);
        break;
      case "xlinkType":
        Qe(t, "http://www.w3.org/1999/xlink", "xlink:type", a);
        break;
      case "xmlBase":
        Qe(t, "http://www.w3.org/XML/1998/namespace", "xml:base", a);
        break;
      case "xmlLang":
        Qe(t, "http://www.w3.org/XML/1998/namespace", "xml:lang", a);
        break;
      case "xmlSpace":
        Qe(t, "http://www.w3.org/XML/1998/namespace", "xml:space", a);
        break;
      case "is":
        Qu(t, "is", a);
        break;
      case "innerText":
      case "textContent":
        break;
      default:
        (!(2 < l.length) || (l[0] !== "o" && l[0] !== "O") || (l[1] !== "n" && l[1] !== "N")) &&
          ((l = Yd.get(l) || l), Qu(t, l, a));
    }
  }
  function tf(t, e, l, a, u, n) {
    switch (l) {
      case "style":
        fs(t, a, n);
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
        if (!Pf.hasOwnProperty(l))
          t: {
            if (
              l[0] === "o" &&
              l[1] === "n" &&
              ((u = l.endsWith("Capture")),
              (e = l.slice(2, u ? l.length - 7 : void 0)),
              (n = t[$t] || null),
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
            l in t ? (t[l] = a) : a === !0 ? t.setAttribute(l, "") : Qu(t, l, a);
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
          g = null;
        for (a in l)
          if (l.hasOwnProperty(a)) {
            var T = l[a];
            if (T != null)
              switch (a) {
                case "name":
                  u = T;
                  break;
                case "type":
                  c = T;
                  break;
                case "checked":
                  h = T;
                  break;
                case "defaultChecked":
                  g = T;
                  break;
                case "value":
                  n = T;
                  break;
                case "defaultValue":
                  s = T;
                  break;
                case "children":
                case "dangerouslySetInnerHTML":
                  if (T != null) throw Error(r(137, e));
                  break;
                default:
                  ht(t, e, a, T, l, null);
              }
          }
        us(t, n, s, h, g, c, u, !1);
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
        is(t, a, u, n);
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
        for (g in l)
          if (l.hasOwnProperty(g) && ((a = l[g]), a != null))
            switch (g) {
              case "children":
              case "dangerouslySetInnerHTML":
                throw Error(r(137, e));
              default:
                ht(t, e, g, a, l, null);
            }
        return;
      default:
        if (yi(e)) {
          for (T in l)
            l.hasOwnProperty(T) && ((a = l[T]), a !== void 0 && tf(t, e, T, a, l, void 0));
          return;
        }
    }
    for (s in l) l.hasOwnProperty(s) && ((a = l[s]), a != null && ht(t, e, s, a, l, null));
  }
  function hm(t, e, l, a) {
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
          g = null,
          T = null;
        for (b in l) {
          var D = l[b];
          if (l.hasOwnProperty(b) && D != null)
            switch (b) {
              case "checked":
                break;
              case "value":
                break;
              case "defaultValue":
                h = D;
              default:
                a.hasOwnProperty(b) || ht(t, e, b, null, a, D);
            }
        }
        for (var S in a) {
          var b = a[S];
          if (((D = l[S]), a.hasOwnProperty(S) && (b != null || D != null)))
            switch (S) {
              case "type":
                n = b;
                break;
              case "name":
                u = b;
                break;
              case "checked":
                g = b;
                break;
              case "defaultChecked":
                T = b;
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
                b !== D && ht(t, e, S, b, a, D);
            }
        }
        hi(t, c, s, h, g, T, n, u);
        return;
      case "select":
        b = c = s = S = null;
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
                S = n;
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
          S != null
            ? Pl(t, !!l, S, !1)
            : !!a != !!l && (e != null ? Pl(t, !!l, e, !0) : Pl(t, !!l, l ? [] : "", !1)));
        return;
      case "textarea":
        b = S = null;
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
                S = u;
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
        ns(t, S, b);
        return;
      case "option":
        for (var Q in l)
          ((S = l[Q]),
            l.hasOwnProperty(Q) &&
              S != null &&
              !a.hasOwnProperty(Q) &&
              (Q === "selected" ? (t.selected = !1) : ht(t, e, Q, null, a, S)));
        for (h in a)
          ((S = a[h]),
            (b = l[h]),
            a.hasOwnProperty(h) &&
              S !== b &&
              (S != null || b != null) &&
              (h === "selected"
                ? (t.selected = S && typeof S != "function" && typeof S != "symbol")
                : ht(t, e, h, S, a, b)));
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
          ((S = l[X]),
            l.hasOwnProperty(X) && S != null && !a.hasOwnProperty(X) && ht(t, e, X, null, a, S));
        for (g in a)
          if (((S = a[g]), (b = l[g]), a.hasOwnProperty(g) && S !== b && (S != null || b != null)))
            switch (g) {
              case "children":
              case "dangerouslySetInnerHTML":
                if (S != null) throw Error(r(137, e));
                break;
              default:
                ht(t, e, g, S, a, b);
            }
        return;
      default:
        if (yi(e)) {
          for (var dt in l)
            ((S = l[dt]),
              l.hasOwnProperty(dt) &&
                S !== void 0 &&
                !a.hasOwnProperty(dt) &&
                tf(t, e, dt, void 0, a, S));
          for (T in a)
            ((S = a[T]),
              (b = l[T]),
              !a.hasOwnProperty(T) ||
                S === b ||
                (S === void 0 && b === void 0) ||
                tf(t, e, T, S, a, b));
          return;
        }
    }
    for (var m in l)
      ((S = l[m]),
        l.hasOwnProperty(m) && S != null && !a.hasOwnProperty(m) && ht(t, e, m, null, a, S));
    for (D in a)
      ((S = a[D]),
        (b = l[D]),
        !a.hasOwnProperty(D) || S === b || (S == null && b == null) || ht(t, e, D, S, a, b));
  }
  function rh(t) {
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
  function dm() {
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
        if (n && s && rh(c)) {
          for (c = 0, s = u.responseEnd, a += 1; a < l.length; a++) {
            var h = l[a],
              g = h.startTime;
            if (g > s) break;
            var T = h.transferSize,
              D = h.initiatorType;
            T && rh(D) && ((h = h.responseEnd), (c += T * (h < s ? 1 : (s - g) / (h - g))));
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
  var ef = null,
    lf = null;
  function qn(t) {
    return t.nodeType === 9 ? t : t.ownerDocument;
  }
  function oh(t) {
    switch (t) {
      case "http://www.w3.org/2000/svg":
        return 1;
      case "http://www.w3.org/1998/Math/MathML":
        return 2;
      default:
        return 0;
    }
  }
  function hh(t, e) {
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
  function af(t, e) {
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
  var uf = null;
  function ym() {
    var t = window.event;
    return t && t.type === "popstate" ? (t === uf ? !1 : ((uf = t), !0)) : ((uf = null), !1);
  }
  var dh = typeof setTimeout == "function" ? setTimeout : void 0,
    mm = typeof clearTimeout == "function" ? clearTimeout : void 0,
    yh = typeof Promise == "function" ? Promise : void 0,
    vm =
      typeof queueMicrotask == "function"
        ? queueMicrotask
        : typeof yh < "u"
          ? function (t) {
              return yh.resolve(null).then(t).catch(gm);
            }
          : dh;
  function gm(t) {
    setTimeout(function () {
      throw t;
    });
  }
  function bl(t) {
    return t === "head";
  }
  function mh(t, e) {
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
            (n[Qa] ||
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
  function nf(t) {
    var e = t.firstChild;
    for (e && e.nodeType === 10 && (e = e.nextSibling); e; ) {
      var l = e;
      switch (((e = e.nextSibling), l.nodeName)) {
        case "HTML":
        case "HEAD":
        case "BODY":
          (nf(l), ri(l));
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
  function Sm(t, e, l, a) {
    for (; t.nodeType === 1; ) {
      var u = l;
      if (t.nodeName.toLowerCase() !== e.toLowerCase()) {
        if (!a && (t.nodeName !== "INPUT" || t.type !== "hidden")) break;
      } else if (a) {
        if (!t[Qa])
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
  function pm(t, e, l) {
    if (e === "") return null;
    for (; t.nodeType !== 3; )
      if (
        ((t.nodeType !== 1 || t.nodeName !== "INPUT" || t.type !== "hidden") && !l) ||
        ((t = Ae(t.nextSibling)), t === null)
      )
        return null;
    return t;
  }
  function gh(t, e) {
    for (; t.nodeType !== 8; )
      if (
        ((t.nodeType !== 1 || t.nodeName !== "INPUT" || t.type !== "hidden") && !e) ||
        ((t = Ae(t.nextSibling)), t === null)
      )
        return null;
    return t;
  }
  function cf(t) {
    return t.data === "$?" || t.data === "$~";
  }
  function ff(t) {
    return t.data === "$!" || (t.data === "$?" && t.ownerDocument.readyState !== "loading");
  }
  function bm(t, e) {
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
  var sf = null;
  function Sh(t) {
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
  function ph(t) {
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
  function bh(t, e, l) {
    switch (((e = qn(l)), t)) {
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
    ri(t);
  }
  var Me = new Map(),
    Eh = new Set();
  function Qn(t) {
    return typeof t.getRootNode == "function"
      ? t.getRootNode()
      : t.nodeType === 9
        ? t
        : t.ownerDocument;
  }
  var tl = x.d;
  x.d = { f: Em, r: Tm, D: Om, C: zm, L: Am, m: Mm, X: Dm, S: _m, M: Rm };
  function Em() {
    var t = tl.f(),
      e = Dn();
    return t || e;
  }
  function Tm(t) {
    var e = kl(t);
    e !== null && e.tag === 5 && e.type === "form" ? Qr(e) : tl.r(t);
  }
  var Da = typeof document > "u" ? null : document;
  function Th(t, e, l) {
    var a = Da;
    if (a && typeof e == "string" && e) {
      var u = Se(e);
      ((u = 'link[rel="' + t + '"][href="' + u + '"]'),
        typeof l == "string" && (u += '[crossorigin="' + l + '"]'),
        Eh.has(u) ||
          (Eh.add(u),
          (t = { rel: t, crossOrigin: l, href: e }),
          a.querySelector(u) === null &&
            ((e = a.createElement("link")), Vt(e, "link", t), Yt(e), a.head.appendChild(e))));
    }
  }
  function Om(t) {
    (tl.D(t), Th("dns-prefetch", t, null));
  }
  function zm(t, e) {
    (tl.C(t, e), Th("preconnect", t, e));
  }
  function Am(t, e, l) {
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
          n = Ra(t);
          break;
        case "script":
          n = Ua(t);
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
  function Mm(t, e) {
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
          n = Ua(t);
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
  function _m(t, e, l) {
    tl.S(t, e, l);
    var a = Da;
    if (a && t) {
      var u = $l(a).hoistableStyles,
        n = Ra(t);
      e = e || "default";
      var c = u.get(n);
      if (!c) {
        var s = { loading: 0, preload: null };
        if ((c = a.querySelector(bu(n)))) s.loading = 5;
        else {
          ((t = N({ rel: "stylesheet", href: t, "data-precedence": e }, l)),
            (l = Me.get(n)) && rf(t, l));
          var h = (c = a.createElement("link"));
          (Yt(h),
            Vt(h, "link", t),
            (h._p = new Promise(function (g, T) {
              ((h.onload = g), (h.onerror = T));
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
  function Dm(t, e) {
    tl.X(t, e);
    var l = Da;
    if (l && t) {
      var a = $l(l).hoistableScripts,
        u = Ua(t),
        n = a.get(u);
      n ||
        ((n = l.querySelector(Eu(u))),
        n ||
          ((t = N({ src: t, async: !0 }, e)),
          (e = Me.get(u)) && of(t, e),
          (n = l.createElement("script")),
          Yt(n),
          Vt(n, "link", t),
          l.head.appendChild(n)),
        (n = { type: "script", instance: n, count: 1, state: null }),
        a.set(u, n));
    }
  }
  function Rm(t, e) {
    tl.M(t, e);
    var l = Da;
    if (l && t) {
      var a = $l(l).hoistableScripts,
        u = Ua(t),
        n = a.get(u);
      n ||
        ((n = l.querySelector(Eu(u))),
        n ||
          ((t = N({ src: t, async: !0, type: "module" }, e)),
          (e = Me.get(u)) && of(t, e),
          (n = l.createElement("script")),
          Yt(n),
          Vt(n, "link", t),
          l.head.appendChild(n)),
        (n = { type: "script", instance: n, count: 1, state: null }),
        a.set(u, n));
    }
  }
  function Oh(t, e, l, a) {
    var u = (u = k.current) ? Qn(u) : null;
    if (!u) throw Error(r(446));
    switch (t) {
      case "meta":
      case "title":
        return null;
      case "style":
        return typeof l.precedence == "string" && typeof l.href == "string"
          ? ((e = Ra(l.href)),
            (l = $l(u).hoistableStyles),
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
          t = Ra(l.href);
          var n = $l(u).hoistableStyles,
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
                n || Um(u, t, l, c.state))),
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
            ? ((e = Ua(l)),
              (l = $l(u).hoistableScripts),
              (a = l.get(e)),
              a || ((a = { type: "script", instance: null, count: 0, state: null }), l.set(e, a)),
              a)
            : { type: "void", instance: null, count: 0, state: null }
        );
      default:
        throw Error(r(444, t));
    }
  }
  function Ra(t) {
    return 'href="' + Se(t) + '"';
  }
  function bu(t) {
    return 'link[rel="stylesheet"][' + t + "]";
  }
  function zh(t) {
    return N({}, t, { "data-precedence": t.precedence, precedence: null });
  }
  function Um(t, e, l, a) {
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
  function Ua(t) {
    return '[src="' + Se(t) + '"]';
  }
  function Eu(t) {
    return "script[async]" + t;
  }
  function Ah(t, e, l) {
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
          u = Ra(l.href);
          var n = t.querySelector(bu(u));
          if (n) return ((e.state.loading |= 4), (e.instance = n), Yt(n), n);
          ((a = zh(l)),
            (u = Me.get(u)) && rf(a, u),
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
            (n = Ua(l.src)),
            (u = t.querySelector(Eu(n)))
              ? ((e.instance = u), Yt(u), u)
              : ((a = l),
                (u = Me.get(n)) && ((a = N({}, l)), of(a, u)),
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
  function rf(t, e) {
    (t.crossOrigin == null && (t.crossOrigin = e.crossOrigin),
      t.referrerPolicy == null && (t.referrerPolicy = e.referrerPolicy),
      t.title == null && (t.title = e.title));
  }
  function of(t, e) {
    (t.crossOrigin == null && (t.crossOrigin = e.crossOrigin),
      t.referrerPolicy == null && (t.referrerPolicy = e.referrerPolicy),
      t.integrity == null && (t.integrity = e.integrity));
  }
  var Yn = null;
  function Mh(t, e, l) {
    if (Yn === null) {
      var a = new Map(),
        u = (Yn = new Map());
      u.set(l, a);
    } else ((u = Yn), (a = u.get(l)), a || ((a = new Map()), u.set(l, a)));
    if (a.has(t)) return a;
    for (a.set(t, null), l = l.getElementsByTagName(t), u = 0; u < l.length; u++) {
      var n = l[u];
      if (
        !(n[Qa] || n[Xt] || (t === "link" && n.getAttribute("rel") === "stylesheet")) &&
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
  function _h(t, e, l) {
    ((t = t.ownerDocument || t),
      t.head.insertBefore(l, e === "title" ? t.querySelector("head > title") : null));
  }
  function Cm(t, e, l) {
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
  function Dh(t) {
    return !(t.type === "stylesheet" && (t.state.loading & 3) === 0);
  }
  function jm(t, e, l, a) {
    if (
      l.type === "stylesheet" &&
      (typeof a.media != "string" || matchMedia(a.media).matches !== !1) &&
      (l.state.loading & 4) === 0
    ) {
      if (l.instance === null) {
        var u = Ra(a.href),
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
          (a = zh(a)),
          (u = Me.get(u)) && rf(a, u),
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
  var hf = 0;
  function Nm(t, e) {
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
            0 < t.imgBytes && hf === 0 && (hf = 62500 * dm());
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
              (t.imgBytes > hf ? 50 : 800) + e,
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
        (t.count++, (Xn = new Map()), e.forEach(xm, t), (Xn = null), Gn.call(t)));
  }
  function xm(t, e) {
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
  function Hm(t, e, l, a, u, n, c, s, h) {
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
      (this.expirationTimes = ii(-1)),
      (this.entangledLanes =
        this.shellSuspendCounter =
        this.errorRecoveryDisabledLanes =
        this.expiredLanes =
        this.warmLanes =
        this.pingedLanes =
        this.suspendedLanes =
        this.pendingLanes =
          0),
      (this.entanglements = ii(0)),
      (this.hiddenUpdates = ii(null)),
      (this.identifierPrefix = a),
      (this.onUncaughtError = u),
      (this.onCaughtError = n),
      (this.onRecoverableError = c),
      (this.pooledCache = null),
      (this.pooledCacheLanes = 0),
      (this.formState = h),
      (this.incompleteTransitions = new Map()));
  }
  function Rh(t, e, l, a, u, n, c, s, h, g, T, D) {
    return (
      (t = new Hm(t, e, l, c, h, g, T, D, s)),
      (e = 1),
      n === !0 && (e |= 24),
      (n = se(3, null, null, e)),
      (t.current = n),
      (n.stateNode = t),
      (e = Ki()),
      e.refCount++,
      (t.pooledCache = e),
      e.refCount++,
      (n.memoizedState = { element: a, isDehydrated: l, cache: e }),
      Fi(n),
      t
    );
  }
  function Uh(t) {
    return t ? ((t = ca), t) : ca;
  }
  function Ch(t, e, l, a, u, n) {
    ((u = Uh(u)),
      a.context === null ? (a.context = u) : (a.pendingContext = u),
      (a = sl(e)),
      (a.payload = { element: l }),
      (n = n === void 0 ? null : n),
      n !== null && (a.callback = n),
      (l = rl(t, a, e)),
      l !== null && (ae(l, t, e), tu(l, t, e)));
  }
  function jh(t, e) {
    if (((t = t.memoizedState), t !== null && t.dehydrated !== null)) {
      var l = t.retryLane;
      t.retryLane = l !== 0 && l < e ? l : e;
    }
  }
  function df(t, e) {
    (jh(t, e), (t = t.alternate) && jh(t, e));
  }
  function Nh(t) {
    if (t.tag === 13 || t.tag === 31) {
      var e = Nl(t, 67108864);
      (e !== null && ae(e, t, 67108864), df(t, 67108864));
    }
  }
  function xh(t) {
    if (t.tag === 13 || t.tag === 31) {
      var e = ye();
      e = ci(e);
      var l = Nl(t, e);
      (l !== null && ae(l, t, e), df(t, e));
    }
  }
  var Zn = !0;
  function qm(t, e, l, a) {
    var u = z.T;
    z.T = null;
    var n = x.p;
    try {
      ((x.p = 2), yf(t, e, l, a));
    } finally {
      ((x.p = n), (z.T = u));
    }
  }
  function Qm(t, e, l, a) {
    var u = z.T;
    z.T = null;
    var n = x.p;
    try {
      ((x.p = 8), yf(t, e, l, a));
    } finally {
      ((x.p = n), (z.T = u));
    }
  }
  function yf(t, e, l, a) {
    if (Zn) {
      var u = mf(a);
      if (u === null) (Pc(t, e, a, Kn, l), qh(t, a));
      else if (Ym(u, t, e, l, a)) a.stopPropagation();
      else if ((qh(t, a), e & 4 && -1 < Bm.indexOf(t))) {
        for (; u !== null; ) {
          var n = kl(u);
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
                    (He(n), (nt & 6) === 0 && ((Mn = ne() + 500), vu(0)));
                  }
                }
                break;
              case 31:
              case 13:
                ((s = Nl(n, 2)), s !== null && ae(s, n, 2), Dn(), df(n, 2));
            }
          if (((n = mf(a)), n === null && Pc(t, e, a, Kn, l), n === u)) break;
          u = n;
        }
        u !== null && a.stopPropagation();
      } else Pc(t, e, a, null, l);
    }
  }
  function mf(t) {
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
          if (((t = C(e)), t !== null)) return t;
          t = null;
        } else if (l === 31) {
          if (((t = q(e)), t !== null)) return t;
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
  function Hh(t) {
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
        switch (Od()) {
          case Lf:
            return 2;
          case Zf:
            return 8;
          case ju:
          case zd:
            return 32;
          case Kf:
            return 268435456;
          default:
            return 32;
        }
      default:
        return 32;
    }
  }
  var gf = !1,
    El = null,
    Tl = null,
    Ol = null,
    Ou = new Map(),
    zu = new Map(),
    zl = [],
    Bm =
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
        e !== null && ((e = kl(e)), e !== null && Nh(e)),
        t)
      : ((t.eventSystemFlags |= a),
        (e = t.targetContainers),
        u !== null && e.indexOf(u) === -1 && e.push(u),
        t);
  }
  function Ym(t, e, l, a, u) {
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
          if (((e = C(l)), e !== null)) {
            ((t.blockedOn = e),
              kf(t.priority, function () {
                xh(l);
              }));
            return;
          }
        } else if (e === 31) {
          if (((e = q(l)), e !== null)) {
            ((t.blockedOn = e),
              kf(t.priority, function () {
                xh(l);
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
      var l = mf(t.nativeEvent);
      if (l === null) {
        l = t.nativeEvent;
        var a = new l.constructor(l.type, l);
        ((mi = a), l.target.dispatchEvent(a), (mi = null));
      } else return ((e = kl(l)), e !== null && Nh(e), (t.blockedOn = l), !1);
      e.shift();
    }
    return !0;
  }
  function Bh(t, e, l) {
    Vn(t) && l.delete(e);
  }
  function Gm() {
    ((gf = !1),
      El !== null && Vn(El) && (El = null),
      Tl !== null && Vn(Tl) && (Tl = null),
      Ol !== null && Vn(Ol) && (Ol = null),
      Ou.forEach(Bh),
      zu.forEach(Bh));
  }
  function Jn(t, e) {
    t.blockedOn === e &&
      ((t.blockedOn = null),
      gf || ((gf = !0), i.unstable_scheduleCallback(i.unstable_NormalPriority, Gm)));
  }
  var wn = null;
  function Yh(t) {
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
          var n = kl(l);
          n !== null &&
            (t.splice(e, 3),
            (e -= 3),
            yc(n, { pending: !0, data: u, method: l.method, action: a }, a, u));
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
          c = u[$t] || null;
        if (typeof n == "function") c || Yh(l);
        else if (c) {
          var s = null;
          if (n && n.hasAttribute("formAction")) {
            if (((u = n), (c = n[$t] || null))) s = c.formAction;
            else if (vf(u) !== null) continue;
          } else s = c.action;
          (typeof s == "function" ? (l[a + 1] = s) : (l.splice(a, 3), (a -= 3)), Yh(l));
        }
      }
  }
  function Gh() {
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
  function Sf(t) {
    this._internalRoot = t;
  }
  ((Fn.prototype.render = Sf.prototype.render =
    function (t) {
      var e = this._internalRoot;
      if (e === null) throw Error(r(409));
      var l = e.current,
        a = ye();
      Ch(l, a, t, e, null, null);
    }),
    (Fn.prototype.unmount = Sf.prototype.unmount =
      function () {
        var t = this._internalRoot;
        if (t !== null) {
          this._internalRoot = null;
          var e = t.containerInfo;
          (Ch(t.current, 2, null, t, null, null), Dn(), (e[Fl] = null));
        }
      }));
  function Fn(t) {
    this._internalRoot = t;
  }
  Fn.prototype.unstable_scheduleHydration = function (t) {
    if (t) {
      var e = Wf();
      t = { blockedOn: null, target: t, priority: e };
      for (var l = 0; l < zl.length && e !== 0 && e < zl[l].priority; l++);
      (zl.splice(l, 0, t), l === 0 && Qh(t));
    }
  };
  var Xh = f.version;
  if (Xh !== "19.2.7") throw Error(r(527, Xh, "19.2.7"));
  x.findDOMNode = function (t) {
    var e = t._reactInternals;
    if (e === void 0)
      throw typeof t.render == "function"
        ? Error(r(188))
        : ((t = Object.keys(t).join(",")), Error(r(268, t)));
    return ((t = E(e)), (t = t !== null ? j(t) : null), (t = t === null ? null : t.stateNode), t);
  };
  var Xm = {
    bundleType: 0,
    version: "19.2.7",
    rendererPackageName: "react-dom",
    currentDispatcherRef: z,
    reconcilerVersion: "19.2.7",
  };
  if (typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ < "u") {
    var Wn = __REACT_DEVTOOLS_GLOBAL_HOOK__;
    if (!Wn.isDisabled && Wn.supportsFiber)
      try {
        ((xa = Wn.inject(Xm)), (ie = Wn));
      } catch {}
  }
  return (
    (_u.createRoot = function (t, e) {
      if (!p(t)) throw Error(r(299));
      var l = !1,
        a = "",
        u = wr,
        n = Fr,
        c = Wr;
      return (
        e != null &&
          (e.unstable_strictMode === !0 && (l = !0),
          e.identifierPrefix !== void 0 && (a = e.identifierPrefix),
          e.onUncaughtError !== void 0 && (u = e.onUncaughtError),
          e.onCaughtError !== void 0 && (n = e.onCaughtError),
          e.onRecoverableError !== void 0 && (c = e.onRecoverableError)),
        (e = Rh(t, 1, !1, null, null, l, a, null, u, n, c, Gh)),
        (t[Fl] = e.current),
        Ic(t),
        new Sf(e)
      );
    }),
    (_u.hydrateRoot = function (t, e, l) {
      if (!p(t)) throw Error(r(299));
      var a = !1,
        u = "",
        n = wr,
        c = Fr,
        s = Wr,
        h = null;
      return (
        l != null &&
          (l.unstable_strictMode === !0 && (a = !0),
          l.identifierPrefix !== void 0 && (u = l.identifierPrefix),
          l.onUncaughtError !== void 0 && (n = l.onUncaughtError),
          l.onCaughtError !== void 0 && (c = l.onCaughtError),
          l.onRecoverableError !== void 0 && (s = l.onRecoverableError),
          l.formState !== void 0 && (h = l.formState)),
        (e = Rh(t, 1, !0, e, l ?? null, a, u, h, n, c, s, Gh)),
        (e.context = Uh(null)),
        (l = e.current),
        (a = ye()),
        (a = ci(a)),
        (u = sl(a)),
        (u.callback = null),
        rl(l, u, a),
        (l = a),
        (e.current.lanes = l),
        qa(e, l),
        He(e),
        (t[Fl] = e.current),
        Ic(t),
        new Fn(e)
      );
    }),
    (_u.version = "19.2.7"),
    _u
  );
}
var $h;
function $m() {
  if ($h) return Ef.exports;
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
  return (i(), (Ef.exports = km()), Ef.exports);
}
var Im = $m(),
  ja = class {
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
  Pm = class extends ja {
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
  Hf = new Pm(),
  tv = {
    setTimeout: (i, f) => setTimeout(i, f),
    clearTimeout: (i) => clearTimeout(i),
    setInterval: (i, f) => setInterval(i, f),
    clearInterval: (i) => clearInterval(i),
  },
  ev = class {
    #t = tv;
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
  Jl = new ev();
function lv(i) {
  setTimeout(i, 0);
}
var av = typeof window > "u" || "Deno" in globalThis;
function kt() {}
function uv(i, f) {
  return typeof i == "function" ? i(f) : i;
}
function Mf(i) {
  return typeof i == "number" && i >= 0 && i !== 1 / 0;
}
function rd(i, f) {
  return Math.max(i + (f || 0) - Date.now(), 0);
}
function Ml(i, f) {
  return typeof i == "function" ? i(f) : i;
}
function me(i, f) {
  return typeof i == "function" ? i(f) : i;
}
function Ih(i, f) {
  const { type: o = "all", exact: r, fetchStatus: p, predicate: M, queryKey: C, stale: q } = i;
  if (C) {
    if (r) {
      if (f.queryHash !== qf(C, f.options)) return !1;
    } else if (!Ru(f.queryKey, C)) return !1;
  }
  if (o !== "all") {
    const A = f.isActive();
    if ((o === "active" && !A) || (o === "inactive" && A)) return !1;
  }
  return !(
    (typeof q == "boolean" && f.isStale() !== q) ||
    (p && p !== f.state.fetchStatus) ||
    (M && !M(f))
  );
}
function Ph(i, f) {
  const { exact: o, status: r, predicate: p, mutationKey: M } = i;
  if (M) {
    if (!f.options.mutationKey) return !1;
    if (o) {
      if (wl(f.options.mutationKey) !== wl(M)) return !1;
    } else if (!Ru(f.options.mutationKey, M)) return !1;
  }
  return !((r && f.state.status !== r) || (p && !p(f)));
}
function qf(i, f) {
  return (f?.queryKeyHashFn || wl)(i);
}
function wl(i) {
  return JSON.stringify(i, (f, o) =>
    _f(o)
      ? Object.keys(o)
          .sort()
          .reduce((r, p) => ((r[p] = o[p]), r), {})
      : o,
  );
}
function Ru(i, f) {
  return i === f
    ? !0
    : typeof i != typeof f
      ? !1
      : i && f && typeof i == "object" && typeof f == "object"
        ? Object.keys(f).every((o) => Ru(i[o], f[o]))
        : !1;
}
var nv = Object.prototype.hasOwnProperty;
function od(i, f, o = 0) {
  if (i === f) return i;
  if (o > 500) return f;
  const r = td(i) && td(f);
  if (!r && !(_f(i) && _f(f))) return f;
  const M = (r ? i : Object.keys(i)).length,
    C = r ? f : Object.keys(f),
    q = C.length,
    A = r ? new Array(q) : {};
  let E = 0;
  for (let j = 0; j < q; j++) {
    const N = r ? j : C[j],
      U = i[N],
      lt = f[N];
    if (U === lt) {
      ((A[N] = U), (r ? j < M : nv.call(i, N)) && E++);
      continue;
    }
    if (U === null || lt === null || typeof U != "object" || typeof lt != "object") {
      A[N] = lt;
      continue;
    }
    const W = od(U, lt, o + 1);
    ((A[N] = W), W === U && E++);
  }
  return M === q && E === M ? i : A;
}
function $n(i, f) {
  if (!f || Object.keys(i).length !== Object.keys(f).length) return !1;
  for (const o in i) if (i[o] !== f[o]) return !1;
  return !0;
}
function td(i) {
  return Array.isArray(i) && i.length === Object.keys(i).length;
}
function _f(i) {
  if (!ed(i)) return !1;
  const f = i.constructor;
  if (f === void 0) return !0;
  const o = f.prototype;
  return !(
    !ed(o) ||
    !o.hasOwnProperty("isPrototypeOf") ||
    Object.getPrototypeOf(i) !== Object.prototype
  );
}
function ed(i) {
  return Object.prototype.toString.call(i) === "[object Object]";
}
function iv(i) {
  return new Promise((f) => {
    Jl.setTimeout(f, i);
  });
}
function Df(i, f, o) {
  return typeof o.structuralSharing == "function"
    ? o.structuralSharing(i, f)
    : o.structuralSharing !== !1
      ? od(i, f)
      : f;
}
function cv(i, f, o = 0) {
  const r = [...i, f];
  return o && r.length > o ? r.slice(1) : r;
}
function fv(i, f, o = 0) {
  const r = [f, ...i];
  return o && r.length > o ? r.slice(0, -1) : r;
}
var Qf = Symbol();
function hd(i, f) {
  return !i.queryFn && f?.initialPromise
    ? () => f.initialPromise
    : !i.queryFn || i.queryFn === Qf
      ? () => Promise.reject(new Error(`Missing queryFn: '${i.queryHash}'`))
      : i.queryFn;
}
function Bf(i, f) {
  return typeof i == "function" ? i(...f) : !!i;
}
function sv(i, f, o) {
  let r = !1,
    p;
  return (
    Object.defineProperty(i, "signal", {
      enumerable: !0,
      get: () => (
        (p ??= f()),
        r || ((r = !0), p.aborted ? o() : p.addEventListener("abort", o, { once: !0 })),
        p
      ),
    }),
    i
  );
}
var Uu = (() => {
  let i = () => av;
  return {
    isServer() {
      return i();
    },
    setIsServer(f) {
      i = f;
    },
  };
})();
function Rf() {
  let i, f;
  const o = new Promise((p, M) => {
    ((i = p), (f = M));
  });
  ((o.status = "pending"), o.catch(() => {}));
  function r(p) {
    (Object.assign(o, p), delete o.resolve, delete o.reject);
  }
  return (
    (o.resolve = (p) => {
      (r({ status: "fulfilled", value: p }), i(p));
    }),
    (o.reject = (p) => {
      (r({ status: "rejected", reason: p }), f(p));
    }),
    o
  );
}
var rv = lv;
function ov() {
  let i = [],
    f = 0,
    o = (q) => {
      q();
    },
    r = (q) => {
      q();
    },
    p = rv;
  const M = (q) => {
      f
        ? i.push(q)
        : p(() => {
            o(q);
          });
    },
    C = () => {
      const q = i;
      ((i = []),
        q.length &&
          p(() => {
            r(() => {
              q.forEach((A) => {
                o(A);
              });
            });
          }));
    };
  return {
    batch: (q) => {
      let A;
      f++;
      try {
        A = q();
      } finally {
        (f--, f || C());
      }
      return A;
    },
    batchCalls:
      (q) =>
      (...A) => {
        M(() => {
          q(...A);
        });
      },
    schedule: M,
    setNotifyFunction: (q) => {
      o = q;
    },
    setBatchNotifyFunction: (q) => {
      r = q;
    },
    setScheduler: (q) => {
      p = q;
    },
  };
}
var qt = ov(),
  hv = class extends ja {
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
  In = new hv();
function dv(i) {
  return Math.min(1e3 * 2 ** i, 3e4);
}
function dd(i) {
  return (i ?? "online") === "online" ? In.isOnline() : !0;
}
var Uf = class extends Error {
  constructor(i) {
    (super("CancelledError"), (this.revert = i?.revert), (this.silent = i?.silent));
  }
};
function yd(i) {
  let f = !1,
    o = 0,
    r;
  const p = Rf(),
    M = () => p.status !== "pending",
    C = (Z) => {
      if (!M()) {
        const St = new Uf(Z);
        (U(St), i.onCancel?.(St));
      }
    },
    q = () => {
      f = !0;
    },
    A = () => {
      f = !1;
    },
    E = () => Hf.isFocused() && (i.networkMode === "always" || In.isOnline()) && i.canRun(),
    j = () => dd(i.networkMode) && i.canRun(),
    N = (Z) => {
      M() || (r?.(), p.resolve(Z));
    },
    U = (Z) => {
      M() || (r?.(), p.reject(Z));
    },
    lt = () =>
      new Promise((Z) => {
        ((r = (St) => {
          (M() || E()) && Z(St);
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
          const Dt = i.retry ?? (Uu.isServer() ? 0 : 3),
            gt = i.retryDelay ?? dv,
            Rt = typeof gt == "function" ? gt(o, st) : gt,
            Qt =
              Dt === !0 ||
              (typeof Dt == "number" && o < Dt) ||
              (typeof Dt == "function" && Dt(o, st));
          if (f || !Qt) {
            U(st);
            return;
          }
          (o++,
            i.onFail?.(o, st),
            iv(Rt)
              .then(() => (E() ? void 0 : lt()))
              .then(() => {
                f ? U(st) : W();
              }));
        });
    };
  return {
    promise: p,
    status: () => p.status,
    cancel: C,
    continue: () => (r?.(), p),
    cancelRetry: q,
    continueRetry: A,
    canStart: j,
    start: () => (j() ? W() : lt().then(W), p),
  };
}
var md = class {
  #t;
  destroy() {
    this.clearGcTimeout();
  }
  scheduleGc() {
    (this.clearGcTimeout(),
      Mf(this.gcTime) &&
        (this.#t = Jl.setTimeout(() => {
          this.optionalRemove();
        }, this.gcTime)));
  }
  updateGcTime(i) {
    this.gcTime = Math.max(this.gcTime || 0, i ?? (Uu.isServer() ? 1 / 0 : 300 * 1e3));
  }
  clearGcTimeout() {
    this.#t !== void 0 && (Jl.clearTimeout(this.#t), (this.#t = void 0));
  }
};
function yv(i) {
  return {
    onFetch: (f, o) => {
      const r = f.options,
        p = f.fetchOptions?.meta?.fetchMore?.direction,
        M = f.state.data?.pages || [],
        C = f.state.data?.pageParams || [];
      let q = { pages: [], pageParams: [] },
        A = 0;
      const E = async () => {
        let j = !1;
        const N = (W) => {
            sv(
              W,
              () => f.signal,
              () => (j = !0),
            );
          },
          U = hd(f.options, f.fetchOptions),
          lt = async (W, Z, St) => {
            if (j) return Promise.reject(f.signal.reason);
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
              gt = await U(Dt),
              { maxPages: Rt } = f.options,
              Qt = St ? fv : cv;
            return { pages: Qt(W.pages, gt, Rt), pageParams: Qt(W.pageParams, Z, Rt) };
          };
        if (p && M.length) {
          const W = p === "backward",
            Z = W ? mv : ld,
            St = { pages: M, pageParams: C },
            st = Z(r, St);
          q = await lt(St, st, W);
        } else {
          const W = i ?? M.length;
          do {
            const Z = A === 0 ? (C[0] ?? r.initialPageParam) : ld(r, q);
            if (A > 0 && Z == null) break;
            ((q = await lt(q, Z)), A++);
          } while (A < W);
        }
        return q;
      };
      f.options.persister
        ? (f.fetchFn = () =>
            f.options.persister?.(
              E,
              { client: f.client, queryKey: f.queryKey, meta: f.options.meta, signal: f.signal },
              o,
            ))
        : (f.fetchFn = E);
    },
  };
}
function ld(i, { pages: f, pageParams: o }) {
  const r = f.length - 1;
  return f.length > 0 ? i.getNextPageParam(f[r], f, o[r], o) : void 0;
}
function mv(i, { pages: f, pageParams: o }) {
  return f.length > 0 ? i.getPreviousPageParam?.(f[0], f, o[0], o) : void 0;
}
var vv = class extends md {
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
      (this.#e = ud(this.options)),
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
      const f = ud(this.options);
      f.data !== void 0 && (this.setState(ad(f.data, f.dataUpdatedAt)), (this.#e = f));
    }
  }
  optionalRemove() {
    !this.observers.length && this.state.fetchStatus === "idle" && this.#a.remove(this);
  }
  setData(i, f) {
    const o = Df(this.state.data, i, this.options);
    return (
      this.#f({ data: o, type: "success", dataUpdatedAt: f?.updatedAt, manual: f?.manual }), o
    );
  }
  setState(i) {
    this.#f({ type: "setState", state: i });
  }
  cancel(i) {
    const f = this.#u?.promise;
    return (this.#u?.cancel(i), f ? f.then(kt).catch(kt) : Promise.resolve());
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
    return this.observers.some((i) => me(i.options.enabled, this) !== !1);
  }
  isDisabled() {
    return this.getObserversCount() > 0
      ? !this.isActive()
      : this.options.queryFn === Qf || !this.isFetched();
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
          : !rd(this.state.dataUpdatedAt, i);
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
      const A = this.observers.find((E) => E.options.queryFn);
      A && this.setOptions(A.options);
    }
    const o = new AbortController(),
      r = (A) => {
        Object.defineProperty(A, "signal", {
          enumerable: !0,
          get: () => ((this.#i = !0), o.signal),
        });
      },
      p = () => {
        const A = hd(this.options, f),
          j = (() => {
            const N = { client: this.#n, queryKey: this.queryKey, meta: this.meta };
            return (r(N), N);
          })();
        return ((this.#i = !1), this.options.persister ? this.options.persister(A, j, this) : A(j));
      },
      C = (() => {
        const A = {
          fetchOptions: f,
          options: this.options,
          queryKey: this.queryKey,
          client: this.#n,
          state: this.state,
          fetchFn: p,
        };
        return (r(A), A);
      })();
    ((this.#t === "infinite" ? yv(this.options.pages) : this.options.behavior)?.onFetch(C, this),
      (this.#l = this.state),
      (this.state.fetchStatus === "idle" || this.state.fetchMeta !== C.fetchOptions?.meta) &&
        this.#f({ type: "fetch", meta: C.fetchOptions?.meta }),
      (this.#u = yd({
        initialPromise: f?.initialPromise,
        fn: C.fetchFn,
        onCancel: (A) => {
          (A instanceof Uf && A.revert && this.setState({ ...this.#l, fetchStatus: "idle" }),
            o.abort());
        },
        onFail: (A, E) => {
          this.#f({ type: "failed", failureCount: A, error: E });
        },
        onPause: () => {
          this.#f({ type: "pause" });
        },
        onContinue: () => {
          this.#f({ type: "continue" });
        },
        retry: C.options.retry,
        retryDelay: C.options.retryDelay,
        networkMode: C.options.networkMode,
        canRun: () => !0,
      })));
    try {
      const A = await this.#u.start();
      if (A === void 0) throw new Error(`${this.queryHash} data is undefined`);
      return (
        this.setData(A),
        this.#a.config.onSuccess?.(A, this),
        this.#a.config.onSettled?.(A, this.state.error, this),
        A
      );
    } catch (A) {
      if (A instanceof Uf) {
        if (A.silent) return this.#u.promise;
        if (A.revert) {
          if (this.state.data === void 0) throw A;
          return this.state.data;
        }
      }
      throw (
        this.#f({ type: "error", error: A }),
        this.#a.config.onError?.(A, this),
        this.#a.config.onSettled?.(this.state.data, A, this),
        A
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
            ...ad(i.data, i.dataUpdatedAt),
            dataUpdateCount: o.dataUpdateCount + 1,
            ...(!i.manual && {
              fetchStatus: "idle",
              fetchFailureCount: 0,
              fetchFailureReason: null,
            }),
          };
          return ((this.#l = i.manual ? r : void 0), r);
        case "error":
          const p = i.error;
          return {
            ...o,
            error: p,
            errorUpdateCount: o.errorUpdateCount + 1,
            errorUpdatedAt: Date.now(),
            fetchFailureCount: o.fetchFailureCount + 1,
            fetchFailureReason: p,
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
      qt.batch(() => {
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
    fetchStatus: dd(f.networkMode) ? "fetching" : "paused",
    ...(i === void 0 && { error: null, status: "pending" }),
  };
}
function ad(i, f) {
  return {
    data: i,
    dataUpdatedAt: f ?? Date.now(),
    error: null,
    isInvalidated: !1,
    status: "success",
  };
}
function ud(i) {
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
var gv = class extends ja {
  constructor(i, f) {
    (super(),
      (this.options = f),
      (this.#t = i),
      (this.#i = null),
      (this.#c = Rf()),
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
  #m = new Set();
  bindMethods() {
    this.refetch = this.refetch.bind(this);
  }
  onSubscribe() {
    this.listeners.size === 1 &&
      (this.#e.addObserver(this),
      nd(this.#e, this.options) ? this.#d() : this.updateResult(),
      this.#p());
  }
  onUnsubscribe() {
    this.hasListeners() || this.destroy();
  }
  shouldFetchOnReconnect() {
    return Cf(this.#e, this.options, this.options.refetchOnReconnect);
  }
  shouldFetchOnWindowFocus() {
    return Cf(this.#e, this.options, this.options.refetchOnWindowFocus);
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
        typeof me(this.options.enabled, this.#e) != "boolean")
    )
      throw new Error("Expected enabled to be a boolean or a callback that returns a boolean");
    (this.#T(),
      this.#e.setOptions(this.options),
      f._defaulted &&
        !$n(this.options, f) &&
        this.#t
          .getQueryCache()
          .notify({ type: "observerOptionsUpdated", query: this.#e, observer: this }));
    const r = this.hasListeners();
    (r && id(this.#e, o, this.options, f) && this.#d(),
      this.updateResult(),
      r &&
        (this.#e !== o ||
          me(this.options.enabled, this.#e) !== me(f.enabled, this.#e) ||
          Ml(this.options.staleTime, this.#e) !== Ml(f.staleTime, this.#e)) &&
        this.#v());
    const p = this.#g();
    r &&
      (this.#e !== o ||
        me(this.options.enabled, this.#e) !== me(f.enabled, this.#e) ||
        p !== this.#s) &&
      this.#S(p);
  }
  getOptimisticResult(i) {
    const f = this.#t.getQueryCache().build(this.#t, i),
      o = this.createResult(f, i);
    return (pv(this, o) && ((this.#a = o), (this.#u = this.options), (this.#n = this.#e.state)), o);
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
    this.#m.add(i);
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
    return (i?.throwOnError || (f = f.catch(kt)), f);
  }
  #v() {
    this.#b();
    const i = Ml(this.options.staleTime, this.#e);
    if (Uu.isServer() || this.#a.isStale || !Mf(i)) return;
    const o = rd(this.#a.dataUpdatedAt, i) + 1;
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
        Uu.isServer() ||
        me(this.options.enabled, this.#e) === !1 ||
        !Mf(this.#s) ||
        this.#s === 0
      ) &&
        (this.#h = Jl.setInterval(() => {
          (this.options.refetchIntervalInBackground || Hf.isFocused()) && this.#d();
        }, this.#s)));
  }
  #p() {
    (this.#v(), this.#S(this.#g()));
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
      p = this.#a,
      M = this.#n,
      C = this.#u,
      A = i !== o ? i.state : this.#l,
      { state: E } = i;
    let j = { ...E },
      N = !1,
      U;
    if (f._optimisticResults) {
      const yt = this.hasListeners(),
        Jt = !yt && nd(i, f),
        _e = yt && id(i, o, f, r);
      ((Jt || _e) && (j = { ...j, ...vd(E.data, i.options) }),
        f._optimisticResults === "isRestoring" && (j.fetchStatus = "idle"));
    }
    let { error: lt, errorUpdatedAt: W, status: Z } = j;
    U = j.data;
    let St = !1;
    if (f.placeholderData !== void 0 && U === void 0 && Z === "pending") {
      let yt;
      (p?.isPlaceholderData && f.placeholderData === C?.placeholderData
        ? ((yt = p.data), (St = !0))
        : (yt =
            typeof f.placeholderData == "function"
              ? f.placeholderData(this.#y?.state.data, this.#y)
              : f.placeholderData),
        yt !== void 0 && ((Z = "success"), (U = Df(p?.data, yt, f)), (N = !0)));
    }
    if (f.select && U !== void 0 && !St)
      if (p && U === M?.data && f.select === this.#r) U = this.#f;
      else
        try {
          ((this.#r = f.select),
            (U = f.select(U)),
            (U = Df(p?.data, U, f)),
            (this.#f = U),
            (this.#i = null));
        } catch (yt) {
          this.#i = yt;
        }
    this.#i && ((lt = this.#i), (U = this.#f), (W = Date.now()), (Z = "error"));
    const st = j.fetchStatus === "fetching",
      Dt = Z === "pending",
      gt = Z === "error",
      Rt = Dt && st,
      Qt = U !== void 0,
      K = {
        status: Z,
        fetchStatus: j.fetchStatus,
        isPending: Dt,
        isSuccess: Z === "success",
        isError: gt,
        isInitialLoading: Rt,
        isLoading: Rt,
        data: U,
        dataUpdatedAt: j.dataUpdatedAt,
        error: lt,
        errorUpdatedAt: W,
        failureCount: j.fetchFailureCount,
        failureReason: j.fetchFailureReason,
        errorUpdateCount: j.errorUpdateCount,
        isFetched: i.isFetched(),
        isFetchedAfterMount:
          j.dataUpdateCount > A.dataUpdateCount || j.errorUpdateCount > A.errorUpdateCount,
        isFetching: st,
        isRefetching: st && !Dt,
        isLoadingError: gt && !Qt,
        isPaused: j.fetchStatus === "paused",
        isPlaceholderData: N,
        isRefetchError: gt && Qt,
        isStale: Yf(i, f),
        refetch: this.refetch,
        promise: this.#c,
        isEnabled: me(f.enabled, i) !== !1,
      };
    if (this.options.experimental_prefetchInRender) {
      const yt = K.data !== void 0,
        Jt = K.status === "error" && !yt,
        _e = (ve) => {
          Jt ? ve.reject(K.error) : yt && ve.resolve(K.data);
        },
        ue = () => {
          const ve = (this.#c = K.promise = Rf());
          _e(ve);
        },
        xt = this.#c;
      switch (xt.status) {
        case "pending":
          i.queryHash === o.queryHash && _e(xt);
          break;
        case "fulfilled":
          (Jt || K.data !== xt.value) && ue();
          break;
        case "rejected":
          (!Jt || K.error !== xt.reason) && ue();
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
      $n(f, i))
    )
      return;
    this.#a = f;
    const o = () => {
      if (!i) return !0;
      const { notifyOnChangeProps: r } = this.options,
        p = typeof r == "function" ? r() : r;
      if (p === "all" || (!p && !this.#m.size)) return !0;
      const M = new Set(p ?? this.#m);
      return (
        this.options.throwOnError && M.add("error"),
        Object.keys(this.#a).some((C) => {
          const q = C;
          return this.#a[q] !== i[q] && M.has(q);
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
    qt.batch(() => {
      (i.listeners &&
        this.listeners.forEach((f) => {
          f(this.#a);
        }),
        this.#t.getQueryCache().notify({ query: this.#e, type: "observerResultsUpdated" }));
    });
  }
};
function Sv(i, f) {
  return (
    me(f.enabled, i) !== !1 &&
    i.state.data === void 0 &&
    !(i.state.status === "error" && me(f.retryOnMount, i) === !1)
  );
}
function nd(i, f) {
  return Sv(i, f) || (i.state.data !== void 0 && Cf(i, f, f.refetchOnMount));
}
function Cf(i, f, o) {
  if (me(f.enabled, i) !== !1 && Ml(f.staleTime, i) !== "static") {
    const r = typeof o == "function" ? o(i) : o;
    return r === "always" || (r !== !1 && Yf(i, f));
  }
  return !1;
}
function id(i, f, o, r) {
  return (
    (i !== f || me(r.enabled, i) === !1) && (!o.suspense || i.state.status !== "error") && Yf(i, o)
  );
}
function Yf(i, f) {
  return me(f.enabled, i) !== !1 && i.isStaleByTime(Ml(f.staleTime, i));
}
function pv(i, f) {
  return !$n(i.getCurrentResult(), f);
}
var bv = class extends md {
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
      (this.state = i.state || gd()),
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
    this.#a = yd({
      fn: () =>
        this.options.mutationFn
          ? this.options.mutationFn(i, o)
          : Promise.reject(new Error("No mutationFn found")),
      onFail: (M, C) => {
        this.#n({ type: "failed", failureCount: M, error: C });
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
      p = !this.#a.canStart();
    try {
      if (r) f();
      else {
        (this.#n({ type: "pending", variables: i, isPaused: p }),
          this.#l.config.onMutate && (await this.#l.config.onMutate(i, this, o)));
        const C = await this.options.onMutate?.(i, o);
        C !== this.state.context &&
          this.#n({ type: "pending", context: C, variables: i, isPaused: p });
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
      } catch (C) {
        Promise.reject(C);
      }
      try {
        await this.options.onError?.(M, i, this.state.context, o);
      } catch (C) {
        Promise.reject(C);
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
      } catch (C) {
        Promise.reject(C);
      }
      try {
        await this.options.onSettled?.(void 0, M, i, this.state.context, o);
      } catch (C) {
        Promise.reject(C);
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
      qt.batch(() => {
        (this.#e.forEach((o) => {
          o.onMutationUpdate(i);
        }),
          this.#l.notify({ mutation: this, type: "updated", action: i }));
      }));
  }
};
function gd() {
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
var Ev = class extends ja {
  constructor(i = {}) {
    (super(), (this.config = i), (this.#t = new Set()), (this.#e = new Map()), (this.#l = 0));
  }
  #t;
  #e;
  #l;
  build(i, f, o) {
    const r = new bv({
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
    const f = kn(i);
    if (typeof f == "string") {
      const o = this.#e.get(f);
      o ? o.push(i) : this.#e.set(f, [i]);
    }
    this.notify({ type: "added", mutation: i });
  }
  remove(i) {
    if (this.#t.delete(i)) {
      const f = kn(i);
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
    const f = kn(i);
    if (typeof f == "string") {
      const r = this.#e.get(f)?.find((p) => p.state.status === "pending");
      return !r || r === i;
    } else return !0;
  }
  runNext(i) {
    const f = kn(i);
    return typeof f == "string"
      ? (this.#e
          .get(f)
          ?.find((r) => r !== i && r.state.isPaused)
          ?.continue() ?? Promise.resolve())
      : Promise.resolve();
  }
  clear() {
    qt.batch(() => {
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
    return this.getAll().find((o) => Ph(f, o));
  }
  findAll(i = {}) {
    return this.getAll().filter((f) => Ph(i, f));
  }
  notify(i) {
    qt.batch(() => {
      this.listeners.forEach((f) => {
        f(i);
      });
    });
  }
  resumePausedMutations() {
    const i = this.getAll().filter((f) => f.state.isPaused);
    return qt.batch(() => Promise.all(i.map((f) => f.continue().catch(kt))));
  }
};
function kn(i) {
  return i.options.scope?.id;
}
var Tv = class extends ja {
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
        $n(this.options, o) ||
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
      const f = this.#l?.state ?? gd();
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
      qt.batch(() => {
        if (this.#a && this.hasListeners()) {
          const o = this.#e.variables,
            r = this.#e.context,
            p = { client: this.#t, meta: this.options.meta, mutationKey: this.options.mutationKey };
          if (f?.type === "success") {
            try {
              this.#a.onSuccess?.(f.data, o, r, p);
            } catch (M) {
              Promise.reject(M);
            }
            try {
              this.#a.onSettled?.(f.data, null, o, r, p);
            } catch (M) {
              Promise.reject(M);
            }
          } else if (f?.type === "error") {
            try {
              this.#a.onError?.(f.error, o, r, p);
            } catch (M) {
              Promise.reject(M);
            }
            try {
              this.#a.onSettled?.(void 0, f.error, o, r, p);
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
  Ov = class extends ja {
    constructor(i = {}) {
      (super(), (this.config = i), (this.#t = new Map()));
    }
    #t;
    build(i, f, o) {
      const r = f.queryKey,
        p = f.queryHash ?? qf(r, f);
      let M = this.get(p);
      return (
        M ||
          ((M = new vv({
            client: i,
            queryKey: r,
            queryHash: p,
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
      qt.batch(() => {
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
      return this.getAll().find((o) => Ih(f, o));
    }
    findAll(i = {}) {
      const f = this.getAll();
      return Object.keys(i).length > 0 ? f.filter((o) => Ih(i, o)) : f;
    }
    notify(i) {
      qt.batch(() => {
        this.listeners.forEach((f) => {
          f(i);
        });
      });
    }
    onFocus() {
      qt.batch(() => {
        this.getAll().forEach((i) => {
          i.onFocus();
        });
      });
    }
    onOnline() {
      qt.batch(() => {
        this.getAll().forEach((i) => {
          i.onOnline();
        });
      });
    }
  },
  zv = class {
    #t;
    #e;
    #l;
    #a;
    #n;
    #u;
    #c;
    #i;
    constructor(i = {}) {
      ((this.#t = i.queryCache || new Ov()),
        (this.#e = i.mutationCache || new Ev()),
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
        C = uv(f, M);
      if (C !== void 0) return this.#t.build(this, r).setData(C, { ...o, manual: !0 });
    }
    setQueriesData(i, f, o) {
      return qt.batch(() =>
        this.#t.findAll(i).map(({ queryKey: r }) => [r, this.setQueryData(r, f, o)]),
      );
    }
    getQueryState(i) {
      const f = this.defaultQueryOptions({ queryKey: i });
      return this.#t.get(f.queryHash)?.state;
    }
    removeQueries(i) {
      const f = this.#t;
      qt.batch(() => {
        f.findAll(i).forEach((o) => {
          f.remove(o);
        });
      });
    }
    resetQueries(i, f) {
      const o = this.#t;
      return qt.batch(
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
        r = qt.batch(() => this.#t.findAll(i).map((p) => p.cancel(o)));
      return Promise.all(r).then(kt).catch(kt);
    }
    invalidateQueries(i, f = {}) {
      return qt.batch(
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
        r = qt.batch(() =>
          this.#t
            .findAll(i)
            .filter((p) => !p.isDisabled() && !p.isStatic())
            .map((p) => {
              let M = p.fetch(void 0, o);
              return (
                o.throwOnError || (M = M.catch(kt)),
                p.state.fetchStatus === "paused" ? Promise.resolve() : M
              );
            }),
        );
      return Promise.all(r).then(kt);
    }
    fetchQuery(i) {
      const f = this.defaultQueryOptions(i);
      f.retry === void 0 && (f.retry = !1);
      const o = this.#t.build(this, f);
      return o.isStaleByTime(Ml(f.staleTime, o)) ? o.fetch(f) : Promise.resolve(o.state.data);
    }
    prefetchQuery(i) {
      return this.fetchQuery(i).then(kt).catch(kt);
    }
    fetchInfiniteQuery(i) {
      return ((i._type = "infinite"), this.fetchQuery(i));
    }
    prefetchInfiniteQuery(i) {
      return this.fetchInfiniteQuery(i).then(kt).catch(kt);
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
          Ru(i, r.queryKey) && Object.assign(o, r.defaultOptions);
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
          Ru(i, r.mutationKey) && Object.assign(o, r.defaultOptions);
        }),
        o
      );
    }
    defaultQueryOptions(i) {
      if (i._defaulted) return i;
      const f = { ...this.#l.queries, ...this.getQueryDefaults(i.queryKey), ...i, _defaulted: !0 };
      return (
        f.queryHash || (f.queryHash = qf(f.queryKey, f)),
        f.refetchOnReconnect === void 0 && (f.refetchOnReconnect = f.networkMode !== "always"),
        f.throwOnError === void 0 && (f.throwOnError = !!f.suspense),
        !f.networkMode && f.persister && (f.networkMode = "offlineFirst"),
        f.queryFn === Qf && (f.enabled = !1),
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
  Sd = Et.createContext(void 0),
  Pn = (i) => {
    const f = Et.useContext(Sd);
    if (!f) throw new Error("No QueryClient set, use QueryClientProvider to set one");
    return f;
  },
  Av = ({ client: i, children: f }) => (
    Et.useEffect(
      () => (
        i.mount(),
        () => {
          i.unmount();
        }
      ),
      [i],
    ),
    O.jsx(Sd.Provider, { value: i, children: f })
  ),
  pd = Et.createContext(!1),
  Mv = () => Et.useContext(pd);
pd.Provider;
function _v() {
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
var Dv = Et.createContext(_v()),
  Rv = () => Et.useContext(Dv),
  Uv = (i, f, o) => {
    const r =
      o?.state.error && typeof i.throwOnError == "function"
        ? Bf(i.throwOnError, [o.state.error, o])
        : i.throwOnError;
    (i.suspense || i.experimental_prefetchInRender || r) && (f.isReset() || (i.retryOnMount = !1));
  },
  Cv = (i) => {
    Et.useEffect(() => {
      i.clearReset();
    }, [i]);
  },
  jv = ({ result: i, errorResetBoundary: f, throwOnError: o, query: r, suspense: p }) =>
    i.isError &&
    !f.isReset() &&
    !i.isFetching &&
    r &&
    ((p && i.data === void 0) || Bf(o, [i.error, r])),
  Nv = (i) => {
    if (i.suspense) {
      const o = (p) => (p === "static" ? p : Math.max(p ?? 1e3, 1e3)),
        r = i.staleTime;
      ((i.staleTime = typeof r == "function" ? (...p) => o(r(...p)) : o(r)),
        typeof i.gcTime == "number" && (i.gcTime = Math.max(i.gcTime, 1e3)));
    }
  },
  xv = (i, f) => i.isLoading && i.isFetching && !f,
  Hv = (i, f) => i?.suspense && f.isPending,
  cd = (i, f, o) =>
    f.fetchOptimistic(i).catch(() => {
      o.clearReset();
    });
function qv(i, f, o) {
  const r = Mv(),
    p = Rv(),
    M = Pn(),
    C = M.defaultQueryOptions(i);
  M.getDefaultOptions().queries?._experimental_beforeQuery?.(C);
  const q = M.getQueryCache().get(C.queryHash),
    A = i.subscribed !== !1;
  ((C._optimisticResults = r ? "isRestoring" : A ? "optimistic" : void 0),
    Nv(C),
    Uv(C, p, q),
    Cv(p));
  const E = !M.getQueryCache().get(C.queryHash),
    [j] = Et.useState(() => new f(M, C)),
    N = j.getOptimisticResult(C),
    U = !r && A;
  if (
    (Et.useSyncExternalStore(
      Et.useCallback(
        (lt) => {
          const W = U ? j.subscribe(qt.batchCalls(lt)) : kt;
          return (j.updateResult(), W);
        },
        [j, U],
      ),
      () => j.getCurrentResult(),
      () => j.getCurrentResult(),
    ),
    Et.useEffect(() => {
      j.setOptions(C);
    }, [C, j]),
    Hv(C, N))
  )
    throw cd(C, j, p);
  if (
    jv({
      result: N,
      errorResetBoundary: p,
      throwOnError: C.throwOnError,
      query: q,
      suspense: C.suspense,
    })
  )
    throw N.error;
  return (
    M.getDefaultOptions().queries?._experimental_afterQuery?.(C, N),
    C.experimental_prefetchInRender &&
      !Uu.isServer() &&
      xv(N, r) &&
      (E ? cd(C, j, p) : q?.promise)?.catch(kt).finally(() => {
        j.updateResult();
      }),
    C.notifyOnChangeProps ? N : j.trackResult(N)
  );
}
function Qv(i, f) {
  return qv(i, gv);
}
function jf(i, f) {
  const o = Pn(),
    [r] = Et.useState(() => new Tv(o, i));
  Et.useEffect(() => {
    r.setOptions(i);
  }, [r, i]);
  const p = Et.useSyncExternalStore(
      Et.useCallback((C) => r.subscribe(qt.batchCalls(C)), [r]),
      () => r.getCurrentResult(),
      () => r.getCurrentResult(),
    ),
    M = Et.useCallback(
      (C, q) => {
        r.mutate(C, q).catch(kt);
      },
      [r],
    );
  if (p.error && Bf(r.options.throwOnError, [p.error])) throw p.error;
  return { ...p, mutate: M, mutateAsync: p.mutate };
}
const Bv = {
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
structuredClone(Bv);
const Yv = "X-Native-Whisperx-Session-Token";
async function Du(i, f) {
  const o = await fetch(i, {
    ...f,
    headers: {
      ...(f?.body ? { "Content-Type": "application/json" } : {}),
      ...(window.nativeWhisperxSessionToken ? { [Yv]: window.nativeWhisperxSessionToken } : {}),
      ...f?.headers,
    },
  });
  if (!o.ok) throw new Error(await o.text());
  return await o.json();
}
const Nf = {
    getState: () => Du("/api/state"),
    updateProfile: async (i, f) => (
      await Du(`/api/profiles/${encodeURIComponent(i)}`, {
        method: "PUT",
        body: JSON.stringify(f),
      }),
      Nf.getState()
    ),
    deleteProfile: async (i) => (
      await Du(`/api/profiles/${encodeURIComponent(i)}`, { method: "DELETE" }),
      Nf.getState()
    ),
    createProfile: () => Du("/api/profiles", { method: "POST", body: JSON.stringify({}) }),
    rebuildTrace: (i) =>
      Du("/api/trace/rebuild", { method: "POST", body: JSON.stringify(i.scanRoot ? i : {}) }),
  },
  Gv = Nf;
function Xv({ api: i = Gv }) {
  const [f] = Et.useState(() => new zv());
  return O.jsx(Av, { client: f, children: O.jsx(Lv, { api: i }) });
}
function Lv({ api: i }) {
  const f = Qv({ queryKey: ["speaker-directory-state"], queryFn: () => i.getState() });
  return f.isLoading
    ? O.jsx("main", { className: "page", children: "Loading Speaker Directory..." })
    : f.isError || !f.data
      ? O.jsxs("main", {
          className: "page",
          children: [
            O.jsx("h1", { children: "Speaker Directory" }),
            O.jsx("p", { role: "alert", children: "Failed to load Speaker Directory state." }),
          ],
        })
      : O.jsx(Zv, { api: i, state: f.data });
}
function Zv({ api: i, state: f }) {
  return O.jsxs("main", {
    className: "page",
    children: [
      O.jsxs("header", {
        className: "header",
        children: [
          O.jsx("p", { className: "eyebrow", children: "CLI workspace" }),
          O.jsx("h1", { children: "Speaker Directory" }),
          O.jsx("p", { className: "path", children: f.path }),
        ],
      }),
      O.jsxs("section", {
        className: "summaryGrid",
        "aria-label": "Speaker Directory summary",
        children: [
          O.jsx(Af, {
            title: "Speaker Library",
            status: f.library.status,
            detail: `${f.library.profileCount} profile${f.library.profileCount === 1 ? "" : "s"}`,
          }),
          O.jsx(Af, {
            title: "Speaker Trace",
            status: f.trace.status,
            detail: f.trace.scanRoot ?? "No scan root",
          }),
          O.jsx(Af, { title: "Scope", status: f.scope, detail: "Speaker Directory" }),
        ],
      }),
      O.jsxs("section", {
        children: [
          O.jsxs("div", {
            className: "sectionHeading",
            children: [
              O.jsx("h2", { children: "Speaker Library profiles" }),
              O.jsx("span", { children: f.profiles.length }),
            ],
          }),
          O.jsx("div", {
            className: "profileList",
            children: f.profiles.map((o) => O.jsx(Kv, { api: i, profile: o }, o.id)),
          }),
        ],
      }),
      O.jsx(Vv, { api: i, trace: f.trace }),
    ],
  });
}
function Af({ title: i, status: f, detail: o }) {
  return O.jsxs("article", {
    className: "statusPanel",
    children: [
      O.jsx("h2", { children: i }),
      O.jsx("p", { className: "status", children: f }),
      O.jsx("p", { children: o }),
    ],
  });
}
function Kv({ api: i, profile: f }) {
  const o = Pn(),
    [r, p] = Et.useState(f.label),
    [M, C] = Et.useState(Wv(f.metadata)),
    [q, A] = Et.useState(null),
    E = jf({
      mutationFn: () => {
        const U = sd(M);
        return (A(null), i.updateProfile(f.id, { id: f.id, label: r, metadata: U }));
      },
      onSuccess: (U) => {
        o.setQueryData(["speaker-directory-state"], U);
      },
      onError: (U) => {
        A(U instanceof Error ? U.message : "Failed to save Speaker Library profile.");
      },
    }),
    j = jf({
      mutationFn: () => (A(null), i.deleteProfile(f.id)),
      onSuccess: (U) => {
        o.setQueryData(["speaker-directory-state"], U);
      },
      onError: (U) => {
        A(U instanceof Error ? U.message : "Failed to delete Speaker Library profile.");
      },
    }),
    N = () => {
      try {
        sd(M);
      } catch (U) {
        A(U instanceof Error ? U.message : "Speaker Library profile metadata is malformed.");
        return;
      }
      E.mutate();
    };
  return O.jsxs("article", {
    className: "profile",
    children: [
      O.jsxs("div", {
        className: "profileIdentity",
        children: [
          O.jsxs("div", {
            children: [
              O.jsx("h3", { children: f.label }),
              O.jsx("p", { className: "identityLabel", children: "Stable profile id" }),
              O.jsx("p", { className: "mono profileId", children: f.id }),
            ],
          }),
          O.jsx("span", { className: "identityBadge", children: "Speaker Library profile" }),
        ],
      }),
      O.jsx("dl", {
        children: Object.entries(f.metadata).map(([U, lt]) =>
          O.jsxs(
            "div",
            { children: [O.jsx("dt", { children: U }), O.jsx("dd", { children: lt })] },
            U,
          ),
        ),
      }),
      O.jsxs("div", {
        className: "profileForm",
        children: [
          O.jsxs("label", {
            children: [
              "Label",
              O.jsx("input", {
                "aria-label": `${f.id} label`,
                value: r,
                onChange: (U) => p(U.currentTarget.value),
              }),
            ],
          }),
          O.jsxs("label", {
            children: [
              "Metadata",
              O.jsx("textarea", {
                "aria-label": `${f.id} metadata`,
                rows: 4,
                value: M,
                onChange: (U) => C(U.currentTarget.value),
              }),
            ],
          }),
          q ? O.jsx("p", { role: "alert", children: q }) : null,
          O.jsxs("div", {
            className: "profileActions",
            children: [
              O.jsx("button", {
                disabled: E.isPending,
                type: "button",
                onClick: N,
                children: "Save profile",
              }),
              O.jsx("button", {
                disabled: j.isPending,
                type: "button",
                onClick: () => j.mutate(),
                children: "Delete profile",
              }),
            ],
          }),
        ],
      }),
    ],
  });
}
function Vv({ api: i, trace: f }) {
  const o = Pn(),
    [r, p] = Et.useState(""),
    [M, C] = Et.useState(null),
    [q, A] = Et.useState(null),
    E = f.speakers.filter((U) => U.kind === "enrolled"),
    j = f.speakers.filter((U) => U.kind === "anonymous"),
    N = jf({
      mutationFn: () => i.rebuildTrace(r.trim() ? { scanRoot: r.trim() } : {}),
      onSuccess: (U) => {
        (C(null), A(U.report), o.setQueryData(["speaker-directory-state"], U.state));
      },
      onError: (U) => {
        C(U instanceof Error ? U.message : "Failed to rebuild Speaker Trace.");
      },
    });
  return O.jsxs("section", {
    children: [
      O.jsxs("div", {
        className: "sectionHeading",
        children: [
          O.jsx("h2", { children: "Speaker Trace" }),
          O.jsx("span", { children: f.speakers.length }),
        ],
      }),
      O.jsxs("div", {
        className: "traceMeta",
        children: [
          O.jsx("span", { children: "Scan root" }),
          O.jsx("code", { children: f.scanRoot || "Not available" }),
        ],
      }),
      f.message ? O.jsx("p", { className: "muted", children: f.message }) : null,
      O.jsxs("div", {
        className: "profile traceRebuild",
        children: [
          O.jsxs("label", {
            children: [
              "Trace rebuild scan root",
              O.jsx("input", {
                "aria-label": "Trace rebuild scan root",
                placeholder: "Optional transcript scan root",
                value: r,
                onChange: (U) => p(U.currentTarget.value),
              }),
            ],
          }),
          M ? O.jsx("p", { role: "alert", children: M }) : null,
          O.jsx("button", {
            disabled: N.isPending,
            type: "button",
            onClick: () => N.mutate(),
            children: "Rebuild Speaker Trace",
          }),
        ],
      }),
      q ? O.jsx(Jv, { report: q }) : null,
      O.jsxs("div", {
        className: "sectionHeading",
        children: [
          O.jsx("h3", { children: "Enrolled Speaker Trace" }),
          O.jsx("span", { children: E.length }),
        ],
      }),
      O.jsx("div", {
        className: "profileList",
        children: E.map((U) =>
          O.jsxs(
            "article",
            {
              className: "profile",
              children: [
                O.jsxs("div", {
                  className: "profileIdentity",
                  children: [
                    O.jsxs("div", {
                      children: [
                        O.jsx("h4", { children: U.label ?? U.profileId }),
                        O.jsx("p", { className: "identityLabel", children: "Stable profile id" }),
                        O.jsx("p", { className: "mono profileId", children: U.profileId }),
                      ],
                    }),
                    O.jsx("span", {
                      className: "identityBadge",
                      children: "Speaker Library profile",
                    }),
                  ],
                }),
                O.jsx(fd, { files: U.files }),
              ],
            },
            U.profileId,
          ),
        ),
      }),
      O.jsxs("div", {
        className: "sectionHeading",
        children: [
          O.jsx("h3", { children: "Anonymous Speaker Labels" }),
          O.jsx("span", { children: j.length }),
        ],
      }),
      O.jsx("p", {
        className: "muted",
        children:
          "Anonymous Speaker Labels are Speaker Trace data, not enrolled Speaker Library identities.",
      }),
      O.jsx("div", {
        className: "profileList",
        children: j.map((U) =>
          O.jsxs(
            "article",
            {
              className: "profile",
              children: [
                O.jsxs("div", {
                  className: "profileIdentity",
                  children: [
                    O.jsxs("div", {
                      children: [
                        O.jsx("h4", { children: U.anonymousLabel ?? "Anonymous Speaker Label" }),
                        O.jsx("p", {
                          className: "identityLabel",
                          children: "Anonymous Speaker Label",
                        }),
                      ],
                    }),
                    O.jsx("span", {
                      className: "identityBadge traceBadge",
                      children: "Trace data only",
                    }),
                  ],
                }),
                O.jsx(fd, { files: U.files }),
              ],
            },
            U.anonymousLabel,
          ),
        ),
      }),
      O.jsxs("div", {
        className: "sectionHeading",
        children: [
          O.jsx("h3", { children: "Malformed transcript JSON" }),
          O.jsx("span", { children: f.errors.length }),
        ],
      }),
      f.errors.length === 0
        ? O.jsx("p", {
            className: "muted",
            children: "No malformed transcript JSON errors recorded.",
          })
        : O.jsx("div", {
            className: "profileList",
            children: f.errors.map((U) =>
              O.jsxs(
                "article",
                {
                  className: "profile",
                  children: [
                    O.jsx("p", { className: "mono profileId", children: U.sourceFile }),
                    O.jsx("p", { children: U.message }),
                  ],
                },
                `${U.sourceFile}:${U.message}`,
              ),
            ),
          }),
    ],
  });
}
function Jv({ report: i }) {
  return O.jsxs("article", {
    className: "profile rebuildReport",
    children: [
      O.jsx("h3", { children: "Rebuild report" }),
      O.jsx("p", { className: "mono profileId", children: i.tracePath }),
      O.jsxs("dl", {
        children: [
          O.jsxs("div", {
            children: [
              O.jsx("dt", { children: "Scanned files" }),
              O.jsx("dd", { children: i.stats.scannedFiles }),
            ],
          }),
          O.jsxs("div", {
            children: [
              O.jsx("dt", { children: "Accepted entries" }),
              O.jsx("dd", { children: i.stats.acceptedEntries }),
            ],
          }),
          O.jsxs("div", {
            children: [
              O.jsx("dt", { children: "Ignored non-JSON files" }),
              O.jsx("dd", { children: i.stats.ignoredNonJsonFiles }),
            ],
          }),
          O.jsxs("div", {
            children: [
              O.jsx("dt", { children: "Malformed JSON errors" }),
              O.jsx("dd", { children: i.stats.malformedJsonErrors }),
            ],
          }),
        ],
      }),
    ],
  });
}
function fd({ files: i }) {
  return i.length === 0
    ? O.jsx("p", { className: "muted", children: "No traced files recorded." })
    : O.jsx("div", {
        className: "traceFileList",
        children: i.map((f) =>
          O.jsxs(
            "div",
            {
              className: "traceFile",
              children: [
                O.jsx("p", { className: "mono profileId", children: f.sourceFile }),
                O.jsxs("div", {
                  className: "traceStats",
                  children: [
                    O.jsxs("span", {
                      children: [f.segmentCount, " segment", f.segmentCount === 1 ? "" : "s"],
                    }),
                    O.jsxs("span", { children: [f.totalDuration.toFixed(2), " seconds"] }),
                  ],
                }),
                O.jsx(wv, { spans: f.spans }),
              ],
            },
            f.sourceFile,
          ),
        ),
      });
}
function wv({ spans: i }) {
  return i.length === 0
    ? O.jsx("p", { className: "muted", children: "No spans recorded." })
    : O.jsx("ol", {
        className: "traceSpanList",
        children: i.map((f, o) =>
          O.jsxs(
            "li",
            {
              children: [
                O.jsx("span", { className: "mono", children: Fv(f) }),
                O.jsx("p", { children: f.snippet }),
              ],
            },
            `${f.startSeconds ?? "unknown"}:${f.endSeconds ?? "unknown"}:${o}`,
          ),
        ),
      });
}
function Fv(i) {
  return i.startSeconds === void 0 || i.endSeconds === void 0
    ? "Timing unavailable"
    : `${i.startSeconds.toFixed(2)}s - ${i.endSeconds.toFixed(2)}s`;
}
function Wv(i) {
  return Object.entries(i).map(([f, o]) => `${f}=${o}`).join(`
`);
}
function sd(i) {
  return i
    ? i
        .split(`
`)
        .reduce((f, o, r) => {
          if (!o.trim())
            throw new Error(`Speaker Library profile metadata line ${r + 1} must be key=value.`);
          const p = o.indexOf("=");
          if (p <= 0 || p === o.length - 1)
            throw new Error(`Speaker Library profile metadata line ${r + 1} must be key=value.`);
          const M = o.slice(0, p).trim(),
            C = o.slice(p + 1).trim();
          if (!M || !C)
            throw new Error(`Speaker Library profile metadata line ${r + 1} must be key=value.`);
          return ((f[M] = C), f);
        }, {})
    : {};
}
Im.createRoot(document.getElementById("root")).render(
  O.jsx(Et.StrictMode, { children: O.jsx(Xv, {}) }),
);
