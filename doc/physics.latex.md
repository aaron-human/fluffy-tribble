
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

## linear_velocities_along_normal
```latex
&& \begin{cases}
\overrightarrow{v_1}' \bullet \widehat{n} = (\overrightarrow{v_1} + m_1^{-1} * mag * \widehat{n}) \bullet \widehat{n} \\
\overrightarrow{v_2}' \bullet \widehat{n} = (\overrightarrow{v_2} + m_2^{-1} * mag * \widehat{n}) \bullet \widehat{n} \\
\end{cases} \\
\vspace{10} \\
&& \begin{cases}
\overrightarrow{v_1}' \bullet \widehat{n} = \overrightarrow{v_1} \bullet \widehat{n} + m_1^{-1} * mag \\
\overrightarrow{v_2}' \bullet \widehat{n} = \overrightarrow{v_2} \bullet \widehat{n} + m_2^{-1} * mag \\
\end{cases}
```

## linear_energy_eqn1
```latex
\frac{m_1 * s_1^2}{2} + \frac{m_2 * s_2^2}{2} = \frac{m_1 * s_1'^2}{2} + \frac{m_2 * s_2'^2}{2}
```

## linear_energy_solve_part1
```latex
\frac{m_1 * s_1^2}{2} + \frac{m_2 * s_2^2}{2} &=& \frac{m_1 * s_1'^2}{2} + \frac{m_2 * s_2'^2}{2} \\
\vspace{10} \\
m_1 * s_1^2 + m_2 * s_2^2 &=& m_1 * s_1'^2 + m_2 * s_2'^2 \\
\vspace{10} \\
m_1 * (\overrightarrow{v_1} \bullet \widehat{n})^2 + m_2 * (\overrightarrow{v_2} \bullet \widehat{n})^2 &=& m_1 * (\overrightarrow{v_1}' \bullet \widehat{n})^2 + m_2 * (\overrightarrow{v_2}' \bullet \widehat{n})^2 \\
\vspace{10} \\
m_1 * (\overrightarrow{v_1} \bullet \widehat{n})^2 + m_2 * (\overrightarrow{v_2} \bullet \widehat{n})^2 &=& m_1 * (\overrightarrow{v_1} \bullet \widehat{n} + m_1^{-1} * mag)^2 + m_2 * (\overrightarrow{v_2} \bullet \widehat{n} - m_2^{-1} * mag)^2
```

## linear_energy_solve_part2
```latex
m_k * (\overrightarrow{v_k} \bullet \widehat{n})^2 + ... &=& m_k * (\overrightarrow{v_k} \bullet \widehat{n} \pm m_k^{-1} * mag)^2 + ... \\
\vspace{10} \\
m_k * (\overrightarrow{v_k} \bullet \widehat{n})^2 + ... &=& m_k * [(\overrightarrow{v_k} \bullet \widehat{n})^2 \pm 2 * m_k^{-1} * mag * (\overrightarrow{v_k} \bullet \widehat{n}) + m_k^{-2} * mag^2] + ... \\
\vspace{10} \\
0 + ... &=& m_k * [\pm 2 * m_k^{-1} * mag * (\overrightarrow{v_k} \bullet \widehat{n}) + * m_k^{-2} * mag ^2] + ... \\
\vspace{10} \\
0 + ... &=& \pm 2 * mag * (\overrightarrow{v_k} \bullet \widehat{n}) + * m_k^{-1} * mag ^2 + ...
```

## linear_energy_solve_part3
```latex
0 &=& (+2 * mag * (\overrightarrow{v_1} \bullet \widehat{n}) + m_1^{-1} * mag^2) + (-2 * mag * (\overrightarrow{v_2} \bullet \widehat{n}) + m_2^{-1} * mag^2) \\
\vspace{10} \\
0 &=& [2 * (\overrightarrow{v_1} \bullet \widehat{n}) + m_1^{-1} * mag - 2 * (\overrightarrow{v_2} \bullet \widehat{n}) + m_2^{-1} * mag] * mag
```

## linear_energy_solve_part4
```latex
0 &=& [2 * (\overrightarrow{v_1} \bullet \widehat{n}) + m_1^{-1} * mag - 2 * (\overrightarrow{v_2} \bullet \widehat{n}) + m_2^{-1} * mag] * mag \\
\vspace{10} \\
0 &=& 2 * (\overrightarrow{v_1} \bullet \widehat{n}) + m_1^{-1} * mag - 2 * (\overrightarrow{v_2} \bullet \widehat{n}) + m_2^{-1} * mag \\
\vspace{10} \\
0 &=& 2[(\overrightarrow{v_1} \bullet \widehat{n}) - (\overrightarrow{v_2} \bullet \widehat{n})] + (m_1^{-1} + m_2^{-1}) * mag \\
\vspace{10} \\
-(m_1^{-1} + m_2^{-1}) * mag &=& 2[(\overrightarrow{v_1} \bullet \widehat{n}) - (\overrightarrow{v_2} \bullet \widehat{n})] \\
\vspace{10} \\
-(m_1^{-1} + m_2^{-1}) * mag &=& 2(\overrightarrow{v_1} - \overrightarrow{v_2}) \bullet \widehat{n} \\
\vspace{10} \\
mag &=& \frac{-2(\overrightarrow{v_1} - \overrightarrow{v_2}) \bullet \widehat{n}}{m_1^{-1} + m_2^{-1}}
```

## linear_restitution_solve
```latex
\overrightarrow{0} &=& \overrightarrow{v_1}' \bullet \widehat{n} - \overrightarrow{v_2}' \bullet \widehat{n} \\
\vspace{10} \\
\overrightarrow{0} &=& (\overrightarrow{v_1} \bullet \widehat{n} + m_1^{-1} * mag) - (\overrightarrow{v_2} \bullet \widehat{n} - m_2^{-1} * mag) \\
\vspace{10} \\
\overrightarrow{0} &=& (\overrightarrow{v_1} \bullet \widehat{n} - \overrightarrow{v_2} \bullet \widehat{n}) + (m_1^{-1} + m_2^{-1}) * mag \\
\vspace{10} \\
\overrightarrow{0} &=& (\overrightarrow{v_1} - \overrightarrow{v_2}) \bullet \widehat{n} + (m_1^{-1} + m_2^{-1}) * \frac{-2(\overrightarrow{v_1} - \overrightarrow{v_2}) \bullet \widehat{n}}{m_1^{-1} + m_2^{-1}} \\
\vspace{10} \\
\overrightarrow{0} &=& (\overrightarrow{v_1} - \overrightarrow{v_2}) \bullet \widehat{n} - 2(\overrightarrow{v_1} - \overrightarrow{v_2}) \bullet \widehat{n}
```

## linear_final_mag
```latex
\begin{cases}
mag = \frac{-(1 + restitution\_cofficient)(\overrightarrow{v_1} - \overrightarrow{v_2}) \bullet \widehat{n}}{m_1^{-1} + m_2^{-1}} \\
\overrightarrow{v_1}' = \overrightarrow{v_1} + m_1^{-1} * mag * \widehat{n} \\
\overrightarrow{v_2}' = \overrightarrow{v_2} - m_2^{-1} * mag * \widehat{n} \\
\end{cases}
```

## angular_momentum_conserved
```latex
I_1 * \overrightarrow{\omega}_1 + I_2 * \overrightarrow{\omega}_2 = I_1 * \overrightarrow{\omega}_1' + I_2 * \overrightarrow{\omega}_2'
```

## angular_momenum_changes
```latex
\begin{cases}
\overrightarrow{\omega}_1' = \overrightarrow{\omega}_1 + \Delta_1 * \widehat{n} \times \widehat{p}_1 \\
\overrightarrow{\omega}_2' = \overrightarrow{\omega}_2 - \Delta_2 * \widehat{n} \times \widehat{p}_2
\end{cases}
```

## angular_momentum_distributed
```latex
I_1 * \overrightarrow{\omega}_1 + I_2 * \overrightarrow{\omega}_2 &=& I_1 * \overrightarrow{\omega}_1' + I_2 * \overrightarrow{\omega}_2' \\
I_1 * \overrightarrow{\omega}_1 + I_2 * \overrightarrow{\omega}_2 &=& I_1 * (\overrightarrow{\omega}_1 + \Delta_1 * \widehat{n} \times \widehat{p}_1) + I_2 * (\overrightarrow{\omega}_2 - \Delta_2 * \widehat{n} \times \widehat{p}_2) \\
\overrightarrow{0} &=& I_1 * \Delta_1 * \widehat{n} \times \widehat{p}_1 - I_2 * \Delta_2 * \widehat{n} \times \widehat{p}_2 \\
I_1 * \Delta_1 * \widehat{n} \times \widehat{p}_1 &=& I_2 * \Delta_2 * \widehat{n} \times \widehat{p}_2
```

## angular_momenum_changes_updated
```latex
\begin{cases}
\overrightarrow{\omega}_1' = \overrightarrow{\omega}_1 + I_1^{-1} * \Delta * \widehat{n} \times \widehat{p}_1 \\
\overrightarrow{\omega}_2' = \overrightarrow{\omega}_2 - I_2^{-1} * \Delta * \widehat{n} \times \widehat{p}_2 
\end{cases}
```
