import React, { useEffect, useState } from 'react';
import { configService } from '../services/configService';

const ConfigViewer = () => {
    const [units, setUnits] = useState([]);
    const [counters, setCounters] = useState([]);
    const [costTypes, setCostTypes] = useState([]);
    const [heizungParams, setHeizungParams] = useState({});
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const [unitsData, countersData, costTypesData, heizungParamsData] = await Promise.all([
                    configService.getUnits(),
                    configService.getCounters(),
                    configService.getCostTypes(),
                    configService.getHeizungParams()
                ]);

                setUnits(unitsData);
                setCounters(countersData);
                setCostTypes(costTypesData);
                setHeizungParams(heizungParamsData);
                setLoading(false);
            } catch (err) {
                setError('Failed to fetch configuration data');
                setLoading(false);
            }
        };

        fetchData();
    }, []);

    if (loading) return <div>Loading configuration...</div>;
    if (error) return <div>{error}</div>;

    return (
        <div>
            <h2>Configuration Viewer</h2>

            <h3>Units</h3>
            <table>
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>Name</th>
                        <th>Real Living Area</th>
                        <th>Heating Area</th>
                        <th>Persons</th>
                    </tr>
                </thead>
                <tbody>
                    {units.map((unit, index) => (
                        <tr key={index}>
                            <td>{unit.id}</td>
                            <td>{unit.name}</td>
                            <td>{unit.wohnflaeche_real}</td>
                            <td>{unit.wohnflaeche_heizung}</td>
                            <td>{unit.personen}</td>
                        </tr>
                    ))}
                </tbody>
            </table>

            <h3>Counters</h3>
            <table>
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>Name</th>
                        <th>Unit</th>
                        <th>Type</th>
                    </tr>
                </thead>
                <tbody>
                    {counters.map((counter, index) => (
                        <tr key={index}>
                            <td>{counter.id}</td>
                            <td>{counter.name}</td>
                            <td>{counter.unit}</td>
                            <td>{counter.type_}</td>
                        </tr>
                    ))}
                </tbody>
            </table>

            <h3>Cost Types</h3>
            <table>
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>Name</th>
                        <th>Allocation Logic</th>
                    </tr>
                </thead>
                <tbody>
                    {costTypes.map((costType, index) => (
                        <tr key={index}>
                            <td>{costType.id}</td>
                            <td>{costType.name}</td>
                            <td>{costType.umlage_logik_ref}</td>
                        </tr>
                    ))}
                </tbody>
            </table>

            <h3>Heizung Parameters</h3>
            <p>Ölverbrauch pro Stunde Stufe 1: {heizungParams.oel_verbrauch_pro_stunde_stufe1}</p>
            <p>Ölverbrauch pro Stunde Stufe 2: {heizungParams.oel_verbrauch_pro_stunde_stufe2}</p>
        </div>
    );
};

export default ConfigViewer;