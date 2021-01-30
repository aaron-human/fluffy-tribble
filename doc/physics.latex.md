
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
