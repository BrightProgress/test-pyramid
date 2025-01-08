% filepath: /home/prahlad/workspace/mat-lab/+testquality/plot_metrics.m
% Load the CSV data
data = readtable('test_pyramid_metrics.csv');

% Extract columns
src_size = data{:, 1};
test_size = data{:, 2};
unit_test_percentage = data{:, 3};
system_test_percentage = data{:, 4};
efficiency_mean = data{:, 5};
efficiency_min = data{:, 6};
efficiency_max = data{:, 7};
efficiency_std = data{:, 8};
defect_detection_mean = data{:, 9};
defect_detection_min = data{:, 10};
defect_detection_max = data{:, 11};
defect_detection_std = data{:, 12};
defect_localization_mean = data{:, 13};
defect_localization_min = data{:, 14};
defect_localization_max = data{:, 15};
defect_localization_std = data{:, 16};

% Filter data to include only source sizes up to 10 components
filter_idx = src_size <= 10;
src_size = src_size(filter_idx);
test_size = test_size(filter_idx);
unit_test_percentage = unit_test_percentage(filter_idx);
system_test_percentage = system_test_percentage(filter_idx);
efficiency_mean = efficiency_mean(filter_idx);
efficiency_min = efficiency_min(filter_idx);
efficiency_max = efficiency_max(filter_idx);
efficiency_std = efficiency_std(filter_idx);
defect_detection_mean = defect_detection_mean(filter_idx);
defect_detection_min = defect_detection_min(filter_idx);
defect_detection_max = defect_detection_max(filter_idx);
defect_detection_std = defect_detection_std(filter_idx);
defect_localization_mean = defect_localization_mean(filter_idx);
defect_localization_min = defect_localization_min(filter_idx);
defect_localization_max = defect_localization_max(filter_idx);
defect_localization_std = defect_localization_std(filter_idx);

% Define unique system sizes
unique_src_sizes = unique(src_size);

% Select min, mid, and max values for system size
selected_src_sizes = unique_src_sizes([1, round(end/2), end]);

% Plot efficiency
figure;
for i = 1:length(selected_src_sizes)
    selected_test_sizes = round([2, 4, 5] .* selected_src_sizes(i) / 5) * 5;
    for j = 1:length(selected_test_sizes)
        subplot(3, 3, (i-1)*3 + j);
        idx = src_size == selected_src_sizes(i) & test_size == selected_test_sizes(j);
        [X, Y] = ndgrid(unique(unit_test_percentage(idx)), unique(system_test_percentage(idx)));
        Z_efficiency = griddata(unit_test_percentage(idx), system_test_percentage(idx), efficiency_mean(idx), X, Y, 'cubic');
        surf(X, Y, Z_efficiency, 'EdgeColor', 'none');
        colormap(parula);
        colorbar;
        title(['Efficiency: Src Size = ', num2str(selected_src_sizes(i)), ', Test Size = ', num2str(selected_test_sizes(j))]);
        xlabel('Unit Test Percentage');
        ylabel('System Test Percentage');
        zlabel('Efficiency Mean');
    end
end

% Plot defect detection
figure;
for i = 1:length(selected_src_sizes)
    selected_test_sizes = round([2, 4, 5] .* selected_src_sizes(i) / 5) * 5;
    for j = 1:length(selected_test_sizes)
        subplot(3, 3, (i-1)*3 + j);
        idx = src_size == selected_src_sizes(i) & test_size == selected_test_sizes(j);
        [X, Y] = ndgrid(unique(unit_test_percentage(idx)), unique(system_test_percentage(idx)));
        Z_defect_detection = griddata(unit_test_percentage(idx), system_test_percentage(idx), defect_detection_mean(idx), X, Y, 'cubic');
        surf(X, Y, Z_defect_detection, 'EdgeColor', 'none');
        colormap(parula);
        colorbar;
        title(['Defect Detection: Src Size = ', num2str(selected_src_sizes(i)), ', Test Size = ', num2str(selected_test_sizes(j))]);
        xlabel('Unit Test Percentage');
        ylabel('System Test Percentage');
        zlabel('Defect Detection Mean');
    end
end

% Plot defect localization
figure;
for i = 1:length(selected_src_sizes)
    selected_test_sizes = round([2, 4, 5] .* selected_src_sizes(i) / 5) * 5;
    for j = 1:length(selected_test_sizes)
        subplot(3, 3, (i-1)*3 + j);
        idx = src_size == selected_src_sizes(i) & test_size == selected_test_sizes(j);
        [X, Y] = ndgrid(unique(unit_test_percentage(idx)), unique(system_test_percentage(idx)));
        Z_defect_localization = griddata(unit_test_percentage(idx), system_test_percentage(idx), defect_localization_mean(idx), X, Y, 'cubic');
        surf(X, Y, Z_defect_localization, 'EdgeColor', 'none');
        colormap(parula);
        colorbar;
        title(['Defect Localization: Src Size = ', num2str(selected_src_sizes(i)), ', Test Size = ', num2str(selected_test_sizes(j))]);
        xlabel('Unit Test Percentage');
        ylabel('System Test Percentage');
        zlabel('Defect Localization Mean');
    end
end

% Plot combined metrics with optimal regions
figure;
for i = 1:length(selected_src_sizes)
    selected_test_sizes = round([2, 4, 5] .* selected_src_sizes(i) / 5) * 5;
    for j = 1:length(selected_test_sizes)
        subplot(3, 3, (i-1)*3 + j);
        idx = src_size == selected_src_sizes(i) & test_size == selected_test_sizes(j);
        
        % Interpolate efficiency
        [X, Y] = ndgrid(unique(unit_test_percentage(idx)), unique(system_test_percentage(idx)));
        Z_efficiency = griddata(unit_test_percentage(idx), system_test_percentage(idx), efficiency_mean(idx), X, Y, 'cubic');
        
        % Interpolate defect detection
        Z_defect_detection = griddata(unit_test_percentage(idx), system_test_percentage(idx), defect_detection_mean(idx), X, Y, 'cubic');
        
        % Interpolate defect localization
        Z_defect_localization = griddata(unit_test_percentage(idx), system_test_percentage(idx), defect_localization_mean(idx), X, Y, 'cubic');
        
        % Define grid for search
        [Xq, Yq] = ndgrid(linspace(min(unit_test_percentage(idx)), max(unit_test_percentage(idx)), 100), ...
                          linspace(min(system_test_percentage(idx)), max(system_test_percentage(idx)), 100));
        
        % Interpolate metrics on the grid
        F_efficiency = griddedInterpolant(X, Y, Z_efficiency, 'cubic');
        F_defect_detection = griddedInterpolant(X, Y, Z_defect_detection, 'cubic');
        F_defect_localization = griddedInterpolant(X, Y, Z_defect_localization, 'cubic');
        
        Zq_efficiency = F_efficiency(Xq, Yq);
        Zq_defect_detection = F_defect_detection(Xq, Yq);
        Zq_defect_localization = F_defect_localization(Xq, Yq);
        
        % Find optimal region
        optimal_idx = Zq_efficiency > prctile(Zq_efficiency(:), 48) & ...
                      Zq_defect_detection > prctile(Zq_defect_detection(:), 48) & ...
                      Zq_defect_localization > prctile(Zq_defect_localization(:), 48);
        
        % Highlight optimal region
        scatter3(Xq(optimal_idx), Yq(optimal_idx), Zq_efficiency(optimal_idx), 'r', 'filled');
        hold on;
        scatter3(Xq(optimal_idx), Yq(optimal_idx), Zq_defect_detection(optimal_idx), 'g', 'filled');
        scatter3(Xq(optimal_idx), Yq(optimal_idx), Zq_defect_localization(optimal_idx), 'b', 'filled');
        
        % Draw bounding circle
        if any(optimal_idx(:))
            optimal_points = [Xq(optimal_idx), Yq(optimal_idx)];
            center = mean(optimal_points);
            radius = max(sqrt(sum((optimal_points - center).^2, 2)));
            theta = linspace(0, 2*pi, 100);
            x_circle = center(1) + radius * cos(theta);
            y_circle = center(2) + radius * sin(theta);
            plot3(x_circle, y_circle, max(Zq_efficiency(:)) * ones(size(x_circle)), 'k-', 'LineWidth', 2);
            
            % Label the minimum and maximum values of unit-test and system-test percentages
            min_unit_test = min(Xq(optimal_idx));
            max_unit_test = max(Xq(optimal_idx));
            min_system_test = min(Yq(optimal_idx));
            max_system_test = max(Yq(optimal_idx));
            text(min_unit_test, min_system_test, max(Zq_efficiency(:)), sprintf('Min: (%.2f, %.2f)', min_unit_test, min_system_test), 'VerticalAlignment', 'bottom', 'HorizontalAlignment', 'right');
            text(max_unit_test, max_system_test, max(Zq_efficiency(:)), sprintf('Max: (%.2f, %.2f)', max_unit_test, max_system_test), 'VerticalAlignment', 'bottom', 'HorizontalAlignment', 'left');
            
            % Plot the 4 extremal points on the circle
            scatter3(min_unit_test, center(2), max(Zq_efficiency(:)), 'k', 'filled');
            scatter3(max_unit_test, center(2), max(Zq_efficiency(:)), 'k', 'filled');
            scatter3(center(1), min_system_test, max(Zq_efficiency(:)), 'k', 'filled');
            scatter3(center(1), max_system_test, max(Zq_efficiency(:)), 'k', 'filled');
        end
        
        colormap(parula);
        colorbar;
        title(['Optimal Region: Src Size = ', num2str(selected_src_sizes(i)), ', Test Size = ', num2str(selected_test_sizes(j))]);
        xlabel('Unit Test Percentage');
        ylabel('System Test Percentage');
        zlabel('Metric Value');
        % xlim([0 100]);
        % ylim([0 100]);
        hold off;
    end
end