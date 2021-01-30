
## linear_whole_system
```latex
\begin{cases}
m_1 * \overrightarrow{v_1} + m_2 * \overrightarrow{v_2} = m_1 * \overrightarrow{v_1}' + m_2 * \overrightarrow{v_2}' \\
\frac{m_1 * s_1^2}{2} + \frac{m_2 * s_2^2}{2} = \frac{m_1 * s_1'^2}{2} + \frac{m_2 * s_2'^2}{2}
\end{cases}
```

## linear_velocities
```latex
\begin{cases}
\overrightarrow{v_1}' = \overrightarrow{v_1} + mag_1 * \widehat{n} \\
\overrightarrow{v_2}' = \overrightarrow{v_2} - mag_2 * \widehat{n} \\
\end{cases}
```

## linear_momentum_eqn1
```latex
m_1 * \overrightarrow{v_1} + m_2 * \overrightarrow{v_2} &=& m_1 * \overrightarrow{v_1}' + m_2 * \overrightarrow{v_2}' \\
\vspace{10} \\
m_1 * \overrightarrow{v_1} + m_2 * \overrightarrow{v_2} &=& m_1 * (\overrightarrow{v_1} + mag_1 * \widehat{n}) + m_2 * (\overrightarrow{v_2} - mag_2 * \widehat{n}) \\
\vspace{10} \\
m_1 * \overrightarrow{v_1} + m_2 * \overrightarrow{v_2} &=& m_1 * \overrightarrow{v_1} + m_1 * mag_1 * \widehat{n} + m_2 * \overrightarrow{v_2} -  m_2 * mag_2 * \widehat{n} \\
\vspace{10} \\
0 &=& m_1 * mag_1 * \widehat{n} -  m_2 * mag_2 * \widehat{n} \\
\vspace{10} \\
0 &=& m_1 * mag_1 -  m_2 * mag_2 \\
\vspace{10} \\
m_1 * mag_1 &=& m_2 * mag_2
```

## linear_velocities_updated
```latex
\begin{cases}
\overrightarrow{v_1}' = \overrightarrow{v_1} + m_1^{-1} * mag * \widehat{n} \\
\overrightarrow{v_2}' = \overrightarrow{v_2} - m_2^{-1} * mag * \widehat{n} \\
\end{cases}
```

## linear_energy_eqn1
```latex
\frac{m_1 * s_1^2}{2} + \frac{m_2 * s_2^2}{2} = \frac{m_1 * s_1'^2}{2} + \frac{m_2 * s_2'^2}{2}
```

## linear_impulse_eqn1
```latex
\begin{cases}
\overrightarrow{v_1}'  = \overrightarrow{v_1} + mag * \widehat{n} \\
\overrightarrow{v_2}' = \overrightarrow{v_2} - mag * \widehat{n} \\
\end{cases}
```

## linear_impulse_along_normal
```latex
\begin{flushleft}
\begin{cases}
\overrightarrow{v_1}'  = \overrightarrow{v_1} + mag * \widehat{n} \\
\overrightarrow{v_2}' = \overrightarrow{v_2} - mag * \widehat{n} \\
\end{cases}\\
\vspace{10} \\
\begin{cases}
\overrightarrow{v_1}' \bullet \widehat{n} = (\overrightarrow{v_1} + mag * \widehat{n}) \bullet \widehat{n} \\
\overrightarrow{v_2}' \bullet \widehat{n} = (\overrightarrow{v_2} - mag * \widehat{n}) \bullet \widehat{n} \\
\end{cases} \\
\vspace{10} \\
\begin{cases}
\overrightarrow{v_1}' \bullet \widehat{n} = \overrightarrow{v_1} \bullet \widehat{n} + mag \\
\overrightarrow{v_2}' \bullet \widehat{n} = \overrightarrow{v_2} \bullet \widehat{n} - mag \\
\end{cases} \\
\end{flushleft}
```

## linear_energy_solve_part1
```latex
\frac{m_1 * s_1^2}{2} + \frac{m_2 * s_2^2}{2} &=& \frac{m_1 * s_1'^2}{2} + \frac{m_2 * s_2'^2}{2} \\
\vspace{10} \\
m_1 * s_1^2 + m_2 * s_2^2 &=& m_1 * s_1'^2 + m_2 * s_2'^2 \\
\vspace{10} \\
m_1 * (\overrightarrow{v_1} \bullet \widehat{n})^2 + m_2 * (\overrightarrow{v_2} \bullet \widehat{n})^2 &=& m_1 * (\overrightarrow{v_1}' \bullet \widehat{n})^2 + m_2 * (\overrightarrow{v_2}' \bullet \widehat{n})^2 \\
\vspace{10} \\
m_1 * (\overrightarrow{v_1} \bullet \widehat{n})^2 + m_2 * (\overrightarrow{v_2} \bullet \widehat{n})^2 &=& m_1 * (\overrightarrow{v_1} \bullet \widehat{n} + mag)^2 + m_2 * (\overrightarrow{v_2} \bullet \widehat{n} - mag)^2 \\
\vspace{10}
```

## linear_energy_solve_part2
```latex
m_k * (\overrightarrow{v_k} \bullet \widehat{n})^2 + ... &=& m_k * (\overrightarrow{v_k} \bullet \widehat{n} \pm mag)^2 + ... \\
\vspace{10} \\
m_k * (\overrightarrow{v_k} \bullet \widehat{n})^2 + ... &=& m_k * [(\overrightarrow{v_k} \bullet \widehat{n})^2 \pm 2 * mag * (\overrightarrow{v_k} \bullet \widehat{n}) + mag^2] + ... \\
\vspace{10} \\
0 + ... &=& m_k * [\pm 2 * mag * (\overrightarrow{v_k} \bullet \widehat{n}) + mag^2] + ...
```

## linear_energy_solve_part3
```latex
0 &=& m_1 * (+2 * mag * (\overrightarrow{v_1} \bullet \widehat{n}) + mag^2) + m_2 * (-2 * mag * (\overrightarrow{v_2} \bullet \widehat{n}) + mag^2) \\
\vspace{10} \\
0 &=& [m_1 * (2 (\overrightarrow{v_1} \bullet \widehat{n}) + mag) + m_2 * (-2 * (\overrightarrow{v_2} \bullet \widehat{n}) + mag)] * mag
```

## linear_energy_solve_part4
```latex
0 &=& [m_1 * (2 (\overrightarrow{v_1} \bullet \widehat{n}) + mag) + m_2 * (-2 * (\overrightarrow{v_2} \bullet \widehat{n}) + mag)] * mag \\
\vspace{10} \\
0 &=& m_1 * (2 (\overrightarrow{v_1} \bullet \widehat{n}) + mag) + m_2 * (-2 * (\overrightarrow{v_2} \bullet \widehat{n}) + mag) \\
\vspace{10} \\
0 &=& 2 * m_1 * (\overrightarrow{v_1} \bullet \widehat{n}) + m_1 * mag - 2 * m_2 * (\overrightarrow{v_2} \bullet \widehat{n}) + m_2 * mag \\
\vspace{10} \\
0 &=& 2 * (m_1 * (\overrightarrow{v_1} \bullet \widehat{n}) - m_2 * (\overrightarrow{v_2} \bullet \widehat{n})) + (m_2 +m_1) * mag \\
\vspace{10} \\
-(m_2 +m_1) * mag &=& 2 * (m_1 * (\overrightarrow{v_1} \bullet \widehat{n}) - m_2 * (\overrightarrow{v_2} \bullet \widehat{n})) \\
\vspace{10} \\
mag &=& \frac{-2 * (m_1 * (\overrightarrow{v_1} \bullet \widehat{n}) - m_2 * (\overrightarrow{v_2} \bullet \widehat{n}))}{m_2 +m_1}
```