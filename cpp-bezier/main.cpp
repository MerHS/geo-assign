#include "curve.h"
#include <GL/glut.h>

CubicBezierCurve curve;
GLsizei width = 640, height = 480;
int edit_ctrlpts_idx = -1;
bool isDrawControlMesh = true;
bool isDottedLine = false;

void init() {
    SET_PT2(curve.control_pts[0], 50, 100);
    SET_PT2(curve.control_pts[1], 200, 300);
    SET_PT2(curve.control_pts[2], 400, 300);
    SET_PT2(curve.control_pts[3], 550, 100);

    glClearColor(1.0, 1.0, 1.0, 1.0);
    glMatrixMode(GL_PROJECTION);
    gluOrtho2D(0, width, 0, height);
}

void reshape_callback(GLint nw, GLint nh) {
    width = nw;
    height = nh;
    glViewport(0, 0, width, height);
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    gluOrtho2D(0, width, 0, height);
}

void draw_arc(Point *center, Point *start, Point *end) {}

void display_callback() {
#define RES 100
    /* curve */
    glClear(GL_COLOR_BUFFER_BIT);
    glColor3ub(0, 0, 0);
    if (isDottedLine)
        glBegin(GL_LINES);
    else
        glBegin(GL_LINE_STRIP);
    for (int i = 0; i <= RES; i++) {
        Point pt;
        const REAL t = (REAL)i / (REAL)RES;
        curve.calc_bezier_to(t, pt);
        glVertex2f(pt[0], pt[1]);
    }
    glEnd();

    /* control mesh */
    if (isDrawControlMesh) {
        glColor3ub(255, 0, 0);
        glBegin(GL_LINE_STRIP);
        for (int i = 0; i < 4; i++) {
            REAL *pt = curve.control_pts[i];
            glVertex2f(pt[0], pt[1]);
        }
        glEnd();
    }

    /* control pts */
    glColor3ub(0, 0, 255);
    glPointSize(10.0);
    glBegin(GL_POINTS);
    for (int i = 0; i < 4; i++) {
        REAL *pt = curve.control_pts[i];
        glVertex2f(pt[0], pt[1]);
    }

    /// IMPLEMENTED <START>

    //

    /// IMPLEMENTED <END>

    glEnd();
    glutSwapBuffers();
}

// void glutMouseFunc(void (*func)(int button, int state, int x, int y));
void mouse_callback(GLint button, GLint action, GLint x, GLint y) {
    if (GLUT_LEFT_BUTTON == button) {
        switch (action) {
        case GLUT_DOWN:
            edit_ctrlpts_idx = curve.hit_index(x, height - y);
            break;
        case GLUT_UP:
            edit_ctrlpts_idx = -1;
            break;
        default:
            break;
        }
    }
    glutPostRedisplay();
}

// void glutMotionFunc(void (*func)(int x, int y));
void mouse_move_callback(GLint x, GLint y) {
    if (edit_ctrlpts_idx != -1) {
        curve.control_pts[edit_ctrlpts_idx][0] = x;
        curve.control_pts[edit_ctrlpts_idx][1] = height - y;
    }
    glutPostRedisplay();
}

// void glutKeyboardFunc(void (*func)(unsigned char key, int x, int y));
void keyboard_callback(unsigned char key, int x, int y) {
    switch (key) {
    case 'i':
    case 'I':
        SET_PT2(curve.control_pts[0], 50, 100);
        SET_PT2(curve.control_pts[1], 200, 300);
        SET_PT2(curve.control_pts[2], 400, 300);
        SET_PT2(curve.control_pts[3], 550, 100);
        break;
    case 'l':
    case 'L':
        isDottedLine ^= true;
        break;
    case 'c':
    case 'C':
        isDrawControlMesh ^= true;
        break;
    case '1':
    case '2':
    case '3':
    case '4':
        break;
    default:
        break;
    }
    glutPostRedisplay();
}

int main(int argc, char *argv[]) {
    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGBA);
    glutInitWindowSize(width, height);
    glutCreateWindow("Bezier Editor");

    init();
    glutReshapeFunc(reshape_callback);
    glutMouseFunc(mouse_callback);
    glutMotionFunc(mouse_move_callback);
    glutDisplayFunc(display_callback);
    glutKeyboardFunc(keyboard_callback);
    glutMainLoop();

    return 0;
}
