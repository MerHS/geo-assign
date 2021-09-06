#ifndef _CURVE_H_
#define _CURVE_H_

#define PRECISION   1e-5
#define EPS         1e-6        /* data type is float */
#define INFINITY    FLT_MAX

typedef float REAL;
typedef REAL  Point[2];

typedef struct CubicBezierCurve 
{
	Point control_pts[4];
} CubicBezierCurve;

#ifdef DEBUG
void PRINT_CTRLPTS(CubicBezierCurve* crv);
#else
#   define PRINT_CTRLPTS(X)
#endif

#define SET_PT2(V, V1, V2) do { (V)[0] = (V1); (V)[1] = (V2); } while (0)

void evaluate(const CubicBezierCurve *curve, const REAL t, Point value);

#endif /* _CURVE_H_ */
